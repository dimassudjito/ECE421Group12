use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ord;
use std::cmp::max;
use std::marker::Copy;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum  NodeColour {
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
    Empty
}


impl <T: Ord + Copy + Debug> PartialEq for RedBlackTree<T>
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            RedBlackTree::Node {colour, data, left_child, right_child} => {
                let d1 = data.clone();
                match other {
                    RedBlackTree::Node {colour, data, left_child, right_child} => {
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


impl <T: Ord + Copy + Debug> RedBlackTree<T> {

    pub fn new() -> Self {
        RedBlackTree::<T>::Empty
    }


    pub fn count_leaves(&self) -> i32 {
        // Recursively count leaves
        match self {
            RedBlackTree::Node {colour, data, left_child, right_child} => {
                return left_child.borrow().count_leaves() + right_child.borrow().count_leaves();
            },
            RedBlackTree::Empty => {
                return 1;
            },
        }
    }

    pub fn get_height(&self) -> i32 {
        // Recursively get height
        // This counts nodes and not edges (e.g. a tree with one node has height 1, and not 0)
        match self {
            RedBlackTree::Node {colour, data, left_child, right_child} => {
                return max(left_child.borrow().get_height(), right_child.borrow().get_height()) + 1;
            },
            RedBlackTree::Empty => {
                return 1;
            },
        }
    }

    pub fn traverse(&self) {
        // Print traversal of left node, then root, then right node
        match self {
            RedBlackTree::Node {colour, data, left_child, right_child} => {
                left_child.borrow().traverse();
                print!("{:?} ", *data);
                right_child.borrow().traverse();
            },
            RedBlackTree::Empty => {},
        }
        
    }

    // traverse with new line printed at the end
    pub fn in_order_traversal(&self) {
        self.traverse();
        print!("\n");
    }

    pub fn display_tree(&self) {
        // Reserve 5 characters for each node of the tree to be printed, e.g. "R:218" or " B:4 " or " R:12"
        let two: u32 = 2;
        let total_elements = two.pow(self.get_height().try_into().unwrap()) - 1;
        let mut layer_order_elements = vec![String::from(""); (total_elements + 1) as usize];
        let mut layer_order_elements_exist = vec![false; (total_elements + 1) as usize];

        fn get_node_string<T: Ord + Copy + Debug>(tree: &RedBlackTree<T>) -> String {
            // Given a node, this returns a string padded up to 5 characters of the color and the value, e.g. " B:7 " or "R:28 "
            match tree {
                RedBlackTree::Node {
                    colour,
                    data,
                    left_child,
                    right_child,
                } => {
                    let colour_string = match colour {
                        NodeColour::Red => "R",
                        NodeColour::Black => "B",
                    };
                    let data_string = format!("{:?}", data);
                    let padding_size = 5 - (data_string.len() + 2);
                    let right_padding = padding_size / 2;
                    let left_padding = padding_size - right_padding;
                    // println!("left pad: {}, right pad: {}", left_padding, right_padding);
                    format!("{}{}:{}{}", " ".repeat(left_padding), colour_string, data_string, " ".repeat(right_padding))
                }
                RedBlackTree::Empty => "".to_string(),
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

        fn extract_elements_in_layer_order<T: Ord + Copy + Debug>(tree: &RedBlackTree<T>, v: &mut Vec<String>, v_exist: &mut Vec<bool>, layer: u32, offset: i32) {
            match tree {
                RedBlackTree::Node {
                    colour,
                    data,
                    left_child,
                    right_child,
                } => {
                    let two: u32 = 2;
                    
                    v[(two.pow(layer) as i32 + offset) as usize] = get_node_string(&tree);
                    v_exist[(two.pow(layer) as i32 + offset) as usize] = true;
                    extract_elements_in_layer_order(&left_child.borrow(), v, v_exist, layer + 1, 2*offset);
                    extract_elements_in_layer_order(&right_child.borrow(), v, v_exist, layer + 1, 2*offset + 1);
                }
                RedBlackTree::Empty => {}
            }
        }

        extract_elements_in_layer_order(self, &mut layer_order_elements, &mut layer_order_elements_exist, 0, 0);

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
        
        //println!("Tree height is {}", self.get_height());
        let two: u32 = 2;
        for layer in 0..(self.get_height() - 1) {
            //println!("Doing layer {}: ", layer);
            // Print left padding for this layer
            print!("{}", " ".repeat((3*two.pow((self.get_height() - 3 - layer + 1).try_into().unwrap()) - 2).try_into().unwrap()));
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
                print!("{}", " ".repeat((3*two.pow((self.get_height() - 1 - layer).try_into().unwrap()) - 5).try_into().unwrap()));
            }
            println!("");
            // Print the two lines of branches between two layers, as long as this isn't the last layer since nothing goes below that (we don't display NIL nodes for simplicity)
            // Only print branches with children! Do checks for this within each loop through all nodes
            if layer != self.get_height() - 2 {
                // Within this loop, self.get_height() - 3 - layer is equal to n within the 0 indexed equations above, indexed from the bottom and not the top
                // FIRST LINE
                // Print left padding
                print!("{}", " ".repeat((3*two.pow((self.get_height() - 3 - layer).try_into().unwrap()) + 1).try_into().unwrap()));
                for node in 0..two.pow(layer as u32) {
                    // Check whether this node has left/right children by jumping to the next layer and probing the left and right sides
                    let has_left_child: bool = match layer_order_elements_exist[(two.pow(layer as u32 + 1) + 2*node) as usize] {
                        true => true,
                        false => false,
                    };
                    let has_right_child: bool = match layer_order_elements_exist[(two.pow(layer as u32 + 1) + 2*node + 1) as usize] {
                        true => true,
                        false => false,
                    };

                    // Print the connector thing
                    if has_left_child {
                        // Print underscores
                        print!("{}", "_".repeat((3*two.pow((self.get_height() - 3 - layer).try_into().unwrap()) - 2).try_into().unwrap()));
                        print!(r"/");
                    } else {
                        print!("{}", " ".repeat((3*two.pow((self.get_height() - 3 - layer).try_into().unwrap()) - 2).try_into().unwrap()));
                        print!(" ");
                    }
                    print!(" ");
                    if has_right_child {
                        print!(r"\");
                        // Print the underscores
                        print!("{}", "_".repeat((3*two.pow((self.get_height() - 3 - layer).try_into().unwrap()) - 2).try_into().unwrap()));
                    } else {
                        print!(" ");
                        print!("{}", " ".repeat((3*two.pow((self.get_height() - 3 - layer).try_into().unwrap()) - 2).try_into().unwrap()));
                    }
                    // Print the right space padding
                    print!("{}", " ".repeat((3*two.pow((self.get_height() - 2 - layer).try_into().unwrap()) + 1).try_into().unwrap()));
                }
                println!("");
                // SECOND LINE
                // Print left padding
                print!("{}", " ".repeat((3*two.pow((self.get_height() - 3 - layer).try_into().unwrap())).try_into().unwrap()));
                for node in 0..two.pow(layer as u32) {
                    // Check whether this node has left/right children by jumping to the next layer and probing the left and right sides
                    let has_left_child: bool = match layer_order_elements_exist[(two.pow(layer as u32 + 1) + 2*node) as usize] {
                        true => true,
                        false => false,
                    };
                    let has_right_child: bool = match layer_order_elements_exist[(two.pow(layer as u32 + 1) + 2*node + 1) as usize] {
                        true => true,
                        false => false,
                    };

                    if has_left_child {
                        print!(r"/");
                    } else {
                        print!(" ");
                    }
                    // Print the padding between nodes
                    print!("{}", " ".repeat((3*two.pow((self.get_height() - 2 - layer).try_into().unwrap()) - 1).try_into().unwrap()));
                    if has_right_child {
                        print!(r"\");
                    } else {
                        print!(" ");
                    }
                    // Print the right space padding
                    print!("{}", " ".repeat((3*two.pow((self.get_height() - 2 - layer).try_into().unwrap()) - 1).try_into().unwrap()));
                }
                println!("");
            }
        }
    }

    pub fn rotate_left(self) -> Self {

        let mut new_parent: RedBlackTree<T> = RedBlackTree::Empty;

        match self {
            RedBlackTree::Node {colour, data, left_child, right_child} => {
                let tl_branch = Rc::clone(&left_child);
                let bl_branch = match &*right_child.borrow() {
                    RedBlackTree::Node {colour, data, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                let br_branch = match &*right_child.borrow() {
                    RedBlackTree::Node {colour, data, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                
                let datatemp = &data;
                let colourtemp = &colour;

                
                new_parent = match &*right_child.borrow() {
                    RedBlackTree::Node {colour, data, left_child, right_child} =>  {
                        let t_colour = *colour;
                        let t_data = *data;
                        let t_lc = Rc::new(RefCell::new(RedBlackTree::Node{
                            colour: *colourtemp,
                            data: *datatemp,
                            left_child: tl_branch,
                            right_child:bl_branch
                        }));
                        let t_rc = Rc::clone(&right_child);
                        RedBlackTree::Node {colour:t_colour, data:t_data, left_child: t_lc, right_child: t_rc}
                    }
                    RedBlackTree::Empty => RedBlackTree::Node {
                        colour: *colourtemp,
                        data: *datatemp,
                        left_child: Rc::clone(&left_child),
                        right_child:Rc::clone(&right_child)
                    },
                };


            },
            RedBlackTree::Empty => {},
        }
        new_parent
    }



    pub fn rotate_right(self) -> Self {

        let mut new_parent: RedBlackTree<T> = RedBlackTree::Empty;

        match self {
            RedBlackTree::Node {colour, data, left_child, right_child} => {
                let tr_branch = Rc::clone(&right_child);
                let br_branch = match &*left_child.borrow() {
                    RedBlackTree::Node {colour, data, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                let bl_branch = match &*left_child.borrow() {
                    RedBlackTree::Node {colour, data, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                };
                
                let datatemp = &data;
                let colourtemp = &colour;

                
                new_parent = match &*left_child.borrow() {
                    RedBlackTree::Node {colour, data, left_child, right_child} =>  {
                        let t_colour = *colour;
                        let t_data = *data;
                        let t_rc = Rc::new(RefCell::new(RedBlackTree::Node{
                            colour: *colourtemp,
                            data: *datatemp,
                            left_child: br_branch,
                            right_child:tr_branch
                        }));
                        let t_lc = Rc::clone(&left_child);
                        RedBlackTree::Node {colour:t_colour, data:t_data, left_child: t_lc, right_child: t_rc}
                    }
                    RedBlackTree::Empty => RedBlackTree::Node {
                        colour: *colourtemp,
                        data: *datatemp,
                        left_child: Rc::clone(&left_child),
                        right_child:Rc::clone(&right_child)
                    },
                };


            },
            RedBlackTree::Empty => {},
        }
        new_parent
    }


    pub fn insert(&mut self, val: T) {
        match self {
            RedBlackTree::Node {data, colour, left_child, right_child} => {},
            RedBlackTree::Empty => {
                *self = RedBlackTree::Node {
                    data:val, 
                    colour:NodeColour::Black, 
                    left_child: Rc::new(RefCell::new(RedBlackTree::Empty)), 
                    right_child: Rc::new(RefCell::new(RedBlackTree::Empty))
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
                RedBlackTree::Node {data, colour, left_child, right_child} => {
                    if val > *data {
                        nodetemp = Rc::clone(&right_child);
                    } else if val < *data {
                        nodetemp = Rc::clone(&left_child);
                    } else {
                        return;
                    }
                },
                RedBlackTree::Empty => {break},
            }
            node = Rc::clone(&nodetemp);
            stack.push(Rc::clone(&node));
        }
        // println!("{:#?}", stack);

        stack[stack.len()-1].replace(RedBlackTree::Node {
            data: val.clone(), 
            colour: NodeColour::Red, 
            left_child: Rc::new(RefCell::new(RedBlackTree::Empty)), 
            right_child: Rc::new(RefCell::new(RedBlackTree::Empty))
        });
        ///// END BINARY TREE INSERT //////


        ///// FIX TREE //////
        let mut idx = stack.len()-1;
        let mut uncle: Rc<RefCell<RedBlackTree<T>>>;
        let mut grandfather: Rc<RefCell<RedBlackTree<T>>>;
        let mut parent: Rc<RefCell<RedBlackTree<T>>>;
        while idx >= 2 {
            // we got the grandfather
            grandfather = Rc::clone(&stack[idx-2]);
            let mut gf_left =  Rc::new(RefCell::new(RedBlackTree::Empty));
            let mut gf_right =  Rc::new(RefCell::new(RedBlackTree::Empty));

            let gf_val = match &*grandfather.borrow() {
                RedBlackTree::Node {data, colour, left_child, right_child} => {
                    gf_left = Rc::clone(&left_child);
                    gf_right = Rc::clone(&right_child);
                    data.clone()
                },
                RedBlackTree::Empty => val, // not possible
            };
            if val > gf_val {
                uncle = Rc::clone(&gf_left);
            } else {
                uncle = Rc::clone(&gf_right);
            }
            // now we got the uncle

            parent = Rc::clone(&stack[idx-1]);
            // and we got the parent


            // If parent is black, then we break
            if NodeColour::Black == match &*parent.borrow() {
                RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                RedBlackTree::Empty => NodeColour::Black,
            } {
                break;
            }
            // otherwise...
            // if uncle is red
            if NodeColour::Red == match &*uncle.borrow() {
                RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                RedBlackTree::Empty => NodeColour::Black,
            } {
                // Set uncle and parent as black
                match *parent.borrow_mut() {
                    RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = NodeColour::Black},
                    RedBlackTree::Empty => {},
                }
                match *uncle.borrow_mut() {
                    RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = NodeColour::Black},
                    RedBlackTree::Empty => {},
                }
                // set grandfather as Red
                if idx - 2 > 0 {
                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = NodeColour::Red},
                        RedBlackTree::Empty => {},
                    }
                }

                idx = idx - 2;

                // println!("\n\n\n ROOT\n{:#?}", stack[0]);


            } else { // else if uncle is black
                // we have 4 cases

                // If grandparent.left.left is the current node
                if (match &*grandfather.borrow() {
                    RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent)) &&
                (match &*parent.borrow(){
                    RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&stack[idx])) {
                    // right rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp = gf_temp.rotate_right();
                    parent.replace(gf_temp);

                    grandfather = match &*parent.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // then swap colours of grandfather and parent
                    let gf_colour = match &*grandfather.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let p_colour = match &*parent.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    match *grandfather.borrow_mut(){
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = p_colour},
                        RedBlackTree::Empty => {},
                    };
                    match *parent.borrow_mut(){
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = gf_colour},
                        RedBlackTree::Empty => {},
                    };

                    if idx - 2 == 0 {
                        *self = parent.borrow().clone();
                        return;
                    } else { // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {ref data, ref colour, ref mut left_child, ref mut right_child} => {
                                let upperval = data.clone();
                                let parent_val = match &*parent.borrow() {
                                    RedBlackTree::Node {data, colour, left_child, right_child} => {
                                        *data
                                    },
                                    RedBlackTree::Empty => {upperval.clone()}
                                };
                                if parent_val > upperval {
                                    *right_child = Rc::clone(&parent);
                                } else if parent_val < upperval {
                                    *left_child = Rc::clone(&parent);
                                }
                            },
                            RedBlackTree::Empty => {},
                        };
                    }

                }

                // If grandparent.left.right is the current node
                else if (match &*grandfather.borrow() {
                    RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent)) &&
                (match &*parent.borrow(){
                    RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&stack[idx])) {
                    
                    // left rotate on parent
                    // println!("\n\n\n PARENT OLD:");
                    // parent.borrow().display_tree();
                    // println!("\n\n\n CURRENT OLD:");
                    // stack[idx].borrow().display_tree();

                    let mut p_temp = parent.borrow().clone();
                    p_temp = p_temp.rotate_left();
                    stack[idx].replace(p_temp);

                    parent = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {ref data, ref colour, ref mut left_child, ref right_child} => {*left_child = Rc::clone(&stack[idx])},
                        RedBlackTree::Empty => {},
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
                    gf_temp = gf_temp.rotate_right();
                    stack[idx].replace(gf_temp);

                    grandfather = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // println!("\n\n\n GRANDPARENT NEW:");
                    // grandfather.borrow().display_tree();

                    // println!("\n\n\n CURRENT NEW:");
                    // stack[idx].borrow().display_tree();


                    // then swap colours of grandfather and current
                    let gf_colour = match &*grandfather.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let c_colour = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = c_colour},
                        RedBlackTree::Empty => {},
                    };

                    match *stack[idx].borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = gf_colour},
                        RedBlackTree::Empty => {},
                    };

                    // if new insert is root, return
                    if idx - 2 == 0 {
                        *self = stack[idx].borrow().clone();
                        return ;
                    } else { // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {ref data, ref colour, ref mut left_child, ref mut right_child} => {
                                let upperval = data.clone();
                                let insert_val = match &*stack[idx].borrow() {
                                    RedBlackTree::Node {data, colour, left_child, right_child} => {
                                        *data
                                    },
                                    RedBlackTree::Empty => {upperval.clone()}
                                };
                                if insert_val > upperval {
                                    *right_child = Rc::clone(&stack[idx]);
                                } else if insert_val < upperval {
                                    *left_child = Rc::clone(&stack[idx]);
                                }
                            },
                            RedBlackTree::Empty => {},
                        };
                    }
                    

                }

                // If grandparent.right.left is the current node
                else if (match &*grandfather.borrow() {
                    RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent)) &&
                (match &*parent.borrow(){
                    RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&left_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&stack[idx])) {
                    // right rotate on parent
                    let mut p_temp = parent.borrow().clone();
                    p_temp = p_temp.rotate_right();
                    stack[idx].replace(p_temp);

                    parent = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&right_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {ref data, ref colour, ref left_child, ref mut right_child} => {*right_child = Rc::clone(&stack[idx])},
                        RedBlackTree::Empty => {},
                    };

                    // left rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp = gf_temp.rotate_left();
                    stack[idx].replace(gf_temp);

                    grandfather = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };

                    // then swap colours of grandfather and current
                    let gf_colour = match &*grandfather.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let c_colour = match &*stack[idx].borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                  
                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = c_colour},
                        RedBlackTree::Empty => {},
                    };


                    match *stack[idx].borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = gf_colour},
                        RedBlackTree::Empty => {},
                    };

                    // if new insert is root, return
                    if idx - 2 == 0 {
                        *self = stack[idx].borrow().clone();
                        return;
                    } else { // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {ref data, ref colour, ref mut left_child, ref mut right_child} => {
                                let upperval = data.clone();
                                let insert_val = match &*stack[idx].borrow() {
                                    RedBlackTree::Node {data, colour, left_child, right_child} => {
                                        *data
                                    },
                                    RedBlackTree::Empty => {upperval.clone()}
                                };
                                if insert_val > upperval {
                                    *right_child = Rc::clone(&stack[idx]);
                                } else if insert_val < upperval {
                                    *left_child = Rc::clone(&stack[idx]);
                                }
                            },
                            RedBlackTree::Empty => {},
                        };
                    }


                }
                
                // If grandparent.right.right is the current node
                else if (match &*grandfather.borrow() {
                    RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&parent)) &&
                (match &*parent.borrow(){
                    RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&right_child),
                    RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                } == Rc::clone(&stack[idx])) {


                    // left rotate on grandfather
                    let mut gf_temp = grandfather.borrow().clone();
                    gf_temp = gf_temp.rotate_left();
                    parent.replace(gf_temp);

                    grandfather = match &*parent.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => Rc::clone(&left_child),
                        RedBlackTree::Empty => Rc::new(RefCell::new(RedBlackTree::Empty)),
                    };


                    // then swap colours of grandfather and parent
                    let gf_colour = match &*grandfather.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                        RedBlackTree::Empty => NodeColour::Black,
                    };
                    let p_colour = match &*parent.borrow(){
                        RedBlackTree::Node {data, colour, left_child, right_child} => colour.clone(),
                        RedBlackTree::Empty => NodeColour::Black,
                    };


                    match *grandfather.borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = p_colour},
                        RedBlackTree::Empty => {},
                    };


                    match *parent.borrow_mut() {
                        RedBlackTree::Node {ref data, ref mut colour, ref left_child, ref right_child} => {*colour = gf_colour},
                        RedBlackTree::Empty => {},
                    };


                    // if parent is root, return
                    if idx - 2 == 0 {
                        *self = parent.borrow().clone();
                        return;
                    } else { // else, attach the parent node to the upper tree
                        match *stack[idx - 3].borrow_mut() {
                            RedBlackTree::Node {ref data, ref colour, ref mut left_child, ref mut right_child} => {
                                let upperval = data.clone();
                                let parent_val = match &*parent.borrow() {
                                    RedBlackTree::Node {data, colour, left_child, right_child} => {
                                        *data
                                    },
                                    RedBlackTree::Empty => {upperval.clone()}
                                };
                                if parent_val > upperval {
                                    *right_child = Rc::clone(&parent);
                                } else if parent_val < upperval {
                                    *left_child = Rc::clone(&parent);
                                }
                            },
                            RedBlackTree::Empty => {},
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


}
