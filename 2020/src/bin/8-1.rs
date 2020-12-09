use std::collections::HashSet;
use std::convert::TryFrom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/8")?;
    let ops: Vec<Op> = (&input).lines().map(Op::from_line).collect();
    let mut program = Program::new(ops);
    let mut touched = HashSet::<usize>::new();

    loop {
        if !touched.insert(program.pc) {
            break;
        }

        program.tick();
    }

    println!("{}", program.acc);

    Ok(())
}

enum Op {
    Acc(i64),
    Jump(isize),
    Nop,
}

impl Op {
    fn from_line(line: &str) -> Self {
        match &line[..3] {
            "nop" => Self::Nop,
            "acc" => Self::Acc(line[4..].parse().unwrap()),
            "jmp" => Self::Jump(line[4..].parse().unwrap()),
            unexpected => panic!("unexpected op: {}", unexpected),
        }
    }
}

struct Program {
    pc: usize,
    ops: Vec<Op>,
    acc: i64,
}

impl Program {
    fn new(ops: Vec<Op>) -> Self {
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
            Op::Nop => self.pc += 1,
        }
    }
}
