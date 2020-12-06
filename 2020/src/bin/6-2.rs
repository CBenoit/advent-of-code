use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf)?;
    let buf = String::from_utf8(buf)?;

    let result: usize = buf
        .trim_end_matches("\n")
        .split("\n\n")
        .map(|group| {
            let mut map = HashMap::new();
            let mut nb_persons = 1;
            for c in group.chars() {
                if c.is_alphabetic() {
                    let counter = map.entry(c).or_insert(0);
                    *counter += 1;
                } else if c == '\n' {
                    nb_persons += 1;
                }
            }
            map.values().filter(|count| **count == nb_persons).count()
        })
        .sum();

    println!("{}", result);

    Ok(())
}
