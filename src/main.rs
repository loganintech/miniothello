mod player;
mod board;
mod human;

use std::env::args;
use player::*;
use board::*;
use human::*;

fn main() {
    let players = args().skip(1).take(2).collect::<Vec<_>>();
    if players.len() < 2 {
        eprintln!("Usage: miniothello <player type> <player type>");
        std::process::exit(1);
    }

    let player_one: &dyn Player = match players[0].as_ref() {
        "human" => {
            &HumanPlayer('X')
        },
        "minimax" => {
            &MinimaxPlayer('X')
        },
        _ => {
            eprintln!("Possible player types are `human` and `minimax`");
            std::process::exit(2);
        }
    };

    let player_two: &dyn Player = match players[1].as_ref() {
        "human" => {
            &HumanPlayer('O')
        },
        "minimax" => {
            &MinimaxPlayer('O')
        },
        _ => {
            eprintln!("Possible player types are `human` and `minimax`");
            std::process::exit(2);
        }
    };

    let game = GameBoard::with_players(player_one, player_two, 4, 4);

    while !game.board.is_full() {
        println!("{}", game);
        let p1_move = game.p_one.get_move(&game.board);

        let p2_move = game.p_two.get_move(&game.board);

    }
}

