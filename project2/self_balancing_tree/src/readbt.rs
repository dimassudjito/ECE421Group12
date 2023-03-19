use std::cell::{Ref, RefCell};
use std::fmt::Debug;
use std::rc::Rc;
enum data<'a, T: Ord + Copy + Debug + 'a> {
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
