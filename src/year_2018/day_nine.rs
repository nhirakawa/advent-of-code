use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use anyhow::bail;
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let params = Params::from_str(input)?;
    Ok(play_marbles(params))
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let params = Params::from_str(input)?;
    let Params {
        number_of_players,
        number_of_marbles,
    } = params;
    let params = (number_of_players, number_of_marbles * 100).into();
    Ok(play_marbles(params))
}

/// Simulates a marble game with the specified number of players and marbles
///
/// # Parameters
///
/// * `number_of_players` - The number of players participating in the game
/// * `number_of_marbles` - The highest marble number to be played (inclusive)
///
/// # Returns
///
/// The winning player's score
fn play_marbles(
    Params {
        number_of_players,
        number_of_marbles,
    }: Params,
) -> usize {
    let mut player_index = 0;

    let mut player_scores = HashMap::new();

    for player in 0..number_of_players {
        player_scores.insert(player, 0);
    }

    let mut marbles = Marbles::new();

    marbles.add_marble(0);

    for marble in 1..=number_of_marbles {
        if let Some(score) = marbles.add_marble(marble) {
            let current_score = player_scores.get(&player_index).copied().unwrap_or(0);
            player_scores.insert(player_index, current_score + score);
        }

        player_index = (player_index + 1) % number_of_players;
    }

    player_scores.values().max().copied().unwrap()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Params {
    number_of_players: usize,
    number_of_marbles: usize,
}

impl FromStr for Params {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed_numbers = s
            .split_ascii_whitespace()
            .filter_map(|token| token.parse::<usize>().ok())
            .collect_vec();

        if parsed_numbers.len() != 2 {
            bail!(
                "Expected 2 tokens, found {} - {:?}",
                parsed_numbers.len(),
                parsed_numbers
            );
        }

        let number_of_players = parsed_numbers[0];
        let number_of_marbles = parsed_numbers[1];

        Ok(Params {
            number_of_players,
            number_of_marbles,
        })
    }
}

impl From<(usize, usize)> for Params {
    fn from((number_of_players, number_of_marbles): (usize, usize)) -> Self {
        Self {
            number_of_players,
            number_of_marbles,
        }
    }
}

/// A wrapper around a circular buffer
/// The current marble is always the head of the queue
struct Marbles {
    inner: VecDeque<usize>,
}

impl Marbles {
    fn new() -> Marbles {
        let inner = VecDeque::new();
        Marbles { inner }
    }

    /// Inserts a marble, handling any necessary rotation
    /// Returns an optional score that should be added for the player
    fn add_marble(&mut self, marble: usize) -> Option<usize> {
        if self.inner.len() < 2 {
            self.inner.push_front(marble);
            return None;
        }

        if marble % 23 == 0 {
            for _ in 0..6 {
                let popped = self.inner.pop_back().unwrap();
                self.inner.push_front(popped);
            }

            let removed = self.inner.pop_back().unwrap();

            return Some(marble + removed);
        } else {
            let current = self.inner.pop_front().unwrap();
            let next_clockwise = self.inner.pop_front().unwrap();

            self.inner.push_front(marble);

            self.inner.push_back(current);
            self.inner.push_back(next_clockwise);

            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_inserts_some_marbles() {
        let mut marbles = Marbles::new();

        marbles.add_marble(0);

        assert_eq!(marbles.inner, [0]);

        marbles.add_marble(1);

        assert_eq!(marbles.inner, [1, 0]);

        marbles.add_marble(2);
        assert_eq!(marbles.inner, [2, 1, 0]);

        marbles.add_marble(3);
        assert_eq!(marbles.inner, [3, 0, 2, 1]);
    }

    #[test]
    fn it_inserts_marble_23() {
        let inner = vec![
            22, 11, 1, 12, 6, 13, 3, 4, 7, 15, 0, 16, 8, 17, 14, 18, 9, 19, 2, 20, 10, 21, 5,
        ]
        .into();
        let mut marbles = Marbles { inner };

        let score = marbles.add_marble(23);

        assert_eq!(score, Some(32));
        assert_eq!(
            marbles.inner,
            [
                19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 4, 7, 15, 0, 16, 8, 17, 14, 18,
            ]
        )
    }

    #[test]
    fn it_plays_full_games_of_marbles() {
        assert_eq!(play_marbles((9, 25).into()), 32);
        assert_eq!(play_marbles((10, 1618).into()), 8317);
        assert_eq!(play_marbles((13, 7999).into()), 146373);
        assert_eq!(play_marbles((17, 1104).into()), 2764);
        assert_eq!(play_marbles((21, 6111).into()), 54718);
        assert_eq!(play_marbles((30, 5807).into()), 37305);
    }
}
