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
            Control::Chaos => {},
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
                let occ = get_adjacent_idx(i, j)
                    .iter()
                    .map(|(a_i, a_j)| current.get(*a_i).map(|row| row.get(*a_j)).flatten())
                    .filter_map(|s_opt| match s_opt {
                        Some(State::Occupied) => s_opt,
                        _ => None,
                    })
                    .count();

                match s {
                    State::Occupied if occ >= 4 => {
                        touched = true;
                        next[i][j] = State::Empty;
                    },
                    State::Empty if occ == 0 => {
                        touched = true;
                        next[i][j] = State::Occupied;
                    },
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

fn get_adjacent_idx(i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut indexes = Vec::with_capacity(8);

    let i_inf = i.checked_sub(1).unwrap_or(i);
    let j_inf = j.checked_sub(1).unwrap_or(j);

    for a_i in i_inf..=i + 1 {
        for a_j in j_inf..=j + 1 {
            if i == a_i && j == a_j {
                continue;
            }

            indexes.push((a_i, a_j));
        }
    }

    indexes
}
