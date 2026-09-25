use anyhow::bail;
use itertools::Itertools;
use md5::Digest;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    bfs(input, Mode::Shortest)
}
pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    bfs(input, Mode::Longest).map(|s| s.len())
}

enum Mode {
    Shortest,
    Longest,
}

fn bfs(passcode: &str, mode: Mode) -> anyhow::Result<String> {
    let mut queue = VecDeque::new();
    queue.push_back(Path::from_passcode(passcode));

    let mut longest_path = String::new();

    while let Some(current) = queue.pop_front() {
        if current.position == Position(3, 3) {
            match mode {
                Mode::Shortest => {
                    return Ok(current.directions.iter().join(""));
                }
                Mode::Longest => {
                    if current.directions.len() > longest_path.len() {
                        longest_path = current.directions.iter().join("");
                    }
                    continue;
                }
            }
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

    if matches!(mode, Mode::Longest) {
        Ok(longest_path)
    } else {
        bail!("No solution found")
    }
}

/// The first four hex digits of the MD5 hash determine the up, down, left, and right doors;
/// a digit of b-f means the door is open
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
        let byte = self.0[index / 2];
        let nibble = if index % 2 == 0 {
            byte >> 4
        } else {
            byte & 0x0f
        };
        nibble >= 0xb
    }

    #[cfg(test)]
    fn is_locked(&self, direction: Direction) -> bool {
        !self.is_unlocked(direction)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locks_is_unlocked() {
        let digest = md5::compute("hijkl");
        let locks = Locks(digest);
        assert!(locks.is_unlocked(Direction::Up));
        assert!(locks.is_unlocked(Direction::Down));
        assert!(locks.is_unlocked(Direction::Left));
        assert!(locks.is_locked(Direction::Right));
    }
}
