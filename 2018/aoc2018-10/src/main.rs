use std::io::{self, Read};

use aoc2018_10::*;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut sky = Sky::from(buffer);
    let mut max_features = 0;
    let mut best_sky = sky.clone();

    for _ in 0..100_000 {
        sky.update();
        let nb_features = sky.count_features();
        if nb_features > max_features {
            best_sky = sky.clone();
            max_features = nb_features;
        } else if (nb_features as f32) < (max_features as f32) * 0.5 {
            break; // we probably already found the best sky!
        }
    }

    println!("Best sky:\n{}", best_sky);

    Ok(())
}

