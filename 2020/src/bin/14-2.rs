use std::collections::{BTreeMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/14")?;

    let program: Vec<Op> = input.lines().map(Op::parse).collect();
    let mut mem = BTreeMap::new();
    let mut mask = Mask([MaskBit::Ignored; 36]);

    for op in program {
        match op {
            Op::SetMask(new_mask) => mask = new_mask,
            Op::SetVal { addr, val } => {
                for addr in mask.decode(addr) {
                    mem.insert(addr, val);
                }
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

#[derive(Debug, Clone, Copy)]
enum MaskBit {
    Set,
    Unset,
    Ignored,
    Float,
}

#[derive(Debug, Clone)]
struct Mask([MaskBit; 36]);

impl Mask {
    fn parse(s: &str) -> Self {
        use std::convert::TryInto;

        Self(
            s.chars()
                .rev()
                .map(|c| match c {
                    'X' => MaskBit::Float,
                    '0' => MaskBit::Ignored,
                    '1' => MaskBit::Set,
                    unexpected => panic!("unexpected mask bit: {}", unexpected),
                })
                .collect::<Vec<MaskBit>>()
                .try_into()
                .unwrap(),
        )
    }

    fn decode(&self, addr: usize) -> HashSet<usize> {
        let mut addrs = HashSet::new();
        decode_impl(addr, &mut addrs, self.clone());
        addrs
    }
}

fn decode_impl(mut addr: usize, addrs: &mut HashSet<usize>, mut mask: Mask) {
    if let Some((i, _)) = mask
        .0
        .iter()
        .cloned()
        .enumerate()
        .find(|(_, bit)| matches!(bit, MaskBit::Float))
    {
        mask.0[i] = MaskBit::Unset;
        decode_impl(addr, addrs, mask.clone());
        mask.0[i] = MaskBit::Set;
        decode_impl(addr, addrs, mask);
    } else {
        for (offset, bit) in mask.0.iter().enumerate() {
            match bit {
                MaskBit::Set => {
                    let mask = 1 << offset;
                    addr |= mask;
                }
                MaskBit::Unset => {
                    let mask = !(1 << offset);
                    addr &= mask;
                }
                MaskBit::Ignored => {}
                MaskBit::Float => panic!(),
            }
        }
        addrs.insert(addr);
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
