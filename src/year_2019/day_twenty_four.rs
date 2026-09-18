use crate::common::parse::griderator;
use anyhow::{anyhow, bail};
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::successors;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let grid = Grid::new(positions_from_str(input), Recursion::No);

    let mut iterations = 0;
    let mut seen = HashSet::new();

    for grid in successors(Some(grid), |grid| Some(grid.tick())) {
        if iterations >= 10_000 {
            break;
        }

        iterations += 1;

        if !seen.insert(grid.biodiversity()) {
            return Ok(grid.biodiversity());
        }
    }

    bail!("No solution found")
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let grid = Grid::new(positions_from_str(input), Recursion::Yes(0));

    successors(Some(grid), |grid| Some(grid.tick()))
        .nth(200)
        .map(|grid| grid.grid.len())
        .ok_or(anyhow!("No solution found"))
}

trait Cell: Eq + Hash + Copy + Clone {
    fn adjacent(&self) -> Vec<Self>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position([isize; 2]);

impl Position {
    #[cfg(test)]
    fn new(x: isize, y: isize) -> Self {
        Self([x, y])
    }

    fn as_bits(&self) -> u32 {
        // Each octet represents a row (with 3 unused bits)
        // Each bit in an octet represents a column in that row (with 3 unused bits)
        // Representation is little-endian
        1 << ((self.0[1] * 5) + self.0[0])
    }
}

impl From<(isize, isize)> for Position {
    fn from((x, y): (isize, isize)) -> Self {
        Self([x, y])
    }
}

impl Cell for Position {
    fn adjacent(&self) -> Vec<Self> {
        let [x, y] = self.0;
        vec![
            Position([x + 1, y]),
            Position([x - 1, y]),
            Position([x, y + 1]),
            Position([x, y - 1]),
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Recursion {
    No,
    Yes(isize),
}
struct Grid {
    grid: HashSet<Position>,
    recursion: Recursion,
}

impl Grid {
    fn new<G: IntoIterator<Item = Position>>(grid: G, recursion: Recursion) -> Self {
        Self {
            grid: grid.into_iter().collect(),
            recursion,
        }
    }

    fn adjacent_count(&self, position: &Position) -> usize {
        position
            .adjacent()
            .iter()
            .copied()
            .filter(|p| self.contains(p))
            .count()
    }

    fn contains(&self, position: &Position) -> bool {
        let [x, y] = position.0;
        if !(0..5).contains(&x) || !(0..5).contains(&y) {
            false
        } else {
            self.grid.contains(position)
        }
    }

    fn tick(&self) -> Self {
        let mut grid = HashSet::new();

        for x in 0..5 {
            for y in 0..5 {
                let position = Position([x, y]);
                let adjacent_count = self.adjacent_count(&position);
                if self.contains(&position) && adjacent_count == 1 {
                    // bug lives
                    grid.insert(position);
                } else if !self.contains(&position) && (adjacent_count == 1 || adjacent_count == 2)
                {
                    // bug spawns
                    grid.insert(position);
                }
            }
        }

        Self {
            grid,
            recursion: self.recursion,
        }
    }

    fn biodiversity(&self) -> u32 {
        self.grid.iter().map(Position::as_bits).sum()
    }
}

fn positions_from_str(s: &str) -> impl Iterator<Item = Position> + '_ {
    griderator(s)
        .filter(|(_, c)| *c == '#')
        .map(|(position, _)| Position::from(position))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_as_bits() {
        let position = Position::new(1, 3);
        assert_eq!(position.as_bits(), 1 << 8);

        let position = Position::new(4, 4);
        assert_eq!(position.as_bits(), 1 << 24);
    }

    #[test]
    fn test_grid_adjacent_count() {
        let grid = Grid::new(
            [
                (4, 0).into(),
                (0, 1).into(),
                (3, 1).into(),
                (0, 2).into(),
                (3, 2).into(),
                (4, 2).into(),
                (2, 3).into(),
                (0, 4).into(),
            ],
            Recursion::No,
        );

        assert_eq!(grid.adjacent_count(&(0, 0).into()), 1);
        assert_eq!(grid.adjacent_count(&(4, 0).into()), 0);
        assert_eq!(grid.adjacent_count(&(3, 2).into()), 2);
    }

    #[test]
    fn test_grid_tick() {
        let grid = Grid::new(
            [
                (4, 0).into(),
                (0, 1).into(),
                (3, 1).into(),
                (0, 2).into(),
                (3, 2).into(),
                (4, 2).into(),
                (2, 3).into(),
                (0, 4).into(),
            ],
            Recursion::No,
        );

        let grid = grid.tick();
        assert!(grid.contains(&(0, 0).into()));
        assert!(grid.contains(&(3, 0).into()));

        assert!(grid.contains(&(0, 1).into()));
        assert!(grid.contains(&(1, 1,).into()));
        assert!(grid.contains(&(2, 1).into()));
        assert!(grid.contains(&(3, 1).into()));

        assert!(grid.contains(&(0, 2).into()));
        assert!(grid.contains(&(1, 2).into()));
        assert!(grid.contains(&(2, 2).into()));
        assert!(grid.contains(&(4, 2).into()));

        assert!(grid.contains(&(0, 3).into()));
        assert!(grid.contains(&(1, 3).into()));
        assert!(grid.contains(&(3, 3).into()));
        assert!(grid.contains(&(4, 3).into()));

        assert!(grid.contains(&(1, 4).into()));
        assert!(grid.contains(&(2, 4).into()));
    }

    #[test]
    fn test_grid_biodiversity() {
        let grid = Grid::new([(0, 3).into(), (1, 4).into()], Recursion::No);
        assert_eq!(grid.biodiversity(), 2129920);
    }
}
