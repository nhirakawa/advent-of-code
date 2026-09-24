use anyhow::{anyhow, bail};
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let favorite_number = usize::from_str(input)?;
    bfs(Position(1, 1), Position(31, 39), favorite_number)
}
pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let favorite_number = usize::from_str(input)?;
    Ok(explore(Position(1, 1), favorite_number))
}

fn bfs(from: Position, to: Position, favorite_number: usize) -> anyhow::Result<usize> {
    let mut tile_cache = HashMap::new();
    let mut seen = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((from, 0));

    while let Some((current, distance)) = queue.pop_front() {
        if current == to {
            return Ok(distance);
        }

        if !seen.insert(current) {
            continue;
        }

        for adjacent in current.adjacent() {
            if seen.contains(&adjacent) {
                continue;
            }

            let tile = tile_cache
                .entry(adjacent)
                .or_insert(tile(adjacent, favorite_number));

            if *tile == Tile::Space {
                queue.push_back((adjacent, distance + 1));
            }
        }
    }

    bail!("No solution found")
}

fn explore(from: Position, favorite_number: usize) -> usize {
    let mut seen = HashSet::new();

    let mut tile_cache = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((from, 0));

    while let Some((current, distance)) = queue.pop_front() {
        if distance > 50 {
            continue;
        }

        if !seen.insert(current) {
            continue;
        }

        for adjacent in current.adjacent() {
            let tile = tile_cache
                .entry(adjacent)
                .or_insert_with(|| tile(adjacent, favorite_number));

            if *tile == Tile::Space {
                queue.push_back((adjacent, distance + 1));
            }
        }
    }

    seen.len()
}

fn tile(position: Position, favorite_number: usize) -> Tile {
    let x = position.0;
    let y = position.1;

    let value = x.pow(2) + (3 * x) + (2 * x * y) + y + y.pow(2);
    let value = value + favorite_number;

    if value.count_ones().is_multiple_of(2) {
        Tile::Space
    } else {
        Tile::Wall
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position(usize, usize);

impl Position {
    fn adjacent(&self) -> Vec<Self> {
        let mut adjacent = Vec::with_capacity(4);

        adjacent.push(Position(self.0 + 1, self.1));
        adjacent.push(Position(self.0, self.1 + 1));

        if self.0 > 0 {
            adjacent.push(Position(self.0 - 1, self.1));
        }

        if self.1 > 0 {
            adjacent.push(Position(self.0, self.1 - 1));
        }

        adjacent
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Space,
}
