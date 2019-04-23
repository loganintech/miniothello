#![allow(dead_code)]

mod board;
mod game;
mod human;

mod minimax;
mod player;
use game::Othello;
use human::*;
use minimax::*;
use player::*;

use std::env::args;

fn main() {
    let players = args().skip(1).take(2).collect::<Vec<_>>();
    if players.len() < 2 {
        eprintln!("Usage: miniothello <player type> <player type>");
        std::process::exit(1);
    }

    let player_one: &dyn Player = match players[0].as_ref() {
        "human" => &HumanPlayer('X'),
        "minimax" => &MinimaxPlayer('X'),
        _ => {
            eprintln!("Possible player types are `human` and `minimax`");
            std::process::exit(2);
        }
    };

    let player_two: &dyn Player = match players[1].as_ref() {
        "human" => &HumanPlayer('O'),
        "minimax" => &MinimaxPlayer('O'),
        _ => {
            eprintln!("Possible player types are `human` and `minimax`");
            std::process::exit(2);
        }
    };

    let mut game = Othello::with_players(player_one, player_two, 4, 4);
    while game.has_more_moves() {
        let p1_success = game.next_turn();
        let p2_success = game.next_turn();

        if !p1_success && !p2_success {
            eprintln!("Both players failed to play");
            break;
        }
    }

    println!(
        "\n================================\n{}\n================================\n\nBoard: \n{}",
        game.get_winner().unwrap_or_else(|| {
            "The game ended with more moves left. I'm not sure how this happened.".to_string()
        }),
        game
    );
}
