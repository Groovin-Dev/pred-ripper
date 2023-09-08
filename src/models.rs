use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PredecessorMatch {
    pub winning_team: i64,
    pub game_duration: i64,
    pub game_mode: String,
    pub match_id: String,
    pub region: String,
    pub start_time: String,
    pub end_time: String,
    pub match_end_reason: String,
    pub player_data: Vec<PlayerData>,
    pub hero_kills: Vec<HeroKill>,
    pub structure_destructions: Vec<StructureDestruction>,
    pub objective_kills: Vec<ObjectiveKill>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerData {
    pub player_id: String,
    pub team_id: i64,
    pub hero_name: String,
    pub role_name: Option<String>,
    pub player_name: Option<String>,
    pub minion_data: MinionData,
    pub combat_data: CombatData,
    pub damage_heal_data: DamageHealData,
    pub wards_data: WardsData,
    pub income_data: IncomeData,
    pub ability_data: Vec<AbilityData>,
    pub inventory_data: Option<Vec<InventoryData>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinionData {
    pub minions_killed: i64,
    pub lane_minions_killed: i64,
    pub neutral_minions_killed: i64,
    pub neutral_minions_team_jungle: i64,
    pub neutral_minions_enemy_jungle: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CombatData {
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub largest_killing_spree: i64,
    pub largest_multi_kill: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DamageHealData {
    magical_damage_taken_from_heroes: i64,
    total_damage_taken_from_heroes: i64,
    physical_damage_taken_from_heroes: i64,
    physical_damage_dealt: i64,
    physical_damage_taken: i64,
    total_damage_dealt_to_heroes: i64,
    magical_damage_dealt_to_heroes: i64,
    total_damage_dealt_to_structures: i64,
    true_damage_taken_from_heroes: i64,
    true_damage_dealt: i64,
    total_damage_dealt_to_objectives: i64,
    true_damage_taken: i64,
    total_damage_dealt: i64,
    magical_damage_taken: i64,
    magical_damage_dealt: i64,
    total_damage_taken: i64,
    physical_damage_dealt_to_heroes: i64,
    total_damage_mitigated: i64,
    true_damage_dealt_to_heroes: i64,
    largest_critical_strike: Option<i64>,
    total_healing_done: Option<i64>,
    item_healing_done: Option<i64>,
    crest_healing_done: Option<i64>,
    utility_healing_done: Option<i64>,
    total_shielding_received: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WardsData {
    pub wards_placed: i64,
    pub wards_destroyed: i64,
    pub ward_destructions: Vec<WardData>,
    pub ward_placements: Vec<WardData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WardData {
    pub type_id: i64,
    pub game_time: i64,
    pub location: Location,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncomeData {
    pub gold_earned: i64,
    pub gold_spent: i64,
    pub gold_earned_at_interval: Vec<i64>,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub item_id: i64,
    pub transaction_type: i64,
    pub game_time: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbilityData {
    pub ability_input_tag: Option<String>,
    pub ability_slot: Option<i64>,
    pub game_time: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryData {
    pub item_slot: i64,
    pub item_id: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeroKill {
    pub killed_player_id: String,
    pub killed_hero_name: String,
    pub killer_player_id: String,
    pub killer_hero_name: String,
    pub killer_entity_type: String,
    pub is_first_blood: bool,
    pub location: Location,
    pub game_time: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StructureDestruction {
    pub destruction_player_id: String,
    pub destruction_hero_name: String,
    pub structure_entity_type: String,
    pub location: Location,
    pub team_id: i64,
    pub game_time: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveKill {
    pub killed_entity_type: String,
    pub killer_player_id: String,
    pub killer_hero_name: String,
    pub location: Location,
    pub game_time: i64,
}
