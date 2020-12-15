use std::collections::HashMap;

use crate::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-15.txt");
    let integers = parse_integers(input);

    let part_one = part_one(&integers);
    let part_two = part_two(&integers);

    Ok((part_one, part_two))
}

fn part_one(integers: &Vec<u64>) -> PartAnswer {
    let start = SystemTime::now();

    let mut spoken_numbers = SpokenNumbers::from_starting_integers(integers);
    spoken_numbers.fast_forward_to_turn_number(2020);

    let solution = spoken_numbers.last_spoken;

    let elapsed = start.elapsed().unwrap();

    Ok((solution, elapsed))
}

fn part_two(integers: &Vec<u64>) -> PartAnswer {
    let start = SystemTime::now();

    let mut spoken_numbers = SpokenNumbers::from_starting_integers(integers);
    spoken_numbers.fast_forward_to_turn_number(30000000);

    let solution = spoken_numbers.last_spoken;
    let elapsed = start.elapsed().unwrap();

    Ok((solution, elapsed))
}

#[derive(Debug, PartialEq, Default)]
struct SpokenNumbers {
    most_recent: HashMap<u64, u64>,
    second_most_recent: HashMap<u64, u64>,
    last_spoken: u64,
    turn_number: u64,
}

impl SpokenNumbers {
    pub fn from_starting_integers(integers: &Vec<u64>) -> Self {
        let mut this = Self::default();

        for integer in integers {
            this.speak_number(*integer);
        }

        this
    }

    pub fn has_been_spoken_more_than_once(&self, integer: &u64) -> bool {
        self.second_most_recent.contains_key(integer)
    }

    fn speak_number(&mut self, integer: u64) {
        self.turn_number += 1;

        match self.most_recent.insert(integer, self.turn_number) {
            Some(turn) => self.second_most_recent.insert(integer, turn),
            None => None,
        };

        self.last_spoken = integer;
    }

    pub fn get_next_number(&self) -> u64 {
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

    pub fn fast_forward_to_turn_number(&mut self, turn_number: u64) {
        while self.turn_number < turn_number {
            self.speak_next_number()
        }
    }
}

fn parse_integers(i: &str) -> Vec<u64> {
    i.split(",").map(|l| l.parse::<u64>()).flatten().collect()
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
