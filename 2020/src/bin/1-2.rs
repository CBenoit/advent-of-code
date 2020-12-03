use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut nums = Vec::new();
    for line in stdin.lines() {
        let line = line.unwrap();
        let num = line.parse::<u32>().unwrap();
        nums.push(num);
    }

    for (i, n1) in nums.iter().enumerate() {
        for (j, n2) in nums[i..].iter().enumerate() {
            for n3 in &nums[i+j..] {
                if n1 + n2 + n3 == 2020 {
                    println!("{} * {} * {} = {}", n1, n2, n3, n1 * n2 * n3);
                }
            }
        }
    }
}
