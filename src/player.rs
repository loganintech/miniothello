use crate::game::Othello;

pub trait Player {
    fn get_symbol(&self) -> char;

    fn get_move(&self, board: &Othello) -> (usize, usize);
}
