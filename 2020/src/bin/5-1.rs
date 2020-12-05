use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let result = stdin.lines().filter_map(to_seat_id).max();

    println!("{:?}", result);

    Ok(())
}

fn to_seat_id(line: Result<String, std::io::Error>) -> Option<u32> {
    let line = line.ok()?;

    let mut row_lower = 0;
    let mut row_upper = 127;

    let mut column_lower = 0;
    let mut column_upper = 7;

    for c in line.chars() {
        match c {
            'F' => row_upper -= ((row_upper as f32 - row_lower as f32) / 2.0).floor() as u32,
            'B' => row_lower += ((row_upper as f32 - row_lower as f32) / 2.0).ceil() as u32,
            'L' => column_upper -= ((column_upper as f32 - column_lower as f32) / 2.0).floor() as u32,
            'R' => column_lower += ((column_upper as f32 - column_lower as f32) / 2.0).ceil() as u32,
            _ => return None,
        }
    }

    Some(row_lower * 8 + column_lower)
}
