use crate::avl::*;
use crate::readbt::ReadableBinaryTree;

use std::cell::RefCell;
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
            new_node.in_order_traversal();
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
            new_node.in_order_traversal();
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
            new_node.in_order_traversal();
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
            new_node.in_order_traversal();
        }
        5 => {
            // Testing leaves function
            // testing readonly functions
            // let n3 = AVLTree::Node {
            //     data: RefCell::new(Rc::new(3)),
            //     left_child: RefCell::new(Rc::new(AVLTree::Empty)),
            //     right_child: RefCell::new(Rc::new(AVLTree::Empty)),
            //     height: RefCell::new(0),
            // };
            // let n4 = AVLTree::Node {
            //     data: RefCell::new(Rc::new(4)),
            //     left_child: RefCell::new(Rc::new(n3)),
            //     right_child: RefCell::new(Rc::new(AVLTree::Empty)),
            //     height: RefCell::new(0),
            // };
            // let n2 = AVLTree::Node {
            //     data: RefCell::new(Rc::new(2)),
            //     left_child: RefCell::new(Rc::new(AVLTree::Empty)),
            //     right_child: RefCell::new(Rc::new(n4)),
            //     height: RefCell::new(0),
            // };
            // // let rc_root = Rc::new(n2);
            // // println!("Enum Root: {:#?}", &rc_root);

            // println!("{}", n2.count_leaves());
            // n2.in_order_traversal();
            // println!("{}", n2.get_tree_height());
            // println!("{}", n2.is_tree_empty());
            // n2.print_tree();

            // let mut rbt2 = RedBlackTree::new(8);
            // rbt2 = rbt2.insert(18);
            // rbt2 = rbt2.insert(5);
            // rbt2 = rbt2.insert(15);
            // rbt2 = rbt2.insert(17);
            // rbt2 = rbt2.insert(25);
            // rbt2 = rbt2.insert(40);
            // rbt2 = rbt2.insert(80);
            // rbt2 = rbt2.insert(4);
            // rbt2 = rbt2.insert(14);
            // println!("{}", rbt2.count_leaves());
            // println!("{}", rbt2.get_tree_height());
            // rbt2.in_order_traversal();
            // println!("{}", rbt2.is_tree_empty());
            // rbt2.print_tree();
            // rbt2.print_tree();

            // let mut rbt3 = RedBlackTree::new(8);
            // println!("{}", rbt3.get_tree_height());
            // println!("{}", RedBlackTree::<i32>::Empty.get_tree_height());
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
    println!("Non-empty: {:?}", root.get_tree_height());
    println!("Empty: {:?}", empty_root.get_tree_height())
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
    rc_root.in_order_traversal();
    println!("----------");
    let new_node = AVLTree::insert_node(&rc_root, &1);
    println!("Enum Root: {:#?}", &new_node);
    new_node.in_order_traversal();
}

pub fn test_insert_node_all(num: i32) {
    match num {
        1 => {
            // left left
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
            rc_root.in_order_traversal();
            println!("----------");
            let new_node = AVLTree::insert_node(&rc_root, &1);
            println!("Enum Root: {:#?}", &new_node);
            new_node.in_order_traversal();
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
            rc_root.in_order_traversal();
            println!("----------");
            let new_node = AVLTree::insert_node(&rc_root, &2);
            println!("Enum Root: {:#?}", &new_node);
            new_node.in_order_traversal();
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
            new_node.in_order_traversal();
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
            new_node.in_order_traversal();
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
    println!("Should be true: {}", n4.search(&2));
    println!("Should be false: {}", n4.search(&100));
}

fn test_empty() {
    println!("--- Empty Case ---");
    let root: AVLTree<i32> = AVLTree::Empty;
    println!("Is tree empty: {:#?}", root.is_tree_empty());
    println!("Tree height: {:#?}", root.get_tree_height());
    println!("Leaf nodes count: {:#?}", root.count_leaves());
    println!("Search 5: {:#?}", root.search(&5));
    println!("--- End of Empty Case ---");
}

fn test_left_left() {
    println!("--- Left Left Case ---");
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

    println!("<<<< Original >>>");
    let root = Rc::new(n4);
    root.print_tree();
    println!("<<<< Insert 1: >>>");
    let inserted_node = AVLTree::insert_node(&root, &1);
    inserted_node.print_tree();
    println!("<<< Delete 4 >>>");
    let deleted_node = AVLTree::delete_node(&inserted_node, &4);
    deleted_node.print_tree();
    println!("--- End of Left Left Case ---");
}

pub fn test_avl() {
    // --- UNCOMMENT TO DEBUG --- ///
    test_empty();
    test_left_left();
    // --- --- //
}