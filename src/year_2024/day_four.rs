use std::collections::HashMap;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let chars = parse_characters(input);

    let mut count = 0;

    for (position, character) in &chars {
        if *character == Char::X {
            for direction in Direction::values() {
                if is_valid_xmas(position, &direction, &chars) {
                    count += 1;
                }
            }
        }
    }

    Ok(count.to_string())
}

fn is_valid_xmas(
    current: &Position,
    direction: &Direction,
    chars: &HashMap<Position, Char>,
) -> bool {
    if !chars.contains_key(current) {
        return false;
    }

    let current_char = chars.get(current).unwrap();

    if current_char.next().is_none() {
        // If the current character is 'S', we can't move anymore
        return true;
    }

    let next_position = current.advance(direction);
    if !chars.contains_key(&next_position) {
        // If the next position is not in the map, we can't move
        return false;
    }

    let next_char = chars.get(&next_position).unwrap();

    if current_char.next().unwrap() != *next_char {
        // If the next character is not the expected one, we can't move
        return false;
    }

    is_valid_xmas(&next_position, direction, chars)
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let chars = parse_characters(input);

    let mut count = 0;

    for (position, character) in &chars {
        if *character == Char::A && is_valid_x_mas(position, &chars) {
            count += 1;
        }
    }

    Ok(count.to_string())
}

fn is_valid_x_mas(current: &Position, chars: &HashMap<Position, Char>) -> bool {
    let northwest = current.advance(&Direction::NorthWest);
    let northwest = chars.get(&northwest).copied().unwrap_or(Char::Unknown);

    let northeast = current.advance(&Direction::NorthEast);
    let northeast = chars.get(&northeast).copied().unwrap_or(Char::Unknown);

    let southwest = current.advance(&Direction::SouthWest);
    let southwest = chars.get(&southwest).copied().unwrap_or(Char::Unknown);

    let southeast = current.advance(&Direction::SouthEast);
    let southeast = chars.get(&southeast).copied().unwrap_or(Char::Unknown);

    matches!(
        (northwest, northeast, southwest, southeast),
        (Char::M, Char::M, Char::S, Char::S)
            | (Char::S, Char::M, Char::S, Char::M)
            | (Char::S, Char::S, Char::M, Char::M)
            | (Char::M, Char::S, Char::M, Char::S)
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn values() -> impl IntoIterator<Item = Direction> {
        [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ]
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn advance(&self, direction: &Direction) -> Position {
        match direction {
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::NorthEast => Position {
                x: self.x + 1,
                y: self.y - 1,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::SouthEast => Position {
                x: self.x + 1,
                y: self.y + 1,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::SouthWest => Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::NorthWest => Position {
                x: self.x - 1,
                y: self.y - 1,
            },
        }
    }
}

impl From<(isize, isize)> for Position {
    fn from(value: (isize, isize)) -> Self {
        let (x, y) = value;
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Char {
    Unknown,
    X,
    M,
    A,
    S,
}

impl Char {
    fn next(&self) -> Option<Char> {
        match self {
            Char::X => Some(Char::M),
            Char::M => Some(Char::A),
            Char::A => Some(Char::S),
            Char::S => None,
            Char::Unknown => None,
        }
    }
}

fn parse_characters(i: &str) -> HashMap<Position, Char> {
    let mut result = HashMap::new();

    for (y, lines) in i.trim().lines().enumerate() {
        for (x, c) in lines.chars().enumerate() {
            let position = Position::from((x as isize, y as isize));
            if let Some(character) = match c {
                'X' => Some(Char::X),
                'M' => Some(Char::M),
                'A' => Some(Char::A),
                'S' => Some(Char::S),
                _ => None,
            } {
                result.insert(position, character);
            }
        }
    }

    result
}
