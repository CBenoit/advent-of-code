fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/10")?;

    let mut adapters: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);

    let ones = adapters.windows(2).filter(|w| w[1] - w[0] == 1).count();
    let threes = adapters.windows(2).filter(|w| w[1] - w[0] == 3).count();

    println!("{} * {} = {}", ones, threes, ones * threes);

    Ok(())
}
