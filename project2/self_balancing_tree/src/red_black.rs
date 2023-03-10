use std::cell::RefCell;
use std::rc::Rc;

pub enum  NodeColour {
    Red, 
    Black,
}


pub enum RedBlackTree<T: Ord> {
    Node {
        colour: NodeColour,
        data: T,
        left_child: Rc<RedBlackTree<T>>,
        right_child: Rc<RedBlackTree<T>>,
    },
    Empty
}

impl <T: std::cmp::Ord> RedBlackTree<T> {

    pub fn new(data: T) -> Self {
        RedBlackTree::<T>::Node {
            colour: NodeColour::Black, 
            data: data,
            left_child: Rc::new(RedBlackTree::Empty),
            right_child: Rc::new(RedBlackTree::Empty),
        }
    }
    pub fn print_hello_world(&self) {
        println!("Hello rb World!");
    }

}