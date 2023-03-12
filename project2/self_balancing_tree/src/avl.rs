use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;

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
                    (*(left_child.borrow_mut())).insert_node(new_data);
                } else if new_data > *data {
                    (*(right_child.borrow_mut())).insert_node(new_data);
                } else {
                    return;
                }
            }
        }

        // Step 2 - Update the height of the ancestor node

        // Step 3 - Get the balance factor

        // Step 4 - If the node is unbalanced, then try out the 4 cases

    }

    pub fn delete_node() {
        // TODO: Josh
    }

    pub fn rotation_left() {

    }

    pub fn rotation_right() {
        
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

    pub fn tree_height(self) -> i32 {
        match self {
            AVLTree::Empty => 0,
            AVLTree::Node { data, left_child, right_child, height } => height,
        }
    }

    pub fn is_tree_empty(self) -> bool {
        match self {
            AVLTree::Empty => true,
            AVLTree::Node { data, left_child, right_child, height } => false,
        }
    }

    pub fn print() {
        // TODO: Dimas
    }

    pub fn print_inorder() {
        // TODO: Josh
    }
}