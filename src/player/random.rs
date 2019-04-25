use crate::Othello;
use crate::player::Player;
use rand::{thread_rng, Rng};

pub struct RandomPlayer(pub char);

impl Player for RandomPlayer {
    fn get_symbol(&self) -> char {
        self.0
    }

    fn get_move(&self, game: &Othello) -> (usize, usize) {
        let mut rng = thread_rng();
        loop {
            let row = rng.gen::<usize>() % game.board().rows();
            let col = rng.gen::<usize>() % game.board().cols();

            if !game.is_legal_move(row, col, self.get_symbol()) {
                continue;
            }

            return (row, col);
        }
    }
}
