use std::collections::HashMap;
use std::env;

use crate::models::game_model::ComputerMetrics;
use crate::models::game_model::Game;
use crate::models::game_model::Player;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<Game>,
}

impl MongoRepo {
    pub fn init() -> Self {
        let uri = String::from("mongodb://localhost:27017");
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<Game> = db.collection("Game");
        MongoRepo { col }
    }

    pub fn create_game(&self, new_game: Game) -> Result<InsertOneResult, Error> {
        let new_doc = Game {
            id: None,
            game_number: new_game.game_number,
            game_type: new_game.game_type,
            player_1_name: new_game.player_1_name,
            player_2_name: new_game.player_2_name,
            winner_name: new_game.winner_name,
            game_date: new_game.game_date,
        };
        let game = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating game");
        Ok(game)
    }

    pub fn get_game(&self, id: &String) -> Result<Game, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let game_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting game's detail");
        Ok(game_detail.unwrap())
    }

    pub fn get_next_game_num(&self) -> u32 {
        let games = self.get_all_games().unwrap_or_else(|_| vec![]); // Use unwrap_or_else to handle errors
        let mut max_game_num = 0;

        for game in games {
            match game.game_number {
                Some(x) => {
                    if x > max_game_num {
                        max_game_num = x;
                    }
                }
                _ => {
                    continue;
                }
            }
        }
        max_game_num + 1
    }

    pub fn update_game(&self, id: &String, new_game: Game) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_game.id,
                    "game_number": new_game.game_number,
                    "game_type": new_game.game_type,
                    "player_1_name": new_game.player_1_name,
                    "player_2_name": new_game.player_2_name,
                    "winner_name": new_game.winner_name,
                    "game_date": new_game.game_date
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating game");
        Ok(updated_doc)
    }

    pub fn delete_game(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let game_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting game");
        Ok(game_detail)
    }

    pub fn get_all_games(&self) -> Result<Vec<Game>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of games");
        let games = cursors.map(|doc| doc.unwrap()).collect();
        Ok(games)
    }

    pub fn get_computer_wins(&self) -> Result<Vec<Game>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of games");
        let games: Vec<Game> = cursors.map(|doc| doc.unwrap()).collect();
        let compGames = games
            .into_iter()
            .filter(|game| game.player_2_name == "Computer" && game.winner_name == "Computer")
            .collect();
        Ok(compGames)
    }

    pub fn get_rankings(&self) -> Result<Vec<Player>, Error> {
        let games = self.get_all_games()?;
        let mut win_count = HashMap::new();

        for game in games {
            // An empty player name signifies the game was a tie, so ignore it as nobody won.
            if game.winner_name.is_empty() {
                continue;
            }

            *win_count.entry(game.winner_name.clone()).or_insert(0) += 1;
        }

        let mut rankings: Vec<Player> = win_count
            .into_iter()
            .map(|(player_name, wins)| Player { player_name, wins })
            .collect();

        rankings.sort_by(|a, b| b.wins.cmp(&a.wins));

        Ok(rankings)
    }

    pub fn get_computer_metrics(&self) -> Result<ComputerMetrics, Error> {
        let games = self.get_all_games()?;
        let mut wins: u32 = 0;
        let mut compGames: u32 = 0;

        for game in &games {
            // An empty player name signifies the game was a tie, so ignore it as nobody won.
            if game.winner_name.is_empty() {
                continue;
            }

            if &game.player_2_name == "Computer" {
                if &game.winner_name == "Computer" {
                    wins = wins + 1;
                }
                compGames = compGames + 1;
            }
        }

        let mut compMetric = ComputerMetrics {
            total_games: games.len() as u32,
            games_against_comp: compGames,
            comp_wins: wins,
        };
        Ok(compMetric)
    }
}
