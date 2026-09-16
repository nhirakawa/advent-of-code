use crate::common::parse::griderator;
use anyhow::anyhow;
use std::collections::HashSet;
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let _grid = Grid::from_str(input)?;
    Err::<usize, _>(anyhow!("Not implemented"))
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position([isize; 2]);

impl Position {
    fn adjacent(&self) -> [Position; 4] {
        let [x, y] = self.0;
        [
            Position([x + 1, y]),
            Position([x - 1, y]),
            Position([x, y + 1]),
            Position([x, y - 1]),
        ]
    }
}

impl From<(isize, isize)> for Position {
    fn from((x, y): (isize, isize)) -> Self {
        Self([x, y])
    }
}

struct Grid {
    // TODO replace with bit vector?
    grid: HashSet<Position>,
}

impl Grid {
    fn new<G: IntoIterator<Item = Position>>(grid: G) -> Self {
        let grid = grid.into_iter().collect();
        Self { grid }
    }

    fn adjacent_count(&self, position: &Position) -> usize {
        position
            .adjacent()
            .iter()
            .filter(|p| self.grid.contains(*p))
            .count()
    }

    fn tick(&self) -> Self {
        let mut grid = HashSet::new();

        for x in 0..5 {
            for y in 0..5 {
                let position = Position([x, y]);
                let adjacent_count = self.adjacent_count(&position);
                if self.grid.contains(&position) && adjacent_count == 1 {
                    // bug lives
                    grid.insert(position);
                } else if !self.grid.contains(&position)
                    && (adjacent_count == 1 || adjacent_count == 2)
                {
                    // bug spawns
                    grid.insert(position);
                }
            }
        }

        Self { grid }
    }

    #[cfg(test)]
    fn contains(&self, position: &Position) -> bool {
        self.grid.contains(position)
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = HashSet::new();
        for (position, c) in griderator(s) {
            if c == '#' {
                grid.insert(position.into());
            }
        }
        Ok(Self { grid })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_adjacent_count() {
        let grid = Grid::new([
            (4, 0).into(),
            (0, 1).into(),
            (3, 1).into(),
            (0, 2).into(),
            (3, 2).into(),
            (4, 2).into(),
            (2, 3).into(),
            (0, 4).into(),
        ]);

        assert_eq!(grid.adjacent_count(&(0, 0).into()), 1);
        assert_eq!(grid.adjacent_count(&(4, 0).into()), 0);
        assert_eq!(grid.adjacent_count(&(3, 2).into()), 2);
    }

    #[test]
    fn test_grid_tick() {
        let grid = Grid::new([
            (4, 0).into(),
            (0, 1).into(),
            (3, 1).into(),
            (0, 2).into(),
            (3, 2).into(),
            (4, 2).into(),
            (2, 3).into(),
            (0, 4).into(),
        ]);

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
}
