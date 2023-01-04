use std::collections::HashSet;

use common::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-3.txt");

    let rucksacks = split_into_rucksacks(input);

    let part_one = part_one(&rucksacks);
    let part_two = part_two(&rucksacks);

    Ok((part_one, part_two))
}

fn part_one(rucksack: &[Rucksack]) -> PartAnswer {
    let start = SystemTime::now();

    let answer: u32 = rucksack
        .iter()
        .map(|r| r.get_common_item_type_from_compartments().unwrap())
        .map(get_priority)
        .sum();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn part_two(rucksacks: &[Rucksack]) -> PartAnswer {
    if rucksacks.len() % 3 != 0 {
        panic!(
            "number of rucksacks ({}) not divisible by 3",
            rucksacks.len()
        )
    }
    let start = SystemTime::now();

    let answer: u32 = rucksacks
        .chunks(3)
        .map(get_common_item_type_from_rucksacks)
        .map(get_priority)
        .sum();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn get_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 65 + 27
    }
}

fn get_common_item_type_from_rucksacks(rucksacks: &[Rucksack]) -> char {
    rucksacks
        .iter()
        .map(Rucksack::get_all_unique_items)
        .reduce(|accum, item| {
            accum
                .intersection(&item)
                .cloned()
                .collect::<HashSet<char>>()
        })
        .iter()
        .next()
        .and_then(|s| s.iter().cloned().next())
        .unwrap_or_else(|| panic!("could not find common item type for {:?}", rucksacks))
}

#[derive(Debug)]
struct Rucksack {
    first_compartment: Vec<char>,
    second_compartment: Vec<char>,
}

impl From<&str> for Rucksack {
    fn from(raw: &str) -> Rucksack {
        let length = raw.len();

        if length % 2 != 0 {
            panic!()
        }

        let first = raw.chars().take(length / 2).collect();
        let second = raw.chars().skip(length / 2).collect();

        Rucksack::new(first, second)
    }
}

impl Rucksack {
    fn new(first_compartment: Vec<char>, second_compartment: Vec<char>) -> Rucksack {
        Rucksack {
            first_compartment,
            second_compartment,
        }
    }

    fn get_common_item_type_from_compartments(&self) -> Option<char> {
        let first_compartment: HashSet<char> = self.first_compartment.iter().cloned().collect();
        let second_compartment: HashSet<char> = self.second_compartment.iter().cloned().collect();

        first_compartment
            .intersection(&second_compartment)
            .next()
            .cloned()
    }

    fn get_all_unique_items(&self) -> HashSet<char> {
        let first: HashSet<char> = self.first_compartment.iter().cloned().collect();
        let second: HashSet<char> = self.second_compartment.iter().cloned().collect();

        first.union(&second).cloned().collect()
    }
}

fn split_into_rucksacks(i: &str) -> Vec<Rucksack> {
    i.lines().map(Rucksack::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_priority() {
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('f'), 6);
        assert_eq!(get_priority('Y'), 51);
    }

    #[test]
    fn test_get_common_item_type_from_compartments() {
        let rucksack = Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(rucksack.get_common_item_type_from_compartments(), Some('p'));

        let rucksack = Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        assert_eq!(rucksack.get_common_item_type_from_compartments(), Some('L'));

        let rucksack = Rucksack::from("PmmdzqPrVvPwwTWBwg");
        assert_eq!(rucksack.get_common_item_type_from_compartments(), Some('P'));

        let rucksack = Rucksack::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        assert_eq!(rucksack.get_common_item_type_from_compartments(), Some('v'));

        let rucksack = Rucksack::from("ttgJtRGJQctTZtZT");
        assert_eq!(rucksack.get_common_item_type_from_compartments(), Some('t'));

        let rucksack = Rucksack::from("CrZsJsPPZsGzwwsLwLmpwMDw");
        assert_eq!(rucksack.get_common_item_type_from_compartments(), Some('s'));
    }

    #[test]
    fn test_get_common_item_type_from_rucksacks() {
        let first = Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp");
        let second = Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        let third = Rucksack::from("PmmdzqPrVvPwwTWBwg");

        let rucksacks = vec![first, second, third];
        assert_eq!(get_common_item_type_from_rucksacks(&rucksacks), 'r');

        let first = Rucksack::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        let second = Rucksack::from("ttgJtRGJQctTZtZT");
        let third = Rucksack::from("CrZsJsPPZsGzwwsLwLmpwMDw");

        let rucksacks = vec![first, second, third];
        assert_eq!(get_common_item_type_from_rucksacks(&rucksacks), 'Z');
    }
}
