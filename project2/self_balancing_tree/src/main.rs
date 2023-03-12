mod avl;

use std::cell::RefCell;
use std::rc::Rc;

use avl::AVLTree;

// fn test_avl_tree() {
//     let mut root: AVLTree<&str> = AVLTree::Node { data: "5", left_child: Rc::new(RefCell::new(AVLTree::Empty)), right_child: Rc::new(RefCell::new(AVLTree::Empty)), height: 1 };
//     root.insert_node("3");
//     root.insert_node("2");
//     root.insert_node("4");
//     root.insert_node("7");
//     root.insert_node("6");
//     root.insert_node("9");
//     println!("Enum Root: {:#?}", root);
// }

fn test_avl_tree_josh() {
    //  left left tree
    // let l2_child_r =  AVLTree::Node { data: RefCell::new("3"), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
    // let l2_child_l =  AVLTree::Node { data: RefCell::new("1"), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
    // let l1_child =  AVLTree::Node { data: RefCell::new("2"), left_child: RefCell::new(Rc::new(l2_child_l)), right_child: RefCell::new(Rc::new(l2_child_r)), height: RefCell::new(1) };
    // let root: AVLTree<&str> = AVLTree::Node { data:RefCell::new("4"), left_child: RefCell::new(Rc::new(l1_child)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(2) };

    // let rc_root = Rc::new(root);
    // println!("Enum Root: {:#?}", &rc_root);

    // let new_node = AVLTree::<&str>::rotate_right(&rc_root);
    // println!("Enum Root: {:#?}", new_node);

    // right right tree
    // let l2_child_r =  AVLTree::Node { data: RefCell::new(Rc::new("4")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
    // let l2_child_l =  AVLTree::Node { data: RefCell::new(Rc::new("2")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
    // let l1_child =  AVLTree::Node { data: RefCell::new(Rc::new("3")), left_child: RefCell::new(Rc::new(l2_child_l)), right_child: RefCell::new(Rc::new(l2_child_r)), height: RefCell::new(1) };
    // let root: AVLTree<&str> = AVLTree::Node { data:RefCell::new(Rc::new("1")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(l1_child)), height: RefCell::new(2) };

    let l2_child_r = AVLTree::Node {
        data: RefCell::new(Rc::new("4")),
        left_child: RefCell::new(Rc::new(AVLTree::Empty)),
        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
        height: RefCell::new(0),
    };
    let l2_child_l = AVLTree::Node {
        data: RefCell::new(Rc::new("2")),
        left_child: RefCell::new(Rc::new(AVLTree::Empty)),
        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
        height: RefCell::new(0),
    };
    let l1_child = AVLTree::Node {
        data: RefCell::new(Rc::new("3")),
        left_child: RefCell::new(Rc::new(l2_child_l)),
        right_child: RefCell::new(Rc::new(l2_child_r)),
        height: RefCell::new(1),
    };
    let root: AVLTree<&str> = AVLTree::Node {
        data: RefCell::new(Rc::new("1")),
        left_child: RefCell::new(Rc::new(AVLTree::Empty)),
        right_child: RefCell::new(Rc::new(l1_child)),
        height: RefCell::new(2),
    };

    let rc_root = Rc::new(root);
    println!("Enum Root: {:#?}", &rc_root);

    let new_node = AVLTree::<&str>::delete_node(&rc_root, &"3");

    // let new_node = AVLTree::<&str>::rotate_right(&rc_root);
    println!("Enum Root: {:#?}", &new_node);
    // println!("Enum Root: {:#?}", &rc_root);

    // root.print_inorder();
}

fn test_is_tree_empty() {
    let root = AVLTree::Node {
        data: RefCell::new(Rc::new("4")),
        left_child: RefCell::new(Rc::new(AVLTree::Empty)),
        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
        height: RefCell::new(0),
    };
    let empty_root: AVLTree<&str> = AVLTree::Empty;
    println!("Non-empty: {:?}", root.is_tree_empty());
    println!("Non-empty: {:?}", empty_root.is_tree_empty())
}

fn main() {
    // test_avl_tree();
    test_avl_tree_josh();
    test_is_tree_empty();
}
