use crate::board::*;
use crate::chip::*;
use crate::fsm::*;
use crate::boardgame::*;
use std::cmp::*;
use rand::prelude::*;



pub struct TootOttoAI {

}


impl TootOttoAI {

    pub fn new() -> Self {
        TootOttoAI {  }
    }

    pub fn mcts(&self, board: &Board<Chip>, iters: usize) -> (Vec<i32>, Vec<i32>) {
        let mut tscore = Vec::new();
        let mut oscore = Vec::new();

        let mut rng = thread_rng();

        for c in 0..board.size.1 {
            let mut score = 0;
            let mut board_clone = board.clone();
            let mut first_start = true;
            let ref_count = board.counter.clone();



            // starting with O
            for i in 0..iters {
                if board_clone.counter >= board_clone.size.0 * board_clone.size.1 {
                    board_clone = board.clone();
                }
                
                let mut pos = rng.gen_range(0..board.size.1);
                let mut chip = if rng.gen_range(0.0..1.0) > 0.5 {Chip::One} else {Chip::Two};

                if first_start {
                    pos = c;
                    chip = Chip::One;
                    first_start = false;
                }

                let mut fsm1 = FSM::<Chip>::new(vec![Chip::Two, Chip::One, Chip::One, Chip::Two]);
                let mut fsm2 = FSM::<Chip>::new(vec![Chip::One, Chip::Two, Chip::Two, Chip::One]);

                let mut scoreval = 0;

                if let Ok(pair) = board_clone.insert(&chip, None, Some(pos)) {
                    scoreval = 1000 * (board_clone.size.0 * board_clone.size.1) as i32;
                    scoreval /= i32::pow((board_clone.counter - ref_count + 1) as i32 , 2);
                    let mut won = false;

                    if board_clone.detect(pair.0, pair.1, &mut fsm1) {
                        score -= 2 * scoreval;
                        won = true;
                    }    
                    if board_clone.detect(pair.0, pair.1, &mut fsm2) {
                        score += scoreval;
                        won = true;
                    }
                    if won {
                        board_clone = board.clone();
                        first_start = true;
                    }

                } else {
                    score -= 1000 / i32::pow((board_clone.counter- ref_count + 1) as i32, 2);
                }

            }

            oscore.push(score);

            let mut score = 0;
            let mut board_clone = board.clone();
            let mut first_start = true;
            let ref_count = board.counter.clone();
            //starting with T
            for i in 0..iters {
                if board_clone.counter >= board_clone.size.0 * board_clone.size.1 {
                    board_clone = board.clone();
                }

                let mut pos = rng.gen_range(0..board.size.1);
                let mut chip = if rng.gen_range(0.0..1.0) > 0.5 {Chip::One} else {Chip::Two};

                
                if first_start {
                    pos = c;
                    chip = Chip::Two;
                    first_start = false;
                }

                let mut fsm1 = FSM::<Chip>::new(vec![Chip::Two, Chip::One, Chip::One, Chip::Two]);
                let mut fsm2 = FSM::<Chip>::new(vec![Chip::One, Chip::Two, Chip::Two, Chip::One]);

                let mut scoreval = 0;

                if let Ok(pair) = board_clone.insert(&chip, None, Some(pos)) {
                    scoreval = 1000 * (board_clone.size.0 * board_clone.size.1) as i32;
                    scoreval /= i32::pow((board_clone.counter - ref_count + 1) as i32 , 2);
                    let mut won = false;

                    if board_clone.detect(pair.0, pair.1, &mut fsm1) {
                        score -= 2 * scoreval;
                        won = true;
                    }    
                    if board_clone.detect(pair.0, pair.1, &mut fsm2) {
                        score += scoreval;
                        won = true;
                    }
                    if won {
                        board_clone = board.clone();
                        first_start = true;
                    }

                } else {
                    score -= 1000 / i32::pow((board_clone.counter- ref_count + 1) as i32, 2);
                }

            }
            
            tscore.push(score);

        }

        return (tscore, oscore);
    }

    // changes boardgame in-place
    pub fn play(&self, boardgame: &mut BoardGame, difficulty: usize, iters: usize) -> (usize, Chip) {
        let (tscore, oscore) = self.mcts(&mut boardgame.board, iters);
        
        let mut rng = thread_rng();

        println!("T probabilities: {:?}", tscore);
        println!("O probabilities: {:?}", oscore);
        
    
        let mut tidx = (0..tscore.len()).collect::<Vec<_>>();
        tidx.sort_by_key(|&i| (-tscore[i]));

        let mut oidx = (0..oscore.len()).collect::<Vec<_>>();
        oidx.sort_by_key(|&i| (-oscore[i]));

        let pick = match difficulty {
            1 => rng.gen_range(0..3),
            2 => rng.gen_range(0..2),
            3 => 0,
            _ => 0,
        };

        let mut candidate_chip = Chip::One;
        let mut idx = 0;

        if tscore[tidx[pick]] > oscore[oidx[pick]] {
            candidate_chip = Chip::Two;
            idx = tidx[pick];
        } else {
            candidate_chip = Chip::One;
            idx = oidx[pick];
        }


        

 


        return (idx, candidate_chip); 

    }


} 
