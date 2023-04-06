use crate::components::table::Table;
use crate::models::Game;
use crate::requests::initiateHistoryRequests;
use serde::Deserialize;
use yew::prelude::*;

#[function_component]
pub fn HistoryPage() -> Html {
    // stateful data
    let games = use_state(|| vec![]);
    let setGames = {
        let games = games.clone();
        Callback::from(move |newGames: Vec<Vec<String>>| games.set(newGames))
    };

    // request flags
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

    // start requests
    if !*hasRequested {
        initiateHistoryRequests(setGames, setReqsFailed, setHasRequested, setFinishedRequest);
    }

    let headers = vec![
        "Game-ID",
        "Game Type",
        "Player1",
        "Player2",
        "Winner",
        "When Played",
    ];

    // swap components
    let fallback = html! {<h3>{"Loading..."}</h3>};
    let failure = html! {<h3>{"Could Not Load"}</h3>};

    html! {
        <div class="w3-container" id="services" style="margin-top:75px">
            <h1 ><b>{"Game History"}</b></h1>

                <div id="game-stream">
                    if (*games).len() == 0  && !*finishedRequest{
                        {fallback}
                    }else if !*reqsFailed{
                        <Table headers = {headers} data ={(*games).clone()}></Table>
                    }else{
                        {failure}
                    }
                </div>
        </div>
    }
}
