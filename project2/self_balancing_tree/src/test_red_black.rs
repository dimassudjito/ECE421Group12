use crate::red_black::*;
use crate::readbt::ReadableBinaryTree;

pub fn test_red_black() {
    let mut rbt2 = RedBlackTree::new();
    rbt2.insert(8);
    rbt2.insert(18);
    rbt2.insert(5);
    rbt2.insert(15);
    rbt2.insert(17);
    rbt2.insert(25);
    rbt2.insert(40);
    rbt2.insert(80);
    rbt2.insert(4);
    rbt2.insert(14);
    rbt2.insert(16);
    rbt2.insert(24);
    rbt2.insert(39);
    rbt2.insert(79);
    rbt2.insert(2);
    rbt2.insert(12);
    // rbt2.insert(11);
    // rbt2.insert(20);
    // rbt2.insert(37);
    // rbt2.insert(27);
    // rbt2.insert(15);
    // rbt2.insert(115);
    // rbt2.insert(117);
    // rbt2.insert(125);
    // rbt2.insert(140);
    // rbt2.insert(180);
    // rbt2.insert(14);
    // rbt2.insert(114);
    // rbt2.insert(116);
    // rbt2.insert(124);
    // rbt2.insert(139);
    // rbt2.insert(179);
    // rbt2.insert(12);
    // rbt2.insert(112);
    // rbt2.insert(111);
    // rbt2.insert(120);
    // rbt2.insert(137);
    // rbt2.insert(127);

    // println!("\n\n{:#?}", rbt2);

    // println!("Leaf nodes: {}", rbt2.count_leaves());
    // println!("Tree height: {}", rbt2.get_height());
    // rbt2.in_order_traversal();
    println!("\n\n\n\n\n");
    rbt2.print_tree();

    println!("\n\n\nSEARCH RESULT:");
    // rbt2.search(8).display_tree();
}