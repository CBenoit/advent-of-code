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

    let directions = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut all_counts = Vec::new();

    for dir in directions.iter() {
        let mut x = 0;
        let mut y = 0;
        let mut count = 0;

        while y < map.len() {
            x %= map[y].len();

            if map[y][x] {
                count += 1;
            }

            x += dir.0;
            y += dir.1;
        }

        all_counts.push(count);
    }

    let result: u64 = all_counts.iter().product();

    println!("{}", result);

    Ok(())
}
