use std::io::prelude::*;
use crate::player::Player;
use crate::board::Board;

pub struct HumanPlayer(pub char);

impl Player for HumanPlayer {
    fn get_symbol(&self) -> char {
        self.0
    }

    fn get_move(&self, board: &Board) -> (usize, usize) {
        // let mut stdin = stdin.lock();
        // let stdin = std::io::stdin();
        // let mut stdout = stdout.lock();
        // let mut row = "".to_string();
        // let mut col = "".to_string();

        loop {
            let mut row = "".to_string();
            let mut col = "".to_string();
            print!("Enter row: ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut row).unwrap();
            print!("Enter col: ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut col).unwrap();

            let rown = match row.trim().parse::<usize>() {
                Ok(rown) if rown < board.rows() => {
                    rown
                },
                Ok(_) => {
                    eprintln!("Your entry is out of range.");
                    continue;
                },
                _ => {
                    eprintln!("You must enter positive numerical numerical values.");
                    continue;
                }
            };

            let coln = match col.trim().parse::<usize>() {
                Ok(coln) if coln < board.cols() => {
                    coln
                },
                Ok(_) => {
                    eprintln!("Your entry is out of range.");
                    continue;
                },
                _ => {
                    eprintln!("You must enter positive numerical numerical values.");
                    continue;
                }
            };

            if board.is_cell_empty(rown, coln) {
                return (rown, coln);
            } else {
                eprintln!("That cell is occupied");
            }
        }
    }
}
