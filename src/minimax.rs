use crate::game::Othello;
use crate::player::Player;

pub struct MinimaxPlayer(pub char);

impl MinimaxPlayer {
    fn minimax(&self, game: &mut Othello, maximize: bool) -> ((usize, usize), usize) {
        let player_char = self.get_symbol();
        let other_char = game.symbol_from_player(
            !game
                .player_from_symbol(self.get_symbol())
                .expect("Tried to match symbol for someone not in the game."),
        );

        if !game.has_more_moves() {
            let counts = game.board().char_counts();
            let our_count = counts.get(&player_char).unwrap_or(&0);
            let opponent_count = counts.get(&other_char).unwrap_or(&0);

            return if our_count >= opponent_count {
                ((0, 0), our_count - opponent_count) // Don't punish quick victories (7 - 0 is better than 25 - 23)
            } else {
                ((0, 0), 0)
            };
        }

        let symbol = if maximize { player_char } else { other_char };
        if !game.symbol_has_more_moves(symbol) {
            return self.minimax(game, !maximize);
        }

        let mut best_res = if maximize { 0 } else { std::usize::MAX };
        let mut best_coords = (game.board().rows(), game.board().cols());
        for row in 0..game.board().rows() {
            for col in 0..game.board().cols() {
                if game.is_legal_move(row, col, symbol) {
                    let mut new_game: Othello = game.clone();
                    new_game.play_move(row, col, symbol);
                    let result = self.minimax(&mut new_game, !maximize).1;
                    if (maximize && result >= best_res) || (!maximize && result <= best_res) {
                        best_res = result;
                        best_coords = (row, col);
                    }
                }
            }
        }
        if best_coords == (game.board().rows(), game.board().cols()) {
            eprintln!("Returning Bad Values: (best, best) == (board.rows, board.cols)");
        }
        (best_coords, best_res)
    }
}

impl Player for MinimaxPlayer {
    fn get_symbol(&self) -> char {
        self.0
    }

    fn get_move(&self, game: &Othello) -> (usize, usize) {
        self.minimax(&mut game.clone(), true).0
    }
}
