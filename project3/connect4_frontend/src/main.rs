use yew::prelude::*;
use std::io;

mod board;
use board::*;

mod fsm;
use fsm::*;

mod boardgame;
use boardgame::*;

mod chip;
use chip::*;

mod ai;
use ai::*;




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


    let mut ai = Connect4AI::new(6);

    let mut con4 = BoardGame::connect4(6, 8);
    
    loop {
        let mut idx=0;
        let chip = if con4.board.counter % 2 == 0 {
            println!("Red's Turn");
            Chip::One
        } else {
            println!("Yellow's Turn");

            let mut scorevec = Vec::new();
            for x in 0..con4.board.size.1 {
                let mut board_clone = con4.board.clone();
                board_clone.insert(&Chip::Two, None, Some(x));
                scorevec.push(ai.mcts(&board_clone, 10000));
            }
            println!("AI recommendation: {:?}", scorevec);
            idx = 0;
            let mut maxi = -1000000;
            for i in 0..scorevec.len() {
                if scorevec[i] > maxi {
                    maxi = scorevec[i];
                    idx = i;
                }
            }


            println!("{}", idx);
            Chip::Two
        };


        println!("\n\n\nYour input: ");

        let mut input_line = String::new();
        let x: i32;
        if let Chip::One = chip {

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
        }
        let res = con4.insert(x as usize, chip);
        
        
        if let Ok(x) = res {
            if let Some(y) = x {
                if y == 1 {
                    println!("Red wins!");
                } else {
                    println!("Yello wins!");
                }
            } 
        } 

        if let Err(s) = res {
            println!("{}", s);
        }

        con4.board.debug_print(false);
    }



}
