use std::cell::RefCell;

#[derive(Debug)]
enum Tree<T: Ord> {
    Node {
        data: T,
        left_child: RefCell<Tree<T>>,
        right_child: RefCell<Tree<T>>,
        height: i32
    },
    Empty,
}

impl<T: Ord> Tree<T> {
    pub fn insert_node(&mut self, new_data: T) {
        // TODO: Dimas
        match self {
            Tree::Empty => {
                *self = Tree::Node {
                    data: new_data,
                    left_child: Box::new(Tree::Empty),
                    right_child: Box::new(Tree::Empty),
                };
            }
            Tree::Node { data, left_child, right_child } => {
                if new_data < *data {
                    left_child.insert_node(new_data);
                } else if new_data > *data {
                    right_child.insert_node(new_data);
                } else {
                    return;
                }
            }
        }
    }

    pub fn delete_node() {
        // TODO: Josh
    }

    pub fn rotation_left_left() {
        // TODO: Dimas
    }

    pub fn rotation_left_right() {
        // TODO: Dimas
    }

    pub fn rotation_right_left() {
        // TODO: Josh
    }

    pub fn rotation_right_right() {
        // TODO: Josh
    }

    pub fn leaf_number() {
        // TODO: Josh
    }

    pub fn tree_height() {
        // TODO: Dimas
    }

    pub fn print() {
        // TODO: Dimas
    }

    pub fn print_inorder() {
        // TODO: Josh
    }

    pub fn is_tree_empty() {
        // TODO: Dimas
    }
}