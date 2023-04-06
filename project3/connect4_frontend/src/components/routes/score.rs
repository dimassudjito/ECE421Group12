use crate::components::table::Table;
use crate::models::*;
use crate::requests::initiateScoresRequests;
use serde::Deserialize;
use yew::prelude::*;

#[function_component]
pub fn ScorePage() -> Html {
    // Table state hooks
    let scores = use_state(|| vec![]);
    let setScores = {
        let scores = scores.clone();
        Callback::from(move |newScores: Vec<Vec<String>>| scores.set(newScores))
    };

    let compWins = use_state(|| vec![]);
    let setCompGames = {
        let compWins = compWins.clone();
        Callback::from(move |newCompWins: Vec<Vec<String>>| compWins.set(newCompWins))
    };

    let compMetrics = use_state(|| vec![]);
    let setCompMetrics = {
        let compMetrics = compMetrics.clone();
        Callback::from(move |newCompMetrics: Vec<Vec<String>>| compMetrics.set(newCompMetrics))
    };

    // Request flags
    let reqsFailed = use_state(|| false);
    let setReqsFailed = {
        let reqsFailed = reqsFailed.clone();
        Callback::from(move |_| {
            reqsFailed.set(true);
        })
    };

    let hasRequested = use_state(|| false);
    let setHasRequested = {
        let hasRequested = hasRequested.clone();
        Callback::from(move |_| {
            hasRequested.set(true);
        })
    };
    let finishedRequest = use_state(|| false);
    let setFinishedRequest = {
        let finishedRequest = finishedRequest.clone();
        Callback::from(move |_| {
            finishedRequest.set(true);
        })
    };

    // Request Data
    if !*hasRequested {
        initiateScoresRequests(
            setCompGames,
            setCompMetrics,
            setScores,
            setReqsFailed,
            setHasRequested,
            setFinishedRequest,
        );
    }

    // Table headers
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

    // Swap Components
    let fallback = html! {<h3>{"Loading..."}</h3>};
    let failure = html! {<h3>{"Could Not Load"}</h3>};

    html! {
        <div class="w3-container" id="services" style="margin-top:75px">
            <h1 ><b>{"Games Won By Computer"}</b></h1>

            <div id="game-stream">
                if (*compMetrics).len() == 0 && !*finishedRequest{
                    {fallback.clone()}
                }else if !*reqsFailed{
                    <Table headers = {headersCompWin}
                    data ={(*compMetrics).clone()}>
                    </Table>
                }else{
                    {failure.clone()}
                }
            </div>
            <br/>
            <br/>

            <h1 ><b>{"Details of Games Won by Computer"}</b></h1>
            <div id="game-stream">
                if (*compWins).len() == 0 && !*finishedRequest{
                    {fallback.clone()}
                }else if !*reqsFailed{
                    <Table headers = {headersCompDetails}
                    data ={(*compWins).clone()}>
                    </Table>
                }else{
                    {failure.clone()}
                }
            </div>
            <br/>
            <br/>

            <h1 ><b>{"Details of Games Won by All Players"}</b></h1>
            <div id="game-stream">
                if (*scores).len() == 0 && !*finishedRequest{
                    {fallback.clone()}
                }else if !*reqsFailed{
                    <Table headers = {headersPlayersWin}
                    data ={(*scores).clone()}>
                    </Table>
                }else{
                    {failure.clone()}
                }
            </div>
        </div>
    }
}
