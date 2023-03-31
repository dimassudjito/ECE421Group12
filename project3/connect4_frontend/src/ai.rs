use crate::board::*;
use crate::chip::*;
use crate::fsm::*;
use std::cmp::*;
use rand::prelude::*;

pub struct Connect4AI {
    max_depth: usize,
}

impl Connect4AI {
    pub fn new(max_depth: usize) -> Self {
        Connect4AI {
            max_depth: max_depth,
        }
    }

    pub fn negamax(&self, board: &mut Board<Chip>, depth: usize) -> i32 {

        let color = if board.counter %2 == 1 {1} else {-1};
        let ichip = if board.counter %2 == 0 {Chip::One} else {Chip::Two};
        let ochip = if board.counter %2 == 0 {Chip::Two} else {Chip::One};


        if depth == 0 || (board.counter >= board.size.0 * board.size.1) {
            return color * ((board.size.0 * board.size.1 + 1 - board.counter) as i32)/ 2;
        }

        for x in 0..board.size.1 {
            if board.insertable(x) && board.game_winning_play(x, &ichip, vec![ichip, ichip ,ichip, ichip]) {
                return color * ((board.size.0 * board.size.1 + 1 - board.counter) as i32) / 2;
            }
        }

        let mut value = -((board.size.0 * board.size.1) as i32);
        for x in 0..board.size.1 {
            if board.insertable(x) {

                if let Ok(pair) = board.insert(&ochip, None, Some(x)) {
                    value  = max(value.clone(), -self.negamax(board, depth - 1));
                    board.remove(pair.0, pair.1);
                }
            }
        }

        return value;
    }


    pub fn mcts(&self, board: &Board<Chip>, iters: usize) -> i32 {
        let mut score = 0;

        let mut rng = rand::thread_rng();
        
        let mut board_clone = board.clone();
        let ref_count = board.counter.clone();
        for i in 0..iters {
            if board_clone.counter >= board.size.0 * board.size.1 {
                board_clone = board.clone();
            }

            let chip = if board_clone.counter % 2 == 0 {Chip::One} else {Chip::Two};

            if let Ok(pair) = board_clone.insert(&chip, None, Some(rng.gen_range(0..board.size.1))) {
                
                if board_clone.detect(pair.0, pair.1, &mut FSM::<Chip>::new(vec![chip, chip, chip, chip])) {
                    if board_clone.counter % 2  == 0 {
                        score +=  (board.size.0 * board.size.1) as i32 / (board_clone.counter- ref_count) as i32;
                    } else {
                        score -= (board.size.0 * board.size.1) as i32 / (board_clone.counter - ref_count) as i32;
                    }
                    board_clone = board.clone();

                }
                
                
            }
        }

        return score;

    }

}
