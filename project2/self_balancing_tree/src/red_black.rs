use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ord;
use std::marker::Copy;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub enum  NodeColour {
    Red, 
    Black,
}

#[derive(Debug)]
pub enum RedBlackTree<T: Ord + Copy + Debug> {
    Node {
        colour: NodeColour,
        data: T,
        left_child: Rc<RedBlackTree<T>>,
        right_child: Rc<RedBlackTree<T>>,
    },
    Empty
}

impl <T: Ord + Copy + Debug> RedBlackTree<T> {

    pub fn new(data: T) -> Self {
        RedBlackTree::<T>::Node {
            colour: NodeColour::Black, 
            data: data,
            left_child: Rc::new(RedBlackTree::Empty),
            right_child: Rc::new(RedBlackTree::Empty),
        }
    }

    pub fn rotate_left(self) -> Self {
        match &self {
            RedBlackTree::Node {colour, data, left_child, right_child} => {
                let tl_branch = left_child.clone();
                let bl_branch = match &*right_child.clone() {
                    RedBlackTree::Node {colour, data, left_child, right_child} => left_child.clone(),
                    RedBlackTree::Empty => Rc::new(RedBlackTree::Empty),
                };
                let br_branch = match &*right_child.clone() {
                    RedBlackTree::Node {colour, data, left_child, right_child} => right_child.clone(),
                    RedBlackTree::Empty => Rc::new(RedBlackTree::Empty),
                };

                println!("{:#?}", tl_branch);
                println!("{:#?}", bl_branch);
                println!("{:#?}", br_branch);

            },
            RedBlackTree::Empty => {},
        }
        self
    }

}
