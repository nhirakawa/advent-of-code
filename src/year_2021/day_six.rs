use crate::common::parse::unsigned_number;
use nom::{bytes::complete::tag, multi::separated_list1, Parser};
use std::collections::HashMap;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let numbers = parse(input);

    let mut number_of_fish_by_day = HashMap::new();

    for number in &numbers {
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

    Ok(number_of_fish_by_day.values().sum::<usize>())
}

fn breed(number_of_fish_by_day: &HashMap<u8, usize>) -> HashMap<u8, usize> {
    let mut result = HashMap::new();

    for (day, count) in number_of_fish_by_day.iter() {
        if *day == 0 {
            *result.entry(6).or_insert(0) += count;
            *result.entry(8).or_insert(0) += count;
        } else {
            let new_day = *day - 1;

            *result.entry(new_day).or_insert(0) += count;
        }
    }
    result
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let numbers = parse(input);

    let mut number_of_fish_by_day = HashMap::new();

    for number in &numbers {
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

    Ok(number_of_fish_by_day.values().sum::<usize>())
}

fn parse(i: &str) -> Vec<u8> {
    separated_list1(tag(","), unsigned_number)
        .parse(i)
        .unwrap()
        .1
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
