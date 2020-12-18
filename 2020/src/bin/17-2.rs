use std::collections::HashMap;
use std::convert::TryInto;

const TARGET_TURN: u64 = 6;

#[rustfmt::skip]
const NEIGHBOURS: [(isize, isize, isize, isize); 80] = [
  (-1,-1,-1,-1), (-1,-1,-1, 0), (-1,-1,-1, 1),
  (-1,-1, 0,-1), (-1,-1, 0, 0), (-1,-1, 0, 1),
  (-1,-1, 1,-1), (-1,-1, 1, 0), (-1,-1, 1, 1),
  (-1, 0,-1,-1), (-1, 0,-1, 0), (-1, 0,-1, 1),
  (-1, 0, 0,-1), (-1, 0, 0, 0), (-1, 0, 0, 1),
  (-1, 0, 1,-1), (-1, 0, 1, 0), (-1, 0, 1, 1),
  (-1, 1,-1,-1), (-1, 1,-1, 0), (-1, 1,-1, 1),
  (-1, 1, 0,-1), (-1, 1, 0, 0), (-1, 1, 0, 1),
  (-1, 1, 1,-1), (-1, 1, 1, 0), (-1, 1, 1, 1),
  ( 0,-1,-1,-1), ( 0,-1,-1, 0), ( 0,-1,-1, 1),
  ( 0,-1, 0,-1), ( 0,-1, 0, 0), ( 0,-1, 0, 1),
  ( 0,-1, 1,-1), ( 0,-1, 1, 0), ( 0,-1, 1, 1),
  ( 0, 0,-1,-1), ( 0, 0,-1, 0), ( 0, 0,-1, 1),
  ( 0, 0, 0,-1),                ( 0, 0, 0, 1),
  ( 0, 0, 1,-1), ( 0, 0, 1, 0), ( 0, 0, 1, 1),
  ( 0, 1,-1,-1), ( 0, 1,-1, 0), ( 0, 1,-1, 1),
  ( 0, 1, 0,-1), ( 0, 1, 0, 0), ( 0, 1, 0, 1),
  ( 0, 1, 1,-1), ( 0, 1, 1, 0), ( 0, 1, 1, 1),
  ( 1,-1,-1,-1), ( 1,-1,-1, 0), ( 1,-1,-1, 1),
  ( 1,-1, 0,-1), ( 1,-1, 0, 0), ( 1,-1, 0, 1),
  ( 1,-1, 1,-1), ( 1,-1, 1, 0), ( 1,-1, 1, 1),
  ( 1, 0,-1,-1), ( 1, 0,-1, 0), ( 1, 0,-1, 1),
  ( 1, 0, 0,-1), ( 1, 0, 0, 0), ( 1, 0, 0, 1),
  ( 1, 0, 1,-1), ( 1, 0, 1, 0), ( 1, 0, 1, 1),
  ( 1, 1,-1,-1), ( 1, 1,-1, 0), ( 1, 1,-1, 1),
  ( 1, 1, 0,-1), ( 1, 1, 0, 0), ( 1, 1, 0, 1),
  ( 1, 1, 1,-1), ( 1, 1, 1, 0), ( 1, 1, 1, 1),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/17")?;

    let grid: Vec<(isize, isize, isize, isize)> = input
        .lines()
        .enumerate()
        .flat_map(|(x, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(y, _)| (x.try_into().unwrap(), y.try_into().unwrap(), 0, 0))
        })
        .collect();

    let last_grid = (0..TARGET_TURN).fold(grid, |grid, _| {
        let mut active_neighbors: HashMap<(isize, isize, isize, isize), u8> = HashMap::new();

        grid.iter().for_each(|(x, y, z, w)| {
            NEIGHBOURS.iter().for_each(|(dx, dy, dz, dw)| {
                let nb_actives = active_neighbors
                    .entry((x + dx, y + dy, z + dz, w + dw))
                    .or_insert(0);
                *nb_actives += 1;
            })
        });

        active_neighbors
            .into_iter()
            .filter_map(|(coords, nb)| match (grid.contains(&coords), nb) {
                (true, 2) | (_, 3) => Some(coords),
                _ => None,
            })
            .collect()
    });

    let result = last_grid.len();

    println!("{}", result);

    Ok(())
}
