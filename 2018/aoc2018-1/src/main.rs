use std::io::{self, Read};
use std::collections::HashSet;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    first_part(&buffer)?;
    second_part(&buffer)?;

    Ok(())
}

fn first_part(input: &str) -> Result<()> {
    let mut acc = 0;
    for line in input.lines() {
        let delta = line.parse::<i64>()?;
        acc += delta;
    }

    println!("Part 1 result: {}", acc);

    Ok(())
}

fn second_part(input: &str) -> Result<()> {
    let mut seen_frequencies = HashSet::new();
    seen_frequencies.reserve(100);

    let mut acc = 0;

    loop {
        for line in input.lines() {
            let delta = line.parse::<i64>()?;
            acc += delta;

            if seen_frequencies.contains(&acc) {
                println!("Part 2 result: {}", acc);
                return Ok(());
            } else {
                seen_frequencies.insert(acc);
            }
        }
    }
}

