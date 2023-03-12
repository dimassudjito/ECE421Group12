use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::max;
use crate::AVLTree::*;

#[derive(Debug)]
pub enum AVLTree<T: Ord> {
    Node {
        data: RefCell<T>,
        left_child: RefCell<Rc<AVLTree<T>>>,
        right_child: RefCell<Rc<AVLTree<T>>>,
        height: RefCell<i32>
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

    pub fn rotate_right(z_rc: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>>{
        // TODO: Josh  make non public later

        // EX:   z
        //      /
        //     y
        //    / \
        //   x   N
        let z = &(**z_rc);
        match z{
            Empty =>{ Rc::clone(z_rc) },
            Node {
                left_child:z_left_child,
                ..
            }=>{
                let y_rc = Rc::clone(&z_left_child.borrow());
                let y  = &(*y_rc);
                match y{
                    Empty=>{ Rc::clone(z_rc)  },
                    Node {  
                        right_child:y_right_child,
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
            },
        }
    }

    pub fn update_heights(& self){
        match self{
            Empty => {},
            Node {   
                left_child,
                right_child,
                height,
                ..
            } =>{
                let left_val;
                let right_val;
                match &(**left_child.borrow()){
                    Empty => {left_val = -1}
                    Node{height,..}=>{left_val = *height.borrow()}
                }
                    match  &(**right_child.borrow()){
                    Empty => {right_val = -1}
                    Node {height,..}=>{right_val = *height.borrow()}
                }
                height.replace(max(left_val,right_val) + 1);
            }
        }
    }

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
                ..
            } => {
                (*left_child.borrow()).print_inorder();
                println!("{}", data.borrow());
                (*right_child.borrow()).print_inorder();
            }
            AVLTree::Empty => return,
        }
    }

    // pub fn is_tree_empty() {
    //     // TODO: Dimas
    // }
}