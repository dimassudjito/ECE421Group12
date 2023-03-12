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
        data: T,
        left_child: Rc<RedBlackTree<T>>,
        right_child: Rc<RedBlackTree<T>>,
        colour: NodeColour,
        sibling: NodeColour,
    },
    Empty
}


impl <T: Ord + Copy + Debug> Clone for RedBlackTree<T> {
    fn clone(&self) -> Self {
        match &*self {
            RedBlackTree::Node {colour, data, left_child, right_child, sibling} => 
                RedBlackTree::Node {
                    colour: colour.clone(),
                    data: data.clone(),
                    left_child: left_child.clone().into(),
                    right_child: right_child.clone().into(),
                    sibling: sibling.clone().into()
                },
            RedBlackTree::Empty => RedBlackTree::Empty,
        }
    }
}

impl <T: Ord + Copy + Debug> RedBlackTree<T> {

    pub fn new(data: T) -> Self {
        RedBlackTree::<T>::Node {
            colour: NodeColour::Black, 
            data: data,
            left_child: Rc::new(RedBlackTree::Empty),
            right_child: Rc::new(RedBlackTree::Empty),
            sibling: NodeColour::Black,
        }
    }

    pub fn set_sibling(&mut self) {
        match *self {
            RedBlackTree::Node {colour, data, ref mut left_child, ref mut right_child, sibling} => {
                let rc = right_child.clone();
                let lc = left_child.clone();
                match Rc::make_mut(right_child) {
                    RedBlackTree::Node {colour, data, left_child, right_child, ref mut sibling} => {
                        *sibling = match &*lc {
                            RedBlackTree::Node {colour, data, left_child, right_child, sibling}=>{
                                colour.clone()
                            },
                            RedBlackTree::Empty => NodeColour::Black,
                        };
                        // println!("{:#?}", sibling.clone());
                        
                    },
                    RedBlackTree::Empty => {},
                }
                // println!("{:#?}", right_child);
                match Rc::make_mut(left_child) {
                    RedBlackTree::Node {colour, data, left_child, right_child, ref mut sibling} => {
                        *sibling = match &*rc {
                            RedBlackTree::Node {colour, data, left_child, right_child, sibling}=>{
                                colour.clone()
                            },
                            RedBlackTree::Empty => NodeColour::Black,
                        };
                        // println!("{:#?}", sibling.clone());
                    },
                    RedBlackTree::Empty => {},
                }
                // println!("{:#?}", left_child);
            },
            RedBlackTree::Empty => {
                
            }
        }
        // println!("{:#?}", self);
        
    } 

    pub fn rotate_left(self) -> Self {

        let mut new_parent: RedBlackTree<T> = RedBlackTree::Empty;

        match self {
            RedBlackTree::Node {colour, data, mut left_child, mut right_child, mut sibling} => {
                let tl_branch = left_child.clone();
                let bl_branch = match &*right_child.clone() {
                    RedBlackTree::Node {colour, data, left_child, right_child, sibling} => left_child.clone(),
                    RedBlackTree::Empty => Rc::new(RedBlackTree::Empty),
                };
                let br_branch = match &*right_child.clone() {
                    RedBlackTree::Node {colour, data, left_child, right_child, sibling} => right_child.clone(),
                    RedBlackTree::Empty => Rc::new(RedBlackTree::Empty),
                };
                
                let datatemp = &data;
                let colourtemp = &colour;

                
                new_parent = match &*right_child.clone() {
                    RedBlackTree::Node {colour, data, left_child, right_child, sibling} =>  {
                        let t_colour = *colour;
                        let t_data = *data;
                        let mut t_lc = Rc::new(RedBlackTree::Node{
                            colour: *colourtemp,
                            data: *datatemp,
                            left_child: tl_branch,
                            right_child:bl_branch, 
                            sibling:sibling.clone()
                        });
                        Rc::get_mut(&mut t_lc).unwrap().set_sibling();
                        let t_rc = right_child.clone();
                        RedBlackTree::Node {colour:t_colour, data:t_data, left_child: t_lc, right_child: t_rc, sibling: sibling.clone()}
                    }
                    RedBlackTree::Empty => RedBlackTree::Node {
                        colour: *colourtemp,
                        data: *datatemp,
                        left_child: left_child.clone(),
                        right_child:right_child.clone(),
                        sibling:sibling.clone()
                    },
                };
                new_parent.set_sibling();


            },
            RedBlackTree::Empty => {},
        }
        new_parent
    }



    pub fn rotate_right(self) -> Self {

        let mut new_parent: RedBlackTree<T> = RedBlackTree::Empty;

        match self {
            RedBlackTree::Node {colour, data, mut left_child, mut right_child, mut sibling} => {
                let tr_branch = right_child.clone();
                let br_branch = match &*left_child.clone() {
                    RedBlackTree::Node {colour, data, left_child, right_child, sibling} => right_child.clone(),
                    RedBlackTree::Empty => Rc::new(RedBlackTree::Empty),
                };
                let bl_branch = match &*left_child.clone() {
                    RedBlackTree::Node {colour, data, left_child, right_child, sibling} => left_child.clone(),
                    RedBlackTree::Empty => Rc::new(RedBlackTree::Empty),
                };
                
                let datatemp = &data;
                let colourtemp = &colour;

                
                new_parent = match &*left_child.clone() {
                    RedBlackTree::Node {colour, data, left_child, right_child, sibling} =>  {
                        let t_colour = *colour;
                        let t_data = *data;
                        let mut t_rc = Rc::new(RedBlackTree::Node{
                            colour: *colourtemp,
                            data: *datatemp,
                            left_child: br_branch,
                            right_child:tr_branch,
                            sibling:sibling.clone()
                        });
                        Rc::get_mut(&mut t_rc).unwrap().set_sibling();
                        let t_lc = left_child.clone();
                        RedBlackTree::Node {colour:t_colour, data:t_data, left_child: t_lc, right_child: t_rc, sibling:sibling.clone()}
                    }
                    RedBlackTree::Empty => RedBlackTree::Node {
                        colour: *colourtemp,
                        data: *datatemp,
                        left_child: left_child.clone(),
                        right_child:right_child.clone(),
                        sibling:sibling.clone()
                    },
                };
                new_parent.set_sibling();
            },
            RedBlackTree::Empty => {},
        }
        new_parent
    }




    pub fn insert(&mut self, val: T) {
        match self {
            RedBlackTree::Node {colour, data, ref mut left_child, ref mut right_child, ref mut sibling} => {
                if val > *data {
                    let mut node = Rc::get_mut(right_child).unwrap();
                    node.insert(val);
                    node.set_sibling();
                }else {
                    let mut node =  Rc::get_mut(left_child).unwrap();
                    node.insert(val);
                    node.set_sibling();
                }
            },
            RedBlackTree::Empty => {
                *self = RedBlackTree::Node {
                    colour: NodeColour::Black, 
                    data: val, 
                    left_child: Rc::new(RedBlackTree::Empty), 
                    right_child: Rc::new(RedBlackTree::Empty),
                    sibling: NodeColour::Black,
                }
            }
        }
    }
}
