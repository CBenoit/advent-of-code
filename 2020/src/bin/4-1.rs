use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf)?;
    let buf = String::from_utf8(buf)?;

    let result = buf
        .split("\n\n")
        .filter(|passport| {
            required.iter().all(|field| passport.contains(field))
        })
        .count();

    println!("{}", result);

    Ok(())
}
