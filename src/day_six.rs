use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{all_consuming, into, map},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use std::{
    collections::HashSet,
    time::{Duration, SystemTime},
};

use crate::answer::{AdventOfCodeError, AdventOfCodeResult, PartAnswer};

pub fn run() -> AdventOfCodeResult {
    let start = SystemTime::now();
    let groups = parse_groups()?;
    let parse_ms = start.elapsed().unwrap().as_millis();

    let part_one = part_one(&groups, parse_ms);
    let part_two = part_two(&groups, parse_ms);

    Ok((part_one, part_two))
}

fn part_one(groups: &Vec<Group>, parse_time: u128) -> PartAnswer {
    let start = SystemTime::now();

    let mut counter = 0;
    for group in groups {
        counter += group.union_size();
    }

    let elapsed = start.elapsed().unwrap().as_millis();
    let elapsed = elapsed + parse_time;
    let elapsed = elapsed as u64;
    let elapsed = Duration::from_millis(elapsed);

    (counter as u64, elapsed)
}

fn part_two(groups: &Vec<Group>, parse_time: u128) -> PartAnswer {
    let start = SystemTime::now();

    let mut counter = 0;

    for group in groups {
        counter += group.intersection_size();
    }

    let elapsed = start.elapsed().unwrap().as_millis();
    let elapsed = elapsed + parse_time;
    let elapsed = elapsed as u64;
    let elapsed = Duration::from_millis(elapsed);

    (counter as u64, elapsed)
}

fn parse_groups() -> Result<Vec<Group>, AdventOfCodeError> {
    let input = include_str!("../input/day-6.txt");

    let result = all_consuming(terminated(groups, tag("\n")))(input);

    let (_, groups) = result.unwrap();

    Ok(groups)
}

fn groups(i: &str) -> IResult<&str, Vec<Group>> {
    separated_list1(tag("\n\n"), group)(i)
}

#[derive(Debug, PartialEq)]
struct Group {
    persons: Vec<Person>,
}

impl Group {
    pub fn union_size(&self) -> u32 {
        let mut base: HashSet<char> = HashSet::new();

        for person in &self.persons {
            base = &base | &person.chars;
        }

        base.len() as u32
    }

    pub fn intersection_size(&self) -> u32 {
        let mut base = "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .collect::<HashSet<char>>();

        for person in &self.persons {
            base = &base & &person.chars;
        }

        base.len() as u32
    }
}

impl From<Vec<Person>> for Group {
    fn from(persons: Vec<Person>) -> Self {
        Self { persons }
    }
}

fn group(i: &str) -> IResult<&str, Group> {
    into(separated_list1(tag("\n"), person))(i)
}

#[derive(Debug, PartialEq)]
struct Person {
    chars: HashSet<char>,
}

impl From<&str> for Person {
    fn from(str: &str) -> Self {
        let chars = str.chars().into_iter().collect::<HashSet<char>>().into();

        Self { chars }
    }
}

fn person(i: &str) -> IResult<&str, Person> {
    map(alpha1, |s: &str| s.into())(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person() {
        assert_eq!(person("abc"), Ok(("", "abc".into())));
    }

    #[test]
    fn test_group() {
        let first = "ab".into();
        let second = "ac".into();

        let expected = vec![first, second];
        let expected = expected.into();

        assert_eq!(group("ab\nac"), Ok(("", expected)));
    }

    #[test]
    fn test_groups() {
        let first = "ab".into();
        let second = "ac".into();

        let third = "a".into();

        let first_group = vec![first, second];
        let second_group = vec![third];

        let actual = groups("ab\nac\n\na");

        assert_eq!(
            actual,
            Ok(("", vec![first_group.into(), second_group.into()]))
        );
    }

    #[test]
    fn test_answers() {
        let (part_one, part_two) = run().unwrap();
        let (part_one, _) = part_one;
        let (part_two, _) = part_two;

        assert_eq!(part_one, 6585);
        assert_eq!(part_two, 3276);
    }
}
