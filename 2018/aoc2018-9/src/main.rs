extern crate aoc2018_9;

use std::io::{self, Read};

use aoc2018_9::marble_game::*;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let game = buffer.parse::<MarbleGame>()?;

    println!(
        "Marble game setting: {} players; last marble is worth {} points.",
        game.num_players, game.last_marble_value
    );

    println!(
        "Part 1: high score is {}",
        compute_marble_game_high_score(&game)
    );

    let game_bis = MarbleGame::new(game.num_players, game.last_marble_value * 100);
    println!(
        "Part 2: high score is {}",
        compute_marble_game_high_score(&game_bis)
    );

    Ok(())
}
