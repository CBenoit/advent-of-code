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
    positions: [usize; 2],
}

impl Policy {
    fn from_str(s: &str) -> Option<Self> {
        let dash = s.find('-')?;
        let space = s.find(' ')?;
        let first = s[..dash].parse().ok()?;
        let second = s[dash+1..space].parse().ok()?;
        let letter = s[space+1..].chars().next()?;

        Some(Self {
            letter,
            positions: [first, second],
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
        let password = line[sep+2..].to_owned();

        Some(Self {
            policy,
            password,
        })
    }

    fn is_valid(&self) -> bool {
        let count = self.policy
            .positions
            .iter()
            .filter(|p| match self.password.chars().nth(*p - 1) {
                Some(c) if c == self.policy.letter => true,
                _ => false,
            })
            .count();
        count == 1
    }
}

