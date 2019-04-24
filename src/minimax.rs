use crate::game::Othello;
use crate::player::Player;

pub struct MinimaxPlayer(pub char);

impl MinimaxPlayer {
    fn move_helper(&self, game: &mut Othello, maximize: bool) -> ((usize, usize), usize) {
        let symbol = if maximize {
            self.get_symbol()
        } else {
            game.symbol_from_player(
                !game
                    .player_from_symbol(self.get_symbol())
                    .expect("Tried to match symbol for someone not in the game."),
            )
        };

        if !game.has_more_moves() {
            return ((0, 0), 0);
        }

        if game.has_more_moves() && !game.symbol_has_more_moves(symbol) {
            game.change_active_player();
            return self.move_helper(game, !maximize);
        }

        let mut possibilities: Vec<((usize, usize), usize, Othello)> = vec![];

        for row in 0..game.board().rows() {
            for col in 0..game.board().cols() {
                if game.is_legal_move(row, col, symbol) && game.board().is_cell_empty(row, col) {
                    let mut new_board: Othello = game.clone();
                    new_board.play_move(row, col, symbol);
                    new_board.change_active_player();
                    let score = new_board
                        .board()
                        .get_char_count(&self.get_symbol())
                        .expect("Tried to get the score of a player that isn't playing.");
                    possibilities.push(((row, col), score, new_board));
                }
            }
        }

        if maximize {
            let mut max = possibilities.into_iter().max_by_key(|x| x.1).unwrap();
            let res = self.move_helper(&mut max.2, false);
            (max.0, max.1 + res.1)
        } else {
            let mut min = possibilities.into_iter().min_by_key(|x| x.1).unwrap();
            let res = self.move_helper(&mut min.2, true);
            min.1 += res.1;
            (min.0, min.1 + res.1)
        }
    }
}

impl Player for MinimaxPlayer {
    fn get_symbol(&self) -> char {
        self.0
    }

    fn get_move(&self, game: &Othello) -> (usize, usize) {
        self.move_helper(&mut game.clone(), true).0
    }
}
