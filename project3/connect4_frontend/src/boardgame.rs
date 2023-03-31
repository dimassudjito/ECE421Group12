
use crate::fsm::*;
use crate::board::*;
use crate::chip::*;




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
 
    pub fn get_turn(&self) -> i32 {
        if self.count %2 == 0 {
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

        let mut fsm: FSM<Chip>;
        
        if self.get_turn() == 1 {
            fsm = FSM::<Chip>::new(self.p1_seq.clone());
        } else {
            fsm = FSM::<Chip>::new(self.p2_seq.clone());
        }

        // insert chip into the board
        let res = self.board.insert(&chip, None, Some(pos_x))?;
        
        if self.board.detect(res.0, res.1, &mut fsm) {
            return Ok(Some(self.get_turn()));
        }

        self.count += 1;
        if self.count >= self.board.size.0 * self.board.size.1 {
            return Err("TIE".to_string());
        }
        return Ok(None);
        
    }

    pub fn get_board(self) -> Board<Chip> {
        return self.board;
    }
}
