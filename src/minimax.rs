use crate::game::Othello;
use crate::player::Player;

pub struct MinimaxPlayer(pub char);

impl MinimaxPlayer {
    fn move_helper(&self, game: &mut Othello, maximize: bool) -> Vec<((usize, usize), usize)> {
        let mut tentative_score = if maximize { 0 } else { std::usize::MAX };
        let mut possible = vec![];

        for row in 0..game.board().rows() {
            for col in 0..game.board().cols() {
                if game.board().is_cell_empty(row, col)
                    && game.is_legal_move(row, col, self.get_symbol())
                {
                    let mut possible_game = game.clone();
                    possible_game.play_move(row, col, self.get_symbol());
                    let score = possible_game.board().get_char_count(&self.get_symbol());
                    match score {
                        Some(score) if maximize && score > tentative_score => {
                            tentative_score = score;
                            possible.push(((row, col), tentative_score));
                            self.move_helper(&mut game.clone(), !maximize);
                        }
                        Some(score) if !maximize && score < tentative_score => {
                            tentative_score = score;
                            possible.push(((row, col), tentative_score));
                            self.move_helper(&mut game.clone(), !maximize);
                        }
                        _ => {}
                    }
                }
            }
        }

        possible
    }
}

impl Player for MinimaxPlayer {
    fn get_symbol(&self) -> char {
        self.0
    }

    fn get_move(&self, game: &Othello) -> (usize, usize) {
        let possible_results = self.move_helper(&mut game.clone(), true);
        let best = possible_results.iter().max_by_key(|x| x.1);

        best.unwrap_or(&((0, 0), 0)).0
    }
}
