use std::fmt::{Display, Formatter};
use std::iter::successors;
use std::ops::BitXor;
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let initial = TileRow::from_str(input)?;
    let number_of_safe_spaces = successors(Some(initial), |row| Some(row.next()))
        .take(40)
        .map(|row| row.number_of_safe_spaces())
        .sum::<usize>();
    Ok(number_of_safe_spaces)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let initial = TileRow::from_str(input)?;
    let number_of_safe_spaces = successors(Some(initial), |row| Some(row.next()))
        .take(400_000)
        .map(|row| row.number_of_safe_spaces())
        .sum::<usize>();
    Ok(number_of_safe_spaces)
}

enum Tile {
    Trap,
    Safe,
}

impl BitXor for Tile {
    type Output = Tile;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let lhs = matches!(self, Tile::Trap);
        let rhs = matches!(rhs, Tile::Trap);

        if lhs ^ rhs { Tile::Trap } else { Tile::Safe }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct TileRow {
    traps: Vec<bool>,
    width: usize,
}

impl TileRow {
    fn next(&self) -> Self {
        let mut traps = Vec::with_capacity(self.width);

        for i in 0..self.width {
            let left = self.left(i);
            let right = self.right(i);

            let xor = left ^ right;
            traps.push(matches!(xor, Tile::Trap));
        }

        Self {
            traps,
            width: self.width,
        }
    }

    fn left(&self, index: usize) -> Tile {
        if index != 0 && self.traps.get(index - 1).copied().unwrap() {
            Tile::Trap
        } else {
            Tile::Safe
        }
    }

    fn right(&self, index: usize) -> Tile {
        if index != self.width && self.traps.get(index + 1).copied().unwrap_or(false) {
            Tile::Trap
        } else {
            Tile::Safe
        }
    }

    fn number_of_safe_spaces(&self) -> usize {
        self.traps.iter().copied().filter(|b| !b).count()
    }
}

impl FromStr for TileRow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut traps = Vec::with_capacity(s.len());

        for c in s.chars() {
            traps.push(c == '^');
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
            if self.traps.get(i).copied().unwrap() {
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
