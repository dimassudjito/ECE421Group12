use crate::components::table::Table;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Deserialize,Debug, Clone)]
pub struct Score {
    pub player_name: String,
    pub wins: i32,
}

async fn updateScores(setScores:Callback<Vec<Vec<String>>>,setFail:Callback<()>){
    // TODO 
    // - Error handling
    // - Post functions
    // - Other tables -> do on backend?
    let body = reqwest::get("http://localhost:8000/rankings").await.unwrap()
        .json::<Vec<Score>>()
        .await.unwrap();

    let scores: Vec<Vec<String>> = body.into_iter().enumerate().map(|(i,score)| vec![(i+1).to_string(),score.player_name,score.wins.to_string()]).collect();
    setScores.emit(scores);
}

fn initiateRequests(setScores:Callback<Vec<Vec<String>>>,setFail:Callback<()>,setHasRequested:Callback<()>){
    setHasRequested.emit(()); 
    wasm_bindgen_futures::spawn_local(updateScores(setScores,setFail));
}
#[function_component]
pub fn ScorePage() -> Html {
    let scores = use_state(||vec![]);
    let setScores = {
        let scores = scores.clone();
        Callback::from(move |newScores : Vec<Vec<String>>| scores.set(newScores))
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
       initiateRequests(setScores,setReqsFailed,setHasRequested);
    }
   

    let headersCompWin = vec![
        "Total Games Played",
        "Games Against Computer",
        "Games Computer Won",
    ];
    let headersCompDetails = vec![
        "SI. No.",
        "Game Type",
        "Winner",
        "Played Against",
        "When Played",
    ];
    let headersPlayersWin = vec!["SI. No.", "Winner or Draw", "No. of Wins"];

    // dummy data
    let dataCompWins = vec![vec!["0".to_string(), "2".to_string(), "3".to_string()]];
    let dataCompDetails = vec![
        vec!["0".to_string(), "Otto".to_string(), "Peter".to_string(), "Polly".to_string(), "Polly".to_string(), "10-04-5002".to_string()],
        vec!["1".to_string(), "Connect4".to_string(), "Johanne".to_string(), "Jax".to_string(), "Jax".to_string(), "11-01-2202".to_string()],
    ];
    let dataPlayers = vec![vec!["0".to_string(), "Win".to_string(), "202".to_string()], vec!["0".to_string(), "Win".to_string(), "202".to_string()]];

    let fallback = html! {<h3>{"Loading..."}</h3>};

    html! {
        <div class="w3-container" id="services" style="margin-top:75px">
            <h1 ><b>{"Games Won By Computer"}</b></h1>
            
            <div id="game-stream">
                <Suspense  fallback = {fallback.clone()}>
                    <Table headers = {headersCompWin} data ={dataCompWins}></Table>
                </Suspense>
            </div>
            <br/>
            <br/>

            <h1 ><b>{"Details of Games Won by Computer"}</b></h1>
            <div id="game-stream">
                <Suspense fallback = {fallback.clone()}>
                    <Table headers = {headersCompDetails} data ={dataCompDetails}></Table>
                </Suspense>
            </div>
            <br/>
            <br/>

            <h1 ><b>{"Details of Games Won by All Players"}</b></h1>
            <div id="game-stream">
                if (*scores).len() == 0 && !*hasRequested{
                    {fallback}
                }else{
                    <Table headers = {headersPlayersWin}
                    data ={(*scores).clone()}>
                    </Table>
                }
            </div>
        </div>
    }
}
