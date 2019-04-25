//! A container module for the minimax player


use crate::player::Player;
use crate::Othello;

/// A player that makes moves based on a simple minimax algorithm (w/o alpha-beta pruning) by recursivly playing the game.
///
/// On a 4x4 space, this player cannot lose if playing as player 2.
///
/// _Note_: This algorithm has a very large time complexity. 6x6 can generate one move on a scale of days or weeks depending on your computer.
/// However, a 4x4 grid can be done in a matter of seconds or subseconds.
pub struct MinimaxPlayer(pub char);

impl MinimaxPlayer {
    /// # An implementation of the minimax recursive algorithm for finding scores.
    ///
    /// Read more about it on wikipedia: https://en.wikipedia.org/wiki/Computer_Othello#Search_techniques
    fn minimax(&self, game: &mut Othello, maximize: bool) -> ((usize, usize), isize) {
        let player_symbol = self.get_symbol();
        let opponent_symbol = game.symbol_from_player(
            //The ! changes an ActivePlayer::PlayerOne into ActivePlayer::PlayerTwo and vice-versa
            !game
                .player_from_symbol(self.get_symbol())
                .expect("Tried to match symbol for someone not in the game."),
        );
        //If we're maximizing, we increase from isize::MIN, otherwise we decrease from isize::MAX
        let (mut best_res, turn_symbol) = if maximize {
            (std::isize::MIN, player_symbol)
        } else {
            (std::isize::MAX, opponent_symbol)
        };

        //If there is no more game to play
        if !game.has_more_moves() {
            let counts = game.board().char_counts();
            let our_count = *counts.get(&player_symbol).unwrap_or(&0) as isize;
            let opponent_count = *counts.get(&opponent_symbol).unwrap_or(&0) as isize;
            return ((0, 0), our_count - opponent_count);
        }

        //If we're at this point the game isn't over but we can't move so let's let our opponent move.
        if !game.symbol_has_more_moves(turn_symbol) {
            return self.minimax(game, !maximize);
        }

        let mut best_coords = (game.board().rows(), game.board().cols());

        for (row, col) in game.successors(turn_symbol) {
            let mut new_game: Othello = game.clone();
            new_game.play_move(row, col, turn_symbol);
            let result = self.minimax(&mut new_game, !maximize).1;
            if (maximize && result > best_res) || (!maximize && result < best_res) {
                best_res = result;
                best_coords = (row, col);
            }
        }

        (best_coords, best_res)
    }
}

impl Player for MinimaxPlayer {
    fn get_symbol(&self) -> char {
        self.0
    }

    fn get_move(&self, game: &Othello) -> (usize, usize) {
        let res = self.minimax(&mut game.clone(), true);
        res.0
    }
}
