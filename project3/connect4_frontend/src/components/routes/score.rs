use crate::components::table::Table;
use yew::prelude::*;
#[function_component]
pub fn ScorePage() -> Html {
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
    let dataCompWins = vec![vec!["0", "2", "3"]];
    let dataCompDetails = vec![
        vec!["0", "Otto", "Peter", "Polly", "Polly", "10-04-5002"],
        vec!["1", "Connect4", "Johanne", "Jax", "Jax", "11-01-2202"],
    ];
    let dataPlayers = vec![vec!["0", "Win", "202"], vec!["0", "Win", "202"]];

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
                <Suspense  fallback = {fallback.clone()}>
                    <Table headers = {headersPlayersWin} data ={dataPlayers}></Table>
                </Suspense>
            </div>
        </div>
    }
}
