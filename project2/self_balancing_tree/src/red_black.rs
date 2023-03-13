use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ord;
use std::cmp::max;
use std::marker::Copy;
use std::fmt::Debug;

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


impl <T: Ord + Copy + Debug> PartialEq for RedBlackTree<T>
{
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

    pub fn new(data: T) -> Self {
        RedBlackTree::<T>::Node {
            colour: NodeColour::Black, 
            data: data,
            left_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
            right_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
        }
    }


    pub fn count_leaves(&self) -> i32 {
        // Recursively count leaves
        match self {
            RedBlackTree::Node {colour, data, left_child, right_child} => {
                return left_child.borrow().count_leaves() + right_child.borrow().count_leaves();
            },
            RedBlackTree::Empty => {
                return 1;
            },
        }
    }

    pub fn get_height(&self) -> i32 {
        // Recursively get height
        // This counts nodes and not edges (e.g. a tree with one node has height 1, and not 0)
        match self {
            RedBlackTree::Node {colour, data, left_child, right_child} => {
                return max(left_child.borrow().get_height(), right_child.borrow().get_height()) + 1;
            },
            RedBlackTree::Empty => {
                return 1;
            },

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
                let bl_branch = match &*right_child.borrow() {
                    RedBlackTree::Node {colour, data, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                let br_branch = match &*right_child.borrow() {
                    RedBlackTree::Node {colour, data, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                
                let datatemp = &data;
                let colourtemp = &colour;

                
                new_parent = match &*right_child.borrow() {
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
                let br_branch = match &*left_child.borrow() {
                    RedBlackTree::Node {colour, data, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                let bl_branch = match &*left_child.borrow() {
                    RedBlackTree::Node {colour, data, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                
                let datatemp = &data;
                let colourtemp = &colour;

                
                new_parent = match &*left_child.borrow() {
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


    pub fn insert(mut self, val: T) -> Self {
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
                        return self;
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
                RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                RedBlackTree::Empty => NodeColour::Black,
            } {
                break;
            }
            // otherwise...
            // if uncle is red
            if NodeColour::Red == match &*uncle.borrow() {
                RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
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
                    gf_temp = gf_temp.rotate_right();
                    parent.replace(gf_temp);

                    grandfather = match &*parent.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // then swap colours of grandfather and parent
                    let mut gf_colour = match &*grandfather.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let mut p_colour = match &*parent.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    match &*grandfather.borrow(){
                        RedBlackTree::Node {data, mut colour, left_child, right_child} => {colour = p_colour},
                        RedBlackTree::Empty => {},
                    };
                    match &*parent.borrow(){
                        RedBlackTree::Node {data, mut colour, left_child, right_child} => {colour = gf_colour},
                        RedBlackTree::Empty => {},
                    };

                    if idx - 2 == 0 {
                        return parent.borrow().clone()
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
                    println!("\n\n\n PARENT OLD: {:#?}", parent);
                    println!("\n\n\n CURRENT OLD: {:#?}", stack[idx]);
                    let mut p_temp = parent.borrow().clone();
                    p_temp = p_temp.rotate_left();
                    stack[idx].replace(p_temp);

                    parent = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {ref data, ref colour, ref mut left_child, ref right_child} => {*left_child = Rc::clone(&stack[idx])},
                        RedBlackTree::Empty => {},
                    };


                    
                    println!("\n\n\n PARENT NEW: {:#?}", parent);
                    println!("\n\n\n CURRENT NEW: {:#?}", stack[idx]);

                    println!("\n\n\n GRANDPARENT OLD: {:#?}", grandfather);
                    println!("\n\n\n CURRENT OLD: {:#?}", stack[idx]);


                    // right rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp = gf_temp.rotate_right();
                    stack[idx].replace(gf_temp);

                    grandfather = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    println!("\n\n\n GRANDPARENT NEW: {:#?}", grandfather);
                    println!("\n\n\n CURRENT NEW: {:#?}", stack[idx]);

                    // then swap colours of grandfather and current
                    let mut gf_colour = match &*grandfather.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let mut c_colour = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
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
                        return stack[idx].borrow().clone()
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
                    p_temp = p_temp.rotate_right();
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
                    gf_temp = gf_temp.rotate_left();
                    stack[idx].replace(gf_temp);

                    grandfather = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // then swap colours of grandfather and current
                    let mut gf_colour = match &*grandfather.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let mut c_colour = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
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
                        return stack[idx].borrow().clone()
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
                    gf_temp = gf_temp.rotate_left();
                    parent.replace(gf_temp);

                    grandfather = match &*parent.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };


                    // then swap colours of grandfather and parent
                    let gf_colour = match &*grandfather.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let p_colour = match &*parent.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
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
                        return parent.borrow().clone()
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
        let root = stack[0].borrow().clone(); 
        root
    }



}
