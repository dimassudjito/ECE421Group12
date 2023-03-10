mod red_black;


use red_black::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
    let rbt = RedBlackTree::new(5);

    rbt.print_hello_world();
}
