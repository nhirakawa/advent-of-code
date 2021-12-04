use common::prelude::*;
use log::debug;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};
use std::{collections::HashSet, fmt::Display};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-3.txt");
    let binary_numbers = parse_binary_numbers(input);

    let part_one = part_one(&binary_numbers);
    let part_two = part_two(&binary_numbers);

    Ok((part_one, part_two))
}

fn part_one(binary_numbers: &BinaryNumbers) -> PartAnswer {
    let start = SystemTime::now();

    let (gamma, epsilon) = calculate_gamma_and_epsilon(binary_numbers);
    let gamma = gamma.value();
    let epsilon = epsilon.value();
    let solution = gamma * epsilon;
    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(solution, elapsed)
}

fn calculate_gamma_and_epsilon(binary_numbers: &BinaryNumbers) -> (BinaryNumber, BinaryNumber) {
    let mut gamma_digits = Vec::new();
    let mut epsilon_digits = Vec::new();

    for index in 0..binary_numbers.cardinality {
        let mut ones = 0;
        let mut zeros = 0;

        for number in binary_numbers.numbers.iter() {
            match number.get(index).unwrap() {
                Bit::One => ones += 1,
                Bit::Zero => zeros += 1,
            }
        }

        let gamma_digit = if ones > zeros { Bit::One } else { Bit::Zero };
        gamma_digits.push(gamma_digit);

        let epsilon_digit = if ones > zeros { Bit::Zero } else { Bit::One };
        epsilon_digits.push(epsilon_digit);
    }

    let gamma = BinaryNumber::new(gamma_digits);
    let epsilon = BinaryNumber::new(epsilon_digits);

    (gamma, epsilon)
}

fn part_two(binary_numbers: &BinaryNumbers) -> PartAnswer {
    let start = SystemTime::now();
    let (oxygen_rating, carbon_dioxide_rating) = calculate_life_support_rating(binary_numbers);

    let solution = oxygen_rating.value() * carbon_dioxide_rating.value();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(solution, elapsed)
}

fn calculate_life_support_rating(binary_numbers: &BinaryNumbers) -> (BinaryNumber, BinaryNumber) {
    let oxygen_rating = life_support_rating(binary_numbers, &LifeSupportMode::Oxygen, 0);
    let carbon_dioxide_rating =
        life_support_rating(binary_numbers, &LifeSupportMode::CarbonDioxide, 0);

    debug!(
        "oxygen rating: {} (value {})",
        oxygen_rating,
        oxygen_rating.value()
    );

    debug!(
        "carbon dioxide rating: {} (value {})",
        carbon_dioxide_rating,
        carbon_dioxide_rating.value()
    );

    (oxygen_rating, carbon_dioxide_rating)
}

fn life_support_rating(
    binary_numbers: &BinaryNumbers,
    life_support_mode: &LifeSupportMode,
    index: usize,
) -> BinaryNumber {
    if binary_numbers.numbers.len() == 1 {
        return binary_numbers.numbers.iter().next().cloned().unwrap();
    }

    let next_binary_numbers = life_support_rating_base(binary_numbers, life_support_mode, index);
    life_support_rating(&next_binary_numbers, life_support_mode, index + 1)
}

fn life_support_rating_base(
    binary_numbers: &BinaryNumbers,
    life_support_mode: &LifeSupportMode,
    index: usize,
) -> BinaryNumbers {
    let mut ones = 0;
    let mut zeros = 0;

    for number in binary_numbers.numbers.iter() {
        match number.get(index).unwrap() {
            Bit::One => ones += 1,
            Bit::Zero => zeros += 1,
        };
    }

    let bit_to_keep = match (ones.cmp(&zeros), life_support_mode) {
        (std::cmp::Ordering::Less, LifeSupportMode::Oxygen) => Bit::Zero,
        (std::cmp::Ordering::Equal, LifeSupportMode::Oxygen) => Bit::One,
        (std::cmp::Ordering::Greater, LifeSupportMode::Oxygen) => Bit::One,
        (std::cmp::Ordering::Less, LifeSupportMode::CarbonDioxide) => Bit::One,
        (std::cmp::Ordering::Equal, LifeSupportMode::CarbonDioxide) => Bit::Zero,
        (std::cmp::Ordering::Greater, LifeSupportMode::CarbonDioxide) => Bit::Zero,
    };

    let mut numbers_to_keep = Vec::new();

    for number in binary_numbers.numbers.iter() {
        if number.get(index).cloned().unwrap() == bit_to_keep {
            numbers_to_keep.push(number.clone());
        }
    }

    BinaryNumbers::new(numbers_to_keep)
}

#[derive(Debug, Clone)]
enum LifeSupportMode {
    Oxygen,
    CarbonDioxide,
}

#[derive(Debug, Clone, PartialEq)]
struct BinaryNumbers {
    numbers: Vec<BinaryNumber>,
    cardinality: usize,
}

impl BinaryNumbers {
    fn new(numbers: Vec<BinaryNumber>) -> Self {
        let lengths: HashSet<usize> = numbers.iter().map(BinaryNumber::len).collect();
        if lengths.len() != 1 {
            panic!(format!("{:?}", lengths))
        }

        let cardinality = lengths.into_iter().next().unwrap();

        Self {
            numbers,
            cardinality,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct BinaryNumber {
    bits: Vec<Bit>,
}

impl BinaryNumber {
    fn new(bits: Vec<Bit>) -> Self {
        Self { bits }
    }

    fn len(&self) -> usize {
        self.bits.len()
    }

    fn get(&self, index: usize) -> Option<&Bit> {
        self.bits.get(index)
    }

    fn value(&self) -> i32 {
        self.bits
            .iter()
            .enumerate()
            .map(|(index, bit)| bit.value() << self.bits.len() - index - 1)
            .fold(0, |accumulator, value| accumulator | value as i32)
    }
}

impl Display for BinaryNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bit_string = self
            .bits
            .iter()
            .map(Bit::as_str)
            .collect::<Vec<&str>>()
            .join("");

        write!(f, "{}", bit_string)
    }
}

impl From<&str> for BinaryNumber {
    fn from(i: &str) -> BinaryNumber {
        binary_number(i).unwrap().1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    fn value(&self) -> i32 {
        match &self {
            Bit::Zero => 0,
            Bit::One => 1,
        }
    }

    fn as_str(&self) -> &str {
        match &self {
            Bit::One => "1",
            Bit::Zero => "0",
        }
    }
}

fn parse_binary_numbers(i: &str) -> BinaryNumbers {
    terminated(binary_numbers, tag("\n"))(i).unwrap().1
}

fn binary_numbers(i: &str) -> IResult<&str, BinaryNumbers> {
    map(
        separated_list1(tag("\n"), binary_number),
        BinaryNumbers::new,
    )(i)
}

fn binary_number(i: &str) -> IResult<&str, BinaryNumber> {
    map(many1(bit), BinaryNumber::new)(i)
}

fn bit(i: &str) -> IResult<&str, Bit> {
    alt((zero, one))(i)
}

fn one(i: &str) -> IResult<&str, Bit> {
    value(Bit::One, tag("1"))(i)
}

fn zero(i: &str) -> IResult<&str, Bit> {
    value(Bit::Zero, tag("0"))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let numbers = vec![
            "00100".into(),
            "11110".into(),
            "10110".into(),
            "10111".into(),
            "10101".into(),
            "01111".into(),
            "00111".into(),
            "11100".into(),
            "10000".into(),
            "11001".into(),
            "00010".into(),
            "01010".into(),
        ];
        let binary_numbers = BinaryNumbers::new(numbers);
        let (gamma, epsilon) = calculate_gamma_and_epsilon(&binary_numbers);

        assert_eq!(gamma.value(), 22);
        assert_eq!(epsilon.value(), 9);
    }

    #[test]
    fn test_oxygen_rating() {
        let numbers = vec![
            "00100".into(),
            "11110".into(),
            "10110".into(),
            "10111".into(),
            "10101".into(),
            "01111".into(),
            "00111".into(),
            "11100".into(),
            "10000".into(),
            "11001".into(),
            "00010".into(),
            "01010".into(),
        ];

        let binary_numbers = BinaryNumbers::new(numbers);

        let numbers_to_keep =
            life_support_rating_base(&binary_numbers, &LifeSupportMode::Oxygen, 0);
        let binary_numbers = BinaryNumbers::new(vec![
            "11110".into(),
            "10110".into(),
            "10111".into(),
            "10101".into(),
            "11100".into(),
            "10000".into(),
            "11001".into(),
        ]);
        assert_eq!(numbers_to_keep, binary_numbers);

        let numbers_to_keep =
            life_support_rating_base(&binary_numbers, &LifeSupportMode::Oxygen, 1);
        let binary_numbers = BinaryNumbers::new(vec![
            "10110".into(),
            "10111".into(),
            "10101".into(),
            "10000".into(),
        ]);
        assert_eq!(numbers_to_keep, binary_numbers);

        let numbers_to_keep =
            life_support_rating_base(&binary_numbers, &LifeSupportMode::Oxygen, 2);
        let binary_numbers =
            BinaryNumbers::new(vec!["10110".into(), "10111".into(), "10101".into()]);
        assert_eq!(numbers_to_keep, binary_numbers);

        let numbers_to_keep =
            life_support_rating_base(&binary_numbers, &LifeSupportMode::Oxygen, 3);
        let binary_numbers = BinaryNumbers::new(vec!["10110".into(), "10111".into()]);
        assert_eq!(numbers_to_keep, binary_numbers);

        let numbers_to_keep =
            life_support_rating_base(&binary_numbers, &LifeSupportMode::Oxygen, 4);
        let binary_numbers = BinaryNumbers::new(vec!["10111".into()]);
        assert_eq!(numbers_to_keep, binary_numbers);
    }

    #[test]
    fn test_carbon_dioxide_rating() {
        let numbers = vec![
            "00100".into(),
            "11110".into(),
            "10110".into(),
            "10111".into(),
            "10101".into(),
            "01111".into(),
            "00111".into(),
            "11100".into(),
            "10000".into(),
            "11001".into(),
            "00010".into(),
            "01010".into(),
        ];

        let binary_numbers = BinaryNumbers::new(numbers);

        let numbers_to_keep =
            life_support_rating_base(&binary_numbers, &LifeSupportMode::CarbonDioxide, 0);
        let binary_numbers = BinaryNumbers::new(vec![
            "00100".into(),
            "01111".into(),
            "00111".into(),
            "00010".into(),
            "01010".into(),
        ]);
        assert_eq!(numbers_to_keep, binary_numbers);

        let numbers_to_keep =
            life_support_rating_base(&binary_numbers, &LifeSupportMode::CarbonDioxide, 1);
        let binary_numbers = BinaryNumbers::new(vec!["01111".into(), "01010".into()]);
        assert_eq!(numbers_to_keep, binary_numbers);

        let numbers_to_keep =
            life_support_rating_base(&binary_numbers, &LifeSupportMode::CarbonDioxide, 2);
        let binary_numbers = BinaryNumbers::new(vec!["01010".into()]);
        assert_eq!(numbers_to_keep, binary_numbers);
    }

    #[test]
    fn test_binary_number_value() {
        let binary_number: BinaryNumber = "1".into();
        assert_eq!(binary_number.value(), 1);

        let binary_number: BinaryNumber = "10".into();
        assert_eq!(binary_number.value(), 2);

        let binary_number: BinaryNumber = "10000000000".into();
        assert_eq!(binary_number.value(), 1024);
    }
}
