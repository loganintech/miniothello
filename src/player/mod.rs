use crate::Othello;

pub mod human;
pub mod minimax;
#[cfg(feature = "with_random")]
pub mod random;
#[cfg(test)]
pub mod specific;

pub trait Player {
    fn get_symbol(&self) -> char;

    fn get_move(&self, board: &Othello) -> (usize, usize);
}
