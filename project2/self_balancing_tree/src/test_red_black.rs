use crate::red_black::*;
use crate::readbt::ReadableBinaryTree;

use std::cell::RefCell;
use std::rc::Rc;

fn test_empty() {
    println!("--- Empty Case ---");
    let root: RedBlackTree<i32> = RedBlackTree::new();
    println!("Is tree empty: {:#?}", root.is_tree_empty());
    println!("Tree height: {:#?}", root.get_tree_height());
    println!("Leaf nodes count: {:#?}", root.count_leaves());
    println!("Search 5: {:#?}", root.search(&5));
    println!("--- End of Empty Case ---");
}

// fn test_left_left() {
//     println!("--- Left Left Case ---");
//     let n2 = AVLTree::Node {
//         data: RefCell::new(Rc::new(2)),
//         left_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         right_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         height: RefCell::new(1),
//     };
//     let n3 = AVLTree::Node {
//         data: RefCell::new(Rc::new(3)),
//         left_child: RefCell::new(Rc::new(n2)),
//         right_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         height: RefCell::new(2),
//     };
//     let n4 = AVLTree::Node {
//         data: RefCell::new(Rc::new(4)),
//         left_child: RefCell::new(Rc::new(n3)),
//         right_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         height: RefCell::new(3),
//     };

//     println!("<<<< Original >>>");
//     let root = Rc::new(n4);
//     root.print_tree();
//     println!("<<< Print In-order >>>");
//     root.in_order_traversal();
//     println!("<<<< Insert 1: >>>");
//     let inserted_node = AVLTree::insert_node(&root, &1);
//     inserted_node.print_tree();
//     println!("<<< Delete 4 >>>");
//     let deleted_node = AVLTree::delete_node(&inserted_node, &4);
//     deleted_node.print_tree();
//     println!("Search 4 (Found): {:#?}", root.search(&4));
//     println!("Search 99 (Not Found): {:#?}", root.search(&99));
//     println!("Leaf nodes count: {:#?}", root.count_leaves());
//     println!("Tree height: {:#?}", root.get_tree_height());
//     println!("Is tree empty (not): {:#?}", root.is_tree_empty());
//     println!("--- End of Left Left Case ---");
// }

// fn test_left_right() {
//     println!("--- Left Right Case ---");
//     let n5 = AVLTree::Node {
//         data: RefCell::new(Rc::new(5)),
//         left_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         right_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         height: RefCell::new(1),
//     };
//     let n7 = AVLTree::Node {
//         data: RefCell::new(Rc::new(7)),
//         left_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         right_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         height: RefCell::new(1),
//     };
//     let n3 = AVLTree::Node {
//         data: RefCell::new(Rc::new(3)),
//         left_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         right_child: RefCell::new(Rc::new(n5)),
//         height: RefCell::new(2),
//     };
//     let n6 = AVLTree::Node {
//         data: RefCell::new(Rc::new(6)),
//         left_child: RefCell::new(Rc::new(n3)),
//         right_child: RefCell::new(Rc::new(n7)),
//         height: RefCell::new(3),
//     };

//     println!("<<<< Original >>>");
//     let root = Rc::new(n6);
//     root.print_tree();
//     println!("<<<< Insert 4: >>>");
//     let inserted_node = AVLTree::insert_node(&root, &4);
//     inserted_node.print_tree();
//     println!("<<< Delete 7 >>>");
//     let deleted_node = AVLTree::delete_node(&root, &7);
//     deleted_node.print_tree();
//     println!("--- End of Left Right Case ---");
// }

// fn test_right_right() {
//     println!("--- Right Right Case ---");
//     let n4 = AVLTree::Node {
//         data: RefCell::new(Rc::new(4)),
//         left_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         right_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         height: RefCell::new(1),
//     };
//     let n3 = AVLTree::Node {
//         data: RefCell::new(Rc::new(3)),
//         right_child: RefCell::new(Rc::new(n4)),
//         left_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         height: RefCell::new(2),
//     };
//     let n1 = AVLTree::Node {
//         data: RefCell::new(Rc::new(1)),
//         right_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         left_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         height: RefCell::new(1),
//     };
//     let n2 = AVLTree::Node {
//         data: RefCell::new(Rc::new(2)),
//         right_child: RefCell::new(Rc::new(n3)),
//         left_child: RefCell::new(Rc::new(n1)),
//         height: RefCell::new(3),
//     };

//     println!("<<<< Original >>>");
//     let root = Rc::new(n2);
//     root.print_tree();
//     println!("<<<< Insert 5: >>>");
//     let inserted_node = AVLTree::insert_node(&root, &5);
//     inserted_node.print_tree();
//     println!("<<< Delete 1 >>>");
//     let deleted_node = AVLTree::delete_node(&root, &1);
//     deleted_node.print_tree();
//     println!("--- End of Right Right Case ---");
// }

// fn test_right_left() {
//     println!("--- Right Left Case ---");
//     let n3 = AVLTree::Node {
//         data: RefCell::new(Rc::new(3)),
//         left_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         right_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         height: RefCell::new(1),
//     };
//     let n4 = AVLTree::Node {
//         data: RefCell::new(Rc::new(4)),
//         left_child: RefCell::new(Rc::new(n3)),
//         right_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         height: RefCell::new(2),
//     };
//     let n1 = AVLTree::Node {
//         data: RefCell::new(Rc::new(1)),
//         left_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         right_child: RefCell::new(Rc::new(AVLTree::Empty)),
//         height: RefCell::new(1),
//     };
//     let n2 = AVLTree::Node {
//         data: RefCell::new(Rc::new(2)),
//         left_child: RefCell::new(Rc::new(n1)),
//         right_child: RefCell::new(Rc::new(n4)),
//         height: RefCell::new(3),
//     };

//     println!("<<<< Original >>>");
//     let root = Rc::new(n2);
//     root.print_tree();
//     println!("<<<< Insert 5: >>>");
//     let inserted_node = AVLTree::insert_node(&root, &5);
//     inserted_node.print_tree();
//     println!("<<< Delete 1 >>>");
//     let deleted_node = AVLTree::delete_node(&root, &1);
//     deleted_node.print_tree();
//     println!("--- End of Right Left Case ---");
// }

pub fn test_red_black() {
    // --- UNCOMMENT TO DEBUG --- //
    test_empty();
    // test_left_left();
    // test_left_right();
    // test_right_right();
    // test_right_left();
    // --- --- //
}