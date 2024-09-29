use std::time::SystemTime;
use crate::common::answer::*;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_opt, multi::separated_list1,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = parse_input(include_str!("input/day-1.txt"));

    let part_one = part_one(&input);
    let part_two = part_two(&input);

    Ok((part_one, part_two))
}

fn part_one(modules: &[u32]) -> PartAnswer {
    let start = SystemTime::now();

    let solution: u32 = modules.iter().map(|module| calculate_fuel(*module)).sum();
    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(solution, elapsed)
}

fn part_two(modules: &[u32]) -> PartAnswer {
    let start = SystemTime::now();

    let solution: u32 = modules
        .iter()
        .map(|module| calculate_fuel_recursive(*module))
        .sum();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(solution, elapsed)
}

fn calculate_fuel(module: u32) -> u32 {
    (module / 3) - 2
}

fn calculate_fuel_recursive(module: u32) -> u32 {
    if module <= 6 {
        0
    } else {
        let cost = calculate_fuel(module);
        cost + calculate_fuel_recursive(cost)
    }
}

fn parse_input(i: &str) -> Vec<u32> {
    let (_, input) = separated_list1(tag("\n"), number)(i).unwrap();
    input
}

fn number(i: &str) -> IResult<&str, u32> {
    map_opt(digit1, |s: &str| s.parse::<u32>().ok())(i)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn test_calculate_fuel_recursive() {
        assert_eq!(calculate_fuel_recursive(14), 2);
        assert_eq!(calculate_fuel_recursive(1969), 966);
        assert_eq!(calculate_fuel_recursive(100756), 50346);
    }
}
