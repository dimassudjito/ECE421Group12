
mod avl;
mod red_black;


use avl::AVLTree;
use red_black::*;


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

fn main() {

    //////// AVL TREE /////////////
    // test_avl_tree_josh(2);
    test_is_tree_empty();
    test_tree_height();
    // test_insert_node_all(4);

    ////// END AVL TREE ///////////


    //////// RED BLACK TREE /////////////

    println!("Hello, world!");

    /////////////////////////////////////

    let mut lower = RedBlackTree::new(6);
    match lower {
        RedBlackTree::Node {colour, data, ref mut left_child, ref mut right_child} => {
            *left_child = Rc::new(RefCell::new(RedBlackTree::new(7)));
            *right_child = Rc::new(RefCell::new(RedBlackTree::new(8)));
        },
        RedBlackTree::Empty => {},
    }
    let mut rbt = RedBlackTree::new(5);
    match rbt {
        RedBlackTree::Node {colour, data, ref mut left_child, ref mut right_child} => {
            *left_child = Rc::new(RefCell::new(RedBlackTree::Empty));
            *right_child = Rc::new(RefCell::new(lower));
        },
        RedBlackTree::Empty => {},
    }

    ////////////EXAMPLE 1/////////////////
    //     |
    //     5
    //   /   \
    //  x     6
    //      /   \
    //     7     8
    //
    //  Watch out for the following nodes: x, 7, 8, where x is empty.

    println!("{:#?}", rbt);
    println!("\n");
    // rotate left should form EXAMPLE 2
    rbt = rbt.rotate_left();
    
    println!("\n\n{:#?}", rbt);
    rbt.in_order_traversal();

       /////////EXAMPLE 2//////////////////
    //         |
    //         6
    //       /   \
    //      5     8
    //     /  \    
    //    x    7     
    // 

    // rotate right should form EXAMPLE 1
    rbt = rbt.rotate_right();
    println!("\n\n{:#?}", rbt);



    let rbt = rbt.insert(9);
    println!("\n\n{:#?}", rbt);
    rbt.in_order_traversal();


    let mut rbt2 = RedBlackTree::new(8);
    rbt2 = rbt2.insert(18);

    rbt2 = rbt2.insert(5);
    rbt2 = rbt2.insert(15);
    rbt2 = rbt2.insert(17);
    rbt2 = rbt2.insert(25);
    rbt2 = rbt2.insert(40);
    rbt2 = rbt2.insert(80);

    println!("\n\n{:#?}", rbt2);
    


    
    println!("Leaf nodes: {}", rbt2.count_leaves());
    println!("Tree height: {}", rbt2.get_height());
    rbt2.in_order_traversal();

    //////// END RED BLACK TREE /////////

}
