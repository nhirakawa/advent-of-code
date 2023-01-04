use std::collections::HashSet;

use common::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-6.txt").trim();

    let part_one = part_one(&input);
    let part_two = part_two(&input);

    Ok((part_one, part_two))
}

fn part_one(buffer: &str) -> PartAnswer {
    let start = SystemTime::now();

    let ending_index = ending_index_with_unique_characters(buffer, 4);

    let answer = ending_index.unwrap() + 1;

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn part_two(buffer: &str) -> PartAnswer {
    let start = SystemTime::now();

    let ending_index = ending_index_with_unique_characters(buffer, 14);

    let answer = ending_index.unwrap() + 1;

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn ending_index_with_unique_characters(s: &str, length: usize) -> Option<usize> {
    let char_indices = s.char_indices().collect::<Vec<(usize, char)>>();

    char_indices
        .windows(length)
        .filter(|w| has_unique_characters(w))
        .filter_map(|window| window.iter().last().map(|(index, _)| index.clone()))
        .next()
}

fn has_unique_characters(window: &[(usize, char)]) -> bool {
    let set = window
        .iter()
        .map(|(_, c)| c)
        .cloned()
        .collect::<HashSet<char>>();

    set.len() == window.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ending_index_with_unique_characters() {
        assert_eq!(
            ending_index_with_unique_characters("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),
            Some(6)
        );
        assert_eq!(
            ending_index_with_unique_characters("bvwbjplbgvbhsrlpgdmjqwftvncz", 4),
            Some(4)
        );
        assert_eq!(
            ending_index_with_unique_characters("nppdvjthqldpwncqszvftbrmjlhg", 4),
            Some(5)
        );
        assert_eq!(
            ending_index_with_unique_characters("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            Some(9)
        );
        assert_eq!(
            ending_index_with_unique_characters("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            Some(10)
        );

        assert_eq!(
            ending_index_with_unique_characters("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            Some(18)
        );
    }
}
