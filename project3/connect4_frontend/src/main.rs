use yew::prelude::*;


mod board;
use board::*;

mod fsm;
use fsm::*;

mod connect4;
use connect4::*;




#[function_component(App)]
fn app() -> Html {
    html! {
    <h1>{ "Project 3 ECE 421" }</h1>
    }
}

fn main() {
    // yew::Renderer::<App>::new().render();







    // //// testing board /////
    //
    // let mut board = Board::<i32>::new(6, 8);
    //
    // let mut res = board.insert(&1, None, Some(3));
    //
    // res = board.insert(&1, None, Some(4));
    // res = board.insert(&1, None, Some(3));
    //
    //
    // res = board.insert(&0, None, Some(4));
    // res = board.insert(&0, None, Some(3));
    // res = board.insert(&0, None, Some(4));
    //
    // res = board.insert(&1, None, Some(4));
    // res = board.insert(&1, None, Some(3));
    // res = board.insert(&1, None, Some(4));
    // res = board.insert(&1, None, Some(4));
    // res = board.insert(&1, None, Some(4));
    // res = board.insert(&1, None, Some(4));
    // res = board.insert(&1, None, Some(4));
    // res = board.insert(&1, None, Some(4));
    //
    //
    // 
    // let ypos = res.as_mut().ok().unwrap().0; 
    // let xpos = res.as_mut().ok().unwrap().1;
    //
    // println!("y: {}, x: {}", ypos, xpos);
    //
    // board.detect(res.as_mut().ok().unwrap().0, res.as_mut().ok().unwrap().1, &mut FSM::<i32>::new(vec![0]));
    // // println!("{:?}", board.container);
    // board.debug_print(true);
    // //// end testing board ////
    //
    //
    //
    //
    //
    // //// testing programmable finite state machine ///// 
    //
    // // creates an FSM that tries to find this sequence
    // let mut fsm = FSM::new(vec![1, 3, 1, 3, 1]);
    // // fsm = FSM::new(vec![3, 1, 1, 3]);
    // let testvec = vec![0, 1, 3, 1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 1, 3, 3, 1, 1, 3, 3, 1, 1, 3, 3 ];
    // 
    //
    // 
    // //fsm = FSM::new(vec![1, 1, 2, 2, 3, 3]);
    //
    // //testvec = vec![1, 1, 2, 2, 1, 1, 2, 2, 3, 3, 1, 1, 2, 2, 3, 3 ];
    //
    //
    // for x in testvec.iter() {
    //     println!("{}, {}", &x, fsm.step(&x));
    // }
    // 
    //
    // //////// end testing programmable fsm ////////////



    let mut con4 = Connect4::new(6, 8);

    let res = con4.insert(3);
    let res = con4.insert(2);

    let res = con4.insert(2);
    let res = con4.insert(1);

    let res = con4.insert(4);
    let res = con4.insert(1);
    //
    let res = con4.insert(1);
    let res = con4.insert(7);
    //
    let res = con4.insert(0);
    let res = con4.insert(0);
    // 
    let res = con4.insert(0);
    let res = con4.insert(7);
    //
    let res = con4.insert(0);
    // let res = con4.insert(6);


    // let res = con4.insert(4);
    // let res = con4.insert(4);








    if let Ok(x) = res {
        if let Some(y) = x {
            if let CChip::Red = y {
                println!("Red wins!");
            } else {
                println!("Yellow wins!");
            }
        } 
    } 

    if let Err(s) = res {
        println!("{}", s);
    }

    con4.board.debug_print(false);


}
