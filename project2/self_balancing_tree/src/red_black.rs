use std::cell::RefCell;
use std::rc::Rc;

enum  NodeColour {
    Red, 
    Black,
}


enum RedBlackTree<T: Ord> {
    Node {
        colour: NodeColour,
        data: T,
        left_child: Rc<RedBlackTree<T>>,
        right_child: Rc<RedBlackTree<T>>,
    },
    Empty
}