use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;



use crate::fsm::*;

pub struct Board<T> {
    pub size: (usize, usize),           // size of board (rows, cols)
    pub items: (Option<T>, Option<T>),  // P1 and P2 objects
    pub container: Vec<Vec<Option<T>>>, // board container
    pub counter: usize,                 // counts the number of items on the board
}

impl <T: Clone + Eq + Hash + Debug> Board<T> {
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
            println!("y specified"); 
        } else {
            push_down = true;
            println!("y not specified"); 
        }

        let mut inserted = false;

        if push_down { // Here we do not use py
            for i in 0..self.size.0 {

                if let Some(cell) = &self.container[i][px] {
                    if i != 0 {
                        self.container[i-1][px] = Some(item.clone());
                        inserted = true;
                        py = i-1;
                        break;
                    } else {
                        break;
                    }
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
        
        Ok((px, py))
    }


pub fn detect(&self, pos_y: usize, pos_x: usize) -> bool {
        return true;
    }







    /**
    * Prints the board with the locations of inserted items. Items will be either marked with
    * X or O, with no particular ordering. Used for debugging only. If more than two different
    * kinds of items were inserted, this function will have a problem with that.
    */
    pub fn debug_print(&self) {
        let mut seen = HashMap::<T, String>::new();

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
                    print!("{}", seen[&x]);
                } else {
                    print!("_");
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
