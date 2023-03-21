mod avl;
mod readbt;
mod red_black;
mod test_avl;
mod test_red_black;

use crate::readbt::ReadableBinaryTree;
use avl::AVLTree;
use red_black::*;
use test_avl::*;
use test_red_black::*;

use std::borrow::Borrow;
use std::io;
use std::rc::Rc;

pub fn cli() {
    loop {
        println!("\n--- Main Menu ---");
        println!("1. Start AVL tree: init avl");
        println!("2. Start Red-Black tree: init rb");
        println!("3. Exit: exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "init avl" => {
                avl_cli();
            }
            "init rb" => {
                rb_cli();
            }
            "exit" => return,
            _ => {
                println!("Invalid input");
            }
        }
    }
}

fn avl_cli() {
    let root: AVLTree<i32> = AVLTree::Empty;
    let mut root_rc = Rc::new(root);
    loop {
        println!("\n--- AVL Tree Menu ---");
        println!("1. Insert node: insert value (e.g., insert 3)");
        println!("2. Search node: search value (e.g., search 4)");
        println!("3. Delete node: del value (e.g., del 2)");
        println!("4. Print: print");
        println!("5. Count leaves: leaves");
        println!("6. Print Height: height");
        println!("7. Print In order Traversal: inorder");
        println!("8. Check if tree is empty: empty");
        println!("9. Exit: exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let cmd: Vec<&str> = input.trim().split_whitespace().collect();

        match cmd[0] {
            "exit" => return,
            "print" => {
                root_rc.print_tree();
            }
            "leaves" => {
                println!("The tree has {} leaves.", root_rc.count_leaves());
            }
            "inorder" => {
                root_rc.in_order_traversal();
            }
            "height" => {
                println!(
                    "The tree has height {} meaning it has {} edges in it's longest path",
                    root_rc.get_tree_height(),
                    root_rc.get_tree_height()
                );
            }
            "empty" => {
                if root_rc.is_tree_empty() {
                    println!("The tree is empty");
                } else {
                    println!("The tree is not empty");
                }
            }
            _ => {
                if cmd.len() < 2 {
                    println!("Invalid input");
                    continue;
                } else {
                    if cmd[1].len() > 3 {
                        println!("Invalid data value, numbers must be below 1000");
                        continue;
                    }
                    match cmd[1].parse() {
                        Err(e) => {
                            println!(
                                "Invalid input value, please try a numeric and or smaller value"
                            );
                            continue;
                        }
                        Ok(i) => {
                            let value: i32 = i;
                            match cmd[0] {
                                "insert" => {
                                    root_rc = AVLTree::insert_node(&root_rc, &value);
                                }
                                "search" => {
                                    if (*root_rc).borrow().search(&value) {
                                        println!("Value {} exists in the tree", &value);
                                    } else {
                                        println!("Value {} does not exist in the tree", &value);
                                    }
                                }
                                "del" => {
                                    root_rc = AVLTree::delete_node(&root_rc, &value);
                                }
                                _ => {
                                    println!("Invalid input command");
                                }
                            }
                        }
                    };
                }
            }
        }
    }
}

fn rb_cli() {
    let mut root: RedBlackTree<i32> = RedBlackTree::new();
    loop {
        println!("\n--- Red-Black Tree Menu ---");
        println!("1. Insert node: insert value (e.g., insert 3)");
        println!("2. Search node: search value (e.g., search 4)");
        println!("3. Delete node: del value (e.g., del 2)");
        println!("4. Print: print");
        println!("5. Count leaves: leaves");
        println!("6. Print Height: height");
        println!("7. Print In order Traversal: inorder");
        println!("8. Check if tree is empty: empty");
        println!("9. Exit: exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let cmd: Vec<&str> = input.trim().split_whitespace().collect();

        match cmd[0] {
            "exit" => return,
            "print" => {
                root.print_tree();
            }
            "leaves" => {
                println!("The tree has {} leaves.", root.count_leaves());
            }
            "inorder" => {
                root.in_order_traversal();
            }
            "height" => {
                println!(
                    "The tree has height {} meaning it has {} edges in it's longest path",
                    root.get_tree_height(),
                    root.get_tree_height()
                );
            }
            "empty" => {
                if root.is_tree_empty() {
                    println!("The tree is empty");
                } else {
                    println!("The tree is not empty");
                }
            }
            _ => {
                if cmd.len() < 2 {
                    println!("Invalid input");
                    continue;
                } else {
                    if cmd[1].len() > 3 {
                        println!("Invalid data value, numbers must be below 1000");
                        continue;
                    }
                    match cmd[1].parse() {
                        Err(e) => {
                            println!(
                                "Invalid input value, please try a numeric and or smaller value"
                            );
                            continue;
                        }
                        Ok(i) => {
                            let value: i32 = i;
                            match cmd[0] {
                                "insert" => {
                                    root.insert(value);
                                }
                                "search" => {
                                    let found: bool = root.search(&value);
                                    if found {
                                        println!("Value {} exists in the tree", &value);
                                    } else {
                                        println!("Value {} does not exist in the tree", &value);
                                    }
                                }
                                "del" => {
                                    // Insert RB delete here
                                }
                                _ => {
                                    println!("Invalid input");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    // --- UNCOMMENT TO DEBUG --- ///
    test_avl();
    test_red_black();
    // --- --- //
    cli();
}
