use std::convert::TryInto;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/11")?;

    let front: Buffer = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    _ => State::Floor,
                })
                .collect::<Vec<State>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[State; 99]>>()
        .try_into()
        .unwrap();

    let mut grid = Grid::new(front);

    loop {
        match grid.tick() {
            Control::Chaos => {}
            Control::Stable => break,
        }
    }

    let (current_grid, _) = grid.get_buffers();
    let count = current_grid
        .iter()
        .flatten()
        .filter(|s| **s == State::Occupied)
        .count();

    println!("{}", count);

    Ok(())
}

enum Control {
    Chaos,
    Stable,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Floor,
    Empty,
    Occupied,
}

type Buffer = [[State; 99]; 98];

const DIRS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

struct Grid {
    front: Buffer,
    back: Buffer,
    current_is_front: bool,
}

impl Grid {
    fn new(front: Buffer) -> Self {
        Self {
            front,
            back: [[State::Floor; 99]; 98],
            current_is_front: true,
        }
    }

    fn get_buffers(&mut self) -> (&Buffer, &mut Buffer) {
        if self.current_is_front {
            (&self.front, &mut self.back)
        } else {
            (&self.back, &mut self.front)
        }
    }

    fn tick(&mut self) -> Control {
        let (current, next) = self.get_buffers();

        let mut touched = false;

        for (i, row) in current.iter().enumerate() {
            for (j, s) in row.iter().enumerate() {
                let occ = DIRS
                    .iter()
                    .filter_map(|dir| match raycast(current, (i, j), *dir) {
                        Some(State::Occupied) => Some(()),
                        _ => None,
                    })
                    .count();

                match s {
                    State::Occupied if occ >= 5 => {
                        touched = true;
                        next[i][j] = State::Empty;
                    }
                    State::Empty if occ == 0 => {
                        touched = true;
                        next[i][j] = State::Occupied;
                    }
                    _ => next[i][j] = *s,
                }
            }
        }

        self.current_is_front = !self.current_is_front;

        if touched {
            Control::Chaos
        } else {
            Control::Stable
        }
    }
}

fn raycast(buf: &Buffer, mut pos: (usize, usize), dir: (i8, i8)) -> Option<State> {
    use std::convert::TryFrom;

    loop {
        pos.0 = usize::try_from((pos.0 as isize) - (dir.0 as isize)).ok()?;
        pos.1 = usize::try_from((pos.1 as isize) - (dir.1 as isize)).ok()?;

        let state = buf.get(pos.0).map(|row| row.get(pos.1)).flatten()?;

        match state {
            State::Floor => {}
            State::Empty => return Some(State::Empty),
            State::Occupied => return Some(State::Occupied),
        }
    }
}
