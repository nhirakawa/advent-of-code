use crate::common::parse::griderator;
use anyhow::{anyhow, bail};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let grid = Grid::from_str(input)?;

    let mut number_graph = HashMap::new();

    for number in grid.numbers_to_positions.values().copied() {
        number_graph.insert(number, bfs(number, &grid));
    }

    let start = grid
        .numbers_to_positions
        .get(&0)
        .copied()
        .ok_or(anyhow!("Could not find start"))?;

    let mut queue = VecDeque::from([(start, DigitSet::default(), 0)]);
    let mut seen = HashSet::new();

    while let Some((current, digits, steps)) = queue.pop_front() {
        if !seen.insert((current, digits)) {
            continue;
        }

        if grid.digits == digits {
            return Ok(steps);
        }

        if let Some(neighbors) = number_graph.get(&current) {
            for (neighbor, number, distance) in neighbors.iter().copied() {
                if digits.contains(number) {
                    // we've already collected this number - skip it
                    continue;
                }

                let mut new_digits = digits.clone();
                new_digits.insert(number);

                // we've already been to this neighbor - may be irrelevant with the check above
                if seen.contains(&(neighbor, new_digits)) {
                    continue;
                }

                queue.push_back((neighbor, new_digits, steps + distance));
            }
        }
    }

    bail!("No solution found")
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

/// Finds the shortest path from `from` to every other number in the grid
fn bfs(from: Position, grid: &Grid) -> Vec<(Position, u8, u32)> {
    let mut queue = VecDeque::from([(from, 0)]);
    let mut seen = HashSet::new();

    let mut distances = Vec::new();

    while let Some((current, steps)) = queue.pop_front() {
        if !seen.insert(current) {
            continue;
        }

        if let Some(Tile::Number(number)) = grid.tiles.get(&current).copied() {
            distances.push((current, number, steps));
        }

        for neighbor in grid.neighbors(current) {
            if seen.contains(&neighbor) {
                continue;
            }

            queue.push_back((neighbor, steps + 1));
        }
    }

    distances
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position(i64, i64);

impl From<(i64, i64)> for Position {
    fn from(value: (i64, i64)) -> Self {
        Self(value.0, value.1)
    }
}

impl TryFrom<(isize, isize)> for Position {
    type Error = anyhow::Error;

    fn try_from((x, y): (isize, isize)) -> Result<Self, Self::Error> {
        let x = x.try_into()?;
        let y = y.try_into()?;
        Ok(Self(x, y))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct DigitSet([bool; 10]);

impl DigitSet {
    fn insert(&mut self, digit: u8) {
        if digit > 9 {
            return;
        }
        self.0[digit as usize] = true;
    }

    fn contains(&self, digit: u8) -> bool {
        if digit > 9 {
            false
        } else {
            self.0[digit as usize]
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Space,
    Wall,
    Number(u8),
}

struct Grid {
    tiles: HashMap<Position, Tile>,
    numbers_to_positions: HashMap<u8, Position>,
    positions_to_numbers: HashMap<Position, u8>,
    digits: DigitSet,
}

impl Grid {
    fn neighbors(&self, position: Position) -> Vec<Position> {
        let x = position.0;
        let y = position.1;

        let mut positions = Vec::new();

        for potential in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            let potential = potential.into();

            if let Some(tile) = self.tiles.get(&potential).copied() {
                if matches!(tile, Tile::Wall) {
                    continue;
                }
                positions.push(potential);
            }
        }

        positions
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = HashMap::new();
        let mut numbers_to_positions = HashMap::new();
        let mut positions_to_numbers = HashMap::new();
        let mut digits = DigitSet::default();
        for (position, c) in griderator(s) {
            let position = position.try_into()?;
            match c {
                '.' => {
                    tiles.insert(position, Tile::Space);
                }
                '#' => {
                    tiles.insert(position, Tile::Wall);
                }
                '0'..='9' => {
                    let number = c as u8 - '0' as u8;
                    tiles.insert(position, Tile::Number(number));
                    numbers_to_positions.insert(number, position);
                    positions_to_numbers.insert(position, number);
                    digits.insert(number);
                }
                _ => bail!("Invalid character: '{c}'"),
            }
        }
        Ok(Self {
            tiles,
            numbers_to_positions,
            positions_to_numbers,
            digits,
        })
    }
}
