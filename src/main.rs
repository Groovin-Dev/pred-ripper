use std::{
    error::Error,
    fs::{create_dir_all, remove_dir_all, File},
    io::{self, ErrorKind},
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use chrono::NaiveDateTime;
use models::PredecessorMatch;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tracing::{info, warn};
use walkdir::WalkDir;
use zip::write::FileOptions;

mod models;

const BASE_URL: &str = "https://backend.production.omeda-aws.com/api/public/get-matches-since";
const FIRST_EPOCH: u64 = 1669882894; // Thursday, December 1, 2022 08:21:34 AM GMT
const WINDOW_SIZE: u64 = 3600; // 1 hour
const POOL_SIZE: u64 = 10;

//#region Work Window

#[derive(Debug, Clone)]
struct WorkWindow {
    start_epoch: u64,
    end_epoch: u64,
}

fn generate_work_window(starting_epoch: u64) -> WorkWindow {
    WorkWindow {
        start_epoch: starting_epoch,
        end_epoch: starting_epoch + WINDOW_SIZE,
    }
}

fn generate_work_windows(starting_epoch: u64) -> Vec<WorkWindow> {
    let mut work_windows: Vec<WorkWindow> = Vec::new();
    let mut starting_epoch = starting_epoch;
    let now = chrono::Utc::now().timestamp() as u64;
    loop {
        let work_window = generate_work_window(starting_epoch);
        if work_window.end_epoch < now {
            work_windows.push(work_window.clone());
            starting_epoch = work_window.end_epoch;
        } else {
            break;
        }
    }
    work_windows
}

//#endregion

//#region Request

fn get_matches_since(epoch: u64) -> Result<Vec<PredecessorMatch>, Box<dyn Error>> {
    let url = format!("{}/{}", BASE_URL, epoch);
    let response = reqwest::blocking::Client::new().get(&url).send()?;

    if response.status().is_success() {
        let matches: Vec<PredecessorMatch> = response.json()?;
        Ok(matches)
    } else {
        Err(Box::new(std::io::Error::new(
            ErrorKind::Other,
            format!("Error getting matches for epoch {}", epoch),
        )))
    }
}

//#endregion

//#region Helpers

fn human_to_unix_epoch(human_time: &str) -> u64 {
    let dt = NaiveDateTime::parse_from_str(human_time, "%Y-%m-%d %H:%M:%S").unwrap();
    dt.timestamp() as u64
}

fn setup_ctrl_c_handler() -> Arc<AtomicBool> {
    let ctrl_c_received = Arc::new(AtomicBool::new(false));
    let ctrl_c_received_clone = ctrl_c_received.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("Ctrl-C received. Finishing open requests and exiting...");
        ctrl_c_received_clone.store(true, Ordering::Relaxed);
    });
    ctrl_c_received
}

fn save_matches(matches: Vec<PredecessorMatch>) -> Result<(), Box<dyn Error>> {
    let first_match_endtime_epoch = human_to_unix_epoch(&matches.first().unwrap().end_time);
    let last_match_endtime_epoch = human_to_unix_epoch(&matches.last().unwrap().end_time);

    let file_name = format!(
        "matches/{}-{}.json",
        first_match_endtime_epoch, last_match_endtime_epoch
    );

    let file = std::fs::File::create(file_name)?;
    serde_json::to_writer(file, &matches)?;

    info!(
        "Saved {} matches for {} to {}",
        matches.len(),
        first_match_endtime_epoch,
        last_match_endtime_epoch
    );

    Ok(())
}

fn zip_matches() -> Result<(), Box<dyn Error>> {
    let match_count = WalkDir::new("matches")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count();

    info!("Zipping {} matches", match_count);

    let path = "matches";
    let output_file = File::create("matches.zip")?;
    let mut zip = zip::ZipWriter::new(output_file);

    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(Path::new("matches"))?;

        if path.is_file() {
            info!("Adding file: {:?}", name);
            zip.start_file(name.to_str().unwrap(), FileOptions::default())?;
            let mut f = File::open(path)?;
            io::copy(&mut f, &mut zip)?;
        }
    }

    zip.finish()?;

    info!("Finished zipping matches");
    Ok(())
}

//#endregion

//#region Loop

fn get_matches_for_work_window(
    work_window: &WorkWindow,
    ctrl_c_received: Arc<AtomicBool>,
) -> Result<(), Box<dyn Error>> {
    let mut current_epoch = work_window.start_epoch;

    info!("Getting matches for work window: {:?}", work_window);

    loop {
        // If we received a ctrl-c, stop the loop
        if ctrl_c_received.load(Ordering::Relaxed) {
            break;
        }

        // Get the matches for the current epoch. Absolutly do NOT continue until we get the matches
        let matches = get_matches_since(current_epoch);

        if matches.is_ok() {
            let matches = matches.unwrap();
            if matches.len() > 0 {
                info!(
                    "Work window: {:?} has {} matches",
                    work_window,
                    matches.len()
                );

                save_matches(matches.clone())?;
                current_epoch = human_to_unix_epoch(&matches.last().unwrap().end_time);
            } else {
                warn!("No matches found for epoch {}", current_epoch);
                break;
            }
        } else {
            warn!("Error getting matches for epoch {}", current_epoch);

            // Debugging: Print the error
            println!("{:?}", matches.err());

            break;
        }
    }

    Ok(())
}

// #endregion

//#region Main

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    // Check if the matches folder exists
    if std::path::Path::new("matches").exists() {
        remove_dir_all("matches")?;
    }
    create_dir_all("matches")?;

    let ctrl_c_received = setup_ctrl_c_handler();

    // Generate the work windows
    let work_windows = generate_work_windows(FIRST_EPOCH);
    info!("Generated {} work windows", work_windows.len());

    // Create the thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(POOL_SIZE as usize)
        .build()
        .unwrap();

    // Tell the thread pool to execute the work windows
    // Only continue once the get_matches_for_work_window function has finished
    // Once that function has finished, the thread will be returned to the pool
    // We not only pass the ctrl_c_received Arc to the thread, but we use it in the parallel iterator to check if we should continue
    // We do this so the parallel iterator doesn't start a new thread if we received a ctrl-c
    pool.install(|| {
        work_windows.par_iter().for_each(|work_window| {
            if !ctrl_c_received.load(Ordering::Relaxed) {
                get_matches_for_work_window(work_window, ctrl_c_received.clone()).unwrap();
            }
        });
    });

    // Zip the matches
    zip_matches()?;

    Ok(())
}

//#endregion
