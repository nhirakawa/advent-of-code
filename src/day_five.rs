use crate::answer::{AdventOfCodeError, AdventOfCodeResult, AnswerWithTiming};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::all_consuming,
    combinator::{map, value},
    multi::many1,
    sequence::terminated,
    IResult,
};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::time::{Duration, SystemTime};

pub fn run() -> AdventOfCodeResult {
    let start = SystemTime::now();

    let seat_pointers = parse_seat_pointers()?;

    let parsed_ms = start.elapsed().unwrap().as_millis();

    let part_one = part_one(&seat_pointers, parsed_ms);
    let part_two = part_two(&seat_pointers, parsed_ms)?;

    Ok((part_one, part_two))
}

fn part_one(seat_pointers: &SeatPointers, parse_duration: u128) -> AnswerWithTiming {
    let start = SystemTime::now();
    let answer = seat_pointers.get_max_seat_id();

    let elapsed = start.elapsed().unwrap();
    let elapsed = (elapsed.as_millis() + parse_duration) as u64;
    let elapsed = Duration::from_millis(elapsed);

    (answer, elapsed)
}

fn part_two(
    seat_pointers: &SeatPointers,
    parse_duration: u128,
) -> Result<AnswerWithTiming, AdventOfCodeError> {
    let start = SystemTime::now();

    let min_seat_pointer = seat_pointers
        .clone()
        .into_iter()
        .min_by_key(SeatPointer::get_seat_id)
        .ok_or(AdventOfCodeError::NoAnswerFoundPartTwo)?;

    let max_seat_pointer = seat_pointers
        .clone()
        .into_iter()
        .max_by_key(SeatPointer::get_seat_id)
        .ok_or(AdventOfCodeError::NoAnswerFoundPartTwo)?;

    let seat_ids: HashSet<u32> = seat_pointers
        .clone()
        .into_iter()
        .map(|s| s.get_seat_id())
        .collect();

    let mut possible_solutions = Vec::new();

    for i in min_seat_pointer.get_seat_id()..max_seat_pointer.get_seat_id() {
        if !seat_ids.contains(&i) {
            possible_solutions.push(i);
        }
    }

    let elapsed = start.elapsed().unwrap().as_millis();
    let elapsed = (elapsed + parse_duration) as u64;
    let elapsed = Duration::from_millis(elapsed);

    match possible_solutions.len() {
        1 => Ok((possible_solutions[0], elapsed)),
        _ => Err(AdventOfCodeError::NoAnswerFoundPartTwo),
    }
}

fn parse_seat_pointers() -> Result<SeatPointers, AdventOfCodeError> {
    let input = include_str!("../input/day-5.txt");

    let result = all_consuming(seat_pointers)(input);
    let result = result.map_err(|_| AdventOfCodeError::NomParseError);

    let (_, seat_pointers) = result?;

    Ok(seat_pointers)
}

fn take_upper_half(endpoints: (u32, u32)) -> (u32, u32) {
    let (low_endpoint, high_endpoint) = endpoints;

    let difference = high_endpoint - low_endpoint;

    let difference = if difference % 2 != 0 {
        difference + 1
    } else {
        difference
    };

    let midpoint = low_endpoint + (difference / 2);

    (midpoint, high_endpoint)
}

fn take_lower_half(endpoints: (u32, u32)) -> (u32, u32) {
    let (low_endpoint, high_endpoint) = endpoints;

    let difference = high_endpoint - low_endpoint;

    let difference = if difference % 2 != 0 {
        difference + 1
    } else {
        difference
    };

    let midpoint = high_endpoint - (difference / 2);

    (low_endpoint, midpoint)
}

#[derive(Debug, Eq, Copy, Clone)]
struct SeatPointer {
    row: u32,
    column: u32,
    seat_id: u32,
}

impl SeatPointer {
    pub fn get_seat_id(&self) -> u32 {
        self.seat_id
    }
}

impl From<(u32, u32)> for SeatPointer {
    fn from(tuple: (u32, u32)) -> SeatPointer {
        let (row, column) = tuple;

        let seat_id = (row * 8) + column;

        SeatPointer {
            row,
            column,
            seat_id,
        }
    }
}

impl PartialOrd for SeatPointer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SeatPointer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.row
            .cmp(&other.row)
            .then(self.column.cmp(&other.column))
    }
}

impl PartialEq for SeatPointer {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}

#[derive(Debug, PartialEq, Clone)]
struct SeatPointers {
    seat_pointers: Vec<SeatPointer>,
}

impl SeatPointers {
    pub fn new(mut pointers: Vec<SeatPointer>) -> SeatPointers {
        pointers.sort();

        SeatPointers {
            seat_pointers: pointers,
        }
    }

    pub fn get_max_seat_id(&self) -> u32 {
        let mut max = 0;

        for seat_pointer in &self.seat_pointers {
            max = max.max(seat_pointer.get_seat_id());
        }

        max
    }
}

impl IntoIterator for SeatPointers {
    type Item = SeatPointer;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.seat_pointers.into_iter()
    }
}

fn seat_pointers(i: &str) -> IResult<&str, SeatPointers> {
    map(many1(seat_pointer), SeatPointers::new)(i)
}

fn seat_pointer(i: &str) -> IResult<&str, SeatPointer> {
    map(directions, to_seat_pointer)(i)
}

fn to_seat_pointer(directions: Vec<Direction>) -> SeatPointer {
    let mut row = (0, 127);
    let mut column = (0, 7);

    for direction in directions {
        match direction {
            Direction::Front => row = take_lower_half(row),
            Direction::Back => row = take_upper_half(row),
            Direction::Left => column = take_lower_half(column),
            Direction::Right => column = take_upper_half(column),
        }
    }

    assert!(row.0 == row.1);
    assert!(column.0 == column.1);

    (row.0, column.0).into()
}

fn directions(i: &str) -> IResult<&str, Vec<Direction>> {
    terminated(many1(direction), tag("\n"))(i)
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Front,
    Back,
    Left,
    Right,
}

fn direction(i: &str) -> IResult<&str, Direction> {
    alt((front, back, left, right))(i)
}

fn front(i: &str) -> IResult<&str, Direction> {
    value(Direction::Front, char('F'))(i)
}

fn back(i: &str) -> IResult<&str, Direction> {
    value(Direction::Back, char('B'))(i)
}

fn left(i: &str) -> IResult<&str, Direction> {
    value(Direction::Left, char('L'))(i)
}

fn right(i: &str) -> IResult<&str, Direction> {
    value(Direction::Right, char('R'))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_pointers_sorting() {
        let first = SeatPointer::from((0, 0));
        let second = SeatPointer::from((0, 1));
        let third = SeatPointer::from((10, 11));
        let fourth = SeatPointer::from((10, 100));

        let actual = SeatPointers::new(vec![third, second, fourth, first]);
        let expected = vec![first, second, third, fourth];

        assert_eq!(actual.seat_pointers, expected);
    }

    #[test]
    fn test_seat_pointer() {
        assert_eq!(seat_pointer("FBFBBFFRLR\n"), Ok(("", (44, 5).into())));
        assert_eq!(seat_pointer("BFFFBBFRRR\n"), Ok(("", (70, 7).into())));
        assert_eq!(seat_pointer("FFFBBBFRRR\n"), Ok(("", (14, 7).into())));
        assert_eq!(seat_pointer("BBFFBBFRLL\n"), Ok(("", (102, 4).into())));
    }

    #[test]
    fn test_take_upper_half() {
        assert_eq!(take_upper_half((0, 63)), (32, 63));
        assert_eq!(take_upper_half((32, 47)), (40, 47));
    }

    #[test]
    fn test_take_lower_half() {
        assert_eq!(take_lower_half((0, 127)), (0, 63));
        assert_eq!(take_lower_half((32, 63)), (32, 47));
    }

    #[test]
    fn test_directions() {
        assert_eq!(
            directions("FBFBBFFRLR\n"),
            Ok((
                "",
                vec![
                    Direction::Front,
                    Direction::Back,
                    Direction::Front,
                    Direction::Back,
                    Direction::Back,
                    Direction::Front,
                    Direction::Front,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right
                ]
            ))
        );
    }

    #[test]
    fn test_direction() {
        assert_eq!(direction("F"), Ok(("", Direction::Front)));
    }

    #[test]
    fn test_front() {
        assert_eq!(front("F"), Ok(("", Direction::Front)));
    }

    #[test]
    fn test_back() {
        assert_eq!(back("B"), Ok(("", Direction::Back)));
    }

    #[test]
    fn test_left() {
        assert_eq!(left("L"), Ok(("", Direction::Left)));
    }

    #[test]
    fn test_right() {
        assert_eq!(right("R"), Ok(("", Direction::Right)));
    }
}
