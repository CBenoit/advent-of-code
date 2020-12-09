use std::collections::HashSet;
use std::convert::TryFrom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/8")?;
    let mut ops: Vec<Op> = (&input).lines().map(Op::from_line).collect();

    for i in 0..ops.len() {
        let result;

        match ops[i] {
            Op::Acc(_) => continue,
            Op::Jump(arg) => {
                ops[i] = Op::Nop(arg);
                result = Program::new(&ops).run();
                ops[i] = Op::Jump(arg);
            }
            Op::Nop(arg) => {
                ops[i] = Op::Jump(arg);
                result = Program::new(&ops).run();
                ops[i] = Op::Nop(arg);
            }
        }

        match result {
            Some(val) => {
                println!("{}", val);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

enum Op {
    Acc(i64),
    Jump(isize),
    Nop(isize),
}

impl Op {
    fn from_line(line: &str) -> Self {
        match &line[..3] {
            "nop" => Self::Nop(line[4..].parse().unwrap()),
            "acc" => Self::Acc(line[4..].parse().unwrap()),
            "jmp" => Self::Jump(line[4..].parse().unwrap()),
            unexpected => panic!("unexpected op: {}", unexpected),
        }
    }
}

struct Program<'a> {
    pc: usize,
    ops: &'a [Op],
    acc: i64,
}

impl<'a> Program<'a> {
    fn new(ops: &'a [Op]) -> Self {
        Self {
            pc: 0,
            ops,
            acc: 0,
        }
    }

    fn tick(&mut self) {
        match self.ops[self.pc] {
            Op::Acc(arg) => {
                self.acc += arg;
                self.pc += 1;
            }
            Op::Jump(arg) => {
                if arg.is_negative() {
                    self.pc -= usize::try_from(-arg).unwrap();
                } else {
                    self.pc += usize::try_from(arg).unwrap();
                }
            }
            Op::Nop(_) => self.pc += 1,
        }
    }

    fn run(mut self) -> Option<i64> {
        let mut touched = HashSet::<usize>::new();

        while self.pc < self.ops.len() {
            if !touched.insert(self.pc) {
                return None;
            }

            self.tick();
        }

        Some(self.acc)
    }
}
