use crate::board::*;
use crate::chip::*;
use crate::fsm::*;

#[derive(Clone)]
pub struct BoardGame {
    pub board: Board<Chip>,
    pub count: usize,
    p1_seq: Vec<Chip>,
    p2_seq: Vec<Chip>,
}

impl BoardGame {
    pub fn connect4(rows: usize, cols: usize) -> Self {
        BoardGame {
            board: Board::new(rows, cols),
            count: 0,
            p1_seq: vec![Chip::One, Chip::One, Chip::One, Chip::One],
            p2_seq: vec![Chip::Two, Chip::Two, Chip::Two, Chip::Two],
        }
    }

    pub fn toot_otto(rows: usize, cols: usize) -> Self {
        BoardGame {
            board: Board::new(rows, cols),
            count: 0,
            p1_seq: vec![Chip::Two, Chip::One, Chip::One, Chip::Two],
            p2_seq: vec![Chip::One, Chip::Two, Chip::Two, Chip::One],
        }
    }

    pub fn connect3(rows: usize, cols: usize) -> Self {
        BoardGame {
            board: Board::new(rows, cols),
            count: 0,
            p1_seq: vec![Chip::One, Chip::One, Chip::One],
            p2_seq: vec![Chip::Two, Chip::Two, Chip::Two],
        }
    }

    pub fn get_turn(&self) -> i32 {
        if self.count % 2 == 0 {
            return 1;
        }
        return 2;
    }

    /**
    * Returns 3 possible states:
    *       Ok(Some(i32)): one of the players won
    *       Ok(None): Normal state in the game where an insertion is successful but no
                    win state was declared
            Err(String): Something wrong happened, or tie game
    */
    pub fn insert(&mut self, pos_x: usize, chip: Chip) -> Result<Option<i32>, String> {
        let mut onewin = false;
        let mut twowin = false;

        // insert chip into the board
        let res = self.board.insert(&chip, None, Some(pos_x))?;

        let mut fsm1 = FSM::<Chip>::new(self.p1_seq.clone());
        let mut fsm2 = FSM::<Chip>::new(self.p2_seq.clone());

        if self.board.detect(res.0, res.1, &mut fsm1) {
            onewin = true;
        }
        if self.board.detect(res.0, res.1, &mut fsm2) {
            twowin = true;
        }

        self.count += 1;
        if self.count >= self.board.size.0 * self.board.size.1 {
            return Err("TIE".to_string());
        }

        if onewin && !twowin {
            return Ok(Some(1));
        } else if !onewin && twowin {
            return Ok(Some(2));
        } else if onewin && twowin {
            return Err("TIE".to_string());
        }

        return Ok(None);
    }

    pub fn get_board(self) -> Board<Chip> {
        return self.board;
    }
}