use crate::{
    models::game_model::ComputerMetrics, models::game_model::Game, models::game_model::Player,
    repository::mongodb_repo::MongoRepo,
};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/game", data = "<new_game>")]
pub fn create_game(
    db: &State<MongoRepo>,
    new_game: Json<Game>,
) -> Result<Json<InsertOneResult>, Status> {
    let next_game_num = db.get_next_game_num();
    let data = Game {
        id: None,
        game_number: Some(next_game_num), //new_game.game_number.to_owned(),
        game_type: new_game.game_type.to_owned(),
        player_1_name: new_game.player_1_name.to_owned(),
        player_2_name: new_game.player_2_name.to_owned(),
        winner_name: new_game.winner_name.to_owned(),
        game_date: new_game.game_date.to_owned(),
    };
    let game_detail = db.create_game(data);
    match game_detail {
        Ok(game) => Ok(Json(game)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/game/<path>")]
pub fn get_game(db: &State<MongoRepo>, path: String) -> Result<Json<Game>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let game_detail = db.get_game(&id);
    match game_detail {
        Ok(game) => Ok(Json(game)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/game/<path>", data = "<new_game>")]
pub fn update_game(
    db: &State<MongoRepo>,
    path: String,
    new_game: Json<Game>,
) -> Result<Json<Game>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = Game {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        game_number: new_game.game_number.to_owned(),
        game_type: new_game.game_type.to_owned(),
        player_1_name: new_game.player_1_name.to_owned(),
        player_2_name: new_game.player_2_name.to_owned(),
        winner_name: new_game.winner_name.to_owned(),
        game_date: new_game.game_date.to_owned(),
    };
    let update_result = db.update_game(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_game_info = db.get_game(&id);
                return match updated_game_info {
                    Ok(game) => Ok(Json(game)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/game/<path>")]
pub fn delete_game(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_game(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Game successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/games")]
pub fn get_all_games(db: &State<MongoRepo>) -> Result<Json<Vec<Game>>, Status> {
    let games = db.get_all_games();
    match games {
        Ok(games) => Ok(Json(games)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/computer/wins")]
pub fn get_computer_wins(db: &State<MongoRepo>) -> Result<Json<Vec<Game>>, Status> {
    // returns games won by the computer
    let games = db.get_computer_wins();
    match games {
        Ok(games) => Ok(Json(games)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/computer/metrics")]
pub fn get_computer_metrics(db: &State<MongoRepo>) -> Result<Json<ComputerMetrics>, Status> {
    let metrics = db.get_computer_metrics();
    match metrics {
        Ok(metrics) => Ok(Json(metrics)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/rankings")]
pub fn get_rankings(db: &State<MongoRepo>) -> Result<Json<Vec<Player>>, Status> {
    let rankings = db.get_rankings();
    match rankings {
        Ok(rankings) => Ok(Json(rankings)),
        Err(_) => Err(Status::InternalServerError),
    }
}
