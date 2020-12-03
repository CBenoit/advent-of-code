fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let stdin = stdin.lock();

    let map: Vec<Vec<bool>> = stdin
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            l.chars()
                .map(|c| if c == '#' { true } else { false })
                .collect::<Vec<bool>>()
        })
        .collect();

    let mut current_x = 0;
    let mut count = 0;
    for row in map {
        current_x %= row.len();
        if row[current_x] {
            count += 1;
        }
        current_x += 3;
    }

    println!("{}", count);

    Ok(())
}
