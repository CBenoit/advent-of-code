use std::collections::BTreeMap;

const DENSE_BUFFER_SZ: usize = 1 << 16;
const TARGET_TURN: usize = 30000000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/15")?;

    let starters: Vec<usize> = input
        .trim()
        .split(',')
        .map(|val| val.parse().unwrap())
        .collect();
    let nb_starters = starters.len();

    let mut dense: [usize; DENSE_BUFFER_SZ] = [usize::MAX; DENSE_BUFFER_SZ];
    let mut sparse: BTreeMap<usize, usize> = BTreeMap::new();

    starters
        .into_iter()
        .enumerate()
        .for_each(|(i, nb)| dense[nb] = i);

    let result = (nb_starters..TARGET_TURN - 1).fold(0, |num, turn| {
        if num < DENSE_BUFFER_SZ {
            if dense[num] == usize::MAX {
                dense[num] = turn;
                0
            } else {
                let last_turn = std::mem::replace(&mut dense[num], turn);
                turn - last_turn
            }
        } else {
            match sparse.insert(num, turn) {
                Some(last_turn) => turn - last_turn,
                None => 0,
            }
        }
    });

    println!("{}", result);

    Ok(())
}
