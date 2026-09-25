use anyhow::{anyhow, bail};
use itertools::Itertools;
use md5::Digest;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    bfs(input)
}
pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

fn bfs(passcode: &str) -> anyhow::Result<String> {
    let mut queue = VecDeque::new();
    queue.push_back(Path::from_passcode(passcode));

    while let Some(current) = queue.pop_front() {
        if current.position == Position(3, 3) {
            return Ok(current.directions.iter().join(""));
        }

        let hash = md5::compute(current.to_string());
        let locks = Locks(hash);

        for direction in Direction::values() {
            if let Some(next_position) = current.position.try_move(direction)
                && locks.is_unlocked(direction)
            {
                let mut directions = current.directions.clone();
                directions.push(direction);
                let path = Path {
                    passcode: passcode.to_owned(),
                    position: next_position,
                    directions,
                };
                queue.push_back(path);
            }
        }
    }

    bail!("No solution found")
}

/// true represents a locked door; false represents an unlocked door
#[derive(Debug)]
struct Locks(Digest);

impl Locks {
    fn is_unlocked(&self, direction: Direction) -> bool {
        let index = match direction {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        };
        (b'b'..=b'f').contains(&self.0[index])
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn values() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Hash)]
struct Position(usize, usize);

impl Position {
    fn try_move(&self, direction: Direction) -> Option<Self> {
        match (self.0, self.1, direction) {
            (0, _, Direction::Left) => None,
            (3, _, Direction::Right) => None,
            (_, 0, Direction::Up) => None,
            (_, 3, Direction::Down) => None,
            (x, y, Direction::Left) => Some(Self(x - 1, y)),
            (x, y, Direction::Right) => Some(Self(x + 1, y)),
            (x, y, Direction::Up) => Some(Self(x, y - 1)),
            (x, y, Direction::Down) => Some(Self(x, y + 1)),
        }
    }
}

#[derive(Debug, Default)]
struct Path {
    passcode: String,
    position: Position,
    directions: Vec<Direction>,
}

impl Path {
    fn from_passcode<S: ToString>(passcode: S) -> Self {
        let passcode = passcode.to_string();
        Path {
            passcode,
            ..Default::default()
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let passcode = &self.passcode;
        let directions = self.directions.iter().map(Direction::to_string).join("");
        write!(f, "{passcode}{directions}")
    }
}
