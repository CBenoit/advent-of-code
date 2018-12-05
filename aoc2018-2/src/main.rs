use std::io::{self, Read};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ids: Vec<&str> = buffer.lines().collect();

    first_part(&ids)?;
    second_part(&ids)
}

fn first_part(ids: &Vec<&str>) -> Result<()> {
    let (mut nb_twos, mut nb_threes) = (0, 0);
    let mut acc = HashMap::with_capacity(15);
    for id in ids {
        acc.clear();
        for c in id.chars()  {
            let counter = acc.entry(c).or_insert(0);
            *counter += 1;
        }

        if acc.values().any(|&count| count == 2) {
            nb_twos += 1;
        }

        if acc.values().any(|&count| count == 3) {
            nb_threes += 1;
        }
    }

    let result = nb_twos * nb_threes;

    println!("Part 1 result: {}", result);

    Ok(())
}

fn second_part(ids: &Vec<&str>) -> Result<()> {
    for i in 0..ids.len() {
        for j in i+1..ids.len() {
            if let Some(common_letters) = exactly_one_difference(&ids[i], &ids[j]) {
                println!("Part 2 result: {}", common_letters);
                return Ok(());
            }
        }
    }

    Err(From::from("Part 2: no answer found..."))
}

/// returns common letters if success
fn exactly_one_difference(id1: &str, id2: &str) -> Option<String> {
    if id1.len() != id2.len() {
        return None;
    }

    let mut first_difference_found = false;
    for (c1, c2) in id1.chars().zip(id2.chars()) {
        if c1 != c2 {
            if first_difference_found {
                // there is two differences or more
                return None;
            }

            first_difference_found = true;
        }
    }

    Some(
        id1.chars()
            .zip(id2.chars())
            .filter(|&(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_difference() {
        let id1 = "fghij";
        let id2 = "fguij";
        let common_letters = exactly_one_difference(id1, id2);
        assert_eq!("fgij", common_letters.unwrap());
    }

    #[test]
    fn two_differences() {
        let id1 = "abcde";
        let id2 = "axcye";
        assert_eq!(None, exactly_one_difference(id1, id2));
    }
}

