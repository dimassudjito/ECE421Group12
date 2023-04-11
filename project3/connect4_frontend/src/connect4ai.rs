use crate::board::*;
use crate::boardgame::*;
use crate::chip::*;
use crate::fsm::*;
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

    pub fn mcts(&self, board: &Board<Chip>, iters: usize) -> Vec<i32> {
        let mut rng = rand::thread_rng();

        let mut cols = Vec::new();

        for c in 0..board.size.1 {
            let mut score = 0;
            let mut board_clone = board.clone();
            let ref_count = board.counter.clone();
            let mut init_move = true;
            let mut next_move = c;
            // println!("NEW ITER");

            for i in 0..iters {
                if board_clone.counter >= board.size.0 * board.size.1 {
                    board_clone = board.clone();
                }

                let chip = if board_clone.counter % 2 == 0 {
                    Chip::One
                } else {
                    Chip::Two
                };

                if init_move {
                    next_move = c;
                    init_move = false;
                } else {
                    next_move = rng.gen_range(0..board.size.1);
                }

                if let Ok(pair) = board_clone.insert(&chip, None, Some(next_move)) {
                    // board_clone.debug_print(false);
                    if board_clone.detect(
                        pair.0,
                        pair.1,
                        &mut FSM::<Chip>::new(vec![chip, chip, chip, chip]),
                    ) {
                        let mut scoreval = (board.size.0 * board.size.1 + 1) as i32 * 1000;
                        scoreval /= i32::pow((board_clone.counter - ref_count) as i32, 2);

                        if board_clone.counter % 2 == 0 {
                            score += scoreval;
                        } else {
                            score -= scoreval;
                        }
                        board_clone = board.clone();
                        next_move = c;
                        init_move = true;
                    }
                } else {
                    score -= 1000 / i32::pow((board_clone.counter - ref_count + 1) as i32, 2);
                }
            }
            cols.push(score.clone());
        }

        return cols;
    }

    pub fn play(&self, boardgame: &mut BoardGame, difficulty: usize, iters: usize) -> usize {
        let mut rng = thread_rng();

        let cols = self.mcts(&boardgame.board, iters);

        println!("AI Probabilities: {:?}", cols);

        let mut indices = (0..cols.len()).collect::<Vec<_>>();
        indices.sort_by_key(|&i| (-cols[i]));

        let pick = match difficulty {
            1 => rng.gen_range(0..3),
            2 => rng.gen_range(0..2),
            3 => 0,
            _ => 0,
        };

        return indices[pick];
    }
}
