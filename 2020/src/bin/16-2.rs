fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/16")?;

    let mut lines = input.lines();

    let rules: Vec<_> = lines
        .by_ref()
        .take_while(|&line| line != "")
        .map(parse_rule)
        .collect();

    lines.next();

    let my_ticket = lines.next().map(parse_ticket).unwrap();

    let mut possible = vec![vec![true; rules.len()]; rules.len()];

    lines
        .skip(2)
        .map(parse_ticket)
        .filter(|ticket| ticket.iter().all(|value| rules.iter().any(|r| r.is_valid(*value))))
        .for_each(|ticket| {
            for (rule_idx, rule) in rules.iter().enumerate() {
                for (field_idx, &value) in ticket.iter().enumerate() {
                    if !rule.is_valid(value) {
                        possible[rule_idx][field_idx] = false;
                    }
                }
            }
        });

    let mut order: Vec<(usize, usize)> = possible.iter()
        .map(|p| p.iter().filter(|v| **v).count())
        .enumerate()
        .collect();

    order.sort_by(|(_, ord1), (_, ord2)| ord1.cmp(ord2));

    let mut ordered_rules = vec![None; rules.len()];

    order.into_iter()
        .for_each(|(rule_idx, _)| {
            let (found_idx, _) = possible[rule_idx]
                .iter()
                .enumerate()
                .find(|(idx, x)| **x && ordered_rules[*idx].is_none())
                .unwrap();

            ordered_rules[found_idx] = Some(rules[rule_idx].name.clone());
        });

    let result: u64 = ordered_rules
        .into_iter()
        .flatten()
        .enumerate()
        .filter(|(_, name)| name.starts_with("departure"))
        .map(|(idx, _)| my_ticket[idx])
        .product();

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

