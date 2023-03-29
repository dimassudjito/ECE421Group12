
use crate::fsm::*;
use crate::board::*;



#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum CChip {
    Red,
    Yellow,
}


pub struct Connect4 {
    pub board: Board<CChip>,
    p1_seq: Vec<CChip>,
    p2_seq: Vec<CChip>,

}
    


impl Connect4 {

    pub fn new(rows: usize, cols: usize) -> Self {
        Connect4 {
            board: Board::new(rows, cols),
            p1_seq: vec![CChip::Red, CChip::Red, CChip::Red, CChip::Red],
            p2_seq: vec![CChip::Yellow, CChip::Yellow, CChip::Yellow, CChip::Yellow],
        }
    }

 
    pub fn get_turn(&self) -> CChip {
        if self.board.counter %2 == 0 {
            return CChip::Red;
        } 
        return CChip::Yellow;
    }

    /**
    * Returns 3 possible states:
    *       Ok(Some(Chip)): one of the players won
    *       Ok(None): Normal state in the game where an insertion is successful but no 
                    win state was declared
            Err(String): Something wrong happened
    */
    pub fn insert(&mut self, pos_x: usize) -> Result<Option<CChip>, String> {

        let mut fsm: FSM<CChip>;
        
        if let CChip::Red = self.get_turn() {
            fsm = FSM::<CChip>::new(self.p1_seq.clone());
        } else {
            fsm = FSM::<CChip>::new(self.p2_seq.clone());
        }

        // insert chip into the board

        let res = self.board.insert(&self.get_turn(), None, Some(pos_x))?;
        
        if self.board.detect(res.0, res.1, &mut fsm) {
            return Ok(Some(self.get_turn()));
        }

        if self.board.counter >= self.board.size.0 * self.board.size.1 {
            return Err("TIE".to_string());
        }
        return Ok(None);
        
    }

    pub fn get_board(self) -> Board<CChip> {
        return self.board;
    }
}
