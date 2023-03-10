mod red_black;


use red_black::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");

    /////////////////////////////////////

    let mut lower = RedBlackTree::new(6);
    match lower {
        RedBlackTree::Node {colour, data, ref mut left_child, ref mut right_child} => {
            *left_child = Rc::new(RedBlackTree::new(7));
            *right_child = Rc::new(RedBlackTree::new(8));
        },
        RedBlackTree::Empty => {},
    }
    let mut rbt = RedBlackTree::new(5);
    match rbt {
        RedBlackTree::Node {colour, data, ref mut left_child, ref mut right_child} => {
            *left_child = Rc::new(RedBlackTree::Empty);
            *right_child = Rc::new(lower);
        },
        RedBlackTree::Empty => {},
    }

    /////////////////////////////////////
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
    println!("\n\n{:#?}", rbt.rotate_left());

       /////////////////////////////////////
    //         |
    //         6
    //       /   \
    //      5     8
    //     /  \    
    //    x    7     
    // 


}
