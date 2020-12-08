use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/7")?;

    let rules: HashMap<&str, _> = input.lines()
        .map(parse_line)
        .collect();

    let result = rules
        .keys()
        .filter(|container| can_contains(*container, "shiny gold", &rules))
        .count();

    println!("{}", result);

    Ok(())
}

#[derive(Debug)]
struct Rule<'a> {
    qty: u32,
    color: &'a str,
}

fn can_contains(container: &str, contained: &str, rules: &HashMap<&str, Vec<Rule<'_>>>) -> bool {
    let mut container_list = vec![container];

    while let Some(container) = container_list.pop() {
        let container_rules = rules.get(container).unwrap();
        for rule in container_rules {
            if rule.color == contained {
                return true;
            } else {
                container_list.push(rule.color);
            }
        }
    }

    false
}

fn parse_line(line: &str) -> (&str, Vec<Rule<'_>>) {
    const COLOR_END_STR: &str = " bags contain ";
    const ELEM_END_STR: &str = " bag";
    const NO_OTHER_STR: &str = "no other";

    let color_end = line.find(COLOR_END_STR).unwrap();
    let color = &line[..color_end];
    let rest = &line[color_end + COLOR_END_STR.len()..];

    let mut rules = Vec::new();
    for elem in rest.split(", ") {
        let elem_end = elem.find(ELEM_END_STR).unwrap();
        let elem = &elem[..elem_end];
        if elem != NO_OTHER_STR {
            let number_end = elem.find(' ').unwrap();
            let qty = elem[..number_end].parse().unwrap();
            let color = &elem[number_end + 1..];
            rules.push(Rule { qty, color });
        }
    }

    (color, rules)
}
