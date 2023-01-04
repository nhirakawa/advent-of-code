use common::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-1.txt").trim();

    let digits = parse(input);

    let part_one = part_one(&digits);
    let part_two = part_two(&digits);

    Ok((part_one, part_two))
}

fn part_one(digits: &[u32]) -> PartAnswer {
    let start = SystemTime::now();

    let sum = sum_similar_digits(digits, 1);

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(sum, elapsed)
}

fn part_two(digits: &[u32]) -> PartAnswer {
    let start = SystemTime::now();

    let sum = sum_similar_digits(digits, digits.len() / 2);

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(sum, elapsed)
}

fn sum_similar_digits(digits: &[u32], step: usize) -> u32 {
    let mut sum = 0;

    for index in 0..digits.len() {
        let first = digits[index];
        let second = digits[(index + step) % digits.len()];

        if first == second {
            sum += first;
        }
    }

    sum
}

fn parse(i: &str) -> Vec<u32> {
    i.chars()
        .filter_map(|d| d.to_string().parse::<u32>().ok())
        .collect()
}
