use std::time::SystemTime;
use multiset::HashMultiSet;

use crate::common::answer::*;

const LOW: u32 = 138241;
const HIGH: u32 = 674034;

pub fn run() -> AdventOfCodeResult {
    let part_one = part_one();
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one() -> PartAnswer {
    let start = SystemTime::now();
    let solution = (LOW..HIGH).filter(|n| is_valid_part_one(*n)).count();
    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two() -> PartAnswer {
    let start = SystemTime::now();
    let solution = (LOW..HIGH).filter(|n| is_valid_part_two(*n)).count();
    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn is_valid_part_one(n: u32) -> bool {
    let split = split(n);

    let mut map = HashMultiSet::new();

    let mut last_seen = 0;

    for num in &split {
        if *num < last_seen {
            return false;
        }

        map.insert(*num);

        last_seen = *num;
    }

    let mut found_double = false;

    for num in split {
        let count = map.count_of(&num);

        if count >= 2 {
            found_double = true;
        }
    }

    if !found_double {
        return false;
    }

    true
}

fn is_valid_part_two(n: u32) -> bool {
    let split = split(n);

    let mut map = HashMultiSet::new();

    let mut last_seen = 0;

    for num in &split {
        if *num < last_seen {
            return false;
        }

        map.insert(*num);

        last_seen = *num;
    }

    let mut found_double = false;

    for num in split {
        let count = map.count_of(&num);

        if count == 2 {
            found_double = true;
        }
    }

    if !found_double {
        return false;
    }

    true
}

fn split(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert_eq!(split(123), vec![1, 2, 3]);
    }

    #[test]
    fn test_valid_part_one() {
        assert!(is_valid_part_one(111111));
        assert!(!is_valid_part_one(223450));
        assert!(!is_valid_part_one(123789));
    }

    #[test]
    fn test_valid_part_two() {
        assert!(is_valid_part_two(112233));
        assert!(!is_valid_part_two(123444));
        assert!(is_valid_part_two(111122));
    }
}
