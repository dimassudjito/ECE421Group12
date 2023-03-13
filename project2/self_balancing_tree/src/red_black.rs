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

#[derive(Debug, Clone)]
pub enum RedBlackTree<T: Ord + Copy + Debug> {
    Node {
        colour: NodeColour,
        data: T,
        left_child: Rc<RefCell<RedBlackTree<T>>>,
        right_child: Rc<RefCell<RedBlackTree<T>>>,
    },
    Empty
}

impl <T: Ord + Copy + Debug> RedBlackTree<T> {

    pub fn new(data: T) -> Self {
        RedBlackTree::<T>::Node {
            colour: NodeColour::Black, 
            data: data,
            left_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
            right_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
        }
    }

    pub fn in_order_traversal(&self) {
        // Print traversal of left node, then root, then right node
        match self {
            RedBlackTree::Node {colour, data, left_child, right_child} => {
                left_child.borrow().in_order_traversal();
                print!("{:?} ", *data);
                right_child.borrow().in_order_traversal();
            },
            RedBlackTree::Empty => {},
        }
    }

    pub fn rotate_left(self) -> Self {

        let mut new_parent: RedBlackTree<T> = RedBlackTree::Empty;

        match self {
            RedBlackTree::Node {colour, data, left_child, right_child} => {
                let tl_branch = Rc::clone(&left_child);
                let bl_branch = match (&*right_child.borrow()) {
                    RedBlackTree::Node {colour, data, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                let br_branch = match (&*right_child.borrow()) {
                    RedBlackTree::Node {colour, data, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                
                let datatemp = &data;
                let colourtemp = &colour;

                
                new_parent = match (&*right_child.borrow()) {
                    RedBlackTree::Node {colour, data, left_child, right_child} =>  {
                        let t_colour = *colour;
                        let t_data = *data;
                        let t_lc = Rc::new(RefCell::new(RedBlackTree::Node{
                            colour: *colourtemp,
                            data: *datatemp,
                            left_child: tl_branch,
                            right_child:bl_branch
                        }));
                        let t_rc = Rc::clone(&right_child);
                        RedBlackTree::Node {colour:t_colour, data:t_data, left_child: t_lc, right_child: t_rc}
                    }
                    RedBlackTree::Empty => RedBlackTree::Node {
                        colour: *colourtemp,
                        data: *datatemp,
                        left_child: Rc::clone(&left_child),
                        right_child:Rc::clone(&right_child)
                    },
                };


            },
            RedBlackTree::Empty => {},
        }
        new_parent
    }



    pub fn rotate_right(self) -> Self {

        let mut new_parent: RedBlackTree<T> = RedBlackTree::Empty;

        match self {
            RedBlackTree::Node {colour, data, left_child, right_child} => {
                let tr_branch = Rc::clone(&right_child);
                let br_branch = match (&*left_child.borrow()) {
                    RedBlackTree::Node {colour, data, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                let bl_branch = match (&*left_child.borrow()) {
                    RedBlackTree::Node {colour, data, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                
                let datatemp = &data;
                let colourtemp = &colour;

                
                new_parent = match (&*left_child.borrow()) {
                    RedBlackTree::Node {colour, data, left_child, right_child} =>  {
                        let t_colour = *colour;
                        let t_data = *data;
                        let t_rc = Rc::new(RefCell::new(RedBlackTree::Node{
                            colour: *colourtemp,
                            data: *datatemp,
                            left_child: br_branch,
                            right_child:tr_branch
                        }));
                        let t_lc = Rc::clone(&left_child);
                        RedBlackTree::Node {colour:t_colour, data:t_data, left_child: t_lc, right_child: t_rc}
                    }
                    RedBlackTree::Empty => RedBlackTree::Node {
                        colour: *colourtemp,
                        data: *datatemp,
                        left_child: Rc::clone(&left_child),
                        right_child:Rc::clone(&right_child)
                    },
                };


            },
            RedBlackTree::Empty => {},
        }
        new_parent
    }

}
