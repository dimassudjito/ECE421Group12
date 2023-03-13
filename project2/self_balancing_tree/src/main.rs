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

fn test_avl_tree_josh(num:i32) {
  
    match num {
        1 =>{
            // right right case
            let n1 =  AVLTree::Node { data: RefCell::new(Rc::new("1")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
            let n2 =  AVLTree::Node { data: RefCell::new(Rc::new("2")), left_child: RefCell::new(Rc::new(n1)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(1) };
            let n3 =  AVLTree::Node { data: RefCell::new(Rc::new("3")), left_child: RefCell::new(Rc::new(n2)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(2) };
            let n4 =  AVLTree::Node { data: RefCell::new(Rc::new("4")), left_child: RefCell::new(Rc::new(n3)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(3) };
        
            let rc_root = Rc::new(n4);
            println!("Enum Root: {:#?}", &rc_root);
        
            let new_node = AVLTree::<&str>::delete_node(&rc_root,&"2");    
            println!("Enum Root: {:#?}", &new_node);
            new_node.print_inorder();
        },
        2 =>{
            // right left case
            let n0 =  AVLTree::Node { data: RefCell::new(Rc::new("0")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
            let l2_child_r =  AVLTree::Node { data: RefCell::new(Rc::new("4")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
            let l2_child_l =  AVLTree::Node { data: RefCell::new(Rc::new("2")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
            let l1_child =  AVLTree::Node { data: RefCell::new(Rc::new("3")), left_child: RefCell::new(Rc::new(l2_child_l)), right_child: RefCell::new(Rc::new(l2_child_r)), height: RefCell::new(1) };
            let root: AVLTree<&str> = AVLTree::Node { data:RefCell::new(Rc::new("1")), left_child: RefCell::new(Rc::new(n0)), right_child: RefCell::new(Rc::new(l1_child)), height: RefCell::new(2) };
        
            let rc_root = Rc::new(root);
            println!("Enum Root: {:#?}", &rc_root);
        
            let new_node = AVLTree::<&str>::delete_node(&rc_root,&"0");    
            println!("Enum Root: {:#?}", &new_node);
            new_node.print_inorder();
        
        },
        3=>{
            // right right case
            let n1 =  AVLTree::Node { data: RefCell::new(Rc::new("4")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
            let n2 =  AVLTree::Node { data: RefCell::new(Rc::new("3")), right_child: RefCell::new(Rc::new(n1)), left_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(1) };
            let n3 =  AVLTree::Node { data: RefCell::new(Rc::new("2")), right_child: RefCell::new(Rc::new(n2)), left_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(2) };
            let n4 =  AVLTree::Node { data: RefCell::new(Rc::new("1")), right_child: RefCell::new(Rc::new(n3)), left_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(3) };

            let rc_root = Rc::new(n4);
            println!("Enum Root: {:#?}", &rc_root);
        
            let new_node = AVLTree::<&str>::delete_node(&rc_root,&"2");    
            println!("Enum Root: {:#?}", &new_node);
            new_node.print_inorder();
        },
        4 =>{
            // left right case
            let n0 =  AVLTree::Node { data: RefCell::new(Rc::new("0")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
            let n2 =  AVLTree::Node { data: RefCell::new(Rc::new("2")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
            let n4 =  AVLTree::Node { data: RefCell::new(Rc::new("4")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
            let n1=  AVLTree::Node { data: RefCell::new(Rc::new("1")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(n2)), height: RefCell::new(1) };
            let n3 =  AVLTree::Node { data: RefCell::new(Rc::new("3")), left_child: RefCell::new(Rc::new(n1)), right_child: RefCell::new(Rc::new(n4)), height: RefCell::new(2) };
        
            let rc_root = Rc::new(n3);
            println!("Enum Root: {:#?}", &rc_root);
        
            let new_node = AVLTree::<&str>::delete_node(&rc_root,&"4");    
            println!("Enum Root: {:#?}", &new_node);
            new_node.print_inorder();
        
        },
        5 =>{
            // Testing leaves function
            let n0 =  AVLTree::Node { data: RefCell::new(Rc::new("0")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
            let n2 =  AVLTree::Node { data: RefCell::new(Rc::new("2")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
            let n4 =  AVLTree::Node { data: RefCell::new(Rc::new("4")), left_child: RefCell::new(Rc::new(AVLTree::Empty)), right_child: RefCell::new(Rc::new(AVLTree::Empty)), height: RefCell::new(0) };
            let n1=  AVLTree::Node { data: RefCell::new(Rc::new("1")), left_child: RefCell::new(Rc::new(n0)), right_child: RefCell::new(Rc::new(n2)), height: RefCell::new(1) };
            let n3 =  AVLTree::Node { data: RefCell::new(Rc::new("3")), left_child: RefCell::new(Rc::new(n1)), right_child: RefCell::new(Rc::new(n4)), height: RefCell::new(2) };
            
            println!("{}",n3.leaf_number());
            let rc_root = Rc::new(n3);
        },
        _=>{}
    }
}

fn main() {
    // test_avl_tree();
    test_avl_tree_josh(5);
}