use std::fmt::Write;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryFrom,
    fmt::Display,
    iter::once,
};

use anyhow::{anyhow, bail};
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let door_codes = parse(input)?;

    let mut sum = 0;

    let mut memo = HashMap::new();

    for door_code in door_codes {
        let mut min_len_sum = 0;

        for (from, to) in door_code.into_iter().tuple_windows() {
            let shortest_directional_paths = from.shortest_directional_paths_to(to);

            let mut min_len = usize::MAX;

            for shortest_directional_path in shortest_directional_paths {
                let shortest_path_len = find_shortest_path_len_directional_keypad(
                    once(DirectionalKeypad::Activate).chain(shortest_directional_path),
                    0,
                    2,
                    &mut memo,
                );

                min_len = min_len.min(shortest_path_len);
            }

            min_len_sum += min_len;
        }

        sum += door_code.value * min_len_sum;
    }

    Ok(sum)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let door_codes = parse(input)?;

    let mut sum = 0;

    let mut memo = HashMap::new();

    for door_code in door_codes {
        let mut min_len_sum = 0;

        for (from, to) in door_code.into_iter().tuple_windows() {
            let shortest_directional_paths = from.shortest_directional_paths_to(to);

            let mut min_len = usize::MAX;

            for shortest_directional_path in shortest_directional_paths {
                let shortest_path_len = find_shortest_path_len_directional_keypad(
                    once(DirectionalKeypad::Activate).chain(shortest_directional_path),
                    0,
                    25,
                    &mut memo,
                );

                min_len = min_len.min(shortest_path_len);
            }

            min_len_sum += min_len;
        }

        sum += door_code.value * min_len_sum;
    }

    Ok(sum)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct MemoKey {
    from: DirectionalKeypad,
    to: DirectionalKeypad,
    index: usize,
}

impl MemoKey {
    fn new(from: DirectionalKeypad, to: DirectionalKeypad, index: usize) -> Self {
        Self { from, to, index }
    }
}

fn find_shortest_path_len_directional_keypad<K: IntoIterator<Item = DirectionalKeypad>>(
    keys: K,
    index: usize,
    max_depth: usize,
    memo: &mut HashMap<MemoKey, usize>,
) -> usize {
    if index == max_depth {
        let keys = keys.into_iter().collect_vec();

        return keys.len() - 1;
    }

    let mut min_len_sum = 0;

    for (from, to) in keys.into_iter().tuple_windows() {
        let memo_state = MemoKey::new(from, to, index);

        let min_len = if let Some(min_len) = memo.get(&memo_state).copied() {
            min_len
        } else {
            let shortest_directional_paths = from.shortest_directional_paths_to(&to);

            let mut min_len = usize::MAX;

            for shortest_directional_path in shortest_directional_paths {
                let shortest_directional_path =
                    once(DirectionalKeypad::Activate).chain(shortest_directional_path);

                let path_len = find_shortest_path_len_directional_keypad(
                    shortest_directional_path,
                    index + 1,
                    max_depth,
                    memo,
                );

                min_len = min_len.min(path_len);
            }

            memo.insert(memo_state, min_len);

            min_len
        };

        min_len_sum += min_len;
    }

    min_len_sum
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn values() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum NumericKeypad {
    Seven,
    Eight,
    Nine,
    Four,
    Five,
    Six,
    One,
    Two,
    Three,
    Zero,
    Activate,
}

impl NumericKeypad {
    fn move_to(&self, direction: &Direction) -> Option<NumericKeypad> {
        match (self, direction) {
            (NumericKeypad::Seven, Direction::Up) => None,
            (NumericKeypad::Seven, Direction::Right) => Some(NumericKeypad::Eight),
            (NumericKeypad::Seven, Direction::Down) => Some(NumericKeypad::Four),
            (NumericKeypad::Seven, Direction::Left) => None,
            (NumericKeypad::Eight, Direction::Up) => None,
            (NumericKeypad::Eight, Direction::Right) => Some(NumericKeypad::Nine),
            (NumericKeypad::Eight, Direction::Down) => Some(NumericKeypad::Five),
            (NumericKeypad::Eight, Direction::Left) => Some(NumericKeypad::Seven),
            (NumericKeypad::Nine, Direction::Up) => None,
            (NumericKeypad::Nine, Direction::Right) => None,
            (NumericKeypad::Nine, Direction::Down) => Some(NumericKeypad::Six),
            (NumericKeypad::Nine, Direction::Left) => Some(NumericKeypad::Eight),
            (NumericKeypad::Four, Direction::Up) => Some(NumericKeypad::Seven),
            (NumericKeypad::Four, Direction::Right) => Some(NumericKeypad::Five),
            (NumericKeypad::Four, Direction::Down) => Some(NumericKeypad::One),
            (NumericKeypad::Four, Direction::Left) => None,
            (NumericKeypad::Five, Direction::Up) => Some(NumericKeypad::Eight),
            (NumericKeypad::Five, Direction::Right) => Some(NumericKeypad::Six),
            (NumericKeypad::Five, Direction::Down) => Some(NumericKeypad::Two),
            (NumericKeypad::Five, Direction::Left) => Some(NumericKeypad::Four),
            (NumericKeypad::Six, Direction::Up) => Some(NumericKeypad::Nine),
            (NumericKeypad::Six, Direction::Right) => None,
            (NumericKeypad::Six, Direction::Down) => Some(NumericKeypad::Three),
            (NumericKeypad::Six, Direction::Left) => Some(NumericKeypad::Five),
            (NumericKeypad::One, Direction::Up) => Some(NumericKeypad::Four),
            (NumericKeypad::One, Direction::Right) => Some(NumericKeypad::Two),
            (NumericKeypad::One, Direction::Down) => None,
            (NumericKeypad::One, Direction::Left) => None,
            (NumericKeypad::Two, Direction::Up) => Some(NumericKeypad::Five),
            (NumericKeypad::Two, Direction::Right) => Some(NumericKeypad::Three),
            (NumericKeypad::Two, Direction::Down) => Some(NumericKeypad::Zero),
            (NumericKeypad::Two, Direction::Left) => Some(NumericKeypad::One),
            (NumericKeypad::Three, Direction::Up) => Some(NumericKeypad::Six),
            (NumericKeypad::Three, Direction::Right) => None,
            (NumericKeypad::Three, Direction::Down) => Some(NumericKeypad::Activate),
            (NumericKeypad::Three, Direction::Left) => Some(NumericKeypad::Two),
            (NumericKeypad::Zero, Direction::Up) => Some(NumericKeypad::Two),
            (NumericKeypad::Zero, Direction::Right) => Some(NumericKeypad::Activate),
            (NumericKeypad::Zero, Direction::Down) => None,
            (NumericKeypad::Zero, Direction::Left) => None,
            (NumericKeypad::Activate, Direction::Up) => Some(NumericKeypad::Three),
            (NumericKeypad::Activate, Direction::Right) => None,
            (NumericKeypad::Activate, Direction::Down) => None,
            (NumericKeypad::Activate, Direction::Left) => Some(NumericKeypad::Zero),
        }
    }

    fn shortest_directional_paths_to(&self, other: &Self) -> Vec<Vec<DirectionalKeypad>> {
        if self == other {
            return vec![vec![DirectionalKeypad::Activate]];
        }

        let mut queue = VecDeque::new();

        queue.push_back((0, *self, vec![]));

        let mut minimum_distance = usize::MAX;
        let mut shortest_paths = vec![];

        let mut seen = HashSet::new();

        while let Some((distance, key, directions)) = queue.pop_front() {
            if !seen.insert((key, directions.clone())) {
                continue;
            }

            if distance > minimum_distance {
                continue;
            }

            if key == *other {
                // If current distance is less than the minimum distance we've found so far, clear the state
                if distance < minimum_distance {
                    minimum_distance = distance;
                    shortest_paths.clear();
                }

                if distance == minimum_distance {
                    shortest_paths.push(
                        directions
                            .clone()
                            .into_iter()
                            .map(|direction: Direction| direction.into())
                            .chain(once(DirectionalKeypad::Activate))
                            .collect_vec(),
                    );
                }
            }

            for next_direction in Direction::values() {
                if let Some(next_key) = key.move_to(&next_direction) {
                    let next_distance = distance + 1;
                    let mut next_directions = directions.clone();
                    next_directions.push(next_direction);

                    queue.push_back((next_distance, next_key, next_directions));
                }
            }
        }

        shortest_paths
    }
}

impl TryFrom<char> for NumericKeypad {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '7' => Ok(NumericKeypad::Seven),
            '8' => Ok(NumericKeypad::Eight),
            '9' => Ok(NumericKeypad::Nine),
            '4' => Ok(NumericKeypad::Four),
            '5' => Ok(NumericKeypad::Five),
            '6' => Ok(NumericKeypad::Six),
            '1' => Ok(NumericKeypad::One),
            '2' => Ok(NumericKeypad::Two),
            '3' => Ok(NumericKeypad::Three),
            '0' => Ok(NumericKeypad::Zero),
            'A' => Ok(NumericKeypad::Activate),
            _ => bail!("Invalid character: '{value}'"),
        }
    }
}

impl Display for NumericKeypad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = match self {
            NumericKeypad::Seven => "7",
            NumericKeypad::Eight => "8",
            NumericKeypad::Nine => "9",
            NumericKeypad::Four => "4",
            NumericKeypad::Five => "5",
            NumericKeypad::Six => "6",
            NumericKeypad::One => "1",
            NumericKeypad::Two => "2",
            NumericKeypad::Three => "3",
            NumericKeypad::Zero => "0",
            NumericKeypad::Activate => "A",
        };

        write!(f, "{key}")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum DirectionalKeypad {
    Up,
    Activate,
    Left,
    Down,
    Right,
}

impl DirectionalKeypad {
    fn move_to(&self, direction: &Direction) -> Option<DirectionalKeypad> {
        match (self, direction) {
            (DirectionalKeypad::Up, Direction::Up) => None,
            (DirectionalKeypad::Up, Direction::Right) => Some(DirectionalKeypad::Activate),
            (DirectionalKeypad::Up, Direction::Down) => Some(DirectionalKeypad::Down),
            (DirectionalKeypad::Up, Direction::Left) => None,
            (DirectionalKeypad::Activate, Direction::Up) => None,
            (DirectionalKeypad::Activate, Direction::Right) => None,
            (DirectionalKeypad::Activate, Direction::Down) => Some(DirectionalKeypad::Right),
            (DirectionalKeypad::Activate, Direction::Left) => Some(DirectionalKeypad::Up),
            (DirectionalKeypad::Left, Direction::Up) => None,
            (DirectionalKeypad::Left, Direction::Right) => Some(DirectionalKeypad::Down),
            (DirectionalKeypad::Left, Direction::Down) => None,
            (DirectionalKeypad::Left, Direction::Left) => None,
            (DirectionalKeypad::Down, Direction::Up) => Some(DirectionalKeypad::Up),
            (DirectionalKeypad::Down, Direction::Right) => Some(DirectionalKeypad::Right),
            (DirectionalKeypad::Down, Direction::Down) => None,
            (DirectionalKeypad::Down, Direction::Left) => Some(DirectionalKeypad::Left),
            (DirectionalKeypad::Right, Direction::Up) => Some(DirectionalKeypad::Activate),
            (DirectionalKeypad::Right, Direction::Right) => None,
            (DirectionalKeypad::Right, Direction::Down) => None,
            (DirectionalKeypad::Right, Direction::Left) => Some(DirectionalKeypad::Down),
        }
    }

    fn shortest_directional_paths_to(&self, other: &Self) -> Vec<Vec<DirectionalKeypad>> {
        if self == other {
            return vec![vec![DirectionalKeypad::Activate]];
        }

        let mut queue = VecDeque::new();

        queue.push_back((0, *self, vec![]));

        let mut minimum_distance = usize::MAX;

        let mut shortest_paths = vec![];

        let mut seen = HashSet::new();

        while let Some((distance, key, directions)) = queue.pop_front() {
            if !seen.insert((key, directions.clone())) {
                continue;
            }

            if distance > minimum_distance {
                continue;
            }

            if key == *other {
                // If current distance is less than the minimum distance we've found so far, clear the state
                if distance < minimum_distance {
                    minimum_distance = distance;
                    shortest_paths.clear();
                }

                if distance == minimum_distance {
                    shortest_paths.push(
                        directions
                            .clone()
                            .into_iter()
                            .map(|direction: Direction| direction.into())
                            .chain(once(DirectionalKeypad::Activate))
                            .collect_vec(),
                    );
                }
            }

            for next_direction in Direction::values() {
                if let Some(next_key) = key.move_to(&next_direction) {
                    let next_distance = distance + 1;
                    let mut next_directions = directions.clone();
                    next_directions.push(next_direction);

                    queue.push_back((next_distance, next_key, next_directions));
                }
            }
        }

        if shortest_paths.is_empty() {
            vec![vec![DirectionalKeypad::Activate]]
        } else {
            shortest_paths
        }
    }
}

impl From<Direction> for DirectionalKeypad {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => DirectionalKeypad::Up,
            Direction::Right => DirectionalKeypad::Right,
            Direction::Down => DirectionalKeypad::Down,
            Direction::Left => DirectionalKeypad::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct DoorCode {
    keys: Vec<NumericKeypad>,
    value: usize,
}

impl DoorCode {
    pub fn new(keys: Vec<NumericKeypad>, value: usize) -> Self {
        Self { keys, value }
    }
}

impl<'a> IntoIterator for &'a DoorCode {
    type Item = &'a NumericKeypad;
    type IntoIter = std::slice::Iter<'a, NumericKeypad>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.iter()
    }
}

impl Display for DoorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key_string = self.keys.iter().fold(String::new(), |mut key_string, key| {
            write!(key_string, "{key}").unwrap();
            key_string
        });

        write!(f, "{key_string}")
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<DoorCode>> {
    let mut door_codes = Vec::new();

    for line in input.lines() {
        if !line.ends_with("A") {
            bail!("Line '{line}' does not end with 'A'");
        }

        let value = line
            .strip_suffix("A")
            .ok_or(anyhow!("Could not strip 'A' from line '{line}'"))?
            .parse()?;

        let keys = line
            .chars()
            .map(NumericKeypad::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let keys = once(NumericKeypad::Activate).chain(keys).collect_vec();

        door_codes.push(DoorCode::new(keys, value));
    }

    Ok(door_codes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_numeric_keypad() {
        let key = NumericKeypad::Activate;
        let shortest_paths = key.shortest_directional_paths_to(&NumericKeypad::Zero);

        assert_eq!(shortest_paths.len(), 1);
        assert_eq!(
            shortest_paths[0],
            vec![DirectionalKeypad::Left, DirectionalKeypad::Activate]
        );

        let key = NumericKeypad::Zero;
        let shortest_paths = key.shortest_directional_paths_to(&NumericKeypad::Two);

        assert_eq!(shortest_paths.len(), 1);
        assert_eq!(
            shortest_paths[0],
            vec![DirectionalKeypad::Up, DirectionalKeypad::Activate]
        );

        let key = NumericKeypad::Two;
        let shortest_paths = key.shortest_directional_paths_to(&NumericKeypad::Nine);

        assert_eq!(shortest_paths.len(), 3);
        assert!(shortest_paths.contains(&vec![
            DirectionalKeypad::Right,
            DirectionalKeypad::Up,
            DirectionalKeypad::Up,
            DirectionalKeypad::Activate
        ]));
        assert!(shortest_paths.contains(&vec![
            DirectionalKeypad::Up,
            DirectionalKeypad::Right,
            DirectionalKeypad::Up,
            DirectionalKeypad::Activate
        ]));
        assert!(shortest_paths.contains(&vec![
            DirectionalKeypad::Up,
            DirectionalKeypad::Up,
            DirectionalKeypad::Right,
            DirectionalKeypad::Activate
        ]));
    }
}
