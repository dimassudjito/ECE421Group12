use std::cmp::PartialEq;


pub struct FSM<T> {
    pub seq: Vec<T>,
    pub size: usize,
    pub idx: usize,
    pub buffer: Vec<Option<T>>,
}

impl <T: Clone + PartialEq + Copy> FSM<T> {
    pub fn new(seq: Vec<T>) -> Self {
        let mut buf = Vec::<Option<T>>::new();
        for i in 0..seq.len() {
            buf.push(None);
        }

        FSM {
            seq: seq.clone(),
            size: seq.len(),
            idx: 0,
            buffer: buf.clone(), 
        }
    }

    pub fn step(&mut self, item: &T) -> bool {
        self.buffer.push(Some(*item));
        self.buffer.remove(0);


        return self.__check_eq__();

    }

    fn __check_eq__(&mut self) -> bool {
        for i in 0..self.size {
            if let Some(x) = &self.buffer[i] {
                if !(*x == self.seq[i]) {
                    return false;
                } 
            } else {
                return false;
            }
        }
        return true;
    }
}
