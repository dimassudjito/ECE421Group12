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


    pub fn mcts(&self, board: &Board<Chip>, iters: usize) -> usize {

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

                let chip = if board_clone.counter % 2 == 0 {Chip::One} else {Chip::Two};

                if init_move {
                    next_move = c;
                    init_move = false;
                } else {
                    next_move = rng.gen_range(0..board.size.1);
                }

                if let Ok(pair) = board_clone.insert(&chip, None, Some(next_move)) {
                    // board_clone.debug_print(false);
                    if board_clone.detect(pair.0, pair.1, &mut FSM::<Chip>::new(vec![chip, chip, chip, chip])) {

                        let mut scoreval = (board.size.0 * board.size.1 + 1) as i32 * 1000;
                        scoreval /= i32::pow((board_clone.counter- ref_count) as i32, 2);


                        if board_clone.counter % 2  == 0 {
                            score +=  scoreval;
                        } else {
                            score -= scoreval;
                        }
                        board_clone = board.clone();
                        next_move = c;
                        init_move = true;
                    
                    }
                } else {
                    score -= 1000 / i32::pow((board_clone.counter- ref_count + 1) as i32, 2);
                }

            }
            cols.push(score.clone());
        }

        let mut idx = 0;
        let mut best = i32::MIN;

        for c in 0..cols.len() {
            if cols[c] > best {
                best = cols[c];
                idx = c;
            }
        }

        println!("AI Recommendation: {:?}", cols);


        return idx;

    }


    pub fn game_over(&self, board: &mut Board<Chip>) -> Option<Chip> {
        let mut p_win = FSM::<Chip>::new(vec![Chip::One, Chip::One, Chip::One, Chip::One]);
        let mut a_win = FSM::<Chip>::new(vec![Chip::Two, Chip::Two, Chip::Two, Chip::Two]);

        for y in 0..board.size.0 {
            for x in 0..board.size.1 {
                if board.detect(y, x, &mut p_win) {
                    return Some(Chip::One);
                }
                if board.detect(y, x, &mut a_win) {
                    return Some(Chip::Two);
                }
            }
        }
        return None;

    } 





    pub fn get_score(&self, board: &mut Board<Chip>, chip: Chip) -> i32 {
        
        let mut score = 0;
        let mut other_chip = Chip::Two;
        if chip == Chip::Two {
            other_chip = Chip::One;
        }


        for y in 0..board.size.0 {
            let mut window = Vec::new();
            for x in 0..board.size.1 {
                window.push(board.container[y][x]);

                if window.len() > 4 {
                    window.remove(0);
                }

                score += self.compute_window(&mut window, chip);
                score -= self.compute_window(&mut window, other_chip);
            }
        }

        for x in 0..board.size.1 {
            let mut window = Vec::new();
            for y in 0..board.size.0 {
                window.push(board.container[y][x]);

                if window.len() > 4 {
                    window.remove(0);
                }

                score += self.compute_window(&mut window, chip);
                score -= self.compute_window(&mut window, other_chip);
            }
        }


        for y in 0..board.size.0-3 {
            for x in 0..board.size.1-3 {
                let mut window = Vec::new();
                for i in 0..4 as usize {
                    let y_pos = y + i;
                    let x_pos = x + i;

                    if y_pos < board.size.0 && x_pos < board.size.1 {
                        window.push(board.container[y][x]);
                        score += self.compute_window(&mut window, chip);
                        score -= self.compute_window(&mut window, other_chip);
                    }

                }
            }
        }
        

        for y in 3..board.size.0 {
            for x in 0..board.size.1-3 {
                let mut window = Vec::new();
                for i in 0..4 as usize {
                    let y_pos = y - i;
                    let x_pos = x + i;

                    if y_pos > 0 && x_pos < board.size.1 {
                        window.push(board.container[y][x]);
                        score += self.compute_window(&mut window, chip);
                        score -= self.compute_window(&mut window, other_chip);
                    }

                }
            }
        }

        return score;

    }

    pub fn compute_window(&self, window: &mut Vec<Option<Chip>>, chip: Chip) -> i32 {
        let mut chip_counter = 0;
        let mut empty_counter = 0;
        
        if window.len() < 4 {
            return 0;
        }

        for i in 0..window.len() {
            if let Some(item) = window[i] {
                if item == chip {
                    chip_counter += 1;
                }
            } else {
                empty_counter += 1;
            }
        }

        if chip_counter == 2 && empty_counter == 2 {
            return 2;
        }
        if chip_counter == 3 && empty_counter == 1 {
            return 10;
        }

        if chip_counter == 4 {
            return 50;
        }
        return 0;
    }


    pub fn minimax(&self, board: &mut Board<Chip>, depth: usize, alpha: &mut i32, beta: &mut i32, maximize: bool) -> i32 {
        
        let mut score = 0;
        let game_state = self.game_over(board); 

        if let Some(chip) = game_state {
            if let Chip::One = chip {
                return -50;
            } else {
                return 50;
            }
        } else if board.counter >= board.size.0 * board.size.1 {
            return 0;
        }

        if depth == 0 {
            return self.get_score(board, Chip::Two); 
        }

        let mut cols = Vec::new();
        for x in 0..board.size.1 {
            if board.insertable(x) {
                cols.push(x);
            }
        }

        // cols.shuffle(&mut thread_rng());


        if maximize {
            let mut value = i32::MIN;
            for x in cols.iter() {
                if let Ok(pair) = board.insert(&Chip::Two, None, Some(*x)) {
                    let new_score = self.minimax(board, depth-1, &mut alpha.clone(), &mut beta.clone(), false);

                    if new_score > value {
                        value = new_score;
                    }


                    board.remove(pair.0, pair.1);
                }
                *alpha = max(*alpha, value);
                if *alpha >= *beta {
                    break;
                }
            }

            return value;
        } else {
            let mut value = i32::MAX;
            for x in cols.iter() {
                if let Ok(pair) = board.insert(&Chip::One, None, Some(*x)) {
                    let new_score = self.minimax(board, depth-1, &mut alpha.clone(), &mut beta.clone(), true);

                    if new_score < value {
                        value = new_score;
                    }


                    board.remove(pair.0, pair.1);
                }
                *beta = min(*beta, value);
                if *alpha >= *beta {
                    break;
                }
            }

            return value;

        }
        return 0;
    }

}
