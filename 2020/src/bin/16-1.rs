fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/16")?;

    let mut lines = input.lines();

    let rules: Vec<_> = lines
        .by_ref()
        .take_while(|&line| line != "")
        .map(parse_rule)
        .collect();

    lines.next();

    let _my_ticket = lines.next().map(parse_ticket).unwrap();

    let nearby_tickets: Vec<_> = lines
        .skip(2)
        .map(parse_ticket)
        .collect();

    let result: u64 = nearby_tickets
        .into_iter()
        .flatten()
        .filter(|value| !rules.iter().any(|r| r.is_valid(*value)))
        .sum();

    println!("{}", result);

    Ok(())
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: [(u64, u64); 2],
}

impl Rule {
    fn is_valid(&self, value: u64) -> bool {
        self.ranges.iter().any(|&(min, max)| value >= min && value <= max)
    }
}

// departure location: 36-626 or 651-973
fn parse_rule(line: &str) -> Rule {
    use std::convert::TryInto;

    let sep_idx = line.find(':').unwrap();

    let name = line[..sep_idx].to_owned();

    let rest = &line[sep_idx+2..];
    let ranges: Vec<_> = rest.split(" or ")
        .map(|range| {
            let bounds: Vec<u64> = range.split('-').map(|bound| bound.parse().unwrap()).collect();
            (bounds[0], bounds[1])
        })
        .collect();

    Rule {
        name,
        ranges: ranges.try_into().unwrap(),
    }
}

fn parse_ticket(line: &str) -> Vec<u64> {
    line.split(',').map(|v| v.parse().unwrap()).collect()
}

