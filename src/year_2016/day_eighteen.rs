use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::iter::successors;
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let initial = TileRow::from_str(input)?;
    let number_of_safe_spaces = successors(Some(initial), |row| Some(row.next()))
        .take(40)
        .map(|row| row.width - row.traps.len())
        .sum::<usize>();
    Ok(number_of_safe_spaces)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let initial = TileRow::from_str(input)?;
    let number_of_safe_spaces = successors(Some(initial), |row| Some(row.next()))
        .take(400_000)
        .map(|row| row.width - row.traps.len())
        .sum::<usize>();
    Ok(number_of_safe_spaces)
}

enum Tile {
    Trap,
    Safe,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct TileRow {
    traps: HashSet<usize>,
    width: usize,
}

impl TileRow {
    fn next(&self) -> Self {
        let mut traps = HashSet::new();

        for i in 0..self.width {
            let left = if i != 0 && self.traps.contains(&(i - 1)) {
                Tile::Trap
            } else {
                Tile::Safe
            };
            let center = if self.traps.contains(&i) {
                Tile::Trap
            } else {
                Tile::Safe
            };
            let right = if i != self.width && self.traps.contains(&(i + 1)) {
                Tile::Trap
            } else {
                Tile::Safe
            };

            let next_tile = match (left, center, right) {
                (Tile::Trap, Tile::Trap, Tile::Safe) => Tile::Trap,
                (Tile::Safe, Tile::Trap, Tile::Trap) => Tile::Trap,
                (Tile::Trap, Tile::Safe, Tile::Safe) => Tile::Trap,
                (Tile::Safe, Tile::Safe, Tile::Trap) => Tile::Trap,
                _ => Tile::Safe,
            };

            if matches!(next_tile, Tile::Trap) {
                traps.insert(i);
            }
        }

        Self {
            traps,
            width: self.width,
        }
    }
}

impl FromStr for TileRow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut traps = HashSet::new();

        for (index, c) in s.chars().enumerate() {
            if c == '^' {
                traps.insert(index);
            }
        }

        Ok(TileRow {
            traps,
            width: s.len(),
        })
    }
}

impl Display for TileRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.width {
            if self.traps.contains(&i) {
                write!(f, "^")?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_row_next() {
        let tile_row = TileRow::from_str("..^^.").unwrap();

        let next = tile_row.next();
        assert_eq!(next, TileRow::from_str(".^^^^").unwrap());

        let next = next.next();
        assert_eq!(next, TileRow::from_str("^^..^").unwrap());
    }
}
