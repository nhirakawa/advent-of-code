use std::collections::HashMap;

use crate::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-15.txt");
    let integers = parse_integers(input);

    let part_one = part_one(&integers);
    let part_two = part_two(&integers);

    Ok((part_one, part_two))
}

fn part_one(integers: &Vec<u32>) -> PartAnswer {
    let start = SystemTime::now();

    let mut spoken_numbers = SpokenNumbers::from_starting_integers(integers);
    spoken_numbers.fast_forward_to_turn_number(2020);

    let solution = spoken_numbers.last_spoken;

    let elapsed = start.elapsed().unwrap();

    Ok((solution as u64, elapsed))
}

fn part_two(integers: &Vec<u32>) -> PartAnswer {
    let start = SystemTime::now();

    let mut spoken_numbers = SpokenNumbers::from_starting_integers(integers);
    spoken_numbers.fast_forward_to_turn_number(30000000);
    println!("{}", spoken_numbers.max_spoken);

    let solution = spoken_numbers.last_spoken;
    let elapsed = start.elapsed().unwrap();

    Ok((solution as u64, elapsed))
}

#[derive(Debug, PartialEq, Default)]
struct SpokenNumbers {
    most_recent: HashMap<u32, u32>,
    second_most_recent: HashMap<u32, u32>,
    last_spoken: u32,
    turn_number: u32,
    max_spoken: u32,
}

impl SpokenNumbers {
    pub fn from_starting_integers(integers: &Vec<u32>) -> Self {
        let mut this = Self::default();

        for integer in integers {
            this.speak_number(*integer);
        }

        this
    }

    pub fn has_been_spoken_more_than_once(&self, integer: &u32) -> bool {
        self.second_most_recent.contains_key(integer)
    }

    fn speak_number(&mut self, integer: u32) {
        self.turn_number += 1;

        match self.most_recent.insert(integer, self.turn_number) {
            Some(turn) => self.second_most_recent.insert(integer, turn),
            None => None,
        };

        self.last_spoken = integer;
        self.max_spoken = self.max_spoken.max(self.last_spoken)
    }

    pub fn get_next_number(&self) -> u32 {
        if !self.has_been_spoken_more_than_once(&self.last_spoken) {
            0
        } else {
            let most_recent = self.most_recent.get(&self.last_spoken).unwrap();
            let second_most_recent = self.second_most_recent.get(&self.last_spoken).unwrap();

            *most_recent - *second_most_recent
        }
    }

    pub fn speak_next_number(&mut self) {
        self.speak_number(self.get_next_number())
    }

    pub fn fast_forward_to_turn_number(&mut self, turn_number: u32) {
        while self.turn_number < turn_number {
            self.speak_next_number()
        }
    }
}

fn parse_integers(i: &str) -> Vec<u32> {
    i.split(",").map(|l| l.parse()).flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_number() {
        let mut spoken_numbers = SpokenNumbers::from_starting_integers(&vec![0, 3, 6]);

        assert_eq!(spoken_numbers.get_next_number(), 0);

        spoken_numbers.speak_number(0); // turn 4

        assert_eq!(spoken_numbers.get_next_number(), 3);

        spoken_numbers.speak_number(3); //turn 5

        assert_eq!(spoken_numbers.get_next_number(), 3);

        spoken_numbers.speak_number(3); // turn 6

        assert_eq!(spoken_numbers.get_next_number(), 1);
    }

    #[test]
    fn test_speak_next_number() {
        let mut spoken_numbers = SpokenNumbers::from_starting_integers(&vec![1, 3, 2]);
        spoken_numbers.fast_forward_to_turn_number(2020);
        assert_eq!(spoken_numbers.last_spoken, 1);

        let mut spoken_numbers = SpokenNumbers::from_starting_integers(&vec![2, 1, 3]);
        spoken_numbers.fast_forward_to_turn_number(2020);
        assert_eq!(spoken_numbers.last_spoken, 10);

        let mut spoken_numbers = SpokenNumbers::from_starting_integers(&vec![1, 2, 3]);
        spoken_numbers.fast_forward_to_turn_number(2020);
        assert_eq!(spoken_numbers.last_spoken, 27);

        let mut spoken_numbers = SpokenNumbers::from_starting_integers(&vec![2, 3, 1]);
        spoken_numbers.fast_forward_to_turn_number(2020);
        assert_eq!(spoken_numbers.last_spoken, 78);

        let mut spoken_numbers = SpokenNumbers::from_starting_integers(&vec![3, 2, 1]);
        spoken_numbers.fast_forward_to_turn_number(2020);
        assert_eq!(spoken_numbers.last_spoken, 438);

        let mut spoken_numbers = SpokenNumbers::from_starting_integers(&vec![3, 1, 2]);
        spoken_numbers.fast_forward_to_turn_number(2020);
        assert_eq!(spoken_numbers.last_spoken, 1836);
    }
}
