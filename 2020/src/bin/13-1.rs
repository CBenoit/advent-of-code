fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/13")?;

    let mut lines = input.lines();
    let arrival: u64 = lines.next().unwrap().parse().unwrap();
    let buses: Vec<u64> = lines.next().unwrap().split(',').filter(|id| *id != "x").map(|id| id.parse().unwrap()).collect();

    let mut earliest = arrival;
    let bus_id = loop {
        match buses.iter().find(|&id| earliest % id == 0) {
            Some(id) => break id,
            None => {}
        }
        earliest += 1;
    };

    println!("{}", (earliest - arrival) * bus_id);

    Ok(())
}
