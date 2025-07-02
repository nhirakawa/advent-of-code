use crate::common::parse::{finish, unsigned_number};
use anyhow::bail;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list1, sequence::preceded,
    IResult, Parser,
};
use std::collections::HashSet;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let directions = parse(input);
    let mut current_direction = CardinalDirection::North;
    let mut current_location: (isize, isize) = (0, 0);

    for direction in directions {
        current_direction = current_direction.apply(&direction);

        match current_direction {
            CardinalDirection::North => current_location.1 += direction.value(),
            CardinalDirection::East => current_location.0 += direction.value(),
            CardinalDirection::South => current_location.1 -= direction.value(),
            CardinalDirection::West => current_location.0 -= direction.value(),
        }
    }

    Ok((current_location.0.abs() + current_location.1.abs()).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let directions = parse(input);
    let mut current_direction = CardinalDirection::North;

    let mut current_location: (isize, isize) = (0, 0);

    let mut seen_locations = HashSet::new();

    for direction in directions {
        current_direction = current_direction.apply(&direction);

        for _ in 0..direction.value() {
            match current_direction {
                CardinalDirection::North => current_location.1 += 1,
                CardinalDirection::East => current_location.0 += 1,
                CardinalDirection::South => current_location.1 -= 1,
                CardinalDirection::West => current_location.0 -= 1,
            };

            if !seen_locations.insert(current_location) {
                return Ok((current_location.0.abs() + current_location.1.abs()).to_string());
            }
        }
    }

    bail!("No answer found")
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Left(isize),
    Right(isize),
}

impl Direction {
    fn value(&self) -> isize {
        match self {
            Direction::Left(value) => *value,
            Direction::Right(value) => *value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    fn apply(&self, direction: &Direction) -> Self {
        match (self, *direction) {
            (CardinalDirection::North, Direction::Left(_)) => CardinalDirection::West,
            (CardinalDirection::East, Direction::Left(_)) => CardinalDirection::North,
            (CardinalDirection::South, Direction::Left(_)) => CardinalDirection::East,
            (CardinalDirection::West, Direction::Left(_)) => CardinalDirection::South,
            (CardinalDirection::North, Direction::Right(_)) => CardinalDirection::East,
            (CardinalDirection::East, Direction::Right(_)) => CardinalDirection::South,
            (CardinalDirection::South, Direction::Right(_)) => CardinalDirection::West,
            (CardinalDirection::West, Direction::Right(_)) => CardinalDirection::North,
        }
    }
}

fn parse(i: &str) -> Vec<Direction> {
    finish(directions, i).unwrap()
}

fn directions(i: &str) -> IResult<&str, Vec<Direction>> {
    separated_list1(tag(", "), direction).parse(i)
}

fn direction(i: &str) -> IResult<&str, Direction> {
    alt((left, right)).parse(i)
}

fn left(i: &str) -> IResult<&str, Direction> {
    map(preceded(tag("L"), unsigned_number), Direction::Left).parse(i)
}

fn right(i: &str) -> IResult<&str, Direction> {
    map(preceded(tag("R"), unsigned_number), Direction::Right).parse(i)
}
