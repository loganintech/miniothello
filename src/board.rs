//! Contains the game board. This does _not_ include logic about how a game is played.
//!
//! The board has no notions of players or even specific restrictions on count.
//! It only keeps an NxM grid of `Option<char>`s. This entire API is safe.

use std::collections::HashMap;
use std::fmt::{self, Write};

#[derive(Clone, Debug)]
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

    pub fn with_state(rows: usize, cols: usize, grid: Vec<Vec<Option<char>>>) -> Self {
        Self { rows, cols, grid }
    }
}

impl Board {
    pub fn with_size(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            grid: vec![vec![None; cols]; rows],
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<char> {
        self.grid[row][col]
    }

    pub fn set_cell(&mut self, row: usize, col: usize, symbol: char) {
        self.grid[row][col] = Some(symbol);
    }

    pub fn is_cell_empty(&self, row: usize, col: usize) -> bool {
        self.grid[row][col].is_none()
    }

    pub fn is_in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }

    pub fn char_counts(&self) -> HashMap<char, usize> {
        let mut map = HashMap::new();
        self.grid.iter().for_each(|row| {
            row.iter().filter_map(|&x| x).for_each(|symbol| {
                map.entry(symbol).and_modify(|y| *y += 1).or_insert(1);
            })
        });

        map
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.rows(), self.cols())
    }
}
