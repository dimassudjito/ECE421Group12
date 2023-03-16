mod avl;

use std::cell::RefCell;
use std::rc::Rc;

use avl::AVLTree;

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
    // test_avl_tree_josh(2);
    test_is_tree_empty();
    test_tree_height();
    // test_insert_node_all(4);
}
