fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/10")?;

    let mut adapters: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);

    let init: [u64; 3] = if adapters[2] <= 3 {
        [1, 1, 2]
    } else {
        [1, 1, 1]
    };

    let acc = adapters.windows(4).fold(init, |acc, w| {
        let current_jolts = w[3];
        let next = w
            .iter()
            .enumerate()
            .take(3)
            .filter(|(_, jolts)| current_jolts - *jolts <= 3)
            .map(|(i, _)| acc[i])
            .sum();
        [acc[1], acc[2], next]
    });

    println!("{}", acc[2]);

    Ok(())
}
