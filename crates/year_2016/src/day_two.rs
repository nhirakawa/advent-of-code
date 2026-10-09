use std::fmt::Display;

use anyhow::anyhow;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, value},
    multi::{many1, separated_list1},
    Finish, IResult, Parser,
};
use nom_language::error::{convert_error, VerboseError};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let directions_list = parse(input)?;

    let mut code = vec![];

    for directions in directions_list {
        let mut key = PartOneKey::Five;

        for direction in directions {
            key = key.next(&direction);
        }

        code.push(key);
    }

    Ok(code.into_iter().map(|key| key.to_string()).join(""))
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let directions_list = parse(input)?;

    let mut code = vec![];

    for directions in directions_list {
        let mut key = PartTwoKey::Five;

        for direction in directions {
            key = key.next(&direction);
        }

        code.push(key);
    }

    Ok(code.into_iter().map(|key| key.to_string()).join(""))
}

#[derive(Debug, Copy, Clone)]
enum PartOneKey {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl PartOneKey {
    fn next(&self, direction: &Direction) -> PartOneKey {
        match (self, direction) {
            (PartOneKey::One, Direction::Down) => PartOneKey::Four,
            (PartOneKey::One, Direction::Right) => PartOneKey::Two,
            (PartOneKey::Two, Direction::Down) => PartOneKey::Five,
            (PartOneKey::Two, Direction::Right) => PartOneKey::Three,
            (PartOneKey::Two, Direction::Left) => PartOneKey::One,
            (PartOneKey::Three, Direction::Left) => PartOneKey::Two,
            (PartOneKey::Three, Direction::Down) => PartOneKey::Six,
            (PartOneKey::Four, Direction::Up) => PartOneKey::One,
            (PartOneKey::Four, Direction::Right) => PartOneKey::Five,
            (PartOneKey::Four, Direction::Down) => PartOneKey::Seven,
            (PartOneKey::Five, Direction::Up) => PartOneKey::Two,
            (PartOneKey::Five, Direction::Right) => PartOneKey::Six,
            (PartOneKey::Five, Direction::Left) => PartOneKey::Four,
            (PartOneKey::Five, Direction::Down) => PartOneKey::Eight,
            (PartOneKey::Six, Direction::Up) => PartOneKey::Three,
            (PartOneKey::Six, Direction::Left) => PartOneKey::Five,
            (PartOneKey::Six, Direction::Down) => PartOneKey::Nine,
            (PartOneKey::Seven, Direction::Up) => PartOneKey::Four,
            (PartOneKey::Seven, Direction::Right) => PartOneKey::Eight,
            (PartOneKey::Eight, Direction::Up) => PartOneKey::Five,
            (PartOneKey::Eight, Direction::Right) => PartOneKey::Nine,
            (PartOneKey::Eight, Direction::Left) => PartOneKey::Seven,
            (PartOneKey::Nine, Direction::Up) => PartOneKey::Six,
            (PartOneKey::Nine, Direction::Left) => PartOneKey::Eight,
            _ => *self,
        }
    }
}

impl Display for PartOneKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            PartOneKey::One => '1',
            PartOneKey::Two => '2',
            PartOneKey::Three => '3',
            PartOneKey::Four => '4',
            PartOneKey::Five => '5',
            PartOneKey::Six => '6',
            PartOneKey::Seven => '7',
            PartOneKey::Eight => '8',
            PartOneKey::Nine => '9',
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug, Copy, Clone)]
enum PartTwoKey {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
}

impl PartTwoKey {
    pub fn next(&self, direction: &Direction) -> PartTwoKey {
        match (self, direction) {
            (PartTwoKey::One, Direction::Down) => PartTwoKey::Three,
            (PartTwoKey::Two, Direction::Right) => PartTwoKey::Three,
            (PartTwoKey::Two, Direction::Down) => PartTwoKey::Six,
            (PartTwoKey::Three, Direction::Up) => PartTwoKey::One,
            (PartTwoKey::Three, Direction::Right) => PartTwoKey::Four,
            (PartTwoKey::Three, Direction::Down) => PartTwoKey::Seven,
            (PartTwoKey::Three, Direction::Left) => PartTwoKey::Two,
            (PartTwoKey::Four, Direction::Down) => PartTwoKey::Eight,
            (PartTwoKey::Four, Direction::Left) => PartTwoKey::Three,
            (PartTwoKey::Five, Direction::Right) => PartTwoKey::Six,
            (PartTwoKey::Six, Direction::Up) => PartTwoKey::Two,
            (PartTwoKey::Six, Direction::Right) => PartTwoKey::Seven,
            (PartTwoKey::Six, Direction::Down) => PartTwoKey::A,
            (PartTwoKey::Six, Direction::Left) => PartTwoKey::Five,
            (PartTwoKey::Seven, Direction::Up) => PartTwoKey::Three,
            (PartTwoKey::Seven, Direction::Right) => PartTwoKey::Eight,
            (PartTwoKey::Seven, Direction::Down) => PartTwoKey::B,
            (PartTwoKey::Seven, Direction::Left) => PartTwoKey::Six,
            (PartTwoKey::Eight, Direction::Up) => PartTwoKey::Four,
            (PartTwoKey::Eight, Direction::Right) => PartTwoKey::Nine,
            (PartTwoKey::Eight, Direction::Down) => PartTwoKey::C,
            (PartTwoKey::Eight, Direction::Left) => PartTwoKey::Seven,
            (PartTwoKey::Nine, Direction::Left) => PartTwoKey::Eight,
            (PartTwoKey::A, Direction::Up) => PartTwoKey::Six,
            (PartTwoKey::A, Direction::Right) => PartTwoKey::B,
            (PartTwoKey::B, Direction::Up) => PartTwoKey::Seven,
            (PartTwoKey::B, Direction::Right) => PartTwoKey::C,
            (PartTwoKey::B, Direction::Down) => PartTwoKey::D,
            (PartTwoKey::B, Direction::Left) => PartTwoKey::A,
            (PartTwoKey::C, Direction::Up) => PartTwoKey::Eight,
            (PartTwoKey::C, Direction::Left) => PartTwoKey::B,
            (PartTwoKey::D, Direction::Up) => PartTwoKey::B,
            _ => *self,
        }
    }
}

impl Display for PartTwoKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            PartTwoKey::One => '1',
            PartTwoKey::Two => '2',
            PartTwoKey::Three => '3',
            PartTwoKey::Four => '4',
            PartTwoKey::Five => '5',
            PartTwoKey::Six => '6',
            PartTwoKey::Seven => '7',
            PartTwoKey::Eight => '8',
            PartTwoKey::Nine => '9',
            PartTwoKey::A => 'A',
            PartTwoKey::B => 'B',
            PartTwoKey::C => 'C',
            PartTwoKey::D => 'D',
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Directions = Vec<Direction>;
type DirectionsList = Vec<Directions>;

fn parse(input: &str) -> anyhow::Result<DirectionsList> {
    all_consuming(directions_list)
        .parse(input)
        .finish()
        .map(|(_remaining, directions_list)| directions_list)
        .map_err(|e| anyhow!(convert_error(input, e)))
}

fn directions_list(input: &str) -> IResult<&str, DirectionsList, VerboseError<&str>> {
    separated_list1(tag("\n"), directions).parse(input)
}

fn directions(input: &str) -> IResult<&str, Directions, VerboseError<&str>> {
    many1(direction).parse(input)
}

fn direction(input: &str) -> IResult<&str, Direction, VerboseError<&str>> {
    alt((
        value(Direction::Up, tag("U")),
        value(Direction::Down, tag("D")),
        value(Direction::Left, tag("L")),
        value(Direction::Right, tag("R")),
    ))
    .parse(input)
}
