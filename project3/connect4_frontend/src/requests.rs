use crate::models::*;
use chrono::{Datelike, Timelike, Utc};
use yew::prelude::*;

// ====================== POSTS ============================
async fn postGameAsync(
    game_type: String,
    player_1_name: String,
    player_2_name: String,
    winner_name: String,
    game_date: String,
    errCallbackOpt: Option<Callback<()>>,
) {
    // sends game to server.
    // Error call back is optional: send Option::None as parameters if no callbacks are needed
    let bod = format!(
        r#"{{ 
            "game_type": "{game_type}",
            "player_1_name": "{player_1_name}",
            "player_2_name": "{player_2_name}",
            "winner_name": "{winner_name}",
            "game_date": "{game_date}"
        }}"#
    );
    let client = reqwest::Client::new();
    match client
        .post("http://localhost:8000/game")
        .body(bod)
        .send()
        .await
    {
        Ok(d) => {}
        Err(e) => match errCallbackOpt {
            Some(errCallback) => {
                // run the error callback if supplied
                errCallback.emit(())
            }
            None => {}
        },
    }
}

pub fn postGame(
    game_type: String,
    player_1_name: String,
    player_2_name: String,
    winner_name: String,
    errCallback: Option<Callback<()>>,
) {
    // has one level of indirection to prevent the need for
    // wasm_bindgen_futures to be used by the caller
    wasm_bindgen_futures::spawn_local(postGameAsync(
        game_type,
        player_1_name,
        player_2_name,
        winner_name,
        get_time(),
        errCallback,
    ));
}

// ====================== GETS ============================
async fn updateGames(
    setGames: Callback<Vec<Vec<String>>>,
    setFail: Callback<()>,
    setFinishedRequest: Callback<()>,
) {
    // makes requests to the backend and returns the data through callbacks
    match reqwest::get("http://localhost:8000/games").await {
        Ok(r) => match r.json::<Vec<Game>>().await {
            Ok(body) => {
                let games: Vec<Vec<String>> = body
                    .into_iter()
                    .enumerate()
                    .map(|(i, game)| {
                        vec![
                            game.game_number.to_string(),
                            game.game_type,
                            game.player_1_name,
                            game.player_2_name,
                            game.winner_name,
                            game.game_date,
                        ]
                    })
                    .collect();
                setGames.emit(games);
            }
            Err(e2) => {
                setFail.emit(());
            }
        },
        Err(e) => {
            setFail.emit(());
        }
    }
    setFinishedRequest.emit(());
}
pub fn initiateHistoryRequests(
    setGames: Callback<Vec<Vec<String>>>,
    setFail: Callback<()>,
    setHasRequested: Callback<()>,
    setFinishedRequest: Callback<()>,
) {
    // sets some preliminary flags and initaites the request
    // has one level of indirection to prevent the need for
    // wasm_bindgen_futures to be used by the caller
    setHasRequested.emit(());
    wasm_bindgen_futures::spawn_local(updateGames(setGames, setFail, setFinishedRequest));
}

async fn updateScores(
    setCompGames: Callback<Vec<Vec<String>>>,
    setCompMetrics: Callback<Vec<Vec<String>>>,
    setScores: Callback<Vec<Vec<String>>>,
    setFail: Callback<()>,
    setFinishedRequest: Callback<()>,
) {
    // makes requests to the backend and returns the data through callbacks
    match reqwest::get("http://localhost:8000/rankings").await {
        Ok(r) => match r.json::<Vec<Score>>().await {
            Ok(body) => {
                let scores: Vec<Vec<String>> = body
                    .into_iter()
                    .enumerate()
                    .map(|(i, score)| {
                        vec![
                            (i + 1).to_string(),
                            score.player_name,
                            score.wins.to_string(),
                        ]
                    })
                    .collect();
                setScores.emit(scores);
            }
            Err(e2) => {
                setFail.emit(());
            }
        },
        Err(e) => {
            setFail.emit(());
        }
    }

    match reqwest::get("http://localhost:8000/computer/wins").await {
        Ok(r) => match r.json::<Vec<Game>>().await {
            Ok(body) => {
                let wins: Vec<Vec<String>> = body
                    .into_iter()
                    .enumerate()
                    .map(|(i, game)| {
                        vec![
                            game.game_number.to_string(),
                            game.game_type,
                            game.winner_name,
                            game.player_1_name,
                            game.game_date,
                        ]
                    })
                    .collect();
                setCompGames.emit(wins);
            }
            Err(e2) => {
                setFail.emit(());
            }
        },
        Err(e) => {
            setFail.emit(());
        }
    }

    match reqwest::get("http://localhost:8000/computer/metrics").await {
        Ok(r) => match r.json::<CompMetric>().await {
            Ok(body) => {
                let metrics = vec![vec![
                    body.total_games.to_string(),
                    body.games_against_comp.to_string(),
                    body.comp_wins.to_string(),
                ]];
                setCompMetrics.emit(metrics);
            }
            Err(e2) => {
                setFail.emit(());
            }
        },
        Err(e) => {
            setFail.emit(());
        }
    }

    setFinishedRequest.emit(())
}
pub fn initiateScoresRequests(
    setCompGames: Callback<Vec<Vec<String>>>,
    setCompMetrics: Callback<Vec<Vec<String>>>,
    setScores: Callback<Vec<Vec<String>>>,
    setFail: Callback<()>,
    setHasRequested: Callback<()>,
    setFinishedRequest: Callback<()>,
) {
    // sets some preliminary flags and initaites the request
    // has one level of indirection to prevent the need for
    // wasm_bindgen_futures to be used by the caller
    setHasRequested.emit(());
    wasm_bindgen_futures::spawn_local(updateScores(
        setCompGames,
        setCompMetrics,
        setScores,
        setFail,
        setFinishedRequest,
    ));
}

// ====================== UTILITIES ============================
fn get_time() -> String {
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    let time = format!(
        "{:02}/{:02}/{:02} {:02}:{:02}:{:02} {} (UTC)",
        now.month(),
        now.day(),
        now.year(),
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );
    time
}
