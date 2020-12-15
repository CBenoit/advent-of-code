use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/14")?;

    let program: Vec<Op> = input.lines().map(Op::parse).collect();
    let mut mem = BTreeMap::new();
    let mut mask = Mask(Vec::new());

    for op in program {
        match op {
            Op::SetMask(new_mask) => mask = new_mask,
            Op::SetVal { addr, val } => {
                let masked_value = mask.apply(val);
                mem.insert(addr, masked_value);
            }
        }
    }

    let mut result = 0;
    for val in mem.values() {
        result += val;
    }

    println!("{}", result);

    Ok(())
}

#[derive(Debug)]
struct Mask(Vec<(usize, bool)>);

impl Mask {
    fn parse(s: &str) -> Self {
        Self(
            s.chars()
                .rev()
                .enumerate()
                .filter_map(|(i, c)| match c {
                    'X' => None,
                    '0' => Some((i, false)),
                    '1' => Some((i, true)),
                    unexpected => panic!("unexpected mask bit: {}", unexpected),
                })
                .collect(),
        )
    }

    fn apply(&self, mut val: u64) -> u64 {
        for &(offset, to_set) in &self.0 {
            if to_set {
                let mask = 1 << offset;
                val |= mask;
            } else {
                let mask = !(1 << offset);
                val &= mask;
            }
        }
        val
    }
}

#[derive(Debug)]
enum Op {
    SetMask(Mask),
    SetVal { addr: usize, val: u64 },
}

impl Op {
    fn parse(line: &str) -> Op {
        match &line[..4] {
            "mask" => Op::SetMask(Mask::parse(&line[7..])),
            "mem[" => {
                let mut chars = line[4..].chars();

                let mut addr = String::new();
                loop {
                    match chars.next() {
                        Some(c) if c.is_numeric() => addr.push(c),
                        _ => break,
                    }
                }

                let mut val = String::new();
                for c in chars {
                    if c.is_numeric() {
                        val.push(c);
                    }
                }

                let addr = addr.parse().unwrap();
                let val = val.parse().unwrap();

                Op::SetVal { addr, val }
            }
            unexpected => panic!("unexpected op: {}", unexpected),
        }
    }
}
