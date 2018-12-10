use regex::Regex;
use std::str::FromStr;

use crate::list::List;

pub struct MarbleGame {
    pub num_players: u64,
    pub last_marble_value: u64,
}

impl MarbleGame {
    pub fn new(num_players: u64, last_marble_value: u64) -> MarbleGame {
        MarbleGame {
            num_players,
            last_marble_value,
        }
    }
}

impl FromStr for MarbleGame {
    type Err = Box<std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+)[a-zA-Z; ]+(\d+)")?;
        let caps = re.captures(input).ok_or("no match from input.")?;
        let num_players = caps
            .get(1)
            .ok_or("missing number of players")?
            .as_str()
            .parse::<u64>()?;
        let last_marble_value = caps
            .get(2)
            .ok_or("missing last marble's value")?
            .as_str()
            .parse::<u64>()?;

        Ok(MarbleGame::new(num_players, last_marble_value))
    }
}

pub fn compute_marble_game_high_score(game: &MarbleGame) -> u64 {
    let mut scores = Vec::with_capacity(game.num_players as usize);
    for _ in 0..game.num_players {
        scores.push(0);
    }

    let mut marbles = List::new();
    marbles.push_front(0); // the first marble is 0!

    let mut cur_marble = marbles.get_front_link();
    let mut new_marble_id = 0;
    let mut current_player = 0;

    loop {
        new_marble_id = new_marble_id + 1;

        if new_marble_id % 23 == 0 {
            // special case!
            let marble_to_remove = marbles.get_nth_prev_link_cyclic(&cur_marble, 7);

            scores[current_player] +=
                new_marble_id + *marbles.peek_link(&marble_to_remove).unwrap();

            if new_marble_id >= game.last_marble_value {
                break;
            } else {
                if new_marble_id % (23 * 200) == 0 {
                    println!(
                        "[trace] expected: {}, current: {}",
                        game.last_marble_value, new_marble_id
                    );
                }
            }

            cur_marble = marbles.get_nth_next_link_cyclic(&marble_to_remove, 1);
            marbles.pop_link(marble_to_remove);
        } else {
            let insert_target = marbles.get_nth_next_link_cyclic(&cur_marble, 1);
            marbles.push_after_link(&insert_target, new_marble_id);
            cur_marble = marbles.get_next_link(&insert_target);
        }

        current_player = (current_player + 1) % game.num_players as usize;
    }

    *scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_score_computation() {
        assert_eq!(compute_marble_game_high_score(&MarbleGame::new(9, 23)), 32);
        assert_eq!(
            compute_marble_game_high_score(&MarbleGame::new(17, 1104)),
            2764
        );
        assert_eq!(
            compute_marble_game_high_score(&MarbleGame::new(30, 5807)),
            37305
        );
        assert_eq!(
            compute_marble_game_high_score(&MarbleGame::new(426, 72058)),
            424112
        );
    }
}
