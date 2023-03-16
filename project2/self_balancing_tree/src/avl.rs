use crate::AVLTree::*;
use std::cell::RefCell;
use std::cmp::max;
use std::fmt::Display;
use std::rc::Rc;

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
                            Empty => {
                                panic!("Given tree is not a proper AVL tree");
                            }
                            Node {
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
                            Empty => {
                                panic!("Given tree is not a proper AVL tree");
                            }
                            Node {
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

    pub fn delete_node(node_rc: &Rc<AVLTree<T>>, targetValue: &T) -> Rc<AVLTree<T>> {
        // recursively deletes the node with the target value if it exists and returns the new root

        match &**node_rc {
            AVLTree::Empty => {
                return Rc::clone(node_rc);
            } // base case
            AVLTree::Node {
                data,
                left_child: left_child_ref,
                right_child: right_child_ref,
                ..
            } => {
                if *targetValue < *Rc::clone(&*data.borrow()) {
                    let new_node =
                        AVLTree::delete_node(&Rc::clone(&*left_child_ref.borrow()), targetValue);
                    left_child_ref.replace(new_node);
                } else if *targetValue > *Rc::clone(&*data.borrow()) {
                    let new_node =
                        AVLTree::delete_node(&Rc::clone(&*right_child_ref.borrow()), targetValue);
                    right_child_ref.replace(new_node);
                } else {
                    let left_node = &*Rc::clone(&*left_child_ref.borrow());
                    let right_node = &*Rc::clone(&*right_child_ref.borrow());
                    // target node found
                    match left_node {
                        Empty => {
                            match right_node {
                                Empty => {
                                    // both children are empty
                                    return Rc::clone(&*right_child_ref.borrow());
                                }
                                Node { .. } => {
                                    // only the left is empty
                                    return Rc::clone(&*right_child_ref.borrow());
                                }
                            }
                        }
                        Node { .. } => {
                            match right_node {
                                Empty => {
                                    // only the right is empty
                                    return Rc::clone(&*left_child_ref.borrow());
                                }
                                Node {
                                    data: right_node_data,
                                    ..
                                } => {
                                    // both are not empty

                                    // steal right child's value...
                                    data.replace(Rc::clone(&*right_node_data.borrow()));

                                    // delete right child recursively since we just stole it's value
                                    let new_right = AVLTree::delete_node(
                                        &Rc::clone(&*right_child_ref.borrow()),
                                        &**right_node_data.borrow(),
                                    );
                                    right_child_ref.replace(new_right);
                                    (*node_rc).update_heights();
                                }
                            }
                        }
                    }
                }
                // balance
                let return_node_rc = AVLTree::delete_node_balance(node_rc);
                (*return_node_rc).update_heights();
                return return_node_rc;
            }
        }
    }

    pub fn delete_node_balance(node_rc: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        // balances the tree for deletion case
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
                            Empty => {
                                panic!("Given tree is not a proper AVL tree");
                            }
                            Node {
                                left_child: y_left_child_ref,
                                right_child: y_right_child_ref,
                                ..
                            } => {
                                let y_left_height = (*y_left_child_ref.borrow()).get_height();
                                let y_right_height = (*y_right_child_ref.borrow()).get_height();
                                if y_left_height > y_right_height {
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
                            Empty => {
                                panic!("Given tree is not a proper AVL tree");
                            }
                            Node {
                                left_child: y_left_child_ref,
                                right_child: y_right_child_ref,
                                ..
                            } => {
                                let y_left_height = (*y_left_child_ref.borrow()).get_height();
                                let y_right_height = (*y_right_child_ref.borrow()).get_height();
                                if y_left_height > y_right_height {
                                    // right-left case
                                    println!("in right left");
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
            Empty => {
                panic!("Invalid AVLTree for left right rotation")
            }
            Node { left_child, .. } => {
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
            Empty => {
                panic!("Invalid AVLTree for right left rotation")
            }
            Node { right_child, .. } => {
                let new_right = AVLTree::rotate_right(&(right_child.borrow()));
                right_child.replace(new_right);
                return AVLTree::rotate_left(z);
            }
        }
    }

    pub fn rotate_right(z_rc: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        println!("rotating right");

        // EX:   z
        //      /
        //     y
        //    / \
        //   x   N
        let z = &(**z_rc);
        match z {
            Empty => Rc::clone(z_rc),
            Node {
                left_child: z_left_child,
                ..
            } => {
                let y_rc = Rc::clone(&z_left_child.borrow());
                let y = &(*y_rc);
                match y {
                    Empty => Rc::clone(z_rc),
                    Node {
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
        println!("rotating left");

        // EX:   z
        //        \
        //         y
        //        / \
        //       n   x
        let z = &(**z_rc);
        match z {
            Empty => Rc::clone(z_rc),
            Node {
                right_child: z_right_child,
                ..
            } => {
                let y_rc = Rc::clone(&z_right_child.borrow());
                let y = &(*y_rc);
                match y {
                    Empty => Rc::clone(z_rc),
                    Node {
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
            Empty => {}
            Node {
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
            Empty => {
                return -1;
            }
            Node { height, .. } => {
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
                Empty => {
                    panic!("leaf_number failed")
                }
                Node {
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
}
