use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum AVLTree<T: Ord> {
    Node {
        data: T,
        left_child: Rc<RefCell<AVLTree<T>>>,
        right_child: Rc<RefCell<AVLTree<T>>>,
        height: i32
    },
    Empty,
}

impl<T: Ord> AVLTree<T> {
    pub fn insert_node(&mut self, new_data: T) {
        // TODO: Dimas
        match self {
            AVLTree::Empty => {
                *self = AVLTree::Node {
                    data: new_data,
                    left_child: Rc::new(RefCell::new(AVLTree::Empty)),
                    right_child: Rc::new(RefCell::new(AVLTree::Empty)),
                    height: 1,
                };
            }
            AVLTree::Node { data, left_child, right_child, height } => {
                if new_data < *data {
                   let mut borrowed_node = left_child.borrow_mut();
                    (*borrowed_node).insert_node(new_data);
                } else if new_data > *data {
                    let mut borrowed_node = right_child.borrow_mut();
                    (*borrowed_node).insert_node(new_data);
                } else {
                    return;
                }
            }
        }
    }

    pub fn delete_node() {
        // TODO: Josh
    }

    pub fn rotation_left_left() {
        // TODO: Dimas
    }

    pub fn rotation_left_right() {
        // TODO: Dimas
    }

    pub fn rotation_right_left() {
        // TODO: Josh
    }

    pub fn rotation_right_right() {
        // TODO: Josh
    }

    pub fn leaf_number() {
        // TODO: Josh
    }

    pub fn tree_height() {
        // TODO: Dimas
    }

    pub fn print() {
        // TODO: Dimas
    }

    pub fn print_inorder() {
        // TODO: Josh
    }

    pub fn is_tree_empty() {
        // TODO: Dimas
    }
}