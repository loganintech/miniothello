use crate::board::Board;

pub trait Player {
    fn get_symbol(&self) -> char;

    fn get_move(&self, board: &Board) -> (usize, usize);
}

