fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/9")?;
    let lines = input.lines().map(|l| l.parse::<u64>().unwrap());
    let initial: Vec<u64> = lines.clone().take(25).collect();
    let rest = lines.skip(25);

    let mut list = CircularList::new(initial);
    for val in rest {
        if list.is_valid(val) {
            list.push(val);
        } else {
            println!("{}", val);
            break;
        }
    }

    Ok(())
}

struct CircularList {
    next: usize,
    vals: Vec<u64>,
}

impl CircularList {
    fn new(initial: Vec<u64>) -> Self {
        Self {
            next: 0,
            vals: initial,
        }
    }

    fn push(&mut self, val: u64) {
        self.vals[self.next] = val;
        self.next = (self.next + 1) % self.vals.len();
    }

    fn is_valid(&self, val: u64) -> bool {
        for (i, a) in self.vals.iter().enumerate() {
            for b in &self.vals[i..] {
                if a + b == val {
                    return true;
                }
            }
        }

        false
    }
}
