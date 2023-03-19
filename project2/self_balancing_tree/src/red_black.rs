use crate::readbt;
use readbt::ReadableBinaryTree;
use std::cell::{Ref, RefCell};
use std::cmp::max;
use std::cmp::Ord;
use std::fmt::Debug;
use std::fmt::Display;
use std::marker::Copy;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeColour {
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
    Empty,
}

impl<T: Ord + Copy + Debug> PartialEq for RedBlackTree<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            RedBlackTree::Node {
                colour,
                data,
                left_child,
                right_child,
            } => {
                let d1 = data.clone();
                match other {
                    RedBlackTree::Node {
                        colour,
                        data,
                        left_child,
                        right_child,
                    } => {
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
impl<D: Ord + Display + Copy + Debug> ReadableBinaryTree<D> for RedBlackTree<D> {
    // template methods must be implemented by implementors
    fn is_node_empty(&self) -> bool {
        match self {
            RedBlackTree::Empty => true,
            RedBlackTree::Node { .. } => false,
        }
    }

    // An Err in these functions signifies an empty node.
    fn immut_right_child(&self) -> Result<Ref<dyn ReadableBinaryTree<D>>, &str> {
        // returns right node for reading
        match self {
            RedBlackTree::Empty => return Result::Err("Node is empty and has no children"),
            RedBlackTree::Node { right_child, .. } => {
                // creates a new Ref to overcome the original Ref falling out of scope
                let rf = Ref::map((*right_child).borrow(), |t| &*t);
                return Result::Ok(rf);
            }
        }
    }
    fn immut_left_child(&self) -> Result<Ref<dyn ReadableBinaryTree<D>>, &str> {
        // returns left node for reading
        match self {
            RedBlackTree::Empty => return Result::Err("Node is empty and has no children"),
            RedBlackTree::Node { left_child, .. } => {
                // creates a new Ref to overcome the original Ref falling out of scope
                let rf = Ref::map((*left_child).borrow(), |t| &*t);
                return Result::Ok(rf);
            }
        }
    }
    fn immut_data_from_ref(&self) -> Result<&D, &str> {
        // returns data for reading
        match self {
            RedBlackTree::Empty => return Result::Err("Node is empty and has no data"),
            RedBlackTree::Node { data, .. } => return Result::Ok(&data),
        }
    }
    fn immut_meta_data_string(&self) -> Result<&str, &str> {
        // returns color information for printing
        match self {
            RedBlackTree::Empty => return Result::Err("Node is empty and has no children"),
            RedBlackTree::Node { colour, .. } => match colour {
                NodeColour::Red => {
                    return Result::Ok("R");
                }
                NodeColour::Black => {
                    return Result::Ok("B");
                }
            },
        }
    }
}

impl<T: Ord + Copy + Debug> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree::<T>::Empty
    }

    pub fn rotate_right(&mut self) {
        match self {
            RedBlackTree::Node {
                data,
                colour,
                left_child,
                right_child,
            } => {
                let mut lc = Rc::clone(&left_child);
                let lc_rc = match &*lc.borrow() {
                    RedBlackTree::Node {
                        data,
                        colour,
                        left_child,
                        right_child,
                    } => Rc::clone(&right_child),
                    RedBlackTree::Empty => {
                        return;
                    }
                };
                let mut oldself = self.clone();
                match oldself {
                    RedBlackTree::Node {
                        data,
                        colour,
                        ref mut left_child,
                        ref mut right_child,
                    } => {
                        *left_child = lc_rc;
                    }
                    RedBlackTree::Empty => {
                        return;
                    }
                }

                *self = lc.borrow().clone();
                match self {
                    RedBlackTree::Node {
                        data,
                        colour,
                        ref mut left_child,
                        ref mut right_child,
                    } => {
                        *right_child = Rc::new(RefCell::new(oldself));
                    }
                    RedBlackTree::Empty => {
                        return;
                    }
                }
            }
            RedBlackTree::Empty => {
                return;
            }
        }
    }

    pub fn rotate_left(&mut self) {
        match self {
            RedBlackTree::Node {
                data,
                colour,
                left_child,
                right_child,
            } => {
                let mut rc = Rc::clone(&right_child);
                let rc_lc = match &*rc.borrow() {
                    RedBlackTree::Node {
                        data,
                        colour,
                        left_child,
                        right_child,
                    } => Rc::clone(&left_child),
                    RedBlackTree::Empty => {
                        return;
                    }
                };
                let mut oldself = self.clone();
                match oldself {
                    RedBlackTree::Node {
                        data,
                        colour,
                        ref mut left_child,
                        ref mut right_child,
                    } => {
                        *right_child = rc_lc;
                    }
                    RedBlackTree::Empty => {
                        return;
                    }
                }

                *self = rc.borrow().clone();
                match self {
                    RedBlackTree::Node {
                        data,
                        colour,
                        ref mut left_child,
                        ref mut right_child,
                    } => {
                        *left_child = Rc::new(RefCell::new(oldself));
                    }
                    RedBlackTree::Empty => {
                        return;
                    }
                }
            }
            RedBlackTree::Empty => {
                return;
            }
        }
    }

    pub fn insert(&mut self, val: T) {
        match self {
            RedBlackTree::Node {
                data,
                colour,
                left_child,
                right_child,
            } => {}
            RedBlackTree::Empty => {
                *self = RedBlackTree::Node {
                    data: val,
                    colour: NodeColour::Black,
                    left_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
                    right_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
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
                RedBlackTree::Node {
                    data,
                    colour,
                    left_child,
                    right_child,
                } => {
                    if val > *data {
                        nodetemp = Rc::clone(&right_child);
                    } else if val < *data {
                        nodetemp = Rc::clone(&left_child);
                    } else {
                        return;
                    }
                }
                RedBlackTree::Empty => break,
            }
            node = Rc::clone(&nodetemp);
            stack.push(Rc::clone(&node));
        }
        // println!("{:#?}", stack);

        stack[stack.len() - 1].replace(RedBlackTree::Node {
            data: val.clone(),
            colour: NodeColour::Red,
            left_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
            right_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
        });
        ///// END BINARY TREE INSERT //////

        ///// FIX TREE //////
        let mut idx = stack.len() - 1;
        let mut uncle: Rc<RefCell<RedBlackTree<T>>>;
        let mut grandfather: Rc<RefCell<RedBlackTree<T>>>;
        let mut parent: Rc<RefCell<RedBlackTree<T>>>;
        while idx >= 2 {
            // we got the grandfather
            grandfather = Rc::clone(&stack[idx - 2]);
            let mut gf_left = Rc::new(RefCell::new(RedBlackTree::Empty));
            let mut gf_right = Rc::new(RefCell::new(RedBlackTree::Empty));

            let mut gf_val = match &*grandfather.borrow() {
                RedBlackTree::Node {
                    data,
                    colour,
                    left_child,
                    right_child,
                } => {
                    gf_left = Rc::clone(&left_child);
                    gf_right = Rc::clone(&right_child);
                    data.clone()
                }
                RedBlackTree::Empty => val, // not possible
            };
            if val > gf_val {
                uncle = Rc::clone(&gf_left);
            } else {
                uncle = Rc::clone(&gf_right);
            }
            // now we got the uncle

            parent = Rc::clone(&stack[idx - 1]);
            // and we got the parent

            // If parent is black, then we break
            if NodeColour::Black
                == match &*parent.borrow() {
                    RedBlackTree::Node {
                        data,
                        colour,
                        left_child,
                        right_child,
                    } => *colour,
                    RedBlackTree::Empty => NodeColour::Black,
                }
            {
                break;
            }
            // otherwise...
            // if uncle is red
            if NodeColour::Red
                == match &*uncle.borrow() {
                    RedBlackTree::Node {
                        data,
                        colour,
                        left_child,
                        right_child,
                    } => *colour,
                    RedBlackTree::Empty => NodeColour::Black,
                }
            {
                // Set uncle and parent as black
                match *parent.borrow_mut() {
                    RedBlackTree::Node {
                        ref data,
                        ref mut colour,
                        ref left_child,
                        ref right_child,
                    } => *colour = NodeColour::Black,
                    RedBlackTree::Empty => {}
                }
                match *uncle.borrow_mut() {
                    RedBlackTree::Node {
                        ref data,
                        ref mut colour,
                        ref left_child,
                        ref right_child,
                    } => *colour = NodeColour::Black,
                    RedBlackTree::Empty => {}
                }
                // set grandfather as Red
                if idx - 2 > 0 {
                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = NodeColour::Red,
                        RedBlackTree::Empty => {}
                    }
                }

                idx = idx - 2;

                // println!("\n\n\n ROOT\n{:#?}", stack[0]);
            } else {
                // else if uncle is black
                // we have 4 cases

                // If grandparent.left.left is the current node
                if (match &*grandfather.borrow() {
                    RedBlackTree::Node {
                        data,
                        mut colour,
                        left_child,
                        right_child,
                    } => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent))
                    && (match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            mut colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    } == Rc::clone(&stack[idx]))
                {
                    // right rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp.rotate_right();
                    parent.replace(gf_temp);

                    grandfather = match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // then swap colours of grandfather and parent
                    let mut gf_colour = match &*grandfather.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let mut p_colour = match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = p_colour,
                        RedBlackTree::Empty => {}
                    };
                    match *parent.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = gf_colour,
                        RedBlackTree::Empty => {}
                    };

                    if idx - 2 == 0 {
                        *self = parent.borrow().clone();
                        return;
                    } else {
                        // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {
                                ref data,
                                ref colour,
                                ref mut left_child,
                                ref mut right_child,
                            } => {
                                let upperval = data.clone();
                                let parent_val = match &*parent.borrow() {
                                    RedBlackTree::Node {
                                        data,
                                        colour,
                                        left_child,
                                        right_child,
                                    } => *data,
                                    RedBlackTree::Empty => upperval.clone(),
                                };
                                if parent_val > upperval {
                                    *right_child = Rc::clone(&parent);
                                } else if parent_val < upperval {
                                    *left_child = Rc::clone(&parent);
                                }
                            }
                            RedBlackTree::Empty => {}
                        };
                    }
                }
                // If grandparent.left.right is the current node
                else if (match &*grandfather.borrow() {
                    RedBlackTree::Node {
                        data,
                        mut colour,
                        left_child,
                        right_child,
                    } => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent))
                    && (match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            mut colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    } == Rc::clone(&stack[idx]))
                {
                    // left rotate on parent
                    // println!("\n\n\n PARENT OLD:");
                    // parent.borrow().display_tree();
                    // println!("\n\n\n CURRENT OLD:");
                    // stack[idx].borrow().display_tree();

                    let mut p_temp = parent.borrow().clone();
                    p_temp.rotate_left();
                    stack[idx].replace(p_temp);

                    parent = match &*stack[idx].borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref colour,
                            ref mut left_child,
                            ref right_child,
                        } => *left_child = Rc::clone(&stack[idx]),
                        RedBlackTree::Empty => {}
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

                    grandfather = match &*stack[idx].borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // println!("\n\n\n GRANDPARENT NEW:");
                    // grandfather.borrow().display_tree();

                    // println!("\n\n\n CURRENT NEW:");
                    // stack[idx].borrow().display_tree();

                    // then swap colours of grandfather and current
                    let mut gf_colour = match &*grandfather.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let mut c_colour = match &*stack[idx].borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = c_colour,
                        RedBlackTree::Empty => {}
                    };

                    match *stack[idx].borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = gf_colour,
                        RedBlackTree::Empty => {}
                    };

                    // if new insert is root, return
                    if idx - 2 == 0 {
                        *self = stack[idx].borrow().clone();
                        return;
                    } else {
                        // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {
                                ref data,
                                ref colour,
                                ref mut left_child,
                                ref mut right_child,
                            } => {
                                let upperval = data.clone();
                                let insert_val = match &*stack[idx].borrow() {
                                    RedBlackTree::Node {
                                        data,
                                        colour,
                                        left_child,
                                        right_child,
                                    } => *data,
                                    RedBlackTree::Empty => upperval.clone(),
                                };
                                if insert_val > upperval {
                                    *right_child = Rc::clone(&stack[idx]);
                                } else if insert_val < upperval {
                                    *left_child = Rc::clone(&stack[idx]);
                                }
                            }
                            RedBlackTree::Empty => {}
                        };
                    }
                }
                // If grandparent.right.left is the current node
                else if (match &*grandfather.borrow() {
                    RedBlackTree::Node {
                        data,
                        mut colour,
                        left_child,
                        right_child,
                    } => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent))
                    && (match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            mut colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    } == Rc::clone(&stack[idx]))
                {
                    // right rotate on parent
                    let mut p_temp = parent.borrow().clone();
                    p_temp.rotate_right();
                    stack[idx].replace(p_temp);

                    parent = match &*stack[idx].borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref colour,
                            ref left_child,
                            ref mut right_child,
                        } => *right_child = Rc::clone(&stack[idx]),
                        RedBlackTree::Empty => {}
                    };

                    // left rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp.rotate_left();
                    stack[idx].replace(gf_temp);

                    grandfather = match &*stack[idx].borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // then swap colours of grandfather and current
                    let mut gf_colour = match &*grandfather.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let mut c_colour = match &*stack[idx].borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };

                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = c_colour,
                        RedBlackTree::Empty => {}
                    };

                    match *stack[idx].borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = gf_colour,
                        RedBlackTree::Empty => {}
                    };

                    // if new insert is root, return
                    if idx - 2 == 0 {
                        *self = stack[idx].borrow().clone();
                        return;
                    } else {
                        // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {
                                ref data,
                                ref colour,
                                ref mut left_child,
                                ref mut right_child,
                            } => {
                                let upperval = data.clone();
                                let insert_val = match &*stack[idx].borrow() {
                                    RedBlackTree::Node {
                                        data,
                                        colour,
                                        left_child,
                                        right_child,
                                    } => *data,
                                    RedBlackTree::Empty => upperval.clone(),
                                };
                                if insert_val > upperval {
                                    *right_child = Rc::clone(&stack[idx]);
                                } else if insert_val < upperval {
                                    *left_child = Rc::clone(&stack[idx]);
                                }
                            }
                            RedBlackTree::Empty => {}
                        };
                    }
                }
                // If grandparent.right.right is the current node
                else if (match &*grandfather.borrow() {
                    RedBlackTree::Node {
                        data,
                        mut colour,
                        left_child,
                        right_child,
                    } => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent))
                    && (match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            mut colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    } == Rc::clone(&stack[idx]))
                {
                    // left rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp.rotate_left();
                    parent.replace(gf_temp);

                    grandfather = match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // then swap colours of grandfather and parent
                    let gf_colour = match &*grandfather.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let p_colour = match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };

                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = p_colour,
                        RedBlackTree::Empty => {}
                    };

                    match *parent.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = gf_colour,
                        RedBlackTree::Empty => {}
                    };

                    // if parent is root, return
                    if idx - 2 == 0 {
                        *self = parent.borrow().clone();
                        return;
                    } else {
                        // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {
                                ref data,
                                ref colour,
                                ref mut left_child,
                                ref mut right_child,
                            } => {
                                let upperval = data.clone();
                                let parent_val = match &*parent.borrow() {
                                    RedBlackTree::Node {
                                        data,
                                        colour,
                                        left_child,
                                        right_child,
                                    } => *data,
                                    RedBlackTree::Empty => upperval.clone(),
                                };
                                if parent_val > upperval {
                                    *right_child = Rc::clone(&parent);
                                } else if parent_val < upperval {
                                    *left_child = Rc::clone(&parent);
                                }
                            }
                            RedBlackTree::Empty => {}
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
}
