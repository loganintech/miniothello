//! A module for the player Trait and player implementations

use crate::Othello;

pub mod human;
pub mod minimax;
#[cfg(feature = "with_random")]
pub mod random;
#[cfg(test)]
pub mod specific;

/// # Models required functions for a player.
///
/// It's important that the get_move function returns valid moves.
/// A player that only returns invalid moves will cause an infinite loop in the game.
pub trait Player {
    /// Returns the player symbol.
    fn get_symbol(&self) -> char;

    /// Returns the game move chosen by the player.
    fn get_move(&self, board: &Othello) -> (usize, usize);
}
