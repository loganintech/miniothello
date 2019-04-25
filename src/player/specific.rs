//! A container module for the specific player


use crate::player::Player;
use crate::Othello;
use std::cell::RefCell;
use std::collections::VecDeque;

/// A player used for testing that picks specific moves in a specific order.
/// This player is not useful for anything but testing and isn't compiled if we aren't in test mode.
pub struct SpecificPlayer {
    symbol: char,
    moves: RefCell<VecDeque<(usize, usize)>>,
}

impl SpecificPlayer {
    pub fn new(symbol: char, moves: &[(usize, usize)]) -> Self {
        Self {
            symbol,
            moves: RefCell::new(moves.iter().cloned().collect::<VecDeque<_>>()),
        }
    }
}

impl Player for SpecificPlayer {
    fn get_symbol(&self) -> char {
        self.symbol
    }

    /// Returns the front of the move vector.
    fn get_move(&self, _: &Othello) -> (usize, usize) {
        self.moves
            .borrow_mut()
            .pop_front()
            .expect("The SpecificPlayer configuration is invalid.")
    }
}
