use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Score {
    pub player_name: String,
    pub wins: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Game {
    pub game_number: i32,
    pub game_type: String,
    pub player_1_name: String,
    pub player_2_name: String,
    pub winner_name: String,
    pub game_date: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CompMetric {
    pub total_games: u32,
    pub games_against_comp: u32,
    pub comp_wins: u32,
}
