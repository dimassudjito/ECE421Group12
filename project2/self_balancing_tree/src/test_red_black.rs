use crate::red_black::*;
use crate::readbt::ReadableBinaryTree;

fn test_empty() {
    println!("--- Empty Case ---");
    let root: RedBlackTree<i32> = RedBlackTree::new();
    println!("Is tree empty: {:#?}", root.is_tree_empty());
    println!("Tree height: {:#?}", root.get_tree_height());
    println!("Leaf nodes count: {:#?}", root.count_leaves());
    println!("Search 5: {:#?}", root.search(&5));
    println!("--- End of Empty Case ---");
}

fn test_non_empty() {
    println!("--- Non-Empty Case ---");
    let mut root: RedBlackTree<i32> = RedBlackTree::new();
    root.insert(3);
    root.insert(4);
    root.insert(2);

    println!("<<<< Original >>>");
    root.print_tree();
    println!("<<< Print In-order >>>");
    root.in_order_traversal();
    println!("Search 4 (Found): {:#?}", root.search(&4));
    println!("Search 99 (Not Found): {:#?}", root.search(&99));
    println!("Leaf nodes count: {:#?}", root.count_leaves());
    println!("Tree height: {:#?}", root.get_tree_height());
    println!("Is tree empty (not): {:#?}", root.is_tree_empty());
    println!("--- End of Non-Empty Case ---");
}

fn test_insert_ascending() {
    println!("--- Ascending Insert Case ---");
    let mut root: RedBlackTree<i32> = RedBlackTree::new();
    root.insert(1);
    root.insert(2);
    root.insert(3);
    root.insert(4);
    root.insert(5);
    root.print_tree();
    println!("--- End of Ascending Insert Case ---");
}

fn test_insert_descending() {
    println!("--- Descending Insert Case ---");
    let mut root: RedBlackTree<i32> = RedBlackTree::new();
    root.insert(5);
    root.insert(4);
    root.insert(3);
    root.insert(2);
    root.insert(1);
    root.print_tree();
    println!("--- End of Descending Insert Case ---");
}

fn test_insert_random() {
    println!("--- Random Insert Case ---");
    let mut root: RedBlackTree<i32> = RedBlackTree::new();
    root.insert(4);
    root.insert(2);
    root.insert(3);
    root.insert(5);
    root.insert(1);
    root.print_tree();
    println!("--- End of Random Insert Case ---");
}

fn test_insert_double() {
    println!("--- Double Insert Case ---");
    let mut root: RedBlackTree<i32> = RedBlackTree::new();
    root.insert(1);
    root.insert(1);
    root.print_tree();
    println!("--- End of Double Insert Case ---");
}

fn test_delete_ascending() {
    println!("--- Ascending Delete Case ---");
    let mut root: RedBlackTree<i32> = RedBlackTree::new();
    root.insert(1);
    root.insert(2);
    root.insert(3);
    root.insert(4);
    root.insert(5);
    root.insert(6);
    root.insert(7);
    root.insert(8);
    root.insert(9);
    root.insert(10);
    // root.delete(1);
    // root.delete(2);
    // root.delete(3);
    root.print_tree();
    println!("--- End of Ascending Delete Case ---");
}

fn test_delete_descending() {
    println!("--- Descending Delete Case ---");
    let mut root: RedBlackTree<i32> = RedBlackTree::new();
    root.insert(1);
    root.insert(2);
    root.insert(3);
    root.insert(4);
    root.insert(5);
    root.insert(6);
    root.insert(7);
    root.insert(8);
    root.insert(9);
    root.insert(10);
    // root.delete(10);
    // root.delete(9);
    // root.delete(8);
    root.print_tree();
    println!("--- End of Descending Delete Case ---");
}

fn test_delete_random() {
    println!("--- Random Delete Case ---");
    let mut root: RedBlackTree<i32> = RedBlackTree::new();
    root.insert(1);
    root.insert(2);
    root.insert(3);
    root.insert(4);
    root.insert(5);
    root.insert(6);
    root.insert(7);
    root.insert(8);
    root.insert(9);
    root.insert(10);
    // root.delete(1);
    // root.delete(9);
    // root.delete(3);
    root.print_tree();
    println!("--- End of Random Delete Case ---");
}

fn test_delete_double() {
    println!("--- Double Delete Case ---");
    let mut root: RedBlackTree<i32> = RedBlackTree::new();
    root.insert(1);
    root.insert(2);
    root.insert(3);
    root.insert(4);
    root.insert(5);
    root.insert(6);
    root.insert(7);
    root.insert(8);
    root.insert(9);
    root.insert(10);
    // root.delete(1);
    // root.delete(1);
    root.print_tree();
    println!("--- End of Double Delete Case ---");
}

pub fn test_red_black() {
    // --- UNCOMMENT TO DEBUG --- //
    test_empty();
    test_non_empty();
    test_insert_ascending();
    test_insert_descending();
    test_insert_random(); 
    test_insert_double();
    // test_delete_ascending();
    // test_delete_descending();
    // test_delete_random();
    // test_delete_double();
    // --- --- //
}