use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub game_number: String,
    pub game_type: String,
    pub player_1_name: String,
    pub player_2_name: String,
    pub winner_name: String,
    pub game_date: String,//Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub player_name: String,
    pub wins: u32,
}
