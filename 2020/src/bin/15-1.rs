use std::collections::BTreeMap;

const TARGET_TURN: usize = 2020;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/15")?;

    let mut numbers: BTreeMap<usize, usize> = input
        .trim()
        .split(',')
        .enumerate()
        .map(|(i, val)| ((val).parse().unwrap(), i))
        .collect();

    let result =
        (numbers.len()..TARGET_TURN - 1).fold(0, |num, turn| match numbers.insert(num, turn) {
            Some(last_turn) => turn - last_turn,
            None => 0,
        });

    println!("{}", result);

    Ok(())
}
