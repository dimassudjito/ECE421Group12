mod components;
// mod routes;
use crate::components::routes::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/history")]
    History,
    #[at("/score")]
    Score,
    #[at("/connect4/local")]
    Connect4Local,
    #[at("/connect4/computer")]
    Connect4Computer,
    #[at("/connect4/help")]
    Connect4Help,
    #[at("/otto/local")]
    OttoLocal,
    #[at("/otto/computer")]
    OttoComputer,
    #[at("/otto/help")]
    OttoHelp,

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <home::HomePage/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::History => html! {<history::HistoryPage/>},
        Route::Score => html! {<score::ScorePage/>},
        Route::OttoHelp => html! {<howToOtto::HowToOttoPage/>},
        Route::Connect4Help => html! {<howToC4::HowToConnect4Page/>},
        Route::Connect4Computer => html! {<connect4comp::Connect4ComputerPage/>},
        Route::Connect4Local => html! {<connect4human::Connect4HumanPage/>},
        Route::OttoComputer => html! {<ottoComp::OttoCompPage/>},
        Route::OttoLocal => html! {<ottoHuman::OttoHumanPage/>},
    }
}


mod board;
use board::*;

#[function_component(App)]
fn app() -> Html {
    // let navigator = use_navigator().unwrap();
    // let navigate = Callback::from(move |_: &str| navigator.push(&Route::Home));
    html! {
        <BrowserRouter>
            <div style="display:flex;flex-direction:row">


                <nav class="side-nav" id="mySidenav">
                    <div style="display:flex;flex-direction:column;width:320px">
                        <br/>
                        <br/>
                        <div class="w3-container">
                            <h3 class="w3-padding title"><b>{"Play"}<br/>{"Connect4 / TOOT-OTTO"}</b></h3>
                        </div>
                        <br/>
                        <Link<Route> to={Route::Connect4Help} classes="w3-padding w3-hover-white">{"How to Play Connect4"}</Link<Route>>
                        <Link<Route> to={Route::Connect4Computer} classes="w3-padding w3-hover-white">{"Play Connect4 With Computer"}</Link<Route>>
                        <Link<Route> to={Route::Connect4Local} classes="w3-padding w3-hover-white">{"Play Connect4 with Another Human"}</Link<Route>>
                        <br/>
                        <br/>
                        <Link<Route> to={Route::OttoHelp} classes="w3-padding w3-hover-white">{"How to Play TOOT-OTTO"}</Link<Route>>
                        <Link<Route> to={Route::OttoComputer} classes="w3-padding w3-hover-white">{"Play Toot-Otto With Computer"}</Link<Route>>
                        <Link<Route> to={Route::OttoLocal} classes="w3-padding w3-hover-white">{"Play Toot-Otto With Another Human"}</Link<Route>>
                        <br/>
                        <br/>
                        <Link<Route> to={Route::History} classes="w3-padding w3-hover-white">{"View Game History"}</Link<Route>>
                        <Link<Route> to={Route::Score} classes="w3-padding w3-hover-white">{"Score Board"}</Link<Route>>
                    </div>
                </nav>
                <div style="padding:5%;width:100%">

                        <Switch<Route> render={switch} />

                </div>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();

    //// testing board /////

    let mut board = Board::<i32>::new(6, 8);

    board.insert(&1, None, Some(3));
    board.insert(&1, None, Some(4));
    board.insert(&1, None, Some(3));


    board.insert(&0, None, Some(4));
    board.insert(&0, None, Some(3));
    board.insert(&0, None, Some(4));

    board.insert(&1, None, Some(4));
    board.insert(&1, None, Some(3));
    board.insert(&1, None, Some(4));
    board.insert(&1, None, Some(4));
    board.insert(&1, None, Some(4));
    board.insert(&1, None, Some(4));
    board.insert(&1, None, Some(4));
    board.insert(&1, None, Some(4));
    
    println!("{:?}", board.container);
    board.debug_print();
    //// end testing board ////
}
