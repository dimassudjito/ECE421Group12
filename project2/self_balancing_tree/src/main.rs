mod avl;
mod red_black;

use avl::AVLTree;
use red_black::*;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

fn test_avl_tree_josh(num: i32) {
    match num {
        1 => {
            // right right case
            let n1 = AVLTree::Node {
                data: RefCell::new(Rc::new(1)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let n2 = AVLTree::Node {
                data: RefCell::new(Rc::new(2)),
                left_child: RefCell::new(Rc::new(n1)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(1),
            };
            let n3 = AVLTree::Node {
                data: RefCell::new(Rc::new(3)),
                left_child: RefCell::new(Rc::new(n2)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(2),
            };
            let n4 = AVLTree::Node {
                data: RefCell::new(Rc::new(4)),
                left_child: RefCell::new(Rc::new(n3)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(3),
            };

            let rc_root = Rc::new(n4);
            println!("Enum Root: {:#?}", &rc_root);

            let new_node = AVLTree::delete_node(&rc_root, &2);
            println!("Enum Root: {:#?}", &new_node);
            new_node.print_inorder();
        }
        2 => {
            // right left case
            let n0 = AVLTree::Node {
                data: RefCell::new(Rc::new(0)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let l2_child_r = AVLTree::Node {
                data: RefCell::new(Rc::new(4)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let l2_child_l = AVLTree::Node {
                data: RefCell::new(Rc::new(2)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let l1_child = AVLTree::Node {
                data: RefCell::new(Rc::new(3)),
                left_child: RefCell::new(Rc::new(l2_child_l)),
                right_child: RefCell::new(Rc::new(l2_child_r)),
                height: RefCell::new(1),
            };
            let root: AVLTree<i32> = AVLTree::Node {
                data: RefCell::new(Rc::new(1)),
                left_child: RefCell::new(Rc::new(n0)),
                right_child: RefCell::new(Rc::new(l1_child)),
                height: RefCell::new(2),
            };

            let rc_root = Rc::new(root);
            println!("Enum Root: {:#?}", &rc_root);

            let new_node = AVLTree::delete_node(&rc_root, &0);
            println!("Enum Root: {:#?}", &new_node);
            new_node.print_inorder();
        }
        3 => {
            // right right case
            let n1 = AVLTree::Node {
                data: RefCell::new(Rc::new(4)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let n2 = AVLTree::Node {
                data: RefCell::new(Rc::new(3)),
                right_child: RefCell::new(Rc::new(n1)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(1),
            };
            let n3 = AVLTree::Node {
                data: RefCell::new(Rc::new(2)),
                right_child: RefCell::new(Rc::new(n2)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(2),
            };
            let n4 = AVLTree::Node {
                data: RefCell::new(Rc::new(1)),
                right_child: RefCell::new(Rc::new(n3)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(3),
            };

            let rc_root = Rc::new(n4);
            println!("Enum Root: {:#?}", &rc_root);

            let new_node = AVLTree::delete_node(&rc_root, &2);
            println!("Enum Root: {:#?}", &new_node);
            new_node.print_inorder();
        }
        4 => {
            // left right case
            let n0 = AVLTree::Node {
                data: RefCell::new(Rc::new(0)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let n2 = AVLTree::Node {
                data: RefCell::new(Rc::new(2)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let n4 = AVLTree::Node {
                data: RefCell::new(Rc::new(4)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let n1 = AVLTree::Node {
                data: RefCell::new(Rc::new(1)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(n2)),
                height: RefCell::new(1),
            };
            let n3 = AVLTree::Node {
                data: RefCell::new(Rc::new(3)),
                left_child: RefCell::new(Rc::new(n1)),
                right_child: RefCell::new(Rc::new(n4)),
                height: RefCell::new(2),
            };

            let rc_root = Rc::new(n3);
            println!("Enum Root: {:#?}", &rc_root);

            let new_node = AVLTree::delete_node(&rc_root, &4);
            println!("Enum Root: {:#?}", &new_node);
            new_node.print_inorder();
        }
        5 => {
            // Testing leaves function
            let n0 = AVLTree::Node {
                data: RefCell::new(Rc::new(0)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let n2 = AVLTree::Node {
                data: RefCell::new(Rc::new(2)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let n4 = AVLTree::Node {
                data: RefCell::new(Rc::new(4)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let n1 = AVLTree::Node {
                data: RefCell::new(Rc::new(1)),
                left_child: RefCell::new(Rc::new(n0)),
                right_child: RefCell::new(Rc::new(n2)),
                height: RefCell::new(1),
            };
            let n3 = AVLTree::Node {
                data: RefCell::new(Rc::new(3)),
                left_child: RefCell::new(Rc::new(n1)),
                right_child: RefCell::new(Rc::new(n4)),
                height: RefCell::new(2),
            };

            println!("{}", n3.leaf_number());
            let rc_root = Rc::new(n3);
        }
        _ => {}
    }
}

fn test_is_tree_empty() {
    let root = AVLTree::Node {
        data: RefCell::new(Rc::new(4)),
        left_child: RefCell::new(Rc::new(AVLTree::Empty)),
        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
        height: RefCell::new(0),
    };
    let empty_root: AVLTree<i32> = AVLTree::Empty;
    println!("Non-empty: {:?}", root.is_tree_empty());
    println!("Empty: {:?}", empty_root.is_tree_empty())
}

fn test_tree_height() {
    let n2 = AVLTree::Node {
        data: RefCell::new(Rc::new(2)),
        left_child: RefCell::new(Rc::new(AVLTree::Empty)),
        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
        height: RefCell::new(1),
    };
    let n3 = AVLTree::Node {
        data: RefCell::new(Rc::new(3)),
        left_child: RefCell::new(Rc::new(n2)),
        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
        height: RefCell::new(2),
    };
    let root = AVLTree::Node {
        data: RefCell::new(Rc::new(4)),
        left_child: RefCell::new(Rc::new(n3)),
        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
        height: RefCell::new(3),
    };
    let empty_root: AVLTree<i32> = AVLTree::Empty;
    println!("Non-empty: {:?}", root.tree_height());
    println!("Empty: {:?}", empty_root.tree_height())
}

fn test_insert_node() {
    let n2 = AVLTree::Node {
        data: RefCell::new(Rc::new(2)),
        left_child: RefCell::new(Rc::new(AVLTree::Empty)),
        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
        height: RefCell::new(1),
    };
    let n3 = AVLTree::Node {
        data: RefCell::new(Rc::new(3)),
        left_child: RefCell::new(Rc::new(n2)),
        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
        height: RefCell::new(2),
    };
    let n4 = AVLTree::Node {
        data: RefCell::new(Rc::new(4)),
        left_child: RefCell::new(Rc::new(n3)),
        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
        height: RefCell::new(3),
    };

    let rc_root = Rc::new(n4);
    println!("Enum Root: {:#?}", &rc_root);
    rc_root.print_inorder();
    println!("----------");
    let new_node = AVLTree::insert_node(&rc_root, &1);
    println!("Enum Root: {:#?}", &new_node);
    new_node.print_inorder();
}

pub fn test_insert_node_all(num: i32) {
    match num {
        1 => {
            let n2 = AVLTree::Node {
                data: RefCell::new(Rc::new(2)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(1),
            };
            let n3 = AVLTree::Node {
                data: RefCell::new(Rc::new(3)),
                left_child: RefCell::new(Rc::new(n2)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(2),
            };
            let n4 = AVLTree::Node {
                data: RefCell::new(Rc::new(4)),
                left_child: RefCell::new(Rc::new(n3)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(3),
            };

            let rc_root = Rc::new(n4);
            println!("Enum Root: {:#?}", &rc_root);
            rc_root.print_inorder();
            println!("----------");
            let new_node = AVLTree::insert_node(&rc_root, &1);
            println!("Enum Root: {:#?}", &new_node);
            new_node.print_inorder();
        }
        2 => {
            // left right case
            let n4 = AVLTree::Node {
                data: RefCell::new(Rc::new(4)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(1),
            };
            let n3 = AVLTree::Node {
                data: RefCell::new(Rc::new(3)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(n4)),
                height: RefCell::new(3),
            };
            let n5 = AVLTree::Node {
                data: RefCell::new(Rc::new(5)),
                left_child: RefCell::new(Rc::new(n3)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(2),
            };

            let rc_root = Rc::new(n5);
            println!("Enum Root: {:#?}", &rc_root);
            rc_root.print_inorder();
            println!("----------");
            let new_node = AVLTree::insert_node(&rc_root, &2);
            println!("Enum Root: {:#?}", &new_node);
            new_node.print_inorder();
        }
        3 => {
            // right right case
            let n4 = AVLTree::Node {
                data: RefCell::new(Rc::new(4)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let n3 = AVLTree::Node {
                data: RefCell::new(Rc::new(3)),
                right_child: RefCell::new(Rc::new(n4)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(1),
            };
            let n2 = AVLTree::Node {
                data: RefCell::new(Rc::new(2)),
                right_child: RefCell::new(Rc::new(n3)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(2),
            };

            let rc_root = Rc::new(n2);
            println!("Enum Root: {:#?}", &rc_root);

            let new_node = AVLTree::insert_node(&rc_root, &5);
            println!("Enum Root: {:#?}", &new_node);
            new_node.print_inorder();
        }
        4 => {
            // right left case
            let n3 = AVLTree::Node {
                data: RefCell::new(Rc::new(3)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let n4 = AVLTree::Node {
                data: RefCell::new(Rc::new(4)),
                left_child: RefCell::new(Rc::new(n3)),
                right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                height: RefCell::new(0),
            };
            let n2 = AVLTree::Node {
                data: RefCell::new(Rc::new(2)),
                left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                right_child: RefCell::new(Rc::new(n4)),
                height: RefCell::new(0),
            };
            let rc_root = Rc::new(n2);
            println!("Enum Root: {:#?}", &rc_root);

            let new_node = AVLTree::insert_node(&rc_root, &5);
            println!("Enum Root: {:#?}", &new_node);
            new_node.print_inorder();
        }
        _ => {}
    }
}

fn test_avl_search() {
    let n2 = AVLTree::Node {
        data: RefCell::new(Rc::new(2)),
        left_child: RefCell::new(Rc::new(AVLTree::Empty)),
        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
        height: RefCell::new(1),
    };
    let n3 = AVLTree::Node {
        data: RefCell::new(Rc::new(3)),
        left_child: RefCell::new(Rc::new(n2)),
        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
        height: RefCell::new(2),
    };
    let n4 = AVLTree::Node {
        data: RefCell::new(Rc::new(4)),
        left_child: RefCell::new(Rc::new(n3)),
        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
        height: RefCell::new(3),
    };
    println!("Should be true: {}", n4.search_node(&2));
    println!("Should be false: {}", n4.search_node(&100));
}

pub fn cli() {
    loop {
        println!("--- Main Menu ---");
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
        println!("--- AVL Tree Menu ---");
        println!("1. Insert node: insert value (e.g., insert 3)");
        println!("2. Search node: search value (e.g., search 4)");
        println!("3. Delete node: del value (e.g., del 2)");
        println!("4. Print: print");
        println!("5. Exit: exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let cmd: Vec<&str> = input.trim().split_whitespace().collect();

        match cmd[0] {
            "exit" => return,
            "print" => {
                println!("{:#?}", root_rc);
            },
            _ => {
                if cmd.len() < 2 {
                    println!("Invalid input"); 
                    return;
                } else {
                    let value: i32 = cmd[1].parse().unwrap();
                    match cmd[0] {
                        "insert" => {
                            root_rc = AVLTree::insert_node(&root_rc, &value);
                        },
                        "search" => {
                            if (*root_rc).borrow().search_node(&value) {
                                println!("Value {} exists in the tree", &value);
                            } else {
                                println!("Value {} does not exist in the tree", &value);
                            }
                        },
                        "del" => {
                            root_rc = AVLTree::delete_node(&root_rc, &value);
                        },
                        _ => {
                            println!("Invalid input");
                        }
                    }
                }
            }
        }
    }
}

fn rb_cli() {
    let mut root: RedBlackTree<i32> = RedBlackTree::new();
    loop {
        println!("--- Red-Black Tree Menu ---");
        println!("1. Insert node: insert value (e.g., insert 3)");
        println!("2. Search node: search value (e.g., search 4)");
        println!("3. Delete node: del value (e.g., del 2)");
        println!("4. Print: print");
        println!("5. Exit: exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let cmd: Vec<&str> = input.trim().split_whitespace().collect();

        match cmd[0] {
            "exit" => return,
            "print" => {
                root.display_tree();
            },
            _ => {
                if cmd.len() < 2 {
                    println!("Invalid input"); 
                    return;
                } else {
                    let value: i32 = cmd[1].parse().unwrap();
                    match cmd[0] {
                        "insert" => {
                            root.insert(value);
                        },
                        "search" => {
                            let found: bool = !(root.search(value) == RedBlackTree::Empty);
                            if found {
                                println!("Value {} exists in the tree", &value);
                            } else {
                                println!("Value {} does not exist in the tree", &value);
                            }
                        },
                        "del" => {
                            // Insert RB delete here
                        },
                        _ => {
                            println!("Invalid input");
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    //////// AVL TREE /////////////
    // test_avl_tree_josh(2);
    test_is_tree_empty();
    test_tree_height();
    // test_insert_node_all(4);
    test_avl_search();

    ////// END AVL TREE ///////////

    //////// RED BLACK TREE /////////////

    println!("Hello, world!");

    /////////////////////////////////////

    let mut rbt2 = RedBlackTree::new();
    rbt2.insert(8);
    rbt2.insert(18);
    rbt2.insert(5);
    rbt2.insert(15);
    rbt2.insert(17);
    rbt2.insert(25);
    rbt2.insert(40);
    rbt2.insert(80);
    rbt2.insert(4);
    rbt2.insert(14);
    rbt2.insert(16);
    rbt2.insert(24);
    rbt2.insert(39);
    rbt2.insert(79);
    rbt2.insert(2);
    rbt2.insert(12);
    // rbt2.insert(11);
    // rbt2.insert(20);
    // rbt2.insert(37);
    // rbt2.insert(27);
    // rbt2.insert(15);
    // rbt2.insert(115);
    // rbt2.insert(117);
    // rbt2.insert(125);
    // rbt2.insert(140);
    // rbt2.insert(180);
    // rbt2.insert(14);
    // rbt2.insert(114);
    // rbt2.insert(116);
    // rbt2.insert(124);
    // rbt2.insert(139);
    // rbt2.insert(179);
    // rbt2.insert(12);
    // rbt2.insert(112);
    // rbt2.insert(111);
    // rbt2.insert(120);
    // rbt2.insert(137);
    // rbt2.insert(127);

    // println!("\n\n{:#?}", rbt2);

    // println!("Leaf nodes: {}", rbt2.count_leaves());
    // println!("Tree height: {}", rbt2.get_height());
    // rbt2.in_order_traversal();
    println!("\n\n\n\n\n");
    rbt2.display_tree();

    println!("\n\n\nSEARCH RESULT:");
    rbt2.search(8).display_tree();

    //////// END RED BLACK TREE /////////

    cli();
}
