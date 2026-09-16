use anyhow::{anyhow, bail};
use itertools::Itertools;
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let shuffles = parse_shuffles(input)?;
    Err::<usize, _>(anyhow!("Not implemented"))
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

fn parse_shuffles(s: &str) -> anyhow::Result<Vec<Shuffle>> {
    let mut shuffles = Vec::new();
    for line in s.lines() {
        shuffles.push(Shuffle::from_str(line)?);
    }
    Ok(shuffles)
}

struct Deck(Vec<usize>);

impl Deck {
    fn new(size: usize) -> Self {
        Self((0..size).collect_vec())
    }

    fn shuffle(&self, shuffle: Shuffle) -> anyhow::Result<Self> {
        match shuffle {
            Shuffle::NewStack => Ok(Self(self.0.iter().rev().copied().collect_vec())),
            Shuffle::Cut(cut) => {
                let split_at = if cut >= 0 {
                    cut.unsigned_abs()
                } else {
                    let cut = cut.unsigned_abs();
                    self.0
                        .len()
                        .checked_sub(cut)
                        .ok_or(anyhow!("Cut {cut} is greater than length {}", self.0.len()))?
                };
                let (first, second) = self
                    .0
                    .split_at_checked(split_at)
                    .ok_or(anyhow!("Could not split at {split_at}"))?;

                Ok(Self(std::iter::chain(second, first).copied().collect_vec()))
            }
            Shuffle::Increment(increment) => {
                let mut cards_with_indices = Vec::new();

                let mut index = 0;

                for card in &self.0 {
                    cards_with_indices.push((index, *card));
                    index = (index + increment) % self.0.len();
                }

                Ok(Self(
                    cards_with_indices
                        .into_iter()
                        .sorted_by_key(|(idx, _)| *idx)
                        .map(|(_, card)| card)
                        .collect_vec(),
                ))
            }
        }
    }
}

enum Shuffle {
    NewStack,
    Cut(isize),
    Increment(usize),
}

impl FromStr for Shuffle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cut) = s.strip_prefix("cut ") {
            let cut = cut.parse()?;
            Ok(Shuffle::Cut(cut))
        } else if let Some(increment) = s.strip_prefix("deal with increment ") {
            let increment = increment.parse()?;
            Ok(Shuffle::Increment(increment))
        } else if s == "deal into new stack" {
            Ok(Shuffle::NewStack)
        } else {
            bail!("Invalid shuffle: {s}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_shuffle_new_stack() {
        let deck = Deck::new(10);
        let deck = deck.shuffle(Shuffle::NewStack).unwrap();
        assert_eq!(deck.0, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_deck_shuffle_cut() {
        let deck = Deck::new(10);
        let deck = deck.shuffle(Shuffle::Cut(3)).unwrap();
        assert_eq!(deck.0, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);

        let deck = Deck::new(10);
        let deck = deck.shuffle(Shuffle::Cut(-4)).unwrap();
        assert_eq!(deck.0, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deck_shuffle_increment() {
        let deck = Deck::new(10);
        let deck = deck.shuffle(Shuffle::Increment(3)).unwrap();
        assert_eq!(deck.0, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }
}
