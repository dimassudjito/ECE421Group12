use std::env;
//extern crate dotenv;
//use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{ InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
};
use crate::models::game_model::Game;

pub struct MongoRepo {
    col: Collection<Game>,
}

impl MongoRepo {
    pub fn init() -> Self {
        //dotenv().ok();
        /*
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };*/
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
}
