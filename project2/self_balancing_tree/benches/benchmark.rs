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

#[path = "../src/red_black.rs"]
mod red_black;

#[path = "../src/avl.rs"]
mod avl;

#[path = "../src/readbt.rs"]
mod readbt;


use avl::*;
use red_black::*;
use readbt::*;





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