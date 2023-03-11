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

impl<T: Ord + std::fmt::Display> AVLTree<T> {
    // pub fn insert_node(&mut self, new_data: T) {
    //     // TODO: Dimas
    //     match self {
    //         AVLTree::Empty => {
    //             *self = AVLTree::Node {
    //                 data: new_data,
    //                 left_child: Rc::new(RefCell::new(AVLTree::Empty)),
    //                 right_child: Rc::new(RefCell::new(AVLTree::Empty)),
    //                 height: 1,
    //             };
    //         }
    //         AVLTree::Node { data, left_child, right_child, height } => {
    //             if new_data < *data {
    //                let mut borrowed_node = left_child.borrow_mut();
    //                 (*borrowed_node).insert_node(new_data);
    //             } else if new_data > *data {
    //                 let mut borrowed_node = right_child.borrow_mut();
    //                 (*borrowed_node).insert_node(new_data);
    //             } else {
    //                 return;
    //             }
    //         }
    //     }
    // }

    // pub fn delete_node() {
    //     // TODO: Josh
    //     match self {
    //         AVLTree::Empty => {
    //             panic!("Node does not exist in tree")
    //         }
    //         AVLTree::Node { data, left_child, right_child } => {
    //             if new_data < *data {
    //                 left_child.delete_node(new_data);
    //             } else if new_data > *data {
    //                 right_child.delete_node(new_data);
    //             } else {
    //                 return;
    //             }
    //         }
    //     }
    // }

    // pub fn rotation_left_left() {
    //     // TODO: Dimas
    // }

    // pub fn rotation_left_right() {
    //     // TODO: Dimas
    // }

    // pub fn rotation_right_left() {
    //     // TODO: Josh
    // }

    // pub fn rotation_right_right() {
    //     // TODO: Josh
    // }

    // pub fn leaf_number(&self) {
    //     // TODO: Josh

    //     fn dfs(node){
    //         if ( node.left.borrow() == AVLTree::Empty &&
    //         node.right.borrow() == AVLTree::Empty )
    //             return 0
    //         else:
    //             return dfs(node.left.borrow()) + dfs(node.right.borrow());
    //     }
    // }

    // pub fn tree_height() {
    //     // TODO: Dimas
    // }

    // pub fn print() {
    //     // TODO: Dimas
    // }

    pub fn print_inorder(&self) {
        // TODO: Josh
        match self {
            AVLTree::Node {
                data,
                left_child,
                right_child,
                height
            } => {
                (*left_child).borrow().print_inorder();
                println!("{}", data);
                (*right_child).borrow().print_inorder();
            }
            AVLTree::Empty => return,
        }
    }

    // pub fn is_tree_empty() {
    //     // TODO: Dimas
    // }
}