use crate::player::Player;
use std::collections::HashMap;
use std::fmt::{self, Write};


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
    pub fn new_coords_from_direction(&self, row: usize, col: usize) -> (usize, usize) {
        match self {
            Direction::N => (row + 1, col),
            Direction::NE => (row + 1, col + 1),
            Direction::E => (row, col + 1),
            Direction::SE => (row - 1, col + 1),
            Direction::S => (row - 1, col),
            Direction::SW => (row - 1, col - 1),
            Direction::W => (row, col - 1),
            Direction::NW => (row + 1, col - 1),
        }
    }
}

impl Iterator for Direction {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        use Direction::*;
        Some(match self {
            N => NE,
            NE => E,
            E => SE,
            SE => S,
            S => SW,
            SW => W,
            W => NW,
            NW => N,
        })
    }
}

pub struct Board {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<Option<char>>>,
}

impl Board {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl Board {
    fn with_size(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: vec![vec![None; cols]; rows],
        }
    }

    fn get_cell(&self, row: usize, col: usize) -> Option<char> {
        self.grid[row][col]
    }

    fn set_cell(&mut self, row: usize, col: usize, chr: char) {
        self.grid[row][col] = Some(chr);
    }

    pub fn is_cell_empty(&self, row: usize, col: usize) -> bool {
        self.grid[row][col].is_none()
    }

    fn is_in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }

    pub fn is_full(&self) -> bool {
        self.grid
            .iter()
            .all(|row| row.iter().all(|chr| chr.is_some()))
    }

    pub fn char_counts(&self) -> HashMap<char, usize> {
        let mut map = HashMap::new();
        self.grid.iter().for_each(|row| {
            row.iter()
                .filter_map(|x| if let Some(x) = x { Some(x) } else { None })
                .for_each(|&chr| {
                    map.entry(chr).and_modify(|y| *y += 1).or_insert(1);
                })
        });

        map
    }

    // pub fn commit_move(&self, coord: (usize, usize), chr: char) -> bool {

    //     true
    // }


    pub fn check_endpoint(&mut self, row: usize, col: usize, chr: char, match_chr: bool, dir: Direction) -> bool {
        if !self.is_in_bounds(row, col) || self.is_cell_empty(row, col) {
            return false;
        }

        if match_chr {
            match self.get_cell(row, col) {
                Some(character) if character == chr => {
                    return true;
                },
                _ => {
                    let (new_row, new_col) = dir.new_coords_from_direction(row, col);
                    return self.check_endpoint(new_row, new_col, chr, match_chr, dir);
                },
            }
        } else {
            match self.get_cell(row, col) {
                Some(character) if character == chr => {
                    return false;
                },
                _ => {
                    let (new_row, new_col) = dir.new_coords_from_direction(row, col);
                    return self.check_endpoint(new_row, new_col, chr, !match_chr, dir);
                }
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut build = "".to_string();

        for row in (0..self.rows).rev() {
            write!(build, "{}:|", row)?;
            for col in 0..self.cols {
                write!(build, " {}", self.grid[row][col].unwrap_or('.'))?;
            }
            writeln!(build, "")?;
        }

        writeln!(
            build,
            "   {}",
            std::iter::repeat("-")
                .take((self.cols * 2) + 1)
                .collect::<String>()
        )?;
        writeln!(
            build,
            "    {}",
            (0..self.cols).fold(String::new(), |mut accum, val| {
                accum.push_str(&format!("{} ", val));
                accum
            })
        )?;
        write!(f, "{}", build)
    }
}

pub struct GameBoard<'a> {
    pub p_one: &'a Player,
    pub p_two: &'a Player,
    pub board: Board,
}

impl<'a> GameBoard<'a> {
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
        }
    }

    // pub fn flip_pieces(&mut self, row: usize, col: usize, chr: char) {
    //     for direction in Direction::N {
    //         let (new_row, new_col) = direction.new_coords_from_direction(row, col);

    //     }
    // }
}

impl<'a> fmt::Display for GameBoard<'a> {
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
