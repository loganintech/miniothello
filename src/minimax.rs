use crate::board::Board;
use crate::player::Player;

pub struct MinimaxPlayer(pub char);

impl Player for MinimaxPlayer {
    fn get_symbol(&self) -> char {
        self.0
    }

    fn get_move(&self, board: &Board) -> (usize, usize) {
        unimplemented!()
    }
}
