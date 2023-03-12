mod red_black;
use std::env;

use red_black::*;
use std::cell::RefCell;
use std::rc::Rc;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    println!("Hello, world!");

    /////////////////////////////////////

    let mut lower = RedBlackTree::new(6);
    match lower {
        RedBlackTree::Node {colour, data, ref mut left_child, ref mut right_child, ref mut sibling} => {
            *left_child = Rc::new(RedBlackTree::Node {
                colour:NodeColour::Black,
                data: 7,
                left_child: Rc::new(RedBlackTree::Empty),
                right_child: Rc::new(RedBlackTree::Empty),
                sibling: NodeColour::Black,
            });
            *right_child = Rc::new(RedBlackTree::Node {
                colour:NodeColour::Red,
                data: 8,
                left_child: Rc::new(RedBlackTree::Empty),
                right_child: Rc::new(RedBlackTree::Empty),
                sibling: NodeColour::Red,
            });
            *sibling = NodeColour::Black;
        },
        RedBlackTree::Empty => {},
    }

    lower.set_sibling();
    // println!("{:#?}", lower);
    // return;


    let mut rbt = RedBlackTree::new(5);
    match rbt {
        RedBlackTree::Node {colour, data, ref mut left_child, ref mut right_child, ref mut sibling} => {
            *left_child = Rc::new(RedBlackTree::Empty);
            *right_child = Rc::new(lower);
            *sibling = NodeColour::Black;
        },
        RedBlackTree::Empty => {},
    }

    ////////////EXAMPLE 1/////////////////
    //     |
    //     5
    //   /   \
    //  x     6
    //      /   \
    //     7     8
    //
    //  Watch out for the following nodes: x, 7, 8, where x is empty.

    println!("{:#?}", rbt);
    println!("\n");
    // rotate left should form EXAMPLE 2
    rbt = rbt.rotate_left();
    
    println!("\n\n{:#?}", rbt);

       /////////EXAMPLE 2//////////////////
    //         |
    //         6
    //       /   \
    //      5     8
    //     /  \    
    //    x    7     
    // 

    // rotate right should form EXAMPLE 1
    rbt = rbt.rotate_right();
    println!("\n\n{:#?}", rbt);

    rbt.insert(9);
    println!("\n\n{:#?}", rbt);
    

}
