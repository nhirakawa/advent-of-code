use crate::common::parse::{finish, unsigned_number};
use nom::{
    bytes::complete::tag, combinator::into, multi::separated_list1, sequence::separated_pair,
    IResult, Parser,
};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let assignments = parse(input);

    let mut count = 0;

    for (first, second) in &assignments {
        if first.completely_overlaps(second) || second.completely_overlaps(first) {
            count += 1;
        }
    }

    Ok(count.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let assignments = parse(input);

    let mut count = 0;

    for (first, second) in &assignments {
        if first.has_any_overlap(second) || second.has_any_overlap(first) {
            count += 1;
        }
    }

    Ok(count.to_string())
}

#[derive(Debug, PartialEq, Eq)]
struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn completely_overlaps(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn has_any_overlap(&self, other: &Self) -> bool {
        self.completely_overlaps(other)
            || (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
    }
}

impl From<(u32, u32)> for Assignment {
    fn from(raw: (u32, u32)) -> Assignment {
        let (start, end) = raw;
        Assignment { start, end }
    }
}

fn parse(i: &str) -> Vec<(Assignment, Assignment)> {
    finish(pairs, i).unwrap()
}

fn pairs(i: &str) -> IResult<&str, Vec<(Assignment, Assignment)>> {
    separated_list1(tag("\n"), pair).parse(i)
}

fn pair(i: &str) -> IResult<&str, (Assignment, Assignment)> {
    separated_pair(assignment, tag(","), assignment).parse(i)
}

fn assignment(i: &str) -> IResult<&str, Assignment> {
    into(separated_pair(unsigned_number, tag("-"), unsigned_number)).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_any_overlap() {
        let first: Assignment = (2, 6).into();
        let second: Assignment = (4, 8).into();

        assert_eq!(
            first.has_any_overlap(&second) || second.has_any_overlap(&first),
            true
        );
    }
}
