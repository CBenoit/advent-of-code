use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let nb_valids = stdin.lines()
        .filter_map(Result::ok)
        .filter_map(Line::new)
        .filter(Line::is_valid)
        .count();

    println!("{}", nb_valids);

    Ok(())
}

struct Policy {
    letter: char,
    min: usize,
    max: usize,
}

impl Policy {
    fn from_str(s: &str) -> Option<Self> {
        let dash = s.find('-')?;
        let space = s.find(' ')?;
        let min = s[..dash].parse().ok()?;
        let max = s[dash+1..space].parse().ok()?;
        let letter = s[space+2..].chars().next()?;

        Some(Self {
            letter,
            min,
            max,
        })
    }
}

struct Line {
    policy: Policy,
    password: String,
}

impl Line {
    fn new(line: String) -> Option<Self> {
        let sep = line.find(":")?;
        let policy = Policy::from_str(&line[..sep])?;
        let password = line[sep+1..].to_owned();

        Some(Self {
            policy,
            password,
        })
    }

    fn is_valid(&self) -> bool {
        let count = self.password
            .chars()
            .filter(|c| *c == self.policy.letter)
            .count();
        count >= self.policy.min && count <= self.policy.max
    }
}

