mod avl;

use std::cell::RefCell;
use std::rc::Rc;

use avl::AVLTree;


fn test_avl_tree() {
    let mut root: AVLTree<&str> = AVLTree::Node { data: "5", left_child: Rc::new(RefCell::new(AVLTree::Empty)), right_child: Rc::new(RefCell::new(AVLTree::Empty)), height: 1 };
    root.insert_node("3");
    root.insert_node("2");
    root.insert_node("4");
    root.insert_node("7");
    root.insert_node("6");
    root.insert_node("9");
    root.print_inorder();
}

fn test_avl_tree_josh() {
    let mut root: AVLTree<&str> = AVLTree::Node { data: "5", left_child: Rc::new(RefCell::new(AVLTree::Empty)), right_child: Rc::new(RefCell::new(AVLTree::Empty)), height: 1 };
    root.insert_node("3");
    root.insert_node("2");
    root.insert_node("4");
    root.insert_node("7");
    root.insert_node("6");
    root.insert_node("9");
    println!("Enum Root: {:#?}", root);
}

fn main() {
    test_avl_tree();
}