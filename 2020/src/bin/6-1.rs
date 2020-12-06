use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf)?;
    let buf = String::from_utf8(buf)?;

    let result: usize = buf
        .split("\n\n")
        .map(|group| {
            let mut set = HashSet::new();
            for c in group.chars() {
                if c.is_alphabetic() {
                    set.insert(c);
                }
            }
            set.len()
        })
        .sum();

    println!("{}", result);

    Ok(())
}
