use crate::board::*;
use crate::player::Player;
use std::fmt;

#[derive(Copy, Clone, Debug)]
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

#[derive(Debug, Copy, Clone)]
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

pub struct Othello<'a> {
    p_one: &'a Player,
    p_two: &'a Player,
    active_player: ActivePlayer,
    board: Board,
}

impl<'a> Othello<'a> {
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

    pub fn active_player(&self) -> &ActivePlayer {
        &self.active_player
    }

    pub fn has_more_moves(&self) -> bool {
        self.player_has_more_moves(ActivePlayer::PlayerOne)
            && self.player_has_more_moves(ActivePlayer::PlayerTwo)
    }

    pub fn get_active_symbol(&self) -> char {
        self.symbol_from_player(self.active_player)
    }

    fn symbol_from_player(&self, player: ActivePlayer) -> char {
        match player {
            ActivePlayer::PlayerOne => self.p_one.get_symbol(),
            ActivePlayer::PlayerTwo => self.p_two.get_symbol(),
        }
    }

    pub fn player_has_more_moves(&self, player: ActivePlayer) -> bool {
        for row in 0..self.board.rows() {
            for col in 0..self.board.cols() {
                if self.board.is_cell_empty(row, col)
                    && self.is_legal_move(row, col, self.symbol_from_player(player))
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn is_legal_move(&self, row: usize, col: usize, symbol: char) -> bool {
        if !self.board.is_in_bounds(row, col) || !self.board.is_cell_empty(row, col) {
            return false;
        }

        for direction in Direction::N.take(8) {
            if let Some((new_row, new_col)) = direction.new_coords_from_direction(row, col) {
                if self.check_endpoint(new_row, new_col, symbol, direction, false) {
                    return true;
                }
            }
        }

        false
    }

    pub fn check_endpoint(
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

        if match_symbol {
            match self.board.get_cell(row, col) {
                Some(symbol) if symbol == self.get_active_symbol() => true,
                _ => {
                    if let Some((new_row, new_col)) = direction.new_coords_from_direction(row, col)
                    {
                        self.check_endpoint(new_row, new_col, symbol, direction, match_symbol)
                    } else {
                        false
                    }
                }
            }
        } else {
            match self.board.get_cell(row, col) {
                Some(symbol) if symbol == self.get_active_symbol() => false,
                _ => {
                    if let Some((new_row, new_col)) = direction.new_coords_from_direction(row, col)
                    {
                        self.check_endpoint(new_row, new_col, symbol, direction, !match_symbol)
                    } else {
                        false
                    }
                }
            }
        }
    }

    pub fn get_move(&mut self) -> (usize, usize) {
        match self.active_player {
            ActivePlayer::PlayerOne => self.p_one.get_move(&self.board),
            ActivePlayer::PlayerTwo => self.p_two.get_move(&self.board),
        }
    }

    pub fn next_turn(&mut self) -> bool {
        println!("{}", self);
        println!(
            "Player {} ({}) move:",
            match self.active_player() {
                ActivePlayer::PlayerOne => 1,
                ActivePlayer::PlayerTwo => 2,
            },
            self.get_active_symbol()
        );

        let mut found_valid_move = false;
        while !found_valid_move && self.player_has_more_moves(*self.active_player()) {
            let (row, col) = self.get_move();
            if !self.is_legal_move(row, col, self.get_active_symbol()) {
                println!("Invalid move.");
            } else {
                println!("[Selected] Row: {}, Col: {}", row, col);
                self.play_move(row, col, self.get_active_symbol());
                found_valid_move = true;
            }
        }

        if !found_valid_move {
            println!(
                "Couldn't find valid move for Player {} ({})",
                match self.active_player() {
                    ActivePlayer::PlayerOne => 1,
                    ActivePlayer::PlayerTwo => 2,
                },
                self.get_active_symbol()
            )
        }
        self.active_player = !self.active_player;
        found_valid_move
    }

    pub fn play_move(&mut self, row: usize, col: usize, symbol: char) {
        self.board.set_cell(row, col, symbol);
        self.flip_pieces(row, col, symbol);
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

    pub fn flip_pieces(&mut self, row: usize, col: usize, symbol: char) -> usize {
        let mut flipped = 0;
        for direction in Direction::N.take(8) {
            if let Some((new_row, new_col)) = direction.new_coords_from_direction(row, col) {
                if self.check_endpoint(new_row, new_col, symbol, direction, false) {
                    flipped += self.flip_helper(new_row, new_col, symbol, direction);
                }
            }
        }

        flipped
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
            Some(format!("It's a tie!"))
        }
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
