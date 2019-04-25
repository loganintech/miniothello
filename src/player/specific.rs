use crate::player::Player;
use crate::Othello;
use std::cell::RefCell;
use std::collections::VecDeque;

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

    fn get_move(&self, _: &Othello) -> (usize, usize) {
        self.moves
            .borrow_mut()
            .pop_front()
            .expect("The SpecificPlayer configuration is invalid.")
    }
}
