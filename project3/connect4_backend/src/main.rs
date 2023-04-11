#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

//add the modules
mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    // source = https://stackoverflow.com/questions/62412361/how-to-set-up-cors-or-options-for-rocket-rs
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

//add imports below
use api::game_api::{
    create_game, delete_game, get_all_games, get_computer_metrics, get_computer_wins, get_game,
    get_rankings, update_game,
};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .attach(CORS)
        .manage(db)
        .mount("/", routes![create_game])
        .mount("/", routes![get_game])
        .mount("/", routes![update_game])
        .mount("/", routes![delete_game])
        .mount("/", routes![get_all_games])
        .mount("/", routes![get_rankings])
        .mount("/", routes![get_computer_wins])
        .mount("/", routes![get_computer_metrics])
}
