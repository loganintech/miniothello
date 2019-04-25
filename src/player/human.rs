use crate::player::Player;
use crate::Othello;
use std::io::prelude::*;

pub struct HumanPlayer(pub char);

impl HumanPlayer {
    fn move_helper(
        &self,
        prompt: &'static str,
        upper_bound: usize,
        game: &Othello,
    ) -> Option<usize> {
        let mut val = "".to_string();

        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut val).unwrap();
        let val = val.trim();

        if val == "?" {
            println!("Moves:");
            for (row, col) in game.successors(self.get_symbol()) {
                println!("Row: {}, Col: {}", row, col);
            }
            return None;
        }

        match val.parse::<usize>() {
            Ok(val) if val < upper_bound => Some(val),
            Ok(_) => {
                eprintln!("Your entry is out of range.");
                None
            }
            _ => {
                eprintln!("You must enter positive numerical numerical values.");
                None
            }
        }
    }
}

const LOOP_ERR: &str = "Tried to unwrap after a loop that prevents None values.";

impl Player for HumanPlayer {
    fn get_symbol(&self) -> char {
        self.0
    }

    fn get_move(&self, game: &Othello) -> (usize, usize) {
        let board = game.board();
        let mut row = None;
        let mut col = None;

        loop {
            if row.is_none() {
                row = self.move_helper("Enter row (or ?): ", board.rows(), game);
                if row.is_none() {
                    continue;
                }
            }
            if col.is_none() {
                col = self.move_helper("Enter col (or ?): ", board.cols(), game);
                if col.is_none() {
                    continue;
                }
            }
            // These expects are safe due to the conditions above. Changing them can result in unexpected behavior
            if board.is_cell_empty(row.expect(LOOP_ERR), col.expect(LOOP_ERR)) {
                return (row.expect(LOOP_ERR), col.expect(LOOP_ERR));
            } else {
                eprintln!("That cell is occupied");
                row = None;
                col = None;
            }
        }
    }
}
