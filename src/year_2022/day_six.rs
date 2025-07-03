use anyhow::anyhow;
use std::collections::HashSet;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let ending_index = ending_index_with_unique_characters(input, 4);

    ending_index
        .map(|idx| idx + 1)
        .ok_or(anyhow!("No index found"))
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let ending_index = ending_index_with_unique_characters(input, 14);

    ending_index
        .map(|idx| idx + 1)
        .ok_or(anyhow!("No index found"))
}

fn ending_index_with_unique_characters(s: &str, length: usize) -> Option<usize> {
    let char_indices = s.char_indices().collect::<Vec<(usize, char)>>();

    char_indices
        .windows(length)
        .filter(|w| has_unique_characters(w))
        .filter_map(|window| window.iter().last().map(|(index, _)| *index))
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
