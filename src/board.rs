use crate::player::Player;

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

pub struct Board {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<Option<char>>>,
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

    fn is_cell_empty(&self, row: usize, col: usize) -> bool {
        self.grid[row][col].is_none()
    }

    fn is_in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }
}

pub struct GameBoard {
    p_one: Box<dyn Player>,
    p_two: Box<dyn Player>,
    board: Board,
}

impl GameBoard {
    fn with_players(p_one: impl Player + 'static, p_two: impl Player + 'static, rows: usize, cols: usize) -> Self {
        Self {
            p_one: Box::new(p_one),
            p_two: Box::new(p_two),
            board: Board::with_size(rows, cols),
        }
    }

}
