use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{all_consuming, into, map},
    multi::separated_list1,
    sequence::terminated,
    IResult, Parser,
};
use std::collections::HashSet;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let groups = parse_groups(input)?;

    Ok(groups.iter().map(|group| group.union_size()).sum::<u32>())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let groups = parse_groups(input)?;

    Ok(groups
        .iter()
        .map(|group| group.intersection_size())
        .sum::<u32>())
}

fn parse_groups(input: &str) -> anyhow::Result<Vec<Group>> {
    let result = all_consuming(terminated(groups, tag("\n"))).parse(input);

    let (_, groups) = result.unwrap();

    Ok(groups)
}

fn groups(i: &str) -> IResult<&str, Vec<Group>> {
    separated_list1(tag("\n\n"), group).parse(i)
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
    into(separated_list1(tag("\n"), person)).parse(i)
}

#[derive(Debug, PartialEq)]
struct Person {
    chars: HashSet<char>,
}

impl From<&str> for Person {
    fn from(str: &str) -> Self {
        let chars = str.chars().collect::<HashSet<char>>();

        Self { chars }
    }
}

fn person(i: &str) -> IResult<&str, Person> {
    map(alpha1, |s: &str| s.into()).parse(i)
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
}
