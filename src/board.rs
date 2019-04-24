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

    pub fn is_full(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(Option::is_some))
    }

    pub fn empty_spaces(&self) -> usize {
        let mut max = self.rows() * self.cols();
        let counts = self.char_counts();
        for (_, count) in counts {
            max -= count;
        }

        max
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

    pub fn get_char_count(&self, symbol: &char) -> Option<usize> {
        let char_map = self.char_counts();
        if let Some(count) = char_map.get(symbol) {
            Some(*count)
        } else {
            None
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
            writeln!(build)?;
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
