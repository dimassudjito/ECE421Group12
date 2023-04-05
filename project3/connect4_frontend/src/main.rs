mod components;
mod env;
mod models;
mod requests;
// mod routes;
use crate::components::routes::*;
use std::io;
use stylist::yew::use_media_query;
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

mod fsm;
use fsm::*;

mod boardgame;
use boardgame::*;

mod chip;
use chip::*;

mod connect4ai;
use connect4ai::*;

mod toai;
use toai::*;

#[function_component(App)]
fn app() -> Html {
    let isSmall = use_media_query("(max-width: 1100px)");
    let isNavVis = use_state(|| !isSmall);

    let isNavVisClone = isNavVis.clone();
    use_effect_with_deps(move |_| isNavVisClone.set(false), isSmall);

    let showNav = {
        let isNavVis = isNavVis.clone();
        Callback::from(move |_| isNavVis.set(true))
    };

    let hideNav = {
        let isNavVis = isNavVis.clone();
        Callback::from(move |_| isNavVis.set(false))
    };

    html! {
        <BrowserRouter>
                <div style="display:flex;flex-direction:row;position:absolute;top:0;width:100%">

                    if isSmall && !*isNavVis {

                        <div onclick={showNav} class="mobile-hamburger">
                            <div class="mobile-hamburger-slice"></div>
                            <div class="mobile-hamburger-slice"></div>
                            <div class="mobile-hamburger-slice"></div>
                            <div class="mobile-hamburger-slice"></div>
                        </div>
                    }else{
                        <nav class="side-nav" id="mySidenav">
                            <div style="display:flex;flex-direction:column;width:320px">
                                if isSmall {
                                    <a onclick={hideNav.clone()} class="w3-hover-white">{"close navigation"}</a>
                                }else{
                                    <br/>
                                }
                                <div class="w3-container">
                                    <h3 class="w3-padding title"><b>{"Play"}<br/>{"Connect4 / TOOT-OTTO"}</b></h3>
                                </div >
                                <br/>
                                <div onclick={hideNav.clone()} class="nav-section">
                                    <Link<Route>  to={Route::Connect4Help} classes="w3-padding w3-hover-white">{"How to Play Connect4"}</Link<Route>>
                                    <Link<Route> to={Route::Connect4Computer} classes="w3-padding w3-hover-white">{"Play Connect4 With Computer"}</Link<Route>>
                                    <Link<Route> to={Route::Connect4Local} classes="w3-padding w3-hover-white">{"Play Connect4 with Another Human"}</Link<Route>>
                                </div>
                                <br/>
                                <br/>
                                <div onclick={hideNav.clone()} class="nav-section">
                                    <Link<Route> to={Route::OttoHelp} classes="w3-padding w3-hover-white">{"How to Play TOOT-OTTO"}</Link<Route>>
                                    <Link<Route> to={Route::OttoComputer} classes="w3-padding w3-hover-white">{"Play Toot-Otto With Computer"}</Link<Route>>
                                    <Link<Route> to={Route::OttoLocal} classes="w3-padding w3-hover-white">{"Play Toot-Otto With Another Human"}</Link<Route>>
                                </div>
                                <br/>
                                <br/>
                                <div onclick={hideNav.clone()} class="nav-section">
                                    <Link<Route> to={Route::History} classes="w3-padding w3-hover-white">{"View Game History"}</Link<Route>>
                                    <Link<Route> to={Route::Score} classes="w3-padding w3-hover-white">{"Score Board"}</Link<Route>>
                                </div>
                            </div>
                        </nav>
                    }
                    <div style="padding:5%;width:100%">

                            <Switch<Route> render={switch} />

                    </div>
                </div>


        </BrowserRouter>
    }
}

fn con4_cli_debug() {
    let mut ai = Connect4AI::new(6);

    let mut con4 = BoardGame::connect4(6, 8);

    loop {
        let mut idx = 0;
        let chip = if con4.board.counter % 2 == 0 {
            println!("Red's Turn");
            Chip::One
        } else {
            println!("Yellow's Turn");

            let mut alpha = i32::MIN + 3;
            let mut beta = i32::MAX - 3;

            idx = ai.mcts(&con4.board.clone(), 5000);

            println!("{}\n", idx);
            Chip::Two
        };

        let mut input_line = String::new();
        let x: i32;
        if let Chip::One = chip {
            println!("Your input (0 - {}): ", con4.board.size.1 - 1);
            io::stdin() // the rough equivalent of `std::cin`
                .read_line(&mut input_line) // actually read the line
                .expect("Failed to read line"); // which can fail, however
            x = input_line
                .trim() // ignore whitespace around input
                .parse() // convert to integers
                .expect("Input not an integer");

            println!("");
        } else {
            x = idx.clone() as i32;
            if !con4.board.insertable(idx.clone()) {
                for i in 0..con4.board.size.1 {
                    if let Ok(something) = con4.insert(i, chip) {
                        break;
                    }
                }
            }
        }
        let res = con4.insert(x as usize, chip);

        con4.board.debug_print(false);
        println!("\n\n");

        if let Ok(x) = res {
            if let Some(y) = x {
                if y == 1 {
                    println!("Red wins!");
                    return;
                } else {
                    println!("Yello wins!");
                    return;
                }
            }
        }

        if let Err(s) = res {
            println!("{}", s);
        }
    }
}

fn to_cli_debug() {
    let mut to = BoardGame::toot_otto(4, 6);
    let mut ai = TootOttoAI::new();
    loop {
        let mut x = 0;
        let mut chip = Chip::One;

        let turn = if to.board.counter % 2 == 0 { 1 } else { 2 };

        if turn == 1 {
            println!("Player 1's Turn");
            println!("Your input (0 - {}): ", to.board.size.1 - 1);

            let mut input_line = String::new();

            io::stdin() // the rough equivalent of `std::cin`
                .read_line(&mut input_line) // actually read the line
                .expect("Failed to read line"); // which can fail, however
            x = input_line
                .trim() // ignore whitespace around input
                .parse() // convert to integers
                .expect("Input not an integer");

            input_line = String::new();
            println!("Choose \"T\" or \"O\": ");
            io::stdin() // the rough equivalent of `std::cin`
                .read_line(&mut input_line) // actually read the line
                .expect("Failed to read line");
            chip = if input_line.trim().to_uppercase().eq(&"T") {
                Chip::Two
            } else {
                Chip::One
            };

            println!("{}", input_line.trim().to_uppercase());
        } else {
            println!("Player 2's Turn");
            println!("AI Thinking...");
            (x, chip) = ai.play(&mut to, 3, 2500);
            println!("AI plays {} at {}", chip, x);
        }

        let mut res = to.insert(x as usize, chip);

        to.board.debug_print(false);
        println!("\n\n\n");

        if let Ok(state) = res {
            if let Some(winner) = state {
                if winner == 1 {
                    println!("Player 1 wins!");
                } else {
                    println!("Player 2 wins!");
                }
                return;
            }
        }
        if let Err(message) = res {
            if message == "TIE" {
                println!("TIE!");
                return;
            } else {
                println!("{}", message);
            }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();

    // example request
    // requests::postGame(
    //     "connect4".to_string(),
    //     "test66".to_string(),
    //     "test33".to_string(),
    //     "test33".to_string(),
    //     "date".to_string(),
    //     None,
    // );
    // cli_debug();
    // cli_connect4_human();
    // yew::Renderer::<App>::new().render();
    // board.insert(&0, None, Some(4));
    // board.insert(&0, None, Some(3));
    // board.insert(&0, None, Some(4));

    // board.insert(&1, None, Some(4));
    // board.insert(&1, None, Some(3));
    // board.insert(&1, None, Some(4));
    // board.insert(&1, None, Some(4));
    // board.insert(&1, None, Some(4));
    // board.insert(&1, None, Some(4));
    // board.insert(&1, None, Some(4));
    // board.insert(&1, None, Some(4));

    // println!("{:?}", board.container);
    // board.debug_print();
    //// end testing board ////

    // con4_cli_debug();
    to_cli_debug();
}
