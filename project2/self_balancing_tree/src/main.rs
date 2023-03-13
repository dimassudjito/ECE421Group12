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
            *left_child = Rc::new(RefCell::new(RedBlackTree::new(7)));
            *right_child = Rc::new(RefCell::new(RedBlackTree::new(8)));
        },
        RedBlackTree::Empty => {},
    }
    let mut rbt = RedBlackTree::new(5);
    match rbt {
        RedBlackTree::Node {colour, data, ref mut left_child, ref mut right_child} => {
            *left_child = Rc::new(RefCell::new(RedBlackTree::Empty));
            *right_child = Rc::new(RefCell::new(lower));
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
    rbt.in_order_traversal();

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
    println!("Leaf nodes: {}", rbt.count_leaves());
    println!("Tree height: {}", rbt.get_height());
    rbt.in_order_traversal();

}
