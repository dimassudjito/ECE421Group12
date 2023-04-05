use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::cmp;
use std::marker::Copy;
use std::fmt::{self, Display, Formatter};

use crate::fsm::*;

#[derive(Clone)]
pub struct Board<T> {
    pub size: (usize, usize),           // size of board (rows, cols)
    pub items: (Option<T>, Option<T>),  // P1 and P2 objects
    pub container: Vec<Vec<Option<T>>>, // board container
    pub counter: usize,                 // counts the number of items on the board
}

impl <T: Clone + Eq + Hash + Debug + Copy + Display> Board<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        
        let mut outer = Vec::<Vec<Option<T>>>::with_capacity(rows);
        for i in 0..rows {
            let mut inner = Vec::<Option<T>>::with_capacity(cols);
            for i in 0..cols {
                inner.push(None);
            }
            outer.push(inner);
        }

        Board {
            size: (rows, cols),
            items: (None, None),
            container: outer,
            counter: 0
        }
    }

    /**
    * private insert function. Does not let you insert if Some() exists at the location
    */
    fn __insert__(&mut self, item: &T, pos_y: usize, pos_x: usize ) -> Result<(), String> {
        if let Some(x) = &self.container[pos_y][pos_x] {
            return Err("Item already exists at this location".to_string());
        }
        self.container[pos_y][pos_x] = Some(item.clone());
        Ok(())
    }


    /**
    * Inserts an item into the board. pos_x must be specified. If pos_y is not specified, 
    * the item will be inserted to the bottom of the board at pos_x.
    */
    pub fn insert(&mut self, item: &T, pos_y: Option<usize>, pos_x: Option<usize>) -> Result<(usize, usize), String> {
        let px = pos_x.expect("Position x must be supplied");
        let mut py: usize = 0;
    
        let mut push_down = false;

        if let Some(y) = pos_y { // if pos_y is defined
            // then we will not push down
            py = y; 
        } else {
            push_down = true;
        }

        let mut inserted = false;

        if push_down { 
            for i in 0..self.size.0 {

                if let Some(cell) = &self.container[i][px] {
                    if i != 0 {
                        self.container[i-1][px] = Some(item.clone());
                        inserted = true;
                        py = i-1;
                    } 
                    break;
                } else {
                    if i == self.size.0 - 1 {
                        self.container[i][px] = Some(item.clone());
                        py = i;
                        inserted = true;
                    }
                }

            }

        } else {
            self.__insert__(item, py, px)?;
            inserted = true;
        }

        if !inserted {
            return Err("Item was not inserted".to_string());
        }

        self.counter += 1;
        match self.counter {
            1 => self.items.0 = Some(item.clone()),
            2 => self.items.1 = Some(item.clone()),
            _ => {},
        }
        
        Ok((py, px))
    }
    
    pub fn remove(&mut self, pos_y: usize, pos_x: usize) -> Result<(), String> {
        if pos_y >= self.size.0 {
            return Err("Position y out of bounds".to_string());
        } 
        if pos_x >= self.size.1 {
            return Err("Position x out of bounds".to_string());
        }

        if let None = self.container[pos_y][pos_x] {
            return Err("Value does not exist at this location".to_string());
        }
        self.container[pos_y][pos_x] = None;
        self.counter -= 1;
        Ok(())
    }

    pub fn detect(&self, pos_y: usize, pos_x: usize, fsm: &mut FSM<T>) -> bool {

        let mut v = Vec::<T>::new();
        let mut h = Vec::<T>::new();
        let mut d1 = Vec::<T>::new();
        let mut d2 = Vec::<T>::new();

        // vertical sequence detection
        fsm.clear();
        for i in  (cmp::max(pos_y as i32 - 3, 0)..cmp::min(pos_y as i32 + 4, self.size.0 as i32)) { 
            if let Some(item) = &self.container[i as usize][pos_x] {
                v.push(item.clone());
                if fsm.step(&item.clone()) {
                    return true;
                }
            } else {
                fsm.clear();
            }
        } 

        // horizontal sequence detection
        fsm.clear();
        for i in  (cmp::max(pos_x as i32 - 3, 0)..cmp::min(pos_x as i32 + 4, self.size.1 as i32)) {
            if let Some(item) = &self.container[pos_y][i as usize] {
                h.push(item.clone());

                if fsm.step(&item.clone()) {
                    return true;
                }
            } else {
                fsm.clear();
            }
        }
        
        // forward diagonal
        fsm.clear();
        for i in -3..4 {
            let y_idx = pos_y as i32 + i;
            let x_idx = pos_x as i32 + i;

            if y_idx >= 0 && y_idx < self.size.0 as i32 && x_idx >= 0 && x_idx < self.size.1 as i32 {
                if let Some(item) = &self.container[y_idx as usize][x_idx as usize] {
                    d1.push(item.clone()); 
                    
                    if fsm.step(&item.clone()) {
                        return true;
                    }
                } else {
                    fsm.clear();
                }
            }
        }

        
        // Backward diagonal
        fsm.clear();
        for i in -3..4 {
            let y_idx = pos_y as i32 - i;
            let x_idx = pos_x as i32 + i;

            if y_idx >= 0 && y_idx < self.size.0 as i32 && x_idx >= 0 && x_idx < self.size.1 as i32 {
                if let Some(item) = &self.container[y_idx as usize][x_idx as usize] {
                    d2.push(item.clone()); 
                    if fsm.step(&item.clone()) {
                        return true;
                    }
                } else {
                    fsm.clear();
                }
            }
        }

        // println!("{:?}", v);
        // println!("{:?}", h);
        // println!("{:?}", d1);
        // println!("{:?}", d2);

        return false;
    }


    pub fn insertable(&self, pos_x: usize) -> bool {
        if let None = self.container[0][pos_x] {
            return true;
        }
        return false;
    }

    pub fn game_winning_play(&mut self, pos_x: usize, item: &T, seq: Vec<T>) -> bool {
        if let Ok(pair) = self.insert(item, None, Some(pos_x)) {
            if self.detect(pair.0, pair.1, &mut FSM::<T>::new(seq)) {
                self.remove(pair.0, pair.1);
                return true;
            }
            self.remove(pair.0, pair.1);
        }
        return false;
    }


    

    
    /**
    * Prints the board with the locations of inserted items. Items will be either marked with
    * X or O, with no particular ordering. Used for debugging only. If more than two different
    * kinds of items were inserted, this function will have a problem with that.
    *
    * realvals is a toggle for printing either placeholders, or real values.
    * print the real values if you are debugging using single-character primatives
    * (for example, single digit integers), and print placeholder values 
    * (the X's and O's mentioned earlier) if you are using objects...
    */
    pub fn debug_print(&self, realvals: bool) {

        let mut seen = HashMap::<T, String>::new(); // stores placeholder values for non-realval printing
        
        print!("  ");
        for i in 0..self.size.1 {
            print!("{} ", i);
        }
        println!(""); 

        print!("[");
        for i in 0..self.container.len() {
            if i != 0 {
                print!(" ");
            } 
            print!("[");
            for j in 0..self.container[i].len() {
                if let Some(x) = &self.container[i][j] {
                    if !seen.contains_key(&x) {
                        match seen.len() {
                            0 => seen.insert(x.clone(), "X".to_string()),
                            1 => seen.insert(x.clone(), "O".to_string()),
                            _ => {
                                println!("INCORRECT VALUES DETECTED");
                                seen.insert(x.clone(), "?".to_string())
                            },
                        };
                    } 
                    if realvals {
                        print!("{:?} ", x.clone());
                    } else {
                        print!("{} ", x.clone());
                    }
                } else {
                    print!("_ ");
                }
            }
            print!("]");
            if i < self.container.len()-1 {
                print!("\n");
            }
        }
        print!("]\n");
    }

    


}
