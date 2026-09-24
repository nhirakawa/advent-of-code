use anyhow::anyhow;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let favorite_number = usize::from_str(input)?;
    let to = Position(31, 39);

    bfs(Position(1, 1), favorite_number)
        .find(|(position, _)| *position == to)
        .map(|(_, distance)| distance)
        .ok_or_else(|| anyhow!("No solution found"))
}
pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let favorite_number = usize::from_str(input)?;

    Ok(bfs(Position(1, 1), favorite_number)
        .take_while(|(_, distance)| *distance <= 50)
        .count())
}

/// Lazily visits every reachable open space in nondecreasing distance order.
fn bfs(from: Position, favorite_number: usize) -> Bfs {
    Bfs {
        favorite_number,
        seen: HashSet::from([from]),
        queue: VecDeque::from([(from, 0)]),
    }
}

struct Bfs {
    favorite_number: usize,
    seen: HashSet<Position>,
    queue: VecDeque<(Position, usize)>,
}

impl Iterator for Bfs {
    type Item = (Position, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (current, distance) = self.queue.pop_front()?;

        for adjacent in current.adjacent() {
            if tile(adjacent, self.favorite_number) == Tile::Space && self.seen.insert(adjacent) {
                self.queue.push_back((adjacent, distance + 1));
            }
        }

        Some((current, distance))
    }
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
