#![warn(clippy::all)]

use othlib::player::{human::*, minimax::*, Player};

#[cfg(feature = "with_random")]
use othlib::player::random::*;

use std::env::args;

fn main() {
    let args = args().skip(1).take(4).collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: miniothello <player type> <player type>");
        std::process::exit(1);
    }

    let player_one: &dyn Player = match args[0].as_ref() {
        "human" => &HumanPlayer('X'),
        #[cfg(feature = "with_random")]
        "random" => &RandomPlayer('X'),
        "minimax" => &MinimaxPlayer('X'),
        _ => {
            #[cfg(feature = "with_random")]
            eprintln!("Possible player types are `human`, `random`, and `minimax`");
            #[cfg(not(feature = "with_random"))]
            eprintln!("Possible player types are `human` and `minimax`");
            std::process::exit(2);
        }
    };

    let player_two: &dyn Player = match args[1].as_ref() {
        "human" => &HumanPlayer('O'),
        #[cfg(feature = "with_random")]
        "random" => &RandomPlayer('O'),
        "minimax" => &MinimaxPlayer('O'),
        _ => {
            #[cfg(feature = "with_random")]
            eprintln!("Possible player types are `human`, `random`, and `minimax`");
            #[cfg(not(feature = "with_random"))]
            eprintln!("Possible player types are `human` and `minimax`");
            std::process::exit(2);
        }
    };

    let mut game = othlib::Othello::with_players(
        player_one,
        player_two,
        args.get(2)
            .and_then(|x| x.parse::<usize>().ok())
            .unwrap_or(4),
        args.get(3)
            .and_then(|x| x.parse::<usize>().ok())
            .unwrap_or(4),
    );

    game.run();

    println!(
        "\n================================\n{}\n================================\n\nBoard: \n{}",
        game.get_winner().unwrap_or_else(|| {
            "The game ended with more moves left. I'm not sure how this happened.".to_string()
        }),
        game
    );

    std::process::exit(match game.get_winner_number() {
        x @ 1..=2 => x as i32,
        _ => 0,
    });
}
