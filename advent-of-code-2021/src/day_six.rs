use std::collections::HashMap;

use common::{parse::unsigned_number, prelude::*};
use nom::{bytes::complete::tag, multi::separated_list1};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-6.txt");
    let numbers = parse(input);

    let part_one = part_one(&numbers);
    let part_two = part_two(&numbers);

    Ok((part_one, part_two))
}

fn part_one(numbers: &[u8]) -> PartAnswer {
    let start = SystemTime::now();
    let mut number_of_fish_by_day = HashMap::new();

    for number in numbers {
        if !number_of_fish_by_day.contains_key(number) {
            number_of_fish_by_day.insert(*number, 0);
        }

        if let Some(count) = number_of_fish_by_day.get_mut(number) {
            *count += 1;
        }
    }

    for _ in 0..80 {
        number_of_fish_by_day = breed(&number_of_fish_by_day);
    }

    let solution: usize = number_of_fish_by_day.values().sum();

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn breed(number_of_fish_by_day: &HashMap<u8, usize>) -> HashMap<u8, usize> {
    let mut result = HashMap::new();

    for (day, count) in number_of_fish_by_day.iter() {
        if *day == 0 {
            if !result.contains_key(&6) {
                result.insert(6, 0);
            }

            if let Some(old_count) = result.get_mut(&6) {
                *old_count += count;
            }

            if !result.contains_key(&8) {
                result.insert(8, 0);
            }

            if let Some(old_count) = result.get_mut(&8) {
                *old_count += count;
            }
        } else {
            let new_day = *day - 1;

            if !result.contains_key(&new_day) {
                result.insert(new_day, 0);
            }

            if let Some(old_count) = result.get_mut(&new_day) {
                *old_count += count;
            }
        }
    }
    result
}

fn part_two(numbers: &[u8]) -> PartAnswer {
    let start = SystemTime::now();
    let mut number_of_fish_by_day = HashMap::new();

    for number in numbers {
        if !number_of_fish_by_day.contains_key(number) {
            number_of_fish_by_day.insert(*number, 0);
        }

        if let Some(count) = number_of_fish_by_day.get_mut(number) {
            *count += 1;
        }
    }

    for _ in 0..256 {
        number_of_fish_by_day = breed(&number_of_fish_by_day);
    }

    let solution: usize = number_of_fish_by_day.values().sum();

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn parse(i: &str) -> Vec<u8> {
    separated_list1(tag(","), unsigned_number)(i).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breed() {
        let count_by_days = vec![(3, 2), (4, 1), (1, 1), (2, 1)]
            .into_iter()
            .collect::<HashMap<u8, usize>>();

        let new_count = breed(&count_by_days);

        assert_eq!(
            new_count,
            vec![(2, 2), (3, 1), (0, 1), (1, 1)].into_iter().collect()
        );

        let count_by_days = new_count;
        let new_count = breed(&count_by_days);

        assert_eq!(
            new_count,
            vec![(1, 2), (2, 1), (6, 1), (0, 1), (8, 1)]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn test_breed_days() {
        let mut count_by_days = vec![(3, 2), (4, 1), (1, 1), (2, 1)]
            .into_iter()
            .collect::<HashMap<u8, usize>>();

        for _ in 0..18 {
            count_by_days = breed(&count_by_days);
        }

        let total_fish: usize = count_by_days.values().sum();

        assert_eq!(total_fish, 26);

        for _ in 18..80 {
            count_by_days = breed(&count_by_days);
        }

        let total_fish: usize = count_by_days.values().sum();

        assert_eq!(total_fish, 5934);
    }
}
