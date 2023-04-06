use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub game_number: Option<u32>,
    pub game_type: String,
    pub player_1_name: String,
    pub player_2_name: String,
    pub winner_name: String,
    pub game_date: String, //Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub player_name: String,
    pub wins: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComputerMetrics {
    pub total_games: u32,
    pub games_against_comp: u32,
    pub comp_wins: u32,
}
