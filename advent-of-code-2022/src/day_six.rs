use std::{collections::HashSet, str::CharIndices};

use common::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-6.txt").trim();

    let part_one = part_one(input);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(buffer: &str) -> PartAnswer {
    let start = SystemTime::now();

    let ending_index = ending_index_with_four_unique_characters(buffer);

    let answer = ending_index.unwrap() + 1;

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn ending_index_with_four_unique_characters(s: &str) -> Option<usize> {
    let char_indices = s.char_indices().collect::<Vec<(usize, char)>>();

    char_indices
        .windows(4)
        .filter(|w| has_four_unique_characters(w))
        .filter_map(|window| window.iter().last().map(|(index, _)| index.clone()))
        .next()
}

fn has_four_unique_characters(window: &[(usize, char)]) -> bool {
    let set = window
        .iter()
        .map(|(_, c)| c)
        .cloned()
        .collect::<HashSet<char>>();

    set.len() == 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ending_index_with_four_unique_characters() {
        assert_eq!(
            ending_index_with_four_unique_characters("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            Some(6)
        );
        assert_eq!(
            ending_index_with_four_unique_characters("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(4)
        );
        assert_eq!(
            ending_index_with_four_unique_characters("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(5)
        );
        assert_eq!(
            ending_index_with_four_unique_characters("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(9)
        );
        assert_eq!(
            ending_index_with_four_unique_characters("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(10)
        );
    }
}
