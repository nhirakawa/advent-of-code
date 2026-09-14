use crate::common::parse::griderator;
use anyhow::{anyhow, bail};
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let map = Map::from_str(input)?;

    if map.keys.is_empty() {
        bail!("No keys");
    }

    if map.spaces.is_empty() {
        bail!("No spaces");
    }

    if map.doors.is_empty() {
        bail!("No doors");
    }

    let answer = search_map(map)?;
    Ok(answer)
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

fn search_map(map: Map) -> anyhow::Result<usize> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((map, 0));

    while let Some((map, steps)) = queue.pop_front() {
        let search_state = SearchState::new(map.current, &map.collected_keys)?;

        let all_keys: HashSet<char> = map.keys.values().copied().collect();
        if all_keys.is_subset(&map.collected_keys) {
            return Ok(steps);
        }

        if !seen.insert(search_state) {
            continue;
        }

        let (x, y) = map.current;

        for next_step in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if map.walls.contains(&next_step) {
                continue;
            } else if map.spaces.contains(&next_step) {
                // move to the available space
                let mut next_map = map.clone();
                next_map.current = next_step;
                queue.push_back((next_map, steps + 1));
            } else if let Some(key) = map.keys.get(&next_step) {
                // collect the key
                let mut next_map = map.clone();
                next_map.current = next_step;
                next_map.collected_keys.insert(*key);
                queue.push_back((next_map, steps + 1));
            } else if let Some(door) = map.doors.get(&next_step) {
                if map.collected_keys.contains(&door.to_ascii_lowercase()) {
                    // move through the doorway
                    let mut next_map = map.clone();
                    next_map.current = next_step;
                    queue.push_back((next_map, steps + 1));
                } else {
                    continue;
                }
            } else {
                bail!("Invalid move from {:?} to {next_step:?}", map.current);
            }
        }
    }

    bail!("No solution found")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SearchState {
    current: (isize, isize),
    collected_keys: u32,
}

impl SearchState {
    fn new(current: (isize, isize), collected_keys: &HashSet<char>) -> anyhow::Result<SearchState> {
        let mut collected_keys_bits = 0;

        for collected_key in collected_keys.iter().copied() {
            if ('a'..='z').contains(&collected_key) {
                let shift = collected_key as u8 - 'a' as u8;
                collected_keys_bits |= 1 << shift;
            } else {
                bail!("Invalid collected key: {}", collected_key);
            }
        }

        let collected_keys = collected_keys_bits;
        Ok(Self {
            current,
            collected_keys,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    current: (isize, isize),
    walls: HashSet<(isize, isize)>,
    keys: HashMap<(isize, isize), char>,
    doors: HashMap<(isize, isize), char>,
    collected_keys: HashSet<char>,
    spaces: HashSet<(isize, isize)>,
}

impl FromStr for Map {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut walls = HashSet::new();
        let mut keys = HashMap::new();
        let mut doors = HashMap::new();
        let collected_keys = HashSet::new();
        let mut spaces = HashSet::new();

        for ((x, y), c) in griderator(s) {
            match c {
                '#' => {
                    walls.insert((x, y));
                }
                '.' => {
                    spaces.insert((x, y));
                }
                '@' => {
                    if let Some((start_x, start_y)) = start {
                        bail!(
                            "found duplicate starting point (previous:({start_x},{start_y}), current:({x},{y})",
                        );
                    }
                    spaces.insert((x, y));
                    start = Some((x, y));
                }
                'a'..='z' => {
                    keys.insert((x, y), c);
                }
                'A'..='Z' => {
                    doors.insert((x, y), c);
                }
                _ => bail!("Invalid tile {c}"),
            };
        }

        if let Some(current) = start {
            Ok(Map {
                current,
                walls,
                keys,
                doors,
                collected_keys,
                spaces,
            })
        } else {
            bail!("No start found");
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct Keychain(u32);

impl Keychain {}

impl TryFrom<HashSet<char>> for Keychain {
    type Error = anyhow::Error;

    fn try_from(keys: HashSet<char>) -> Result<Self, Self::Error> {
        let mut keychain = 0;

        for key in keys {
            if ('a'..='z').contains(&key) {
                let shift = key as u8 - 'a' as u8;
                keychain |= 1 << shift;
            } else {
                bail!("Invalid key '{key}'");
            }
        }

        Ok(Keychain(keychain))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position((isize, isize));

impl Position {
    fn next(&self) -> [Position; 4] {
        let (x, y) = self.0;
        [
            (x + 1, y).into(),
            (x - 1, y).into(),
            (x, y + 1).into(),
            (x, y - 1).into(),
        ]
    }
}

impl From<(isize, isize)> for Position {
    fn from(value: (isize, isize)) -> Self {
        Self(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Space,
    Key(char),
    Door(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keychain_from_str() {
        let keychain =
            Keychain::try_from(['a', 's', 'd', 'f'].into_iter().collect::<HashSet<_>>()).unwrap();
        assert_eq!(keychain.0, 1 << 0 | 1 << 3 | 1 << 5 | 1 << 18);
    }
}
