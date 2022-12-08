use common::prelude::*;
use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::separated_list1, sequence::preceded,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-1.txt");

    let directions = parse(input);

    let part_one = part_one(&directions);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(directions: &[Direction]) -> PartAnswer {
    let start = SystemTime::now();

    let mut current_direction = CardinalDirection::North;

    let mut units_north: isize = 0;
    let mut units_east: isize = 0;
    let mut units_south: isize = 0;
    let mut units_west: isize = 0;

    for direction in directions {
        current_direction = current_direction.apply(direction);

        match current_direction {
            CardinalDirection::North => units_north += direction.value(),
            CardinalDirection::East => units_east += direction.value(),
            CardinalDirection::South => units_south += direction.value(),
            CardinalDirection::West => units_west += direction.value(),
        }
    }

    let net_units_east_west = units_east.abs_diff(units_west);
    let net_units_north_south = units_north.abs_diff(units_south);

    let answer = net_units_east_west + net_units_north_south;

    let elapsed = start.elapsed().unwrap();

    // 455 is too high
    PartAnswer::new(answer, elapsed)
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
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
    finish(directions)(i).unwrap().1
}

fn directions(i: &str) -> IResult<&str, Vec<Direction>> {
    separated_list1(tag(", "), direction)(i)
}

fn direction(i: &str) -> IResult<&str, Direction> {
    alt((left, right))(i)
}

fn left(i: &str) -> IResult<&str, Direction> {
    map(preceded(tag("L"), unsigned_number), Direction::Left)(i)
}

fn right(i: &str) -> IResult<&str, Direction> {
    map(preceded(tag("R"), unsigned_number), Direction::Right)(i)
}
