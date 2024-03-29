use crate::readbt;
use readbt::ReadableBinaryTree;
use std::cell::{Ref, RefCell};
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

    pub fn is_empty(&self) -> bool {
        match self {
            RedBlackTree::Node { .. } => false,
            RedBlackTree::Empty => true,
        }
    }

    pub fn get_left_child(&self) -> &Rc<RefCell<RedBlackTree<T>>> {
        match self {
            RedBlackTree::Node { left_child, .. } => {
                return left_child;
            }
            RedBlackTree::Empty => {
                panic!("Tree is empty!");
            }
        }
    }

    pub fn get_right_child(&self) -> &Rc<RefCell<RedBlackTree<T>>> {
        match self {
            RedBlackTree::Node { right_child, .. } => {
                return right_child;
            }
            RedBlackTree::Empty => {
                panic!("Tree is empty!");
            }
        }
    }

    pub fn set_left_child(&mut self, child: Rc<RefCell<RedBlackTree<T>>>) {
        match self {
            RedBlackTree::Node {
                colour,
                data,
                left_child,
                right_child,
            } => {
                *left_child = child;
            }
            RedBlackTree::Empty => {
                panic!("Node is empty!");
            }
        }
    }

    pub fn set_right_child(&mut self, child: Rc<RefCell<RedBlackTree<T>>>) {
        match self {
            RedBlackTree::Node {
                colour,
                data,
                left_child,
                right_child,
            } => {
                *right_child = child;
            }
            RedBlackTree::Empty => {
                panic!("Node is empty!");
            }
        }
    }

    fn get_data(&self) -> &T {
        match self {
            RedBlackTree::Node { data, .. } => {
                return data;
            }
            RedBlackTree::Empty => {
                panic!("Tree is empty!");
            }
        }
    }

    fn set_data(&mut self, new_data: T) {
        match self {
            RedBlackTree::Node { data, .. } => {
                *data = new_data;
            }
            RedBlackTree::Empty => {
                panic!("Tree is empty!");
            }
        }
    }

    fn get_colour(&self) -> &NodeColour {
        match self {
            RedBlackTree::Node { colour, .. } => colour,
            RedBlackTree::Empty => {
                // Null nodes are black
                &NodeColour::Black
            }
        }
    }

    fn set_colour(&mut self, new_colour: NodeColour) {
        match self {
            RedBlackTree::Node { colour, .. } => {
                *colour = new_colour;
            }
            RedBlackTree::Empty => {
                // set color of empty node
            }
        }
    }

    pub fn delete(&mut self, val: T) {
        fn delete_fixup<T: Ord + Copy + Debug>(stack: &Vec<Rc<RefCell<RedBlackTree<T>>>>) {
            let mut index = stack.len() - 1;
            let mut x = Rc::clone(&stack[index]);
            let mut x_parent = Rc::clone(&stack[index]); // temp
            if index != 0 {
                x_parent = Rc::clone(&stack[index - 1]);
            }
            let mut x_parent_left = Rc::clone(&stack[index]); // temp
            let mut x_parent_right = Rc::clone(&stack[index]); // temp
            match &*x_parent.borrow() {
                RedBlackTree::Node {
                    left_child,
                    right_child,
                    ..
                } => {
                    x_parent_left = Rc::clone(&left_child);
                    x_parent_right = Rc::clone(&right_child);
                }
                RedBlackTree::Empty => {
                    panic!("Tree is empty!");
                }
            }

            while index != 0 && *x.borrow().get_colour() == NodeColour::Black {
                if x == x_parent_left {
                    let mut s = Rc::clone(&x_parent_right);

                    let mut s_left = Rc::clone(&stack[index]); // temp
                    let mut s_right = Rc::clone(&stack[index]); // temp
                    match &*s.borrow() {
                        RedBlackTree::Node {
                            left_child,
                            right_child,
                            ..
                        } => {
                            s_left = Rc::clone(&left_child);
                            s_right = Rc::clone(&right_child);
                        }
                        RedBlackTree::Empty => {
                            s_left = Rc::new(RefCell::new(RedBlackTree::Empty));
                            s_right = Rc::new(RefCell::new(RedBlackTree::Empty));
                        }
                    }

                    if *s.borrow().get_colour() == NodeColour::Red {
                        s.borrow_mut().set_colour(NodeColour::Black);
                        x_parent.borrow_mut().set_colour(NodeColour::Red);
                        x_parent.borrow_mut().rotate_left();
                        s = Rc::clone(&x_parent_right);
                    }

                    if *s_left.borrow().get_colour() == NodeColour::Black
                        && *s_right.borrow().get_colour() == NodeColour::Black
                    {
                        s.borrow_mut().set_colour(NodeColour::Red);
                        x = Rc::clone(&x_parent);
                        index = index - 1;
                    } else {
                        if *s_right.borrow().get_colour() == NodeColour::Black {
                            s_left.borrow_mut().set_colour(NodeColour::Black);
                            s.borrow_mut().set_colour(NodeColour::Red);
                            s.borrow_mut().rotate_right();
                            s = Rc::clone(&x_parent_right);
                        }

                        s.borrow_mut().set_colour(*x_parent.borrow().get_colour());
                        x_parent.borrow_mut().set_colour(NodeColour::Black);
                        s_right.borrow_mut().set_colour(NodeColour::Black);
                        x_parent.borrow_mut().rotate_left();
                        index = 0;
                        x = Rc::clone(&stack[0]);
                    }
                } else {
                    // same thing other side, just with left and right swapped
                    let mut s = Rc::clone(&x_parent_left);

                    let mut s_left = Rc::clone(&stack[index]); // temp
                    let mut s_right = Rc::clone(&stack[index]); // temp
                    match &*s.borrow() {
                        RedBlackTree::Node {
                            left_child,
                            right_child,
                            ..
                        } => {
                            s_left = Rc::clone(&left_child);
                            s_right = Rc::clone(&right_child);
                        }
                        RedBlackTree::Empty => {
                            s_left = Rc::new(RefCell::new(RedBlackTree::Empty));
                            s_right = Rc::new(RefCell::new(RedBlackTree::Empty));
                        }
                    }

                    if *s.borrow().get_colour() == NodeColour::Red {
                        s.borrow_mut().set_colour(NodeColour::Black);
                        x_parent.borrow_mut().set_colour(NodeColour::Red);
                        x_parent.borrow_mut().rotate_right();
                        s = Rc::clone(&x_parent_left);
                    }

                    if *s_left.borrow().get_colour() == NodeColour::Black
                        && *s_right.borrow().get_colour() == NodeColour::Black
                    {
                        s.borrow_mut().set_colour(NodeColour::Red);
                        x = Rc::clone(&x_parent);
                        index = index - 1;
                    } else {
                        if *s_left.borrow().get_colour() == NodeColour::Black {
                            s_right.borrow_mut().set_colour(NodeColour::Black);
                            s.borrow_mut().set_colour(NodeColour::Red);
                            s.borrow_mut().rotate_left();
                            s = Rc::clone(&x_parent_left);
                        }

                        s.borrow_mut().set_colour(*x_parent.borrow().get_colour());
                        x_parent.borrow_mut().set_colour(NodeColour::Black);
                        s_left.borrow_mut().set_colour(NodeColour::Black);
                        x_parent.borrow_mut().rotate_right();
                        index = 0;
                        x = Rc::clone(&stack[0]);
                    }
                }
            }
            x.borrow_mut().set_colour(NodeColour::Black);
        }

        // Search through binary tree to find this value
        // Track the path down the tree in a stack
        let mut stack = vec![Rc::new(RefCell::new(self.clone()))];
        let mut node = Rc::new(RefCell::new(self.clone()));

        let mut nodetemp = Rc::clone(&node);
        let mut parent = None;
        let mut is_left_child = false;
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
                        is_left_child = false;
                    } else if val < *data {
                        nodetemp = Rc::clone(&left_child);
                        is_left_child = true;
                    } else {
                        break;
                    }
                }
                RedBlackTree::Empty => {
                    return;
                }
            }
            parent = Some(Rc::clone(&node));
            node = Rc::clone(&nodetemp);
            stack.push(Rc::clone(&node));
        }

        nodetemp = Rc::new(RefCell::new(node.borrow().clone()));

        let is_root = match parent {
            Some(ref p) => false,
            None => true,
        };

        let binding = nodetemp.borrow();
        let del_node_color = &mut binding.get_colour().clone();

        match &*nodetemp.borrow() {
            RedBlackTree::Node {
                data,
                colour,
                left_child,
                right_child,
            } => {
                if left_child.borrow().is_empty() {
                    // Node has child on right
                    stack.pop();
                    stack.push(Rc::clone(&right_child));
                    // Replace the node with the child
                    if true || stack.len() > 2 {
                        match parent {
                            Some(ref p) => {
                                if is_left_child {
                                    (*p).borrow_mut().set_left_child(Rc::clone(&right_child));
                                } else {
                                    (*p).borrow_mut().set_right_child(Rc::clone(&right_child));
                                }
                            }
                            None => {
                                stack[0] = Rc::clone(&right_child);
                            }
                        }
                    } else {
                        // Parent is the root! We need to handle this differently.
                    }
                } else if right_child.borrow().is_empty() {
                    // Node has child on left
                    stack.pop();
                    stack.push(Rc::clone(&left_child));
                    // Replace the node with the child
                    if true || stack.len() > 2 {
                        match parent {
                            Some(ref p) => {
                                if is_left_child {
                                    (*p).borrow_mut().set_left_child(Rc::clone(&left_child));
                                } else {
                                    (*p).borrow_mut().set_right_child(Rc::clone(&left_child));
                                }
                            }
                            None => {
                                stack[0] = Rc::clone(&left_child);
                            }
                        }
                    } else {
                        // Parent is the root! We need to handle this differently.
                    }
                } else {
                    // Node has two children (or no children)
                    // Replace the node with its in-order successor, which is the leftmost node in the right subtree. Then delete the in-order successor node as if it has at most one child.
                    let mut new_successor = Rc::clone(&left_child); // temp
                    match &*node.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => {
                            new_successor = Rc::clone(&right_child);
                            stack.push(Rc::clone(&new_successor));
                        }
                        RedBlackTree::Empty => {
                            // Bad case
                        }
                    }
                    let mut successor_parent = None;

                    while {
                        let borrow_new_successor = new_successor.borrow();
                        let while_condition =
                            !borrow_new_successor.get_left_child().borrow().is_empty();
                        while_condition
                    } {
                        successor_parent = Some(Rc::clone(&new_successor));
                        let temp = Rc::clone(&(new_successor.borrow().get_left_child()));
                        new_successor = temp;
                        stack.push(Rc::clone(&new_successor));
                    }
                    let mut successor = Rc::clone(&new_successor);
                    *del_node_color = new_successor.borrow().get_colour().clone();
                    stack.push(Rc::clone(&(successor.borrow().get_right_child())));

                    // Swap the value to delete with the value of the successor node
                    let temp = (*(node.borrow().get_data())).clone();
                    let temp_data = (*(successor.borrow().get_data())).clone();
                    (*node).borrow_mut().set_data(temp_data);
                    (*successor).borrow_mut().set_data(temp);
                    if is_root {
                        // Case where thing was root, cloning to replace root"
                    }

                    // Delete the successor node as if it has at most one child
                    let successor_child = Rc::clone(&successor.borrow().get_right_child());

                    // Replace the node with the successor node, whether that successor is empty or not!
                    match successor_parent {
                        Some(ref p) => {
                            // We know this will be the left child
                            (*p).borrow_mut().set_left_child(successor_child);
                        }
                        None => {
                            // This means the successor is just the right child of the node
                            if node.borrow().get_data() == (*self).get_data() {
                                // Because the node is the root, this is the only node with a clone copy instead of the actual thing, so we need to set self instead of the reference
                                (*node).borrow_mut().set_right_child(successor_child);
                            } else {
                                (*node).borrow_mut().set_right_child(successor_child);
                            }
                        }
                    }
                }
            }
            RedBlackTree::Empty => {
                return;
            }
        };

        if *del_node_color == NodeColour::Black {
            delete_fixup(&stack);
        }

        if is_root {
            *self = stack[0].borrow().clone();
        }
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

        *self = stack[0].borrow().clone();
    }

    pub fn insert_no_fix(&mut self, val: T) {
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

        let mut node = Rc::new(RefCell::new(self.clone()));
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
        }

        node.replace(RedBlackTree::Node {
            data: val.clone(),
            colour: NodeColour::Red,
            left_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
            right_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
        });
        ///// END BINARY TREE INSERT //////
    }
}
