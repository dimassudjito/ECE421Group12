use std::io;

pub fn cli() {
    println!("1. Start AVL tree: init avl");
    println!("2. Start Red-Black tree: init rb");
    println!("3. Exit: exit");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");

    match input.trim() {
        "init avl" => {avl_cli();},
        "init rb" => {rb_cli();},
        "exit" => return,
        _ => {println!("Invalid input");}
    }
}

fn avl_cli() {
    println!("Executing avl cli");
}

fn rb_cli() {
    println!("Executing rb cli");
}