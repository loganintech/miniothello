//! A container module for the random player


use crate::player::Player;
use crate::Othello;
use rand::{thread_rng, Rng};

/// A player that always returns a random valid move (or 0, 0)
pub struct RandomPlayer(pub char);

impl Player for RandomPlayer {
    fn get_symbol(&self) -> char {
        self.0
    }

    /// Generates a list of successors for itself and chooses a random one.
    fn get_move(&self, game: &Othello) -> (usize, usize) {
        let mut rng = thread_rng();
        let successors = game.successors(self.get_symbol());
        successors.get(rng.gen() % successors.len()).unwrap_or((0, 0))
    }
}
