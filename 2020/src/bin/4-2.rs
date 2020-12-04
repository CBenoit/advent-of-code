use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf)?;
    let buf = String::from_utf8(buf)?;

    let result = buf
        .split("\n\n")
        .filter_map(validate_password)
        .count();

    println!("{}", result);

    Ok(())
}

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

macro_rules! ensures {
    ($cond:expr) => {{
        if (!$cond) {
            return None;
        }
    }}
}

fn extract_field<'a>(passport: &'a str, field: &str) -> Option<&'a str> {
    let start = passport.find(field)?;
    let end_1 = passport[start..].find(' ').map(|i| i + start);
    let end_2 = passport[start..].find('\n').map(|i| i + start);
    let end = match (end_1, end_2) {
        (None, Some(end)) => end,
        (Some(end), None) => end,
        (Some(end_1), Some(end_2)) => std::cmp::min(end_1, end_2),
        _ => passport.len(),
    };
    passport[start..end].split(":").nth(1)
}

fn validate_password(passport: &str) -> Option<&str> {
    let byr = extract_field(passport, "byr")?.parse::<u32>().ok()?;
    ensures!(byr >= 1920 && byr <= 2002);

    let iyr = extract_field(passport, "iyr")?.parse::<u32>().ok()?;
    ensures!(iyr >= 2010 && iyr <= 2020);

    let eyr = extract_field(passport, "eyr")?.parse::<u32>().ok()?;
    ensures!(eyr >= 2020 && eyr <= 2030);

    let hgt = extract_field(passport, "hgt")?;
    let hgt_parsed = hgt[..hgt.len() - 2].parse::<u32>().ok()?;
    match &hgt[hgt.len() - 2..] {
        "cm" => ensures!(hgt_parsed >= 150 && hgt_parsed <= 193),
        "in" => ensures!(hgt_parsed >= 59 && hgt_parsed <= 76),
        _ => return None,
    }

    let hcl = extract_field(passport, "hcl")?;
    let mut hcl_iter = hcl.chars();
    ensures!(hcl_iter.next() == Some('#'));
    ensures!(hcl_iter.all(|c| c.is_numeric() || c.is_alphabetic() && c.is_lowercase()));

    let ecl = extract_field(passport, "ecl")?;
    ensures!(EYE_COLORS.iter().any(|c| *c == ecl));

    let pid = extract_field(passport, "pid")?;
    ensures!(pid.len() == 9);
    ensures!(pid.chars().all(|c| c.is_digit(10)));

    Some(passport)
}

