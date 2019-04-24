use crate::game::Othello;
use crate::player::Player;

pub struct MinimaxPlayer(pub char);

impl MinimaxPlayer {
    fn minimax(&self, game: &mut Othello, maximize: bool) -> ((usize, usize), usize) {
        // let game = dbg!(game);

        let player_char = self.get_symbol();
        let other_char = game.symbol_from_player(
            !game
                .player_from_symbol(self.get_symbol())
                .expect("Tried to match symbol for someone not in the game."),
        );

        let symbol = if maximize { player_char } else { other_char };

        if !game.has_more_moves() {
            let counts = game.board().char_counts();
            let our_count = counts.get(&player_char).unwrap_or(&0);
            let opponent_count = counts.get(&other_char).unwrap_or(&0);

            return dbg!(if our_count > opponent_count {
                ((0, 0), *our_count)
            } else {
                ((0, 0), 0)
            });
        }

        if game.has_more_moves() && !game.symbol_has_more_moves(symbol) {
            return dbg!(self.minimax(game, !maximize));
        }

        if maximize {
            let mut best_res = 0;
            let mut best_coords = (0, 0);
            for row in 0..game.board().rows() {
                for col in 0..game.board().cols() {
                    if game.is_legal_move(row, col, symbol) && game.board().is_cell_empty(row, col)
                    {
                        let mut new_game: Othello = game.clone();
                        new_game.play_move(row, col, symbol);
                        let result = dbg!(self.minimax(&mut new_game, false)).1;
                        if result > best_res {
                            best_res = result;
                            best_coords = (row, col);
                        }
                    }
                }
            }

            (best_coords, best_res)
        } else {
            let mut best_res = std::usize::MAX;
            let mut best_coords = (0, 0);
            for row in 0..game.board().rows() {
                for col in 0..game.board().cols() {
                    if game.is_legal_move(row, col, symbol) && game.board().is_cell_empty(row, col)
                    {
                        let mut new_game: Othello = game.clone();
                        new_game.play_move(row, col, symbol);
                        let result = dbg!(self.minimax(&mut new_game, true)).1;
                        if result < best_res {
                            best_res = result;
                            best_coords = (row, col);
                        }
                    }
                }
            }

            (best_coords, best_res)
        }
    }
}

impl Player for MinimaxPlayer {
    fn get_symbol(&self) -> char {
        self.0
    }

    fn get_move(&self, game: &Othello) -> (usize, usize) {
        dbg!(self.minimax(&mut game.clone(), true)).0
    }
}
