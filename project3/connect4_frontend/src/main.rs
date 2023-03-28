use yew::prelude::*;


mod board;
use board::*;

mod fsm;
use fsm::*;

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

    
    //// testing programmable finite state machine ///// 
   
    // creates an FSM that tries to find this sequence
    let mut fsm = FSM::new(vec![1, 3, 1, 3, 1]);
    // fsm = FSM::new(vec![3, 1, 1, 3]);
    let mut testvec = vec![0, 1, 3, 1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 1, 3, 3, 1, 1, 3, 3, 1, 1, 3, 3 ];


    
    //fsm = FSM::new(vec![1, 1, 2, 2, 3, 3]);

    //testvec = vec![1, 1, 2, 2, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ];


    for x in testvec.iter() {
        println!("{}, {}", &x, fsm.step(&x));
    }
    

    //////// end testing programmable fsm ////////////


}
