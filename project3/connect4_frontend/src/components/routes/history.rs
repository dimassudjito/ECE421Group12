use crate::components::table::Table;
use yew::prelude::*;
#[function_component]
pub fn HistoryPage() -> Html {
    let headers = vec![
        "Game-ID",
        "Game Type",
        "Player1",
        "Player2",
        "Winner",
        "When Played",
    ];
    let data = vec![
        vec!["0", "Otto", "Peter", "Polly", "Polly", "10-04-5002"],
        vec!["1", "Connect4", "Johanne", "Jax", "Jax", "11-01-2202"],
    ];
    let fallback = html! {<h3>{"Loading..."}</h3>};

    html! {
        <div class="w3-container" id="services" style="margin-top:75px">
            <h1 ><b>{"Game History"}</b></h1>

                <div id="game-stream">
                    <Suspense {fallback}>
                        <Table headers = {headers} data ={data}></Table>
                    </Suspense>
                </div>
        </div>
    }
}
