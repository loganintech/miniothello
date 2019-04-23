use crate::game::Othello;
use crate::player::Player;

pub struct MinimaxPlayer(pub char);

impl Player for MinimaxPlayer {
    fn get_symbol(&self) -> char {
        self.0
    }

    fn get_move(&self, game: &Othello) -> (usize, usize) {
        let mut max = 0;
        let mut res = (0, 0);

        for row in 0..game.board().rows() {
            for col in 0..game.board().cols() {
                if game.board().is_cell_empty(row, col) && game.is_legal_move(row, col, self.get_symbol()) {
                    let mut possible_game = game.clone();
                    possible_game.play_move(row, col, self.get_symbol());
                    let score = possible_game.board().get_char_count(&self.get_symbol());
                    match score {
                        Some(score) if score > max => {
                            max = score;
                            res = (row, col);
                        },
                        _ => {}
                    }
                }
            }
        }

        res
    }
}
