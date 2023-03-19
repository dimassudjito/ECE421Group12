use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ord;
use std::cmp::max;
use std::marker::Copy;
use std::fmt::Debug;
use rand::thread_rng;
use rand::Rng;
use rand::prelude::SliceRandom;
use std::time::Duration;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
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


impl <T: Ord + Copy + Debug> PartialEq for RedBlackTree<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            RedBlackTree::Node {colour, data, left_child, right_child} => {
                let d1 = data.clone();
                match other {
                    RedBlackTree::Node {colour, data, left_child, right_child} => {
                        let d2 = data.clone();
                        return d1 == d2;
                    }
                    RedBlackTree::Empty => return false,
                }
            }
            RedBlackTree::Empty => return false,
        };
    }
}


impl <T: Ord + Copy + Debug> RedBlackTree<T> {

    pub fn new() -> Self {
        RedBlackTree::<T>::Empty
    }

    pub fn search(&self, val: T) -> Self {
        match self {
            RedBlackTree::Node {data, colour, left_child, right_child} => {
                if val == *data {
                    return self.clone();
                }
                else if val > *data {
                    return right_child.borrow().search(val);
                } else {
                    return left_child.borrow().search(val);
                }
            }, 
            RedBlackTree::Empty => {
                return RedBlackTree::Empty;
            }
        }
    }

    pub fn rotate_right(&mut self) {
        match self {
            RedBlackTree::Node{data, colour, left_child, right_child} => {
                let mut lc = Rc::clone(&left_child);
                let lc_rc = match &*lc.borrow() {
                    RedBlackTree::Node{data, colour, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => {return;}
                };
                let mut oldself = self.clone();
                match oldself {
                    RedBlackTree::Node{data, colour, ref mut left_child, ref mut right_child} => {
                        *left_child = lc_rc;
                    },
                    RedBlackTree::Empty => {return;}
                }

                *self = lc.borrow().clone();
                match self {
                    RedBlackTree::Node{data, colour, ref mut left_child, ref mut right_child} => {
                        *right_child = Rc::new(RefCell::new(oldself));
                    },
                    RedBlackTree::Empty => {return;}
                }
            },
            RedBlackTree::Empty => {return;}
        }
    }


    pub fn rotate_left(&mut self) {
        match self {
            RedBlackTree::Node{data, colour, left_child, right_child} => {
                let mut rc = Rc::clone(&right_child);
                let rc_lc = match &*rc.borrow() {
                    RedBlackTree::Node{data, colour, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => {return;}
                };
                let mut oldself = self.clone();
                match oldself {
                    RedBlackTree::Node{data, colour, ref mut left_child, ref mut right_child} => {
                        *right_child = rc_lc;
                    },
                    RedBlackTree::Empty => {return;}
                }

                *self = rc.borrow().clone();
                match self {
                    RedBlackTree::Node{data, colour, ref mut left_child, ref mut right_child} => {
                        *left_child = Rc::new(RefCell::new(oldself));
                    },
                    RedBlackTree::Empty => {return;}
                }
            },
            RedBlackTree::Empty => {return;}
        }
    }

    pub fn insert(&mut self, val: T) {
        match self {
            RedBlackTree::Node {data, colour, left_child, right_child} => {},
            RedBlackTree::Empty => {
                *self = RedBlackTree::Node {
                    data:val, 
                    colour:NodeColour::Black, 
                    left_child: Rc::new(RefCell::new(RedBlackTree::Empty)), 
                    right_child: Rc::new(RefCell::new(RedBlackTree::Empty))
                };
                return;
            }
        };

        let mut stack = vec![Rc::new(RefCell::new(self.clone()))];
        let mut node = Rc::clone(&stack[0]);
        let mut nodetemp = Rc::clone(&node);

        ///// BINARY TREE INSERT //////
        loop {
            match &*node.borrow() {
                RedBlackTree::Node {data, colour, left_child, right_child} => {
                    if val > *data {
                        nodetemp = Rc::clone(&right_child);
                    } else if val < *data {
                        nodetemp = Rc::clone(&left_child);
                    } else {
                        return;
                    }
                },
                RedBlackTree::Empty => {break},
            }
            node = Rc::clone(&nodetemp);
            stack.push(Rc::clone(&node));
        }
        // println!("{:#?}", stack);

        stack[stack.len()-1].replace(RedBlackTree::Node {
            data: val.clone(), 
            colour: NodeColour::Red, 
            left_child: Rc::new(RefCell::new(RedBlackTree::Empty)), 
            right_child: Rc::new(RefCell::new(RedBlackTree::Empty))
        });
        ///// END BINARY TREE INSERT //////


        ///// FIX TREE //////
        let mut idx = stack.len()-1;
        let mut uncle: Rc<RefCell<RedBlackTree<T>>>;
        let mut grandfather: Rc<RefCell<RedBlackTree<T>>>;
        let mut parent: Rc<RefCell<RedBlackTree<T>>>;
        while idx >= 2 {
            // we got the grandfather
            grandfather = Rc::clone(&stack[idx-2]);
            let mut gf_left =  Rc::new(RefCell::new(RedBlackTree::Empty));
            let mut gf_right =  Rc::new(RefCell::new(RedBlackTree::Empty));

            let mut gf_val = match &*grandfather.borrow() {
                RedBlackTree::Node {data, colour, left_child, right_child} => {
                    gf_left = Rc::clone(&left_child);
                    gf_right = Rc::clone(&right_child);
                    data.clone()
                },
                RedBlackTree::Empty => val, // not possible
            };
            if val > gf_val {
                uncle = Rc::clone(&gf_left);
            } else {
                uncle = Rc::clone(&gf_right);
            }
            // now we got the uncle

            parent = Rc::clone(&stack[idx-1]);
            // and we got the parent


            // If parent is black, then we break
            if NodeColour::Black == match &*parent.borrow() {
                RedBlackTree::Node {data, colour, left_child, right_child} => *colour,
                RedBlackTree::Empty => NodeColour::Black,
            } {
                break;
            }
            // otherwise...
            // if uncle is red
            if NodeColour::Red == match &*uncle.borrow() {
                RedBlackTree::Node {data, colour, left_child, right_child} => *colour,
                RedBlackTree::Empty => NodeColour::Black,
            } {
                // Set uncle and parent as black
                match *parent.borrow_mut() {
                    RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = NodeColour::Black},
                    RedBlackTree::Empty => {},
                }
                match *uncle.borrow_mut() {
                    RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = NodeColour::Black},
                    RedBlackTree::Empty => {},
                }
                // set grandfather as Red
                if idx - 2 > 0 {
                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = NodeColour::Red},
                        RedBlackTree::Empty => {},
                    }
                }

                idx = idx - 2;

                // println!("\n\n\n ROOT\n{:#?}", stack[0]);


            } else { // else if uncle is black
                // we have 4 cases

                // If grandparent.left.left is the current node
                if (match &*grandfather.borrow() {
                    RedBlackTree::Node {data, mut colour, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent)) &&
                (match &*parent.borrow(){
                    RedBlackTree::Node {data, mut colour, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&stack[idx])) {

                    // right rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp.rotate_right();
                    parent.replace(gf_temp);

                    grandfather = match &*parent.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // then swap colours of grandfather and parent
                    let mut gf_colour = match &*grandfather.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let mut p_colour = match &*parent.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    match *grandfather.borrow_mut(){
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = p_colour},
                        RedBlackTree::Empty => {},
                    };
                    match *parent.borrow_mut(){
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = gf_colour},
                        RedBlackTree::Empty => {},
                    };

                    if idx - 2 == 0 {
                        *self = parent.borrow().clone();
                        return;
                    } else { // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {ref data, ref colour, ref mut left_child, ref mut right_child} => {
                                let upperval = data.clone();
                                let parent_val = match &*parent.borrow() {
                                    RedBlackTree::Node {data, colour, left_child, right_child} => {
                                        *data
                                    },
                                    RedBlackTree::Empty => {upperval.clone()}
                                };
                                if parent_val > upperval {
                                    *right_child = Rc::clone(&parent);
                                } else if parent_val < upperval {
                                    *left_child = Rc::clone(&parent);
                                }
                            },
                            RedBlackTree::Empty => {},
                        };
                    }

                }

                // If grandparent.left.right is the current node
                else if (match &*grandfather.borrow() {
                    RedBlackTree::Node {data, mut colour, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent)) &&
                (match &*parent.borrow(){
                    RedBlackTree::Node {data, mut colour, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&stack[idx])) {
                    
                    // left rotate on parent
                    // println!("\n\n\n PARENT OLD:");
                    // parent.borrow().display_tree();
                    // println!("\n\n\n CURRENT OLD:");
                    // stack[idx].borrow().display_tree();

                    let mut p_temp = parent.borrow().clone();
                    p_temp.rotate_left();
                    stack[idx].replace(p_temp);

                    parent = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {ref data, ref colour, ref mut left_child, ref right_child} => {*left_child = Rc::clone(&stack[idx])},
                        RedBlackTree::Empty => {},
                    };


                    
                    // println!("\n\n\n PARENT NEW:");
                    // parent.borrow().display_tree();

                    // println!("\n\n\n CURRENT NEW:");
                    // stack[idx].borrow().display_tree();





                    // println!("\n\n\n GRANDPARENT OLD:");
                    // grandfather.borrow().display_tree();

                    // println!("\n\n\n CURRENT OLD:");
                    // stack[idx].borrow().display_tree();



                    // right rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp.rotate_right();
                    stack[idx].replace(gf_temp);

                    grandfather = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // println!("\n\n\n GRANDPARENT NEW:");
                    // grandfather.borrow().display_tree();

                    // println!("\n\n\n CURRENT NEW:");
                    // stack[idx].borrow().display_tree();


                    // then swap colours of grandfather and current
                    let mut gf_colour = match &*grandfather.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let mut c_colour = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = c_colour},
                        RedBlackTree::Empty => {},
                    };

                    match *stack[idx].borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = gf_colour},
                        RedBlackTree::Empty => {},
                    };

                    // if new insert is root, return
                    if idx - 2 == 0 {
                        *self = stack[idx].borrow().clone();
                        return ;
                    } else { // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {ref data, ref colour, ref mut left_child, ref mut right_child} => {
                                let upperval = data.clone();
                                let insert_val = match &*stack[idx].borrow() {
                                    RedBlackTree::Node {data, colour, left_child, right_child} => {
                                        *data
                                    },
                                    RedBlackTree::Empty => {upperval.clone()}
                                };
                                if insert_val > upperval {
                                    *right_child = Rc::clone(&stack[idx]);
                                } else if insert_val < upperval {
                                    *left_child = Rc::clone(&stack[idx]);
                                }
                            },
                            RedBlackTree::Empty => {},
                        };
                    }
                    

                }

                // If grandparent.right.left is the current node
                else if (match &*grandfather.borrow() {
                    RedBlackTree::Node {data, mut colour, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent)) &&
                (match &*parent.borrow(){
                    RedBlackTree::Node {data, mut colour, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&stack[idx])) {
                    // right rotate on parent
                    let mut p_temp = parent.borrow().clone();
                    p_temp.rotate_right();
                    stack[idx].replace(p_temp);

                    parent = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {ref data, ref colour, ref left_child, ref mut right_child} => {*right_child = Rc::clone(&stack[idx])},
                        RedBlackTree::Empty => {},
                    };

                    // left rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp.rotate_left();
                    stack[idx].replace(gf_temp);

                    grandfather = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // then swap colours of grandfather and current
                    let mut gf_colour = match &*grandfather.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let mut c_colour = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                  
                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = c_colour},
                        RedBlackTree::Empty => {},
                    };


                    match *stack[idx].borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = gf_colour},
                        RedBlackTree::Empty => {},
                    };

                    // if new insert is root, return
                    if idx - 2 == 0 {
                        *self = stack[idx].borrow().clone();
                        return;
                    } else { // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {ref data, ref colour, ref mut left_child, ref mut right_child} => {
                                let upperval = data.clone();
                                let insert_val = match &*stack[idx].borrow() {
                                    RedBlackTree::Node {data, colour, left_child, right_child} => {
                                        *data
                                    },
                                    RedBlackTree::Empty => {upperval.clone()}
                                };
                                if insert_val > upperval {
                                    *right_child = Rc::clone(&stack[idx]);
                                } else if insert_val < upperval {
                                    *left_child = Rc::clone(&stack[idx]);
                                }
                            },
                            RedBlackTree::Empty => {},
                        };
                    }


                }
                
                // If grandparent.right.right is the current node
                else if (match &*grandfather.borrow() {
                    RedBlackTree::Node {data, mut colour, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent)) &&
                (match &*parent.borrow(){
                    RedBlackTree::Node {data, mut colour, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&stack[idx])) {


                    // left rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp.rotate_left();
                    parent.replace(gf_temp);

                    grandfather = match &*parent.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };


                    // then swap colours of grandfather and parent
                    let gf_colour = match &*grandfather.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let p_colour = match &*parent.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };


                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = p_colour},
                        RedBlackTree::Empty => {},
                    };


                    match *parent.borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = gf_colour},
                        RedBlackTree::Empty => {},
                    };


                    // if parent is root, return
                    if idx - 2 == 0 {
                        *self = parent.borrow().clone();
                        return;
                    } else { // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {ref data, ref colour, ref mut left_child, ref mut right_child} => {
                                let upperval = data.clone();
                                let parent_val = match &*parent.borrow() {
                                    RedBlackTree::Node {data, colour, left_child, right_child} => {
                                        *data
                                    },
                                    RedBlackTree::Empty => {upperval.clone()}
                                };
                                if parent_val > upperval {
                                    *right_child = Rc::clone(&parent);
                                } else if parent_val < upperval {
                                    *left_child = Rc::clone(&parent);
                                }
                            },
                            RedBlackTree::Empty => {},
                        };
                    }


                }

                break;
            }

        }
        ///// END FIX TREE //////
        
        // println!("{:#?}" , stack);
        // println!("\n\n\nFINAL:");
        // let root = stack[0].borrow().display_tree(); 
        *self = stack[0].borrow().clone();
    }

    pub fn insert_no_fix(&mut self, val: T) {
        match self {
            RedBlackTree::Node {data, colour, left_child, right_child} => {},
            RedBlackTree::Empty => {
                *self = RedBlackTree::Node {
                    data:val, 
                    colour:NodeColour::Black, 
                    left_child: Rc::new(RefCell::new(RedBlackTree::Empty)), 
                    right_child: Rc::new(RefCell::new(RedBlackTree::Empty))
                };
                return;
            }
        };

        let mut node = Rc::new(RefCell::new(self.clone()));
        let mut nodetemp = Rc::clone(&node);

        ///// BINARY TREE INSERT //////
        loop {
            match &*node.borrow() {
                RedBlackTree::Node {data, colour, left_child, right_child} => {
                    if val > *data {
                        nodetemp = Rc::clone(&right_child);
                    } else if val < *data {
                        nodetemp = Rc::clone(&left_child);
                    } else {
                        return;
                    }
                },
                RedBlackTree::Empty => {break},
            }
            node = Rc::clone(&nodetemp);
            // stack.push(Rc::clone(&node));
        }

        node.replace(RedBlackTree::Node {
                    data: val.clone(), 
                    colour: NodeColour::Red, 
                    left_child: Rc::new(RefCell::new(RedBlackTree::Empty)), 
                    right_child: Rc::new(RefCell::new(RedBlackTree::Empty))
                });

        ///// END BINARY TREE INSERT //////

    }
}


#[derive(Debug)]
pub enum AVLTree<T: Ord> {
    Node {
        data: RefCell<Rc<T>>,
        left_child: RefCell<Rc<AVLTree<T>>>,
        right_child: RefCell<Rc<AVLTree<T>>>,
        height: RefCell<i32>,
    },
    Empty,
}

impl<T: Ord + Display + Copy> AVLTree<T> {
    pub fn insert_node(node_rc: &Rc<AVLTree<T>>, new_data: &T) -> Rc<AVLTree<T>> {
        match &**node_rc {
            AVLTree::Empty => {
                let new_node: AVLTree<T> = AVLTree::Node {
                    data: RefCell::new(Rc::new(*new_data)),
                    left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    height: RefCell::new(1),
                };
                Rc::new(new_node)
            }
            AVLTree::Node {
                data,
                left_child,
                right_child,
                height,
            } => {
                if *new_data < **(data.borrow()) {
                    let new_node = AVLTree::insert_node(&*(left_child.borrow()), new_data);
                    left_child.replace(new_node);
                } else if *new_data > **(data.borrow()) {
                    let new_node = AVLTree::insert_node(&*(right_child.borrow()), new_data);
                    right_child.replace(new_node);
                } else {
                    return Rc::clone(node_rc);
                }
                
                let return_node_rc = AVLTree::insert_node_balance(node_rc);
                (*return_node_rc).update_heights();
                return return_node_rc;
            }
        }
    }

    pub fn insert_node_balance(node_rc: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        match &**node_rc {
            AVLTree::Empty => {
                return Rc::clone(node_rc);
            }
            AVLTree::Node {
                data,
                left_child: left_child_ref,
                right_child: right_child_ref,
                ..
            } => {
                // balance
                let left_node = &*Rc::clone(&*left_child_ref.borrow());
                let right_node = &*Rc::clone(&*right_child_ref.borrow());

                let left_height = (*left_child_ref.borrow()).get_height();
                let right_height = (*right_child_ref.borrow()).get_height();
                if (left_height - right_height).abs() > 1 {
                    // this is an unbalanced node
                    if left_height > right_height {
                        // left-<?> case
                        match left_node {
                            AVLTree::Empty => {
                                panic!("Given tree is not a proper AVL tree");
                            }
                            AVLTree::Node {
                                left_child: y_left_child_ref,
                                right_child: y_right_child_ref,
                                ..
                            } => {
                                let y_left_height = (*y_left_child_ref.borrow()).get_height();
                                let y_right_height = (*y_right_child_ref.borrow()).get_height();
                                if y_left_height > y_right_height{
                                    // left-left case
                                    return AVLTree::rotation_left_left(node_rc);
                                } else {
                                    // left-right case
                                    return AVLTree::rotation_left_right(node_rc);
                                }
                            }
                        }
                    } else {
                        // right-<?> case
                        match right_node {
                            AVLTree::Empty => {
                                panic!("Given tree is not a proper AVL tree");
                            }
                            AVLTree::Node {
                                left_child: y_left_child_ref,
                                right_child: y_right_child_ref,
                                ..
                            } => {
                                let y_left_height = (*y_left_child_ref.borrow()).get_height();
                                let y_right_height = (*y_right_child_ref.borrow()).get_height();
                                if y_left_height > y_right_height {
                                    // right-left case
                                    return AVLTree::rotation_right_left(node_rc);
                                } else {
                                    // right-right case
                                    return AVLTree::rotation_right_right(node_rc);
                                }
                            }
                        }
                    }
                } else {
                    // is a balanced node
                    return Rc::clone(node_rc);
                }
            }
        }
    }

    pub fn rotation_left_left(z: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        return AVLTree::rotate_right(z);
    }

    pub fn rotation_left_right(z: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        //   z
        //  /
        // y
        match &(**z) {
            AVLTree::Empty => {
                panic!("Invalid AVLTree for left right rotation")
            }
            AVLTree::Node { left_child, .. } => {
                let new_left = AVLTree::rotate_left(&(left_child.borrow()));
                left_child.replace(new_left);
                return AVLTree::rotate_right(z);
            }
        }
    }

    pub fn rotation_right_right(z: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        return AVLTree::rotate_left(z);
    }

    pub fn rotation_right_left(z: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        //   z
        //    \
        //     y
        match &(**z) {
            AVLTree::Empty => {
                panic!("Invalid AVLTree for right left rotation")
            }
            AVLTree::Node { right_child, .. } => {
                let new_right = AVLTree::rotate_right(&(right_child.borrow()));
                right_child.replace(new_right);
                return AVLTree::rotate_left(z);
            }
        }
    }

    pub fn rotate_right(z_rc: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        // println!("rotating right");

        // EX:   z
        //      /
        //     y
        //    / \
        //   x   N
        let z = &(**z_rc);
        match z {
            AVLTree::Empty => Rc::clone(z_rc),
            AVLTree::Node {
                left_child: z_left_child,
                ..
            } => {
                let y_rc = Rc::clone(&z_left_child.borrow());
                let y = &(*y_rc);
                match y {
                    AVLTree::Empty => Rc::clone(z_rc),
                    AVLTree::Node {
                        right_child: y_right_child,
                        ..
                    } => {
                        //       y
                        //      / \
                        //     x   z      N
                        let n = (y_right_child).replace(Rc::clone(z_rc));

                        //       y
                        //      / \
                        //     x   z
                        //        /
                        //       n
                        z_left_child.replace(n);

                        y.update_heights();
                        z.update_heights();

                        y_rc
                    }
                }
            }
        }
    }

    pub fn rotate_left(z_rc: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        // println!("rotating left");

        // EX:   z
        //        \
        //         y
        //        / \
        //       n   x
        let z = &(**z_rc);
        match z {
            AVLTree::Empty => Rc::clone(z_rc),
            AVLTree::Node {
                right_child: z_right_child,
                ..
            } => {
                let y_rc = Rc::clone(&z_right_child.borrow());
                let y = &(*y_rc);
                match y {
                    AVLTree::Empty => Rc::clone(z_rc),
                    AVLTree::Node {
                        left_child: y_left_child,
                        ..
                    } => {
                        //       y
                        //      / \
                        //     z   x
                        let n = (y_left_child).replace(Rc::clone(z_rc));

                        //       y
                        //      / \
                        //     z   x
                        //      \
                        //        n
                        z_right_child.replace(n);

                        y.update_heights();
                        z.update_heights();

                        y_rc
                    }
                }
            }
        }
    }

    pub fn update_heights(&self) {
        // updates the heights of an node based on it's direct children's heights.
        // IT IS NOT recursive. If the children's heights are incorrect, the height of this node will be as well.
        // TODO: leaf node should be 1 not 0
        match self {
            AVLTree::Empty => {}
            AVLTree::Node {
                left_child,
                right_child,
                height,
                ..
            } => {
                let left_val = (**left_child.borrow()).get_height();
                let right_val = (**right_child.borrow()).get_height();
                height.replace(max(left_val, right_val) + 1);
            }
        }
    }

    fn get_height(&self) -> i32 {
        // Returns the height of a AVLTree node, including empty nodes
        match self {
            AVLTree::Empty => {
                return -1;
            }
            AVLTree::Node { height, .. } => {
                return *height.borrow();
            }
        }
    }

    pub fn leaf_number(&self) -> i32 {
        // Returns the number of leaves
        // Note: an empty tree has no leaves

        match self.get_height() {
            0 => {
                // leaf node
                return 1;
            }
            -1 => {
                // empty node
                return 0;
            }
            _ => match self {
                AVLTree::Empty => {
                    panic!("leaf_number failed")
                }
                AVLTree::Node {
                    left_child,
                    right_child,
                    ..
                } => {
                    return (**left_child.borrow()).leaf_number()
                        + (**right_child.borrow()).leaf_number()
                }
            },
        }
    }

    pub fn tree_height(&self) -> i32 {
        // TODO: Dimas
        match self {
            AVLTree::Empty => 0,
            AVLTree::Node { height, .. } => *(height.borrow()),
        }
    }

    // pub fn print() {
    //     // TODO: Dimas
    // }

    pub fn print_inorder(&self) {
        // Prints an AVL tree in a inorder traversal
        match self {
            AVLTree::Node {
                data,
                left_child,
                right_child,
                ..
            } => {
                (*left_child.borrow()).print_inorder();
                println!("{}", **data.borrow());
                (*right_child.borrow()).print_inorder();
            }
            AVLTree::Empty => return,
        }
    }

    pub fn is_tree_empty(&self) -> bool {
        match self {
            AVLTree::Empty => true,
            AVLTree::Node { .. } => false,
        }
    }

    pub fn search_node(&self, value: &T) -> bool {
        match self {
            AVLTree::Node { data, left_child, right_child, .. } => {
                if *value == **data.borrow() {
                    true
                } else if *value < **data.borrow() {
                    left_child.borrow().search_node(value)
                } else {
                    right_child.borrow().search_node(value)
                }
            },
            AVLTree::Empty => false,
        }
    }

}











fn red_black_insert_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("red_black_insert_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("red_black_insert_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len()-100 {
                    rbt.insert(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.insert(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn red_black_insert_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("red_black_insert_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("red_black_insert_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len()-100 {
                    rbt.insert(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.insert(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn red_black_insert_block_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("red_black_insert_block_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("red_black_insert_block_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                
                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in 0..insertions.len() {
                        rbt.insert(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn red_black_insert_block_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("red_black_insert_block_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("red_black_insert_block_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in 0..insertions.len() {
                        rbt.insert(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}



fn avl_insert_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("avl_insert_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("avl_insert_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut avl = Rc::new(AVLTree::Node {
                    data: RefCell::new(Rc::new(black_box(&insertions[0]))),
                    left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    height: RefCell::new(0),
                });


                for i in 0..insertions.len()-100 {
                    avl = AVLTree::insert_node(&avl, &&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        avl = AVLTree::insert_node(&avl, black_box(&&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn avl_insert_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("avl_insert_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("avl_insert_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut avl = Rc::new(AVLTree::Node {
                    data: RefCell::new(Rc::new(black_box(&insertions[0]))),
                    left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    height: RefCell::new(0),
                });


                for i in 0..insertions.len()-100 {
                    avl = AVLTree::insert_node(&avl, &&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        avl = AVLTree::insert_node(&avl, black_box(&&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();     
}

fn avl_insert_block_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("avl_insert_block_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("avl_insert_block_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                
                b.iter(|| {
                    // Code to benchmark goes here
                    // let y = black_box(x);
                    let mut avl = Rc::new(AVLTree::Node {
                        data: RefCell::new(Rc::new(black_box(&insertions[0]))),
                        left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                        height: RefCell::new(0),
                    });
    
    
                    for i in 0..insertions.len() {
                        avl = AVLTree::insert_node(&avl, &&insertions[i]);
                    }
                })
            },
        );
    }

    group.finish();    
}

fn avl_insert_block_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("avl_insert_block_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("avl_insert_block_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                
                b.iter(|| {
                    // Code to benchmark goes here
                    // let y = black_box(x);
                    let mut avl = Rc::new(AVLTree::Node {
                        data: RefCell::new(Rc::new(black_box(&insertions[0]))),
                        left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                        height: RefCell::new(0),
                    });
    
    
                    for i in 0..insertions.len() {
                        avl = AVLTree::insert_node(&avl, &&insertions[i]);
                    }
                })
            },
        );
    }

    group.finish();    
}


fn vanilla_bst_insert_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("vanilla_bst_insert_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("vanilla_bst_insert_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len()-100 {
                    rbt.insert_no_fix(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.insert_no_fix(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn vanilla_bst_insert_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("vanilla_bst_insert_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("vanilla_bst_insert_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len()-100 {
                    rbt.insert_no_fix(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.insert_no_fix(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn vanilla_bst_insert_block_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("vanilla_bst_insert_block_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("vanilla_bst_insert_block_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                
                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in 0..insertions.len() {
                        rbt.insert_no_fix(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn vanilla_bst_insert_block_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("vanilla_bst_insert_block_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("vanilla_bst_insert_block_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in 0..insertions.len() {
                        rbt.insert_no_fix(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}






fn red_black_search_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("red_black_search_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("red_black_search_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len() {
                    rbt.insert(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.search(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn red_black_search_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("red_black_search_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("red_black_search_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len() {
                    rbt.insert(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.search(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}



fn avl_search_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("avl_search_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("avl_search_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut avl = Rc::new(AVLTree::Node {
                    data: RefCell::new(Rc::new(black_box(&insertions[0]))),
                    left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    height: RefCell::new(0),
                });


                for i in 0..insertions.len() {
                    avl = AVLTree::insert_node(&avl, &&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        avl.search_node(black_box(&&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn avl_search_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("avl_search_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("avl_search_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut avl = Rc::new(AVLTree::Node {
                    data: RefCell::new(Rc::new(black_box(&insertions[0]))),
                    left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    height: RefCell::new(0),
                });


                for i in 0..insertions.len() {
                    avl = AVLTree::insert_node(&avl, &&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        avl.search_node(black_box(&&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();     
}


fn vanilla_bst_search_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("vanilla_bst_search_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("vanilla_bst_search_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len() {
                    rbt.insert_no_fix(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.search(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn vanilla_bst_search_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("vanilla_bst_search_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("vanilla_bst_search_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len() {
                    rbt.insert_no_fix(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.search(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}










criterion_group!(insert_random, red_black_insert_random, avl_insert_random, vanilla_bst_insert_random);
criterion_group!(insert_sequential, red_black_insert_sequential, avl_insert_sequential, vanilla_bst_insert_sequential);
criterion_group!(insert_block_random, red_black_insert_block_random, avl_insert_block_random, vanilla_bst_insert_block_random);
criterion_group!(insert_block_sequential, red_black_insert_block_sequential, avl_insert_block_sequential, vanilla_bst_insert_block_sequential);
criterion_group!(search_random, red_black_search_random, avl_search_random, vanilla_bst_search_random);
criterion_group!(search_sequential, red_black_search_sequential, avl_search_sequential, vanilla_bst_search_sequential);

criterion_main!(insert_random, insert_sequential, insert_block_random, insert_block_sequential, search_random, search_sequential);