use crate::components::table::Table;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Deserialize,Debug, Clone)]
pub struct Game {
    pub game_number: i32,
    pub game_type: String,
    pub player_1_name: String,
    pub player_2_name: String,
    pub winner_name: String,
    pub game_date:String
}
async fn updateGames(setGames:Callback<Vec<Vec<String>>>,setFail:Callback<()>){
    let body = reqwest::get("http://localhost:8000/games").await.unwrap()
        .json::<Vec<Game>>()
        .await.unwrap();
    
    let games: Vec<Vec<String>> = body.into_iter().enumerate().map(|(i,game)| vec![
        game.game_number.to_string(),
        game.game_type,
        game.player_1_name,
        game.player_2_name,
        game.winner_name,
        game.game_date
    ]).collect();
    setGames.emit(games);
}

fn initiateRequests(setGames:Callback<Vec<Vec<String>>>,setFail:Callback<()>,setHasRequested:Callback<()>){
    setHasRequested.emit(()); 
    wasm_bindgen_futures::spawn_local(updateGames(setGames,setFail));
}


#[function_component]
pub fn HistoryPage() -> Html {

    let games = use_state(||vec![]);
    let setGames = {
        let games = games.clone();
        Callback::from(move |newGames : Vec<Vec<String>>| games.set(newGames))
    };

    let reqsFailed = use_state(||false);
    let setReqsFailed = {
        let reqsFailed = reqsFailed.clone();
        Callback::from(move |_| {reqsFailed.set(true);})
    };

    let hasRequested = use_state(||false);
    let setHasRequested = {
        let hasRequested = hasRequested.clone();
        Callback::from(move |_| { hasRequested.set(true);})
    };

    if !*hasRequested{
      initiateRequests(setGames,setReqsFailed,setHasRequested);
    }


    let headers = vec![
        "Game-ID",
        "Game Type",
        "Player1",
        "Player2",
        "Winner",
        "When Played",
    ];
    let data = vec![
        vec![
            "0".to_string(),
            "Otto".to_string(),
            "Peter".to_string(),
            "Polly".to_string(),
            "Polly".to_string(),
            "10-04-5002".to_string(),
        ],
        vec![
            "1".to_string(),
            "Connect4".to_string(),
            "Johanne".to_string(),
            "Jax".to_string(),
            "Jax".to_string(),
            "11-01-2202".to_string(),
        ],
    ];
    let fallback = html! {<h3>{"Loading..."}</h3>};

    html! {
        <div class="w3-container" id="services" style="margin-top:75px">
            <h1 ><b>{"Game History"}</b></h1>

                <div id="game-stream">
                    if (*games).len() == 0  && !*hasRequested{
                        {fallback}
                    }else{
                        <Table headers = {headers} data ={(*games).clone()}></Table>
                    }
                </div>
        </div>
    }
}
