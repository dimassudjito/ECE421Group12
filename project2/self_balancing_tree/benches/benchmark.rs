use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ord;
use std::cmp::max;
use std::marker::Copy;
use std::fmt::Debug;
use rand::thread_rng;
use rand::Rng;
use rand::prelude::SliceRandom;
use std::time::Duration;
use std::fmt::Display;
use std::cell::Ref;



pub enum data<'a, T: Ord + Copy + Debug + 'a> {
    RefType(Ref<'a, T>),
    refType(&'a T),
}

pub trait ReadableBinaryTree<T: Ord + Copy + Debug> {
    // sources:
    //https://stackoverflow.com/questions/53085270/how-do-i-implement-a-trait-with-a-generic-method
    //https://stackoverflow.com/questions/29401626/how-do-i-return-a-reference-to-something-inside-a-refcell-without-breaking-encap

    // provides read only type methods to give inforamation about
    // a binary tree. The template methods must be implemented for the shared methods to work.

    // template methods must be implemented by implementors
    fn is_node_empty(&self) -> bool;

    // An Err in these three functions signifies an empty node.
    fn immut_right_child(&self) -> Result<Ref<dyn ReadableBinaryTree<T>>, &str>;
    fn immut_left_child(&self) -> Result<Ref<dyn ReadableBinaryTree<T>>, &str>;

    // on of the following two template methods must be implemented by implementors
    fn immut_data_from_ref(&self) -> Result<&T, &str> {
        // default implementation
        Result::Err("Node cannot return data as a ref")
    }
    fn immut_data_from_Ref(&self) -> Result<Ref<T>, &str> {
        // default implementation
        Result::Err("Node cannot return data as a Ref")
    }
    fn immut_data(&self) -> data<T> {
        // the nested methods allows for multiple formats of data return for the trait objects
        match self.immut_data_from_Ref() {
            Err(..) => {
                match self.immut_data_from_ref() {
                    Err(..) => {
                        panic!("Readable binary tree has no suitable implementations for data retrieval")
                    }
                    Ok(data) => {
                        return data::refType(data);
                    }
                }
            }
            Ok(dataRef) => return data::RefType(dataRef),
        }
    }

    fn immut_meta_data_string(&self) -> Result<&str, &str> {
        // default implementation
        Result::Err("Node does not have meta data")
    }

    // Shared Methods
    fn search(&self, val: &T) -> bool {
        if self.is_node_empty() {
            return false;
        } else {
            match self.immut_data() {
                data::RefType(data_value) => {
                    let left_child = self.immut_left_child().unwrap();
                    let right_child = self.immut_right_child().unwrap();
                    if *val == *data_value {
                        return true;
                    } else if *val < *data_value {
                        left_child.search(val)
                    } else {
                        right_child.search(val)
                    }
                }
                data::refType(data_value) => {
                    let left_child = self.immut_left_child().unwrap();
                    let right_child = self.immut_right_child().unwrap();
                    if *val == *data_value {
                        return true;
                    } else if *val < *data_value {
                        left_child.search(val)
                    } else {
                        right_child.search(val)
                    }
                }
            }
        }
    }
    fn count_leaves(&self) -> i32 {
        // Returns the number of leaves
        // leaf is defined as a node with no children
        // Note: an empty tree has no leaves

        if self.is_node_empty() {
            // empty node
            return 0;
        } else {
            if (*self.immut_right_child().unwrap()).is_node_empty()
                && (*self.immut_left_child().unwrap()).is_node_empty()
            {
                // leaf node
                return 1;
            } else {
                // has at least one child node
                return (*self.immut_right_child().unwrap()).count_leaves()
                    + (*self.immut_left_child().unwrap()).count_leaves();
            }
        }
    }
    fn is_tree_empty(&self) -> bool {
        // is the tree empty
        return self.is_node_empty();
    }
    fn get_tree_height(&self) -> i32 {
        // Recursively gets height
        // note that height in this case is defined as the maximum
        // number of edges along a root to leaf path.
        // An empty tree will return 0 as the height.

        if self.is_node_empty() {
            // empty node
            return 0;
        } else {
            if self.immut_right_child().unwrap().is_node_empty()
                && self.immut_left_child().unwrap().is_node_empty()
            {
                // leaf node that is not empty
                return 0;
            } else {
                // node with at least one child
                return std::cmp::max(
                    self.immut_right_child().unwrap().get_tree_height(),
                    self.immut_left_child().unwrap().get_tree_height(),
                ) + 1;
            }
        }
    }
    fn get_node_height(&self) -> i32 {
        // Recursively gets node height
        // This counts nodes and not edges (e.g. a tree with one node has height 1, and not 0)

        if self.is_node_empty() {
            // empty node
            return 1;
        } else {
            return std::cmp::max(
                self.immut_right_child().unwrap().get_node_height(),
                self.immut_left_child().unwrap().get_node_height(),
            ) + 1;
        }
    }

    // traverse with new line printed at the end
    fn in_order_traversal(&self) {
        self.traverse();
        print!("\n");
    }
    fn traverse(&self) {
        // Print traversal of left node, then root, then right node

        if self.is_node_empty() {
            // empty node
        } else {
            self.immut_left_child().unwrap().traverse();
            match self.immut_data() {
                data::RefType(r) => {
                    print!("{:?} ", r);
                }
                data::refType(r) => {
                    print!("{:?} ", r);
                }
            }

            self.immut_right_child().unwrap().traverse();
        }
    }

    fn print_tree(&self)
    where
        Self: Sized,
    {
        // Reserve 5 characters for each node of the tree to be printed, e.g. "R:218" or " B:4 " or " R:12"
        let two: u32 = 2;
        let total_elements = two.pow(self.get_node_height().try_into().unwrap()) - 1;
        let mut layer_order_elements = vec![String::from(""); (total_elements + 1) as usize];
        let mut layer_order_elements_exist = vec![false; (total_elements + 1) as usize];

        fn get_node_string<T: Ord + Copy + Debug>(tree: &dyn ReadableBinaryTree<T>) -> String {
            // Given a node, this returns a string padded up to 5 characters of the color and the value, e.g. " B:7 " or "R:28 "

            if tree.is_node_empty() {
                return "".to_string();
            } else {
                let colour_string = match tree.immut_meta_data_string() {
                    Err(e) => "",
                    Ok(metaData) => metaData,
                };

                let data_string = match tree.immut_data() {
                    data::RefType(r) => format!("{:?}", *r),
                    data::refType(r) => format!("{:?}", *r),
                };

                let padding_size = 5 - (data_string.len() + 2);
                let right_padding = padding_size / 2;
                let left_padding = padding_size - right_padding;
                println!("left pad: {}, right pad: {}", left_padding, right_padding);
                if colour_string == "" {
                    return format!(
                        "{} {} {}",
                        " ".repeat(left_padding),
                        data_string,
                        " ".repeat(right_padding)
                    );
                } else {
                    return format!(
                        "{}{}:{}{}",
                        " ".repeat(left_padding),
                        colour_string,
                        data_string,
                        " ".repeat(right_padding)
                    );
                }
            }
        }

        /*
                   _______1_______
                  /               \
              ____2____            3_
             /         \          /   \
          __4__         5       6       7
         /     \       / \     / \     / \
        8      9     10  11  12  13  14  15

        Layer 0's range is 2^0 to 2^1 - 1
        Layer 1's range is 2^1 to 2^2 - 1
        Layer 2's range is 2^2 to 2^3 - 1
        Layer 3's range is 2^3 to 2^4 - 1

        To get the layer offset, track how we got to that node. For each left path taken, that's a binary 0 and for each right path taken, it's a binary one.
        The number in the tree using layer by layer traversal is 2^layer + offset
        */
        fn extract_elements_in_layer_order<T: Ord + Copy + Debug>(
            tree: &dyn ReadableBinaryTree<T>,
            v: &mut Vec<String>,
            v_exist: &mut Vec<bool>,
            layer: u32,
            offset: i32,
        ) {
            if tree.is_node_empty() {
                // empty node
            } else {
                let left_child = &*tree.immut_left_child().unwrap();
                let right_child = &*tree.immut_right_child().unwrap();
                let two: u32 = 2;

                v[(two.pow(layer) as i32 + offset) as usize] = get_node_string(tree);
                v_exist[(two.pow(layer) as i32 + offset) as usize] = true;
                extract_elements_in_layer_order(left_child, v, v_exist, layer + 1, 2 * offset);
                extract_elements_in_layer_order(right_child, v, v_exist, layer + 1, 2 * offset + 1);
            }
        }

        extract_elements_in_layer_order(
            self,
            &mut layer_order_elements,
            &mut layer_order_elements_exist,
            0,
            0,
        );

        // DEBUG OP
        //println!("Elements: {}", total_elements);
        //println!("element 2: {}", layer_order_elements[2]);
        //println!("element 2 exist: {}", layer_order_elements_exist[2]);
        // println!("\n{:#?}", layer_order_elements);

        // Iterate through each layer of the tree and print the nodes

        /*
                                                     xxxxx
                                ______________________/ \______________________
                               /                                               \
                             xxxxx                                           xxxxx
                    __________/ \__________                         __________/ \__________
                   /                       \                       /                       \
                 xxxxx                   xxxxx                   xxxxx                   xxxxx
              ____/ \____             ____/ \____             ____/ \____             ____/ \____
             /           \           /           \           /           \           /           \
           xxxxx       xxxxx       xxxxx       xxxxx       xxxxx       xxxxx       xxxxx       xxxxx
           _/ \_       _/ \_       _/ \_       _/ \_       _/ \_       _/ \_       _/ \_       _/ \_
          /     \     /     \     /     \     /     \     /     \     /     \     /     \     /     \
        xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx

        The gaps between the nodes increase by 2x+5 when going up a layer, and (x-5)/2 when going down a layer
        The closed form equation of sequence a(n+1) = 2*a(n)+5 and a(0)=1 is a(n) = 3*2^(n + 1) - 5
        The initial padding on the left starts at 1 at the bottom, and then it goes 3*2^layer - 2 where layer starts counting from 0 at the bottom, or if the layer starts counting from 0 at the top, it goes 3*2^(num_layers - layer) - 2
        Between any two layers are two lines to display the branches.
         - The first line has a series of _ and / \ followed by more _, repeated for each node it goes under
           - The left padding for the first line goes 4, 7, 13, 25, which is 3*2^n + 1 (0 indexed)
           - The amount of underscores follows the pattern 1, 4, 10, 22 which is 3*2^n - 2 (0 indexed)
           - The spaces in between the middle underscores for the child nodes follows the pattern 7, 13, 25 which is 3*2^(n+1) + 1 (0 indexed)
         - The second line puts / over each left child and \ over each right child
           - The left padding is one less than the left padding of the first line, so it's 3*2^n (0 indexed)
           - The padding between the / and \ follows the pattern 5, 11, 23, 47 which is 3*2^(n+1) - 1 (0 indexed)
           - The padding between the nodes is 2 less than the padding from the previous line, so it's 3*2^(n+1) - 1 (0 indexed)
        */
        //println!("Tree height is {}", self.get_node_height());
        let two: u32 = 2;
        for layer in 0..(self.get_node_height() - 1) {
            //println!("Doing layer {}: ", layer);
            // Print left padding for this layer
            print!(
                "{}",
                " ".repeat(
                    (3 * two.pow((self.get_node_height() - 3 - layer + 1).try_into().unwrap()) - 2)
                        .try_into()
                        .unwrap()
                )
            );
            for elem in two.pow(layer as u32)..two.pow(layer as u32 + 1) {
                //println!("layer elem: {}", layer_order_elements[elem as usize]);
                // Print the element
                match layer_order_elements_exist[elem as usize] {
                    true => {
                        print!("{}", layer_order_elements[elem as usize]);
                    }
                    false => {
                        print!("     ");
                    }
                }
                // Print the right padding for this element
                print!(
                    "{}",
                    " ".repeat(
                        (3 * two.pow((self.get_node_height() - 1 - layer).try_into().unwrap()) - 5)
                            .try_into()
                            .unwrap()
                    )
                );
            }
            println!("");
            // Print the two lines of branches between two layers, as long as this isn't the last layer since nothing goes below that (we don't display NIL nodes for simplicity)
            // Only print branches with children! Do checks for this within each loop through all nodes
            if layer != self.get_node_height() - 2 {
                // Within this loop, self.get_node_height() - 3 - layer is equal to n within the 0 indexed equations above, indexed from the bottom and not the top
                // FIRST LINE
                // Print left padding
                print!(
                    "{}",
                    " ".repeat(
                        (3 * two.pow((self.get_node_height() - 3 - layer).try_into().unwrap()) + 1)
                            .try_into()
                            .unwrap()
                    )
                );
                for node in 0..two.pow(layer as u32) {
                    // Check whether this node has left/right children by jumping to the next layer and probing the left and right sides
                    let has_left_child: bool = match layer_order_elements_exist
                        [(two.pow(layer as u32 + 1) + 2 * node) as usize]
                    {
                        true => true,
                        false => false,
                    };
                    let has_right_child: bool = match layer_order_elements_exist
                        [(two.pow(layer as u32 + 1) + 2 * node + 1) as usize]
                    {
                        true => true,
                        false => false,
                    };

                    // Print the connector thing
                    if has_left_child {
                        // Print underscores
                        print!(
                            "{}",
                            "_".repeat(
                                (3 * two
                                    .pow((self.get_node_height() - 3 - layer).try_into().unwrap())
                                    - 2)
                                .try_into()
                                .unwrap()
                            )
                        );
                        print!(r"/");
                    } else {
                        print!(
                            "{}",
                            " ".repeat(
                                (3 * two
                                    .pow((self.get_node_height() - 3 - layer).try_into().unwrap())
                                    - 2)
                                .try_into()
                                .unwrap()
                            )
                        );
                        print!(" ");
                    }
                    print!(" ");
                    if has_right_child {
                        print!(r"\");
                        // Print the underscores
                        print!(
                            "{}",
                            "_".repeat(
                                (3 * two
                                    .pow((self.get_node_height() - 3 - layer).try_into().unwrap())
                                    - 2)
                                .try_into()
                                .unwrap()
                            )
                        );
                    } else {
                        print!(" ");
                        print!(
                            "{}",
                            " ".repeat(
                                (3 * two
                                    .pow((self.get_node_height() - 3 - layer).try_into().unwrap())
                                    - 2)
                                .try_into()
                                .unwrap()
                            )
                        );
                    }
                    // Print the right space padding
                    print!(
                        "{}",
                        " ".repeat(
                            (3 * two.pow((self.get_node_height() - 2 - layer).try_into().unwrap())
                                + 1)
                            .try_into()
                            .unwrap()
                        )
                    );
                }
                println!("");
                // SECOND LINE
                // Print left padding
                print!(
                    "{}",
                    " ".repeat(
                        (3 * two.pow((self.get_node_height() - 3 - layer).try_into().unwrap()))
                            .try_into()
                            .unwrap()
                    )
                );
                for node in 0..two.pow(layer as u32) {
                    // Check whether this node has left/right children by jumping to the next layer and probing the left and right sides
                    let has_left_child: bool = match layer_order_elements_exist
                        [(two.pow(layer as u32 + 1) + 2 * node) as usize]
                    {
                        true => true,
                        false => false,
                    };
                    let has_right_child: bool = match layer_order_elements_exist
                        [(two.pow(layer as u32 + 1) + 2 * node + 1) as usize]
                    {
                        true => true,
                        false => false,
                    };

                    if has_left_child {
                        print!(r"/");
                    } else {
                        print!(" ");
                    }
                    // Print the padding between nodes
                    print!(
                        "{}",
                        " ".repeat(
                            (3 * two.pow((self.get_node_height() - 2 - layer).try_into().unwrap())
                                - 1)
                            .try_into()
                            .unwrap()
                        )
                    );
                    if has_right_child {
                        print!(r"\");
                    } else {
                        print!(" ");
                    }
                    // Print the right space padding
                    print!(
                        "{}",
                        " ".repeat(
                            (3 * two.pow((self.get_node_height() - 2 - layer).try_into().unwrap())
                                - 1)
                            .try_into()
                            .unwrap()
                        )
                    );
                }
                println!("");
            }
        }
    }
}



#[derive(Debug)]
pub enum AVLTree<T: Ord + Debug + Copy> {
    Node {
        data: RefCell<Rc<T>>,
        left_child: RefCell<Rc<AVLTree<T>>>,
        right_child: RefCell<Rc<AVLTree<T>>>,
        height: RefCell<i32>,
    },
    Empty,
}

impl<D: Ord + Copy + Debug> ReadableBinaryTree<D> for AVLTree<D> {
    // template methods must be implemented by implementors
    fn is_node_empty(&self) -> bool {
        match self {
            AVLTree::Empty => true,
            AVLTree::Node { .. } => false,
        }
    }

    // An Err in these three functions signifies an empty node.
    fn immut_right_child(&self) -> Result<Ref<dyn ReadableBinaryTree<D>>, &str> {
        match self {
            AVLTree::Empty => return Result::Err("Node is empty and has no children"),
            AVLTree::Node { right_child, .. } => {
                // creates a new Ref to overcome the original Ref falling out of scope
                let rf = Ref::map(right_child.borrow(), |t| &**t);
                return Result::Ok(rf);
            }
        }
    }
    fn immut_left_child(&self) -> Result<Ref<dyn ReadableBinaryTree<D>>, &str> {
        match self {
            AVLTree::Empty => return Result::Err("Node is empty and has no children"),
            AVLTree::Node { left_child, .. } => {
                // creates a new Ref to overcome the original Ref falling out of scope
                let rf = Ref::map(left_child.borrow(), |t| &**t);
                return Result::Ok(rf);
            }
        }
    }
    fn immut_data_from_Ref(&self) -> Result<Ref<D>, &str> {
        match self {
            AVLTree::Empty => return Result::Err("Node is empty and has no data"),
            AVLTree::Node { data, .. } => {
                let rf = Ref::map(data.borrow(), |t| &**t);
                return Result::Ok(rf);
            }
        }
    }
}

impl<T: Ord + Debug + Copy> AVLTree<T> {
    // pub fn search_node(&self, value: &T) -> bool {
    //     match self {
    //         AVLTree::Node {
    //             data,
    //             left_child,
    //             right_child,
    //             ..
    //         } => {
    //             if *value == **data.borrow() {
    //                 true
    //             } else if *value < **data.borrow() {
    //                 left_child.borrow().search_node(value)
    //             } else {
    //                 right_child.borrow().search_node(value)
    //             }
    //         }
    //         AVLTree::Empty => false,
    //     }
    // }
    pub fn insert_node(node_rc: &Rc<AVLTree<T>>, new_data: &T) -> Rc<AVLTree<T>> {
        match &**node_rc {
            AVLTree::Empty => {
                let new_node: AVLTree<T> = AVLTree::Node {
                    data: RefCell::new(Rc::new(*new_data)),
                    left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    height: RefCell::new(1),
                };
                Rc::new(new_node)
            }
            AVLTree::Node {
                data,
                left_child,
                right_child,
                height,
            } => {
                if *new_data < **(data.borrow()) {
                    let new_node = AVLTree::insert_node(&*(left_child.borrow()), new_data);
                    left_child.replace(new_node);
                } else if *new_data > **(data.borrow()) {
                    let new_node = AVLTree::insert_node(&*(right_child.borrow()), new_data);
                    right_child.replace(new_node);
                } else {
                    return Rc::clone(node_rc);
                }

                let return_node_rc = AVLTree::insert_node_balance(node_rc);
                (*return_node_rc).update_heights();
                return return_node_rc;
            }
        }
    }

    pub fn insert_node_balance(node_rc: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        match &**node_rc {
            AVLTree::Empty => {
                return Rc::clone(node_rc);
            }
            AVLTree::Node {
                data,
                left_child: left_child_ref,
                right_child: right_child_ref,
                ..
            } => {
                // balance
                let left_node = &*Rc::clone(&*left_child_ref.borrow());
                let right_node = &*Rc::clone(&*right_child_ref.borrow());

                let left_height = (*left_child_ref.borrow()).get_height();
                let right_height = (*right_child_ref.borrow()).get_height();
                if (left_height - right_height).abs() > 1 {
                    // this is an unbalanced node
                    if left_height > right_height {
                        // left-<?> case
                        match left_node {
                            AVLTree::Empty => {
                                panic!("Given tree is not a proper AVL tree");
                            }
                            AVLTree::Node {
                                left_child: y_left_child_ref,
                                right_child: y_right_child_ref,
                                ..
                            } => {
                                let y_left_height = (*y_left_child_ref.borrow()).get_height();
                                let y_right_height = (*y_right_child_ref.borrow()).get_height();
                                if y_left_height > y_right_height {
                                    // left-left case
                                    return AVLTree::rotation_left_left(node_rc);
                                } else {
                                    // left-right case
                                    return AVLTree::rotation_left_right(node_rc);
                                }
                            }
                        }
                    } else {
                        // right-<?> case
                        match right_node {
                            AVLTree::Empty => {
                                panic!("Given tree is not a proper AVL tree");
                            }
                            AVLTree::Node {
                                left_child: y_left_child_ref,
                                right_child: y_right_child_ref,
                                ..
                            } => {
                                let y_left_height = (*y_left_child_ref.borrow()).get_height();
                                let y_right_height = (*y_right_child_ref.borrow()).get_height();
                                if y_left_height > y_right_height {
                                    // right-left case
                                    return AVLTree::rotation_right_left(node_rc);
                                } else {
                                    // right-right case
                                    return AVLTree::rotation_right_right(node_rc);
                                }
                            }
                        }
                    }
                } else {
                    // is a balanced node
                    return Rc::clone(node_rc);
                }
            }
        }
    }

    pub fn delete_node(node_rc: &Rc<AVLTree<T>>, targetValue: &T) -> Rc<AVLTree<T>> {
        // recursively deletes the node with the target value if it exists and returns the new root

        match &**node_rc {
            AVLTree::Empty => {
                return Rc::clone(node_rc);
            } // base case
            AVLTree::Node {
                data,
                left_child: left_child_ref,
                right_child: right_child_ref,
                ..
            } => {
                if *targetValue < *Rc::clone(&*data.borrow()) {
                    let new_node =
                        AVLTree::delete_node(&Rc::clone(&*left_child_ref.borrow()), targetValue);
                    left_child_ref.replace(new_node);
                } else if *targetValue > *Rc::clone(&*data.borrow()) {
                    let new_node =
                        AVLTree::delete_node(&Rc::clone(&*right_child_ref.borrow()), targetValue);
                    right_child_ref.replace(new_node);
                } else {
                    let left_node = &*Rc::clone(&*left_child_ref.borrow());
                    let right_node = &*Rc::clone(&*right_child_ref.borrow());
                    // target node found
                    match left_node {
                        AVLTree::Empty => {
                            match right_node {
                                AVLTree::Empty => {
                                    // both children are empty
                                    return Rc::clone(&*right_child_ref.borrow());
                                }
                                AVLTree::Node { .. } => {
                                    // only the left is empty
                                    return Rc::clone(&*right_child_ref.borrow());
                                }
                            }
                        }
                        AVLTree::Node { .. } => {
                            match right_node {
                                AVLTree::Empty => {
                                    // only the right is empty
                                    return Rc::clone(&*left_child_ref.borrow());
                                }
                                AVLTree::Node {
                                    data: right_node_data,
                                    ..
                                } => {
                                    // both are not empty

                                    // steal right child's value...
                                    data.replace(Rc::clone(&*right_node_data.borrow()));

                                    // delete right child recursively since we just stole it's value
                                    let new_right = AVLTree::delete_node(
                                        &Rc::clone(&*right_child_ref.borrow()),
                                        &**right_node_data.borrow(),
                                    );
                                    right_child_ref.replace(new_right);
                                    (*node_rc).update_heights();
                                }
                            }
                        }
                    }
                }
                // balance
                let return_node_rc = AVLTree::delete_node_balance(node_rc);
                (*return_node_rc).update_heights();
                return return_node_rc;
            }
        }
    }

    pub fn delete_node_balance(node_rc: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        // balances the tree for deletion case
        match &**node_rc {
            AVLTree::Empty => {
                return Rc::clone(node_rc);
            }
            AVLTree::Node {
                data,
                left_child: left_child_ref,
                right_child: right_child_ref,
                ..
            } => {
                // balance
                let left_node = &*Rc::clone(&*left_child_ref.borrow());
                let right_node = &*Rc::clone(&*right_child_ref.borrow());

                let left_height = (*left_child_ref.borrow()).get_height();
                let right_height = (*right_child_ref.borrow()).get_height();
                if (left_height - right_height).abs() > 1 {
                    // this is an unbalanced node
                    if left_height > right_height {
                        // left-<?> case
                        match left_node {
                            AVLTree::Empty => {
                                panic!("Given tree is not a proper AVL tree");
                            }
                            AVLTree::Node {
                                left_child: y_left_child_ref,
                                right_child: y_right_child_ref,
                                ..
                            } => {
                                let y_left_height = (*y_left_child_ref.borrow()).get_height();
                                let y_right_height = (*y_right_child_ref.borrow()).get_height();
                                if y_left_height > y_right_height {
                                    // left-left case
                                    return AVLTree::rotation_left_left(node_rc);
                                } else {
                                    // left-right case
                                    return AVLTree::rotation_left_right(node_rc);
                                }
                            }
                        }
                    } else {
                        // right-<?> case
                        match right_node {
                            AVLTree::Empty => {
                                panic!("Given tree is not a proper AVL tree");
                            }
                            AVLTree::Node {
                                left_child: y_left_child_ref,
                                right_child: y_right_child_ref,
                                ..
                            } => {
                                let y_left_height = (*y_left_child_ref.borrow()).get_height();
                                let y_right_height = (*y_right_child_ref.borrow()).get_height();
                                if y_left_height > y_right_height {
                                    // right-left case
                                    println!("in right left");
                                    return AVLTree::rotation_right_left(node_rc);
                                } else {
                                    // right-right case

                                    return AVLTree::rotation_right_right(node_rc);
                                }
                            }
                        }
                    }
                } else {
                    // is a balanced node
                    return Rc::clone(node_rc);
                }
            }
        }
    }

    pub fn rotation_left_left(z: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        return AVLTree::rotate_right(z);
    }

    pub fn rotation_left_right(z: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        //   z
        //  /
        // y
        match &(**z) {
            AVLTree::Empty => {
                panic!("Invalid AVLTree for left right rotation")
            }
            AVLTree::Node { left_child, .. } => {
                let new_left = AVLTree::rotate_left(&(left_child.borrow()));
                left_child.replace(new_left);
                return AVLTree::rotate_right(z);
            }
        }
    }

    pub fn rotation_right_right(z: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        return AVLTree::rotate_left(z);
    }

    pub fn rotation_right_left(z: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {
        //   z
        //    \
        //     y
        match &(**z) {
            AVLTree::Empty => {
                panic!("Invalid AVLTree for right left rotation")
            }
            AVLTree::Node { right_child, .. } => {
                let new_right = AVLTree::rotate_right(&(right_child.borrow()));
                right_child.replace(new_right);
                return AVLTree::rotate_left(z);
            }
        }
    }

    pub fn rotate_right(z_rc: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {

        // EX:   z
        //      /
        //     y
        //    / \
        //   x   N
        let z = &(**z_rc);
        match z {
            AVLTree::Empty => Rc::clone(z_rc),
            AVLTree::Node {
                left_child: z_left_child,
                ..
            } => {
                let y_rc = Rc::clone(&z_left_child.borrow());
                let y = &(*y_rc);
                match y {
                    AVLTree::Empty => Rc::clone(z_rc),
                    AVLTree::Node {
                        right_child: y_right_child,
                        ..
                    } => {
                        //       y
                        //      / \
                        //     x   z      N
                        let n = (y_right_child).replace(Rc::clone(z_rc));

                        //       y
                        //      / \
                        //     x   z
                        //        /
                        //       n
                        z_left_child.replace(n);

                        y.update_heights();
                        z.update_heights();

                        y_rc
                    }
                }
            }
        }
    }

    pub fn rotate_left(z_rc: &Rc<AVLTree<T>>) -> Rc<AVLTree<T>> {

        // EX:   z
        //        \
        //         y
        //        / \
        //       n   x
        let z = &(**z_rc);
        match z {
            AVLTree::Empty => Rc::clone(z_rc),
            AVLTree::Node {
                right_child: z_right_child,
                ..
            } => {
                let y_rc = Rc::clone(&z_right_child.borrow());
                let y = &(*y_rc);
                match y {
                    AVLTree::Empty => Rc::clone(z_rc),
                    AVLTree::Node {
                        left_child: y_left_child,
                        ..
                    } => {
                        //       y
                        //      / \
                        //     z   x
                        let n = (y_left_child).replace(Rc::clone(z_rc));

                        //       y
                        //      / \
                        //     z   x
                        //      \
                        //        n
                        z_right_child.replace(n);

                        y.update_heights();
                        z.update_heights();

                        y_rc
                    }
                }
            }
        }
    }

    pub fn update_heights(&self) {
        // updates the heights of an node based on it's direct children's heights.
        // IT IS NOT recursive. If the children's heights are incorrect, the height of this node will be as well.
        // TODO: leaf node should be 1 not 0
        match self {
            AVLTree::Empty => {}
            AVLTree::Node {
                left_child,
                right_child,
                height,
                ..
            } => {
                let left_val = (**left_child.borrow()).get_height();
                let right_val = (**right_child.borrow()).get_height();
                height.replace(max(left_val, right_val) + 1);
            }
        }
    }

    fn get_height(&self) -> i32 {
        // Returns the height of a AVLTree node, including empty nodes
        match self {
            AVLTree::Empty => {
                return -1;
            }
            AVLTree::Node { height, .. } => {
                return *height.borrow();
            }
        }
    }
}



#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeColour {
    Red,
    Black,
}

#[derive(Debug, Clone)]
pub enum RedBlackTree<T: Ord + Copy + Debug> {
    Node {
        colour: NodeColour,
        data: T,
        left_child: Rc<RefCell<RedBlackTree<T>>>,
        right_child: Rc<RefCell<RedBlackTree<T>>>,
    },
    Empty,
}

impl<T: Ord + Copy + Debug> PartialEq for RedBlackTree<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            RedBlackTree::Node {
                colour,
                data,
                left_child,
                right_child,
            } => {
                let d1 = data.clone();
                match other {
                    RedBlackTree::Node {
                        colour,
                        data,
                        left_child,
                        right_child,
                    } => {
                        let d2 = data.clone();
                        return d1 == d2;
                    }
                    RedBlackTree::Empty => return false,
                }
            }
            RedBlackTree::Empty => return false,
        };
    }
}
impl<D: Ord + Display + Copy + Debug> ReadableBinaryTree<D> for RedBlackTree<D> {
    // template methods must be implemented by implementors
    fn is_node_empty(&self) -> bool {
        match self {
            RedBlackTree::Empty => true,
            RedBlackTree::Node { .. } => false,
        }
    }

    // An Err in these functions signifies an empty node.
    fn immut_right_child(&self) -> Result<Ref<dyn ReadableBinaryTree<D>>, &str> {
        // returns right node for reading
        match self {
            RedBlackTree::Empty => return Result::Err("Node is empty and has no children"),
            RedBlackTree::Node { right_child, .. } => {
                // creates a new Ref to overcome the original Ref falling out of scope
                let rf = Ref::map((*right_child).borrow(), |t| &*t);
                return Result::Ok(rf);
            }
        }
    }
    fn immut_left_child(&self) -> Result<Ref<dyn ReadableBinaryTree<D>>, &str> {
        // returns left node for reading
        match self {
            RedBlackTree::Empty => return Result::Err("Node is empty and has no children"),
            RedBlackTree::Node { left_child, .. } => {
                // creates a new Ref to overcome the original Ref falling out of scope
                let rf = Ref::map((*left_child).borrow(), |t| &*t);
                return Result::Ok(rf);
            }
        }
    }
    fn immut_data_from_ref(&self) -> Result<&D, &str> {
        // returns data for reading
        match self {
            RedBlackTree::Empty => return Result::Err("Node is empty and has no data"),
            RedBlackTree::Node { data, .. } => return Result::Ok(&data),
        }
    }
    fn immut_meta_data_string(&self) -> Result<&str, &str> {
        // returns color information for printing
        match self {
            RedBlackTree::Empty => return Result::Err("Node is empty and has no children"),
            RedBlackTree::Node { colour, .. } => match colour {
                NodeColour::Red => {
                    return Result::Ok("R");
                }
                NodeColour::Black => {
                    return Result::Ok("B");
                }
            },
        }
    }
}

impl<T: Ord + Copy + Debug> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree::<T>::Empty
    }

    pub fn rotate_right(&mut self) {
        match self {
            RedBlackTree::Node {
                data,
                colour,
                left_child,
                right_child,
            } => {
                let mut lc = Rc::clone(&left_child);
                let lc_rc = match &*lc.borrow() {
                    RedBlackTree::Node {
                        data,
                        colour,
                        left_child,
                        right_child,
                    } => Rc::clone(&right_child),
                    RedBlackTree::Empty => {
                        return;
                    }
                };
                let mut oldself = self.clone();
                match oldself {
                    RedBlackTree::Node {
                        data,
                        colour,
                        ref mut left_child,
                        ref mut right_child,
                    } => {
                        *left_child = lc_rc;
                    }
                    RedBlackTree::Empty => {
                        return;
                    }
                }

                *self = lc.borrow().clone();
                match self {
                    RedBlackTree::Node {
                        data,
                        colour,
                        ref mut left_child,
                        ref mut right_child,
                    } => {
                        *right_child = Rc::new(RefCell::new(oldself));
                    }
                    RedBlackTree::Empty => {
                        return;
                    }
                }
            }
            RedBlackTree::Empty => {
                return;
            }
        }
    }

    pub fn rotate_left(&mut self) {
        match self {
            RedBlackTree::Node {
                data,
                colour,
                left_child,
                right_child,
            } => {
                let mut rc = Rc::clone(&right_child);
                let rc_lc = match &*rc.borrow() {
                    RedBlackTree::Node {
                        data,
                        colour,
                        left_child,
                        right_child,
                    } => Rc::clone(&left_child),
                    RedBlackTree::Empty => {
                        return;
                    }
                };
                let mut oldself = self.clone();
                match oldself {
                    RedBlackTree::Node {
                        data,
                        colour,
                        ref mut left_child,
                        ref mut right_child,
                    } => {
                        *right_child = rc_lc;
                    }
                    RedBlackTree::Empty => {
                        return;
                    }
                }

                *self = rc.borrow().clone();
                match self {
                    RedBlackTree::Node {
                        data,
                        colour,
                        ref mut left_child,
                        ref mut right_child,
                    } => {
                        *left_child = Rc::new(RefCell::new(oldself));
                    }
                    RedBlackTree::Empty => {
                        return;
                    }
                }
            }
            RedBlackTree::Empty => {
                return;
            }
        }
    }

    pub fn insert(&mut self, val: T) {
        match self {
            RedBlackTree::Node {
                data,
                colour,
                left_child,
                right_child,
            } => {}
            RedBlackTree::Empty => {
                *self = RedBlackTree::Node {
                    data: val,
                    colour: NodeColour::Black,
                    left_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
                    right_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                return;
            }
        };

        let mut stack = vec![Rc::new(RefCell::new(self.clone()))];
        let mut node = Rc::clone(&stack[0]);
        let mut nodetemp = Rc::clone(&node);

        ///// BINARY TREE INSERT //////
        loop {
            match &*node.borrow() {
                RedBlackTree::Node {
                    data,
                    colour,
                    left_child,
                    right_child,
                } => {
                    if val > *data {
                        nodetemp = Rc::clone(&right_child);
                    } else if val < *data {
                        nodetemp = Rc::clone(&left_child);
                    } else {
                        return;
                    }
                }
                RedBlackTree::Empty => break,
            }
            node = Rc::clone(&nodetemp);
            stack.push(Rc::clone(&node));
        }
        // println!("{:#?}", stack);

        stack[stack.len() - 1].replace(RedBlackTree::Node {
            data: val.clone(),
            colour: NodeColour::Red,
            left_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
            right_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
        });
        ///// END BINARY TREE INSERT //////

        ///// FIX TREE //////
        let mut idx = stack.len() - 1;
        let mut uncle: Rc<RefCell<RedBlackTree<T>>>;
        let mut grandfather: Rc<RefCell<RedBlackTree<T>>>;
        let mut parent: Rc<RefCell<RedBlackTree<T>>>;
        while idx >= 2 {
            // we got the grandfather
            grandfather = Rc::clone(&stack[idx - 2]);
            let mut gf_left = Rc::new(RefCell::new(RedBlackTree::Empty));
            let mut gf_right = Rc::new(RefCell::new(RedBlackTree::Empty));

            let mut gf_val = match &*grandfather.borrow() {
                RedBlackTree::Node {
                    data,
                    colour,
                    left_child,
                    right_child,
                } => {
                    gf_left = Rc::clone(&left_child);
                    gf_right = Rc::clone(&right_child);
                    data.clone()
                }
                RedBlackTree::Empty => val, // not possible
            };
            if val > gf_val {
                uncle = Rc::clone(&gf_left);
            } else {
                uncle = Rc::clone(&gf_right);
            }
            // now we got the uncle

            parent = Rc::clone(&stack[idx - 1]);
            // and we got the parent

            // If parent is black, then we break
            if NodeColour::Black
                == match &*parent.borrow() {
                    RedBlackTree::Node {
                        data,
                        colour,
                        left_child,
                        right_child,
                    } => *colour,
                    RedBlackTree::Empty => NodeColour::Black,
                }
            {
                break;
            }
            // otherwise...
            // if uncle is red
            if NodeColour::Red
                == match &*uncle.borrow() {
                    RedBlackTree::Node {
                        data,
                        colour,
                        left_child,
                        right_child,
                    } => *colour,
                    RedBlackTree::Empty => NodeColour::Black,
                }
            {
                // Set uncle and parent as black
                match *parent.borrow_mut() {
                    RedBlackTree::Node {
                        ref data,
                        ref mut colour,
                        ref left_child,
                        ref right_child,
                    } => *colour = NodeColour::Black,
                    RedBlackTree::Empty => {}
                }
                match *uncle.borrow_mut() {
                    RedBlackTree::Node {
                        ref data,
                        ref mut colour,
                        ref left_child,
                        ref right_child,
                    } => *colour = NodeColour::Black,
                    RedBlackTree::Empty => {}
                }
                // set grandfather as Red
                if idx - 2 > 0 {
                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = NodeColour::Red,
                        RedBlackTree::Empty => {}
                    }
                }

                idx = idx - 2;

                // println!("\n\n\n ROOT\n{:#?}", stack[0]);
            } else {
                // else if uncle is black
                // we have 4 cases

                // If grandparent.left.left is the current node
                if (match &*grandfather.borrow() {
                    RedBlackTree::Node {
                        data,
                        mut colour,
                        left_child,
                        right_child,
                    } => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent))
                    && (match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            mut colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    } == Rc::clone(&stack[idx]))
                {
                    // right rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp.rotate_right();
                    parent.replace(gf_temp);

                    grandfather = match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // then swap colours of grandfather and parent
                    let mut gf_colour = match &*grandfather.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let mut p_colour = match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = p_colour,
                        RedBlackTree::Empty => {}
                    };
                    match *parent.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = gf_colour,
                        RedBlackTree::Empty => {}
                    };

                    if idx - 2 == 0 {
                        *self = parent.borrow().clone();
                        return;
                    } else {
                        // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {
                                ref data,
                                ref colour,
                                ref mut left_child,
                                ref mut right_child,
                            } => {
                                let upperval = data.clone();
                                let parent_val = match &*parent.borrow() {
                                    RedBlackTree::Node {
                                        data,
                                        colour,
                                        left_child,
                                        right_child,
                                    } => *data,
                                    RedBlackTree::Empty => upperval.clone(),
                                };
                                if parent_val > upperval {
                                    *right_child = Rc::clone(&parent);
                                } else if parent_val < upperval {
                                    *left_child = Rc::clone(&parent);
                                }
                            }
                            RedBlackTree::Empty => {}
                        };
                    }
                }
                // If grandparent.left.right is the current node
                else if (match &*grandfather.borrow() {
                    RedBlackTree::Node {
                        data,
                        mut colour,
                        left_child,
                        right_child,
                    } => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent))
                    && (match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            mut colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    } == Rc::clone(&stack[idx]))
                {
                    // left rotate on parent
                    // println!("\n\n\n PARENT OLD:");
                    // parent.borrow().display_tree();
                    // println!("\n\n\n CURRENT OLD:");
                    // stack[idx].borrow().display_tree();

                    let mut p_temp = parent.borrow().clone();
                    p_temp.rotate_left();
                    stack[idx].replace(p_temp);

                    parent = match &*stack[idx].borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref colour,
                            ref mut left_child,
                            ref right_child,
                        } => *left_child = Rc::clone(&stack[idx]),
                        RedBlackTree::Empty => {}
                    };

                    // println!("\n\n\n PARENT NEW:");
                    // parent.borrow().display_tree();

                    // println!("\n\n\n CURRENT NEW:");
                    // stack[idx].borrow().display_tree();

                    // println!("\n\n\n GRANDPARENT OLD:");
                    // grandfather.borrow().display_tree();

                    // println!("\n\n\n CURRENT OLD:");
                    // stack[idx].borrow().display_tree();

                    // right rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp.rotate_right();
                    stack[idx].replace(gf_temp);

                    grandfather = match &*stack[idx].borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // println!("\n\n\n GRANDPARENT NEW:");
                    // grandfather.borrow().display_tree();

                    // println!("\n\n\n CURRENT NEW:");
                    // stack[idx].borrow().display_tree();

                    // then swap colours of grandfather and current
                    let mut gf_colour = match &*grandfather.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let mut c_colour = match &*stack[idx].borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = c_colour,
                        RedBlackTree::Empty => {}
                    };

                    match *stack[idx].borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = gf_colour,
                        RedBlackTree::Empty => {}
                    };

                    // if new insert is root, return
                    if idx - 2 == 0 {
                        *self = stack[idx].borrow().clone();
                        return;
                    } else {
                        // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {
                                ref data,
                                ref colour,
                                ref mut left_child,
                                ref mut right_child,
                            } => {
                                let upperval = data.clone();
                                let insert_val = match &*stack[idx].borrow() {
                                    RedBlackTree::Node {
                                        data,
                                        colour,
                                        left_child,
                                        right_child,
                                    } => *data,
                                    RedBlackTree::Empty => upperval.clone(),
                                };
                                if insert_val > upperval {
                                    *right_child = Rc::clone(&stack[idx]);
                                } else if insert_val < upperval {
                                    *left_child = Rc::clone(&stack[idx]);
                                }
                            }
                            RedBlackTree::Empty => {}
                        };
                    }
                }
                // If grandparent.right.left is the current node
                else if (match &*grandfather.borrow() {
                    RedBlackTree::Node {
                        data,
                        mut colour,
                        left_child,
                        right_child,
                    } => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent))
                    && (match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            mut colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    } == Rc::clone(&stack[idx]))
                {
                    // right rotate on parent
                    let mut p_temp = parent.borrow().clone();
                    p_temp.rotate_right();
                    stack[idx].replace(p_temp);

                    parent = match &*stack[idx].borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref colour,
                            ref left_child,
                            ref mut right_child,
                        } => *right_child = Rc::clone(&stack[idx]),
                        RedBlackTree::Empty => {}
                    };

                    // left rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp.rotate_left();
                    stack[idx].replace(gf_temp);

                    grandfather = match &*stack[idx].borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // then swap colours of grandfather and current
                    let mut gf_colour = match &*grandfather.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let mut c_colour = match &*stack[idx].borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };

                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = c_colour,
                        RedBlackTree::Empty => {}
                    };

                    match *stack[idx].borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = gf_colour,
                        RedBlackTree::Empty => {}
                    };

                    // if new insert is root, return
                    if idx - 2 == 0 {
                        *self = stack[idx].borrow().clone();
                        return;
                    } else {
                        // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {
                                ref data,
                                ref colour,
                                ref mut left_child,
                                ref mut right_child,
                            } => {
                                let upperval = data.clone();
                                let insert_val = match &*stack[idx].borrow() {
                                    RedBlackTree::Node {
                                        data,
                                        colour,
                                        left_child,
                                        right_child,
                                    } => *data,
                                    RedBlackTree::Empty => upperval.clone(),
                                };
                                if insert_val > upperval {
                                    *right_child = Rc::clone(&stack[idx]);
                                } else if insert_val < upperval {
                                    *left_child = Rc::clone(&stack[idx]);
                                }
                            }
                            RedBlackTree::Empty => {}
                        };
                    }
                }
                // If grandparent.right.right is the current node
                else if (match &*grandfather.borrow() {
                    RedBlackTree::Node {
                        data,
                        mut colour,
                        left_child,
                        right_child,
                    } => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent))
                    && (match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            mut colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    } == Rc::clone(&stack[idx]))
                {
                    // left rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp.rotate_left();
                    parent.replace(gf_temp);

                    grandfather = match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // then swap colours of grandfather and parent
                    let gf_colour = match &*grandfather.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let p_colour = match &*parent.borrow() {
                        RedBlackTree::Node {
                            data,
                            colour,
                            left_child,
                            right_child,
                        } => *colour,
                        RedBlackTree::Empty => NodeColour::Black,
                    };

                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = p_colour,
                        RedBlackTree::Empty => {}
                    };

                    match *parent.borrow_mut() {
                        RedBlackTree::Node {
                            ref data,
                            ref mut colour,
                            ref left_child,
                            ref right_child,
                        } => *colour = gf_colour,
                        RedBlackTree::Empty => {}
                    };

                    // if parent is root, return
                    if idx - 2 == 0 {
                        *self = parent.borrow().clone();
                        return;
                    } else {
                        // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {
                                ref data,
                                ref colour,
                                ref mut left_child,
                                ref mut right_child,
                            } => {
                                let upperval = data.clone();
                                let parent_val = match &*parent.borrow() {
                                    RedBlackTree::Node {
                                        data,
                                        colour,
                                        left_child,
                                        right_child,
                                    } => *data,
                                    RedBlackTree::Empty => upperval.clone(),
                                };
                                if parent_val > upperval {
                                    *right_child = Rc::clone(&parent);
                                } else if parent_val < upperval {
                                    *left_child = Rc::clone(&parent);
                                }
                            }
                            RedBlackTree::Empty => {}
                        };
                    }
                }

                break;
            }
        }
        ///// END FIX TREE //////

        // println!("{:#?}" , stack);
        // println!("\n\n\nFINAL:");
        // let root = stack[0].borrow().display_tree();
        *self = stack[0].borrow().clone();
    }

    pub fn insert_no_fix(&mut self, val: T) {
        match self {
            RedBlackTree::Node {
                data,
                colour,
                left_child,
                right_child,
            } => {}
            RedBlackTree::Empty => {
                *self = RedBlackTree::Node {
                    data: val,
                    colour: NodeColour::Black,
                    left_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
                    right_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                return;
            }
        };

        let mut node = Rc::new(RefCell::new(self.clone()));
        let mut nodetemp = Rc::clone(&node);

        ///// BINARY TREE INSERT //////
        loop {
            match &*node.borrow() {
                RedBlackTree::Node {
                    data,
                    colour,
                    left_child,
                    right_child,
                } => {
                    if val > *data {
                        nodetemp = Rc::clone(&right_child);
                    } else if val < *data {
                        nodetemp = Rc::clone(&left_child);
                    } else {
                        return;
                    }
                }
                RedBlackTree::Empty => break,
            }
            node = Rc::clone(&nodetemp);
        }
        // println!("{:#?}", stack);

        node.replace(RedBlackTree::Node {
            data: val.clone(),
            colour: NodeColour::Red,
            left_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
            right_child: Rc::new(RefCell::new(RedBlackTree::Empty)),
        });
        ///// END BINARY TREE INSERT //////
    }
}









fn red_black_insert_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("red_black_insert_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("red_black_insert_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len()-100 {
                    rbt.insert(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.insert(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn red_black_insert_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("red_black_insert_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("red_black_insert_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len()-100 {
                    rbt.insert(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.insert(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn red_black_insert_block_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("red_black_insert_block_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("red_black_insert_block_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                
                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in 0..insertions.len() {
                        rbt.insert(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn red_black_insert_block_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("red_black_insert_block_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("red_black_insert_block_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in 0..insertions.len() {
                        rbt.insert(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}



fn avl_insert_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("avl_insert_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("avl_insert_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut avl = Rc::new(AVLTree::Node {
                    data: RefCell::new(Rc::new(black_box(&insertions[0]))),
                    left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    height: RefCell::new(0),
                });


                for i in 0..insertions.len()-100 {
                    avl = AVLTree::insert_node(&avl, &&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        avl = AVLTree::insert_node(&avl, black_box(&&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn avl_insert_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("avl_insert_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("avl_insert_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut avl = Rc::new(AVLTree::Node {
                    data: RefCell::new(Rc::new(black_box(&insertions[0]))),
                    left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    height: RefCell::new(0),
                });


                for i in 0..insertions.len()-100 {
                    avl = AVLTree::insert_node(&avl, &&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        avl = AVLTree::insert_node(&avl, black_box(&&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();     
}

fn avl_insert_block_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("avl_insert_block_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("avl_insert_block_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                
                b.iter(|| {
                    // Code to benchmark goes here
                    // let y = black_box(x);
                    let mut avl = Rc::new(AVLTree::Node {
                        data: RefCell::new(Rc::new(black_box(&insertions[0]))),
                        left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                        height: RefCell::new(0),
                    });
    
    
                    for i in 0..insertions.len() {
                        avl = AVLTree::insert_node(&avl, &&insertions[i]);
                    }
                })
            },
        );
    }

    group.finish();    
}

fn avl_insert_block_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("avl_insert_block_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("avl_insert_block_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                
                b.iter(|| {
                    // Code to benchmark goes here
                    // let y = black_box(x);
                    let mut avl = Rc::new(AVLTree::Node {
                        data: RefCell::new(Rc::new(black_box(&insertions[0]))),
                        left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                        right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                        height: RefCell::new(0),
                    });
    
    
                    for i in 0..insertions.len() {
                        avl = AVLTree::insert_node(&avl, &&insertions[i]);
                    }
                })
            },
        );
    }

    group.finish();    
}


fn vanilla_bst_insert_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("vanilla_bst_insert_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("vanilla_bst_insert_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len()-100 {
                    rbt.insert_no_fix(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.insert_no_fix(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn vanilla_bst_insert_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("vanilla_bst_insert_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("vanilla_bst_insert_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len()-100 {
                    rbt.insert_no_fix(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.insert_no_fix(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn vanilla_bst_insert_block_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("vanilla_bst_insert_block_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("vanilla_bst_insert_block_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                
                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in 0..insertions.len() {
                        rbt.insert_no_fix(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn vanilla_bst_insert_block_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("vanilla_bst_insert_block_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("vanilla_bst_insert_block_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in 0..insertions.len() {
                        rbt.insert_no_fix(black_box(&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}






fn red_black_search_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("red_black_search_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("red_black_search_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len() {
                    rbt.insert(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.search(black_box(&&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn red_black_search_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("red_black_search_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("red_black_search_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len() {
                    rbt.insert(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.search(black_box(&&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}



fn avl_search_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("avl_search_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("avl_search_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut avl = Rc::new(AVLTree::Node {
                    data: RefCell::new(Rc::new(black_box(&insertions[0]))),
                    left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    height: RefCell::new(0),
                });


                for i in 0..insertions.len() {
                    avl = AVLTree::insert_node(&avl, &&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        avl.search(black_box(&&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn avl_search_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("avl_search_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("avl_search_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut avl = Rc::new(AVLTree::Node {
                    data: RefCell::new(Rc::new(black_box(&insertions[0]))),
                    left_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    right_child: RefCell::new(Rc::new(AVLTree::Empty)),
                    height: RefCell::new(0),
                });


                for i in 0..insertions.len() {
                    avl = AVLTree::insert_node(&avl, &&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        avl.search(black_box(&&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();     
}


fn vanilla_bst_search_random(c: &mut Criterion) {

    let mut group = c.benchmark_group("vanilla_bst_search_random");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000, 32000, 64000, 128000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        let slice: &mut [u32] = &mut insertions;
        slice.shuffle(&mut thread_rng());
        insertions = Vec::from(slice);


        group.bench_function(
            ("vanilla_bst_search_random_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len() {
                    rbt.insert_no_fix(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.search(black_box(&&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}

fn vanilla_bst_search_sequential(c: &mut Criterion) {

    let mut group = c.benchmark_group("vanilla_bst_search_sequential");
    group.sample_size(100);
    group.warm_up_time(Duration::from_millis(5));
    group.measurement_time(Duration::from_millis(5));

    for upper in vec![1000, 2000, 4000, 8000, 16000] {
        let mut insertions:Vec<u32> = (0..upper).collect();
        // let slice: &mut [u32] = &mut insertions;
        // slice.shuffle(&mut thread_rng());
        // insertions = Vec::from(slice);


        group.bench_function(
            ("vanilla_bst_search_sequential_".to_owned() + &upper.to_string()).as_str(),
            |b| {

                let mut rbt = RedBlackTree::new();
                for i in 0..insertions.len() {
                    rbt.insert_no_fix(&insertions[i]);
                }

                b.iter(|| {
                // Code to benchmark goes here
                    // let y = black_box(x);
                    for i in insertions.len()-100..insertions.len() {
                        rbt.search(black_box(&&insertions[i]));
                    }
                })
            },
        );
    }

    group.finish();    
}










criterion_group!(insert_random, red_black_insert_random, avl_insert_random, vanilla_bst_insert_random);
criterion_group!(insert_sequential, red_black_insert_sequential, avl_insert_sequential, vanilla_bst_insert_sequential);
criterion_group!(insert_block_random, red_black_insert_block_random, avl_insert_block_random, vanilla_bst_insert_block_random);
criterion_group!(insert_block_sequential, red_black_insert_block_sequential, avl_insert_block_sequential, vanilla_bst_insert_block_sequential);
criterion_group!(search_random, red_black_search_random, avl_search_random, vanilla_bst_search_random);
criterion_group!(search_sequential, red_black_search_sequential, avl_search_sequential, vanilla_bst_search_sequential);

criterion_main!(insert_random, insert_sequential, insert_block_random, insert_block_sequential, search_random, search_sequential);