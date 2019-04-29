//! # Othello
//!
//! [How to play](https://www.youtube.com/watch?v=Ol3Id7xYsY4)
//!
//! The program won't let you play an invalid move. Bots (minimax or random) will move without user input.
//!
//! ```sh
//! Player 1 (X) score: 2
//! Player 2 (O) score: 2
//!
//! 3:| . . . .
//! 2:| . O X .
//! 1:| . X O .
//! 0:| . . . .
//!    ---------
//!     0 1 2 3
//!
//! Player 1 (X) move:
//! Enter row (or ?): 3
//! Enter col (or ?): -2
//! You must enter positive numerical numerical values.
//! Enter col (or ?): what
//! You must enter positive numerical numerical values.
//! Enter col (or ?): ?
//! Moves:
//! Row: 0, Col: 2
//! Row: 1, Col: 3
//! Row: 2, Col: 0
//! Row: 3, Col: 1
//! Enter col (or ?): 1
//! [Selected] Row: 3, Col: 1
//!
//! Player 1 (X) score: 4
//! Player 2 (O) score: 1
//!
//! 3:| . X . .
//! 2:| . X X .
//! 1:| . X O .
//! 0:| . . . .
//!    ---------
//!     0 1 2 3
//!
//! Player 2 (O) move:
//! [Selected] Row: 3, Col: 0
//!
//! Player 1 (X) score: 3
//! Player 2 (O) score: 3
//!
//! 3:| O X . .
//! 2:| . O X .
//! 1:| . X O .
//! 0:| . . . .
//!    ---------
//!     0 1 2 3
//!
//! Player 1 (X) move:
//! Enter row (or ?):
//! ```
//!
//! ## Compiling
//!
//! ### With Make
//!
//! ```sh
//! make
//! ./othello <player> <player> [SIZE] [SIZE] # I wouldn't suggest over 5x5. It will take your computer a long time (minutes or hours) to finish this game.
//! ```
//!
//! #### Compiling with Random Player Option
//!
//! Enables usage of the `random` player which picks the first valid move it randomly generates. Useful for testing.
//!
//! ```sh
//! make with_random
//! ./othello <player> <player> [SIZE] [SIZE] # I wouldn't suggest over 5x5. It will take your computer a long time (minutes or hours) to finish this game.
//! ```
//!
//! ### With Cargo
//!
//! ```sh
//! cargo build --release && mv target/release/othello .
//! ./othello <player> <player> [SIZE] [SIZE] # I wouldn't suggest over 5x5. It will take your computer a long time (minutes or hours) to finish this game.
//! ```
//!
//! #### Compiling with Random Player Option
//!
//! Enables usage of the `random` player which picks the first valid move it randomly generates. Useful for testing.
//!
//! ```sh
//! cargo build --release --features with_random && mv target/release/othello .
//! ./othello <player> <player> [SIZE] [SIZE] # I wouldn't suggest over 5x5. It will take your computer a long time (minutes or hours) to finish this game.
//! ```

#![allow(dead_code)]
#![deny(clippy::all)]

pub mod board;
pub mod player;

use crate::board::*;
use crate::player::Player;
use std::fmt;
use std::iter::Take;

/// # Direction
///
/// Represents cardinal directions on the board.
/// Combination directions represent diagonal movement.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    /// Given a row and column, new_coords_from_direction attempts to generate a valid next move.
    /// Returns `none` for underflowing values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use othlib::Direction;
    ///
    /// assert_eq!(None, Direction::SE.new_coords_from_direction(0, 1));
    /// assert_eq!(None, Direction::S.new_coords_from_direction(0, 1));
    /// assert_eq!(None, Direction::SW.new_coords_from_direction(0, 1));
    /// assert_eq!(None, Direction::SW.new_coords_from_direction(1, 0));
    /// assert_eq!(None, Direction::W.new_coords_from_direction(1, 0));
    /// assert_eq!(None, Direction::NW.new_coords_from_direction(1, 0));
    ///
    /// assert_eq!(Some((2, 1)), Direction::N.new_coords_from_direction(1, 1));
    /// assert_eq!(Some((2, 2)), Direction::NE.new_coords_from_direction(1, 1));
    /// assert_eq!(Some((1, 2)), Direction::E.new_coords_from_direction(1, 1));
    /// assert_eq!(Some((0, 2)), Direction::SE.new_coords_from_direction(1, 1));
    /// assert_eq!(Some((0, 1)), Direction::S.new_coords_from_direction(1, 1));
    /// assert_eq!(Some((0, 0)), Direction::SW.new_coords_from_direction(1, 1));
    /// assert_eq!(Some((1, 0)), Direction::W.new_coords_from_direction(1, 1));
    /// assert_eq!(Some((2, 0)), Direction::NW.new_coords_from_direction(1, 1));
    /// ````
    pub fn new_coords_from_direction(self, row: usize, col: usize) -> Option<(usize, usize)> {
        match self {
            Direction::N => Some((row + 1, col)),
            Direction::NE => Some((row + 1, col + 1)),
            Direction::E => Some((row, col + 1)),
            Direction::SE if row > 0 => Some((row - 1, col + 1)),
            Direction::S if row > 0 => Some((row - 1, col)),
            Direction::SW if row > 0 && col > 0 => Some((row - 1, col - 1)),
            Direction::W if col > 0 => Some((row, col - 1)),
            Direction::NW if col > 0 => Some((row + 1, col - 1)),
            _ => None,
        }
    }

    /// Loops through the cardinal directions once starting at north.
    ///
    /// # Examples
    ///
    /// ```
    /// # use othlib::Direction;
    ///
    /// let mut cardinals = Direction::cardinals();
    /// assert_eq!(Some(Direction::NE), cardinals.next());
    /// assert_eq!(Some(Direction::E), cardinals.next());
    /// assert_eq!(Some(Direction::SE), cardinals.next());
    /// assert_eq!(Some(Direction::S), cardinals.next());
    /// assert_eq!(Some(Direction::SW), cardinals.next());
    /// assert_eq!(Some(Direction::W), cardinals.next());
    /// assert_eq!(Some(Direction::NW), cardinals.next());
    /// assert_eq!(Some(Direction::N), cardinals.next());
    /// ```
    pub fn cardinals() -> Take<Self> {
        Direction::N.take(8)
    }

    /// Loops through the cardinal directions once starting at own index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use othlib::Direction;
    ///
    /// let mut cardinals = Direction::SE.cardinals_from_here();
    /// assert_eq!(Some(Direction::S), cardinals.next());
    /// assert_eq!(Some(Direction::SW), cardinals.next());
    /// assert_eq!(Some(Direction::W), cardinals.next());
    /// assert_eq!(Some(Direction::NW), cardinals.next());
    /// assert_eq!(Some(Direction::N), cardinals.next());
    /// assert_eq!(Some(Direction::NE), cardinals.next());
    /// assert_eq!(Some(Direction::E), cardinals.next());
    /// assert_eq!(Some(Direction::SE), cardinals.next());
    /// ```
    pub fn cardinals_from_here(self) -> Take<Self> {
        self.take(8)
    }
}

impl Iterator for Direction {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        use Direction::*;
        let next = match self {
            N => NE,
            NE => E,
            E => SE,
            SE => S,
            S => SW,
            SW => W,
            W => NW,
            NW => N,
        };
        *self = next;
        Some(*self)
    }
}

/// The active player is used as a marker in `Othello` to keep track of whose turn it is.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ActivePlayer {
    PlayerOne,
    PlayerTwo,
}

impl std::ops::Not for ActivePlayer {
    type Output = ActivePlayer;

    fn not(self) -> Self::Output {
        match self {
            ActivePlayer::PlayerOne => ActivePlayer::PlayerTwo,
            ActivePlayer::PlayerTwo => ActivePlayer::PlayerOne,
        }
    }
}

/// # Othello game implementation
///
/// Accepts two different players and the starting player. Automatically initializes the board.
/// The two players must implement the `Player` trait.
#[derive(Clone)]
pub struct Othello<'a> {
    p_one: &'a dyn Player,
    p_two: &'a dyn Player,
    active_player: ActivePlayer,
    board: Board,
}

impl<'a> Othello<'a> {
    /// # Creates an Othello game
    ///
    ///
    /// ## Accepts two players and dimensions as input.
    /// ```
    /// # use othlib::player::human::HumanPlayer;
    /// # use othlib::Othello;
    /// let game = Othello::with_players(&HumanPlayer('X'), &HumanPlayer('O'), 4, 4);
    /// assert_eq!(Some('X'), game.board().get_cell(2, 2));
    /// assert_eq!(Some('X'), game.board().get_cell(1, 1));
    /// assert_eq!(Some('O'), game.board().get_cell(2, 1));
    /// assert_eq!(Some('O'), game.board().get_cell(1, 2));
    /// ```
    pub fn with_players(p_one: &'a Player, p_two: &'a Player, rows: usize, cols: usize) -> Self {
        let mut board = Board::with_size(rows, cols);
        board.set_cell(rows / 2 - 1, cols / 2 - 1, p_one.get_symbol());
        board.set_cell(rows / 2, cols / 2, p_one.get_symbol());
        board.set_cell(rows / 2, cols / 2 - 1, p_two.get_symbol());
        board.set_cell(rows / 2 - 1, cols / 2, p_two.get_symbol());
        Self {
            p_one,
            p_two,
            board,
            active_player: ActivePlayer::PlayerOne,
        }
    }

    pub fn active_as_num(&self) -> usize {
        match self.active_player() {
            ActivePlayer::PlayerOne => 1,
            ActivePlayer::PlayerTwo => 2,
        }
    }

    pub fn active_player(&self) -> ActivePlayer {
        self.active_player
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    fn check_endpoint(
        &self,
        row: usize,
        col: usize,
        symbol: char,
        direction: Direction,
        match_symbol: bool,
    ) -> bool {
        if !self.board.is_in_bounds(row, col) || self.board.is_cell_empty(row, col) {
            return false;
        }

        match self.board.get_cell(row, col) {
            Some(symbol) if symbol == self.get_active_symbol() => match_symbol,
            _ => {
                if let Some((new_row, new_col)) = direction.new_coords_from_direction(row, col) {
                    self.check_endpoint(new_row, new_col, symbol, direction, true)
                } else {
                    false
                }
            }
        }
    }

    pub fn change_active_player(&mut self) {
        self.active_player = !self.active_player;
    }

    pub fn flip_pieces(&mut self, row: usize, col: usize, symbol: char) -> usize {
        let mut flipped = 0;
        //Loops through the cardinal directions once
        for direction in Direction::cardinals() {
            if let Some((new_row, new_col)) = direction.new_coords_from_direction(row, col) {
                if self.check_endpoint(new_row, new_col, symbol, direction, false) {
                    flipped += self.flip_helper(new_row, new_col, symbol, direction);
                }
            }
        }

        flipped
    }

    fn flip_helper(&mut self, row: usize, col: usize, symbol: char, direction: Direction) -> usize {
        match self.board.get_cell(row, col) {
            Some(symbol) if symbol == self.get_active_symbol() => 0,
            _ => {
                self.board.set_cell(row, col, symbol);
                if let Some((new_row, new_col)) = direction.new_coords_from_direction(row, col) {
                    1 + self.flip_helper(new_row, new_col, symbol, direction)
                } else {
                    0
                }
            }
        }
    }

    pub fn get_active_symbol(&self) -> char {
        self.symbol_from_player(self.active_player)
    }

    pub fn get_move(&mut self) -> (usize, usize) {
        match self.active_player {
            ActivePlayer::PlayerOne => self.p_one.get_move(&self),
            ActivePlayer::PlayerTwo => self.p_two.get_move(&self),
        }
    }

    pub fn get_winner(&self) -> Option<String> {
        if self.has_more_moves() {
            return None;
        }

        let char_map = self.board.char_counts();
        let p_one_count = char_map.get(&self.p_one.get_symbol()).unwrap_or(&0);
        let p_two_count = char_map.get(&self.p_two.get_symbol()).unwrap_or(&0);
        if p_one_count > p_two_count {
            Some(format!("Player 1 wins with {} points!", p_one_count))
        } else if p_two_count > p_one_count {
            Some(format!("Player 2 wins with {} points!", p_two_count))
        } else {
            Some("It's a tie!".to_string())
        }
    }

    pub fn get_winner_number(&self) -> usize {
        let char_map = self.board.char_counts();
        let p_one_count = char_map.get(&self.p_one.get_symbol()).unwrap_or(&0);
        let p_two_count = char_map.get(&self.p_two.get_symbol()).unwrap_or(&0);
        if p_one_count > p_two_count {
            1
        } else if p_two_count > p_one_count {
            2
        } else {
            0
        }
    }

    pub fn has_more_moves(&self) -> bool {
        self.player_has_more_moves(ActivePlayer::PlayerOne)
            && self.player_has_more_moves(ActivePlayer::PlayerTwo)
    }

    pub fn is_legal_move(&self, row: usize, col: usize, symbol: char) -> bool {
        if !self.board.is_in_bounds(row, col) || !self.board.is_cell_empty(row, col) {
            return false;
        }

        //Loops through the cardinal directions once
        Direction::cardinals().any(|direction| {
            if let Some((new_row, new_col)) = direction.new_coords_from_direction(row, col) {
                self.check_endpoint(new_row, new_col, symbol, direction, false)
            } else {
                false
            }
        })
    }

    pub fn next_turn(&mut self) -> bool {
        let symbol = self.get_active_symbol();

        println!("{}", self);
        println!("Player {} ({}) move:", self.active_as_num(), symbol);

        let mut found_valid_move = false;
        while !found_valid_move
            && self.player_has_more_moves(self.active_player())
            && self.has_more_moves()
        {
            let (row, col) = self.get_move();
            if !self.is_legal_move(row, col, symbol) {
                println!("Invalid move.");
                continue;
            }
            println!("[Selected] Row: {}, Col: {}", row, col);
            self.play_move(row, col, symbol);
            found_valid_move = true;
        }

        if !found_valid_move {
            println!(
                "Couldn't find valid move for Player {} ({})",
                self.active_as_num(),
                symbol
            )
        }
        found_valid_move
    }

    pub fn player_from_symbol(&self, symbol: char) -> Option<ActivePlayer> {
        let player_one_symbol = self.p_one.get_symbol();
        let player_two_symbol = self.p_two.get_symbol();
        match symbol {
            symbol if symbol == player_one_symbol => Some(ActivePlayer::PlayerOne),
            symbol if symbol == player_two_symbol => Some(ActivePlayer::PlayerTwo),
            _ => None,
        }
    }

    pub fn player_has_more_moves(&self, player: ActivePlayer) -> bool {
        self.symbol_has_more_moves(self.symbol_from_player(player))
    }

    pub fn play_move(&mut self, row: usize, col: usize, symbol: char) {
        self.board.set_cell(row, col, symbol);
        self.flip_pieces(row, col, symbol);
        self.change_active_player();
    }

    pub fn run(&mut self) -> usize {
        let mut iter_count = self.active_as_num() - 1;
        let mut played_successfully = [true, true];
        while self.has_more_moves() {
            played_successfully[iter_count] = self.next_turn();
            if !played_successfully[0] && !played_successfully[1] {
                break;
            }

            // If we're on the second iteration and they aren't both true, reset them
            if iter_count == 1 {
                played_successfully[0] = true;
                played_successfully[1] = true;
            }
            iter_count = (iter_count + 1) % 2;
        }

        self.get_winner_number()
    }

    pub fn successors(&self, symbol: char) -> Vec<(usize, usize)> {
        let mut successors = vec![];
        for row in 0..self.board().rows() {
            for col in 0..self.board().cols() {
                if self.is_legal_move(row, col, symbol) {
                    successors.push((row, col));
                }
            }
        }
        successors
    }
    pub fn symbol_from_player(&self, player: ActivePlayer) -> char {
        match player {
            ActivePlayer::PlayerOne => self.p_one.get_symbol(),
            ActivePlayer::PlayerTwo => self.p_two.get_symbol(),
        }
    }

    pub fn symbol_has_more_moves(&self, symbol: char) -> bool {
        (0..self.board.rows())
            .any(|row| (0..self.board.cols()).any(|col| self.is_legal_move(row, col, symbol)))
    }
}

impl<'a> fmt::Display for Othello<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let counts = self.board.char_counts();
        let build = format!(
            "\nPlayer 1 ({}) score: {}\nPlayer 2 ({}) score: {}",
            self.p_one.get_symbol(),
            counts.get(&self.p_one.get_symbol()).unwrap_or(&0),
            self.p_two.get_symbol(),
            counts.get(&self.p_two.get_symbol()).unwrap_or(&0)
        );
        write!(f, "{}\n\n{}", build, self.board)
    }
}

impl<'a> fmt::Debug for Othello<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
        "\n=================\nDEBUG\n=================\nPlayer {}'s Turn\n{}\n=================\nEND DEBUG\n=================",
        self.active_as_num(),
        self)
    }
}

#[cfg(test)]
mod test {
    use super::player::*;
    use super::Othello;

    #[cfg(feature = "with_random")]
    #[test]
    #[ignore]
    fn try_random() {
        let iterations = 1000;
        for i in 0..iterations {
            println!("Running Game: {} {} left", i, iterations - i);
            let mut game = Othello::with_players(
                &random::RandomPlayer('X'),
                &minimax::MinimaxPlayer('O'),
                4,
                4,
            );
            game.run();
            assert_ne!(1, game.get_winner_number());
        }
    }

    #[test]
    fn try_minimax() {
        let mut game = Othello::with_players(
            &minimax::MinimaxPlayer('X'),
            &minimax::MinimaxPlayer('O'),
            4,
            4,
        );
        game.run();
        assert_ne!(1, game.get_winner_number());
    }
}
