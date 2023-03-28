use yew::prelude::*;


mod board;
use board::*;

#[function_component(App)]
fn app() -> Html {
    html! {
    <h1>{ "Project 3 ECE 421" }</h1>
    }
}

fn main() {
    // yew::Renderer::<App>::new().render();

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
