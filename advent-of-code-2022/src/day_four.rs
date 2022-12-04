use common::prelude::*;
use nom::{
    bytes::complete::tag, combinator::into, multi::separated_list1, sequence::separated_pair,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-4.txt");

    let assignments = parse(input);

    let part_one = part_one(&assignments);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(assignments: &[(Assignment, Assignment)]) -> PartAnswer {
    let start = SystemTime::now();

    let mut count = 0;

    for (first, second) in assignments {
        if first.contains(second) || second.contains(first) {
            count += 1;
        }
    }

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(count, elapsed)
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

#[derive(Debug, PartialEq, Eq)]
struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }
}

impl From<(u32, u32)> for Assignment {
    fn from(raw: (u32, u32)) -> Assignment {
        let (start, end) = raw;
        Assignment { start, end }
    }
}

fn parse(i: &str) -> Vec<(Assignment, Assignment)> {
    finish(pairs)(i).unwrap().1
}

fn pairs(i: &str) -> IResult<&str, Vec<(Assignment, Assignment)>> {
    separated_list1(tag("\n"), pair)(i)
}

fn pair(i: &str) -> IResult<&str, (Assignment, Assignment)> {
    separated_pair(assignment, tag(","), assignment)(i)
}

fn assignment(i: &str) -> IResult<&str, Assignment> {
    into(separated_pair(unsigned_number, tag("-"), unsigned_number))(i)
}
