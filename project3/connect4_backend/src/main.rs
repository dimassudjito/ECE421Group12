#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

//add the modules
mod api; 
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};
use mongodb::{Client, options::ClientOptions};

/*
#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)User
}

#[get("/hello/<name>")]
fn hi(name: String) -> String {
    name
}

fn main() {
    rocket::ignite().mount("/", routes![hello, hi]).launch();
}
*/

//add imports below
use api::game_api::{create_game, get_game, update_game, delete_game, get_all_games, get_rankings};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_game])
        .mount("/", routes![get_game])
        .mount("/", routes![update_game])
        .mount("/", routes![delete_game])
        .mount("/", routes![get_all_games])
        .mount("/", routes![get_rankings])
}
