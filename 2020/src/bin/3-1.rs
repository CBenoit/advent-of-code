fn main() {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let count = stdin.lock().lines().filter_map(Result::ok).enumerate().filter(|(i, l)| l.chars().nth(i * 3 % l.len()).unwrap() == '#').count();
    println!("{}", count);
}
