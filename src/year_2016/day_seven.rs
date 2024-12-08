use std::{collections::VecDeque, ops::RangeInclusive};

use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let count = input
        .trim()
        .lines()
        .filter(|line| supports_tls(line))
        .count();
    Ok(count.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let count = input
        .trim()
        .lines()
        .filter(|line| supports_ssl(line))
        .count();

    Ok(count.to_string())
}

fn supports_tls(ip: &str) -> bool {
    let bracket_indexes = find_bracket_indexes(ip);
    let mut match_indexes = vec![];

    for ((first_index, first_char), (_, second_char), (_, third_char), (_, fourth_char)) in
        ip.char_indices().tuple_windows()
    {
        if first_char == '['
            || first_char == ']'
            || second_char == '['
            || second_char == ']'
            || third_char == '['
            || third_char == ']'
            || fourth_char == '['
            || fourth_char == ']'
        {
            continue;
        }
        if first_char == second_char {
            continue;
        }

        if first_char != fourth_char {
            continue;
        }

        if second_char != third_char {
            continue;
        }

        match_indexes.push(first_index);
    }

    if match_indexes.is_empty() {
        return false;
    }

    for index in match_indexes {
        if within_any_brackets(index, &bracket_indexes) {
            return false;
        }
    }

    true
}

fn supports_ssl(ip: &str) -> bool {
    let bracket_indexes = find_bracket_indexes(ip);
    let aba_indexes = find_aba_indexes(ip, &bracket_indexes);

    for (_, a, b) in aba_indexes {
        if has_bab_sequence(ip, a, b, &bracket_indexes) {
            return true;
        }
    }

    false
}

/// Returns a list of indexes of the first character of an ABA sequence
/// and the two characters that make up the ABA sequence (A followed by B).
/// Ignores ABA sequences that are within brackets.
fn find_aba_indexes(
    ip: &str,
    bracket_ranges: &[RangeInclusive<usize>],
) -> Vec<(usize, char, char)> {
    let mut aba_indexes = vec![];

    for ((first_index, first_char), (_, second_char), (_, third_char)) in
        ip.char_indices().tuple_windows()
    {
        if first_char == '['
            || first_char == ']'
            || second_char == '['
            || second_char == ']'
            || third_char == '['
            || third_char == ']'
        {
            continue;
        }

        if first_char == second_char {
            continue;
        }

        if first_char != third_char {
            continue;
        }

        if within_any_brackets(first_index, bracket_ranges) {
            continue;
        }

        aba_indexes.push((first_index, first_char, second_char));
    }

    aba_indexes
}

fn has_bab_sequence(ip: &str, a: char, b: char, bracket_indexes: &[RangeInclusive<usize>]) -> bool {
    for ((first_index, first_char), (_, second_char), (_, third_char)) in
        ip.char_indices().tuple_windows()
    {
        if first_char == '['
            || first_char == ']'
            || second_char == '['
            || second_char == ']'
            || third_char == '['
            || third_char == ']'
        {
            continue;
        }

        if first_char == b
            && second_char == a
            && third_char == b
            && within_any_brackets(first_index, bracket_indexes)
        {
            return true;
        }
    }

    false
}

fn within_any_brackets(index: usize, bracket_indexes: &[RangeInclusive<usize>]) -> bool {
    for bracket_range in bracket_indexes {
        if bracket_range.contains(&index) {
            return true;
        }
    }
    false
}

fn find_bracket_indexes(s: &str) -> Vec<RangeInclusive<usize>> {
    let mut result = vec![];
    let mut stack = VecDeque::new();
    for (index, c) in s.char_indices() {
        match c {
            '[' => stack.push_back(index),
            ']' => {
                if let Some(start) = stack.pop_back() {
                    result.push(start..=index);
                }
            }
            _ => {}
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supports_tls() {
        assert_eq!(supports_tls("abba[mnop]qrst"), true);
        assert_eq!(supports_tls("abcd[bddb]xyyx"), false);
        assert_eq!(supports_tls("aaaa[qwer]tyui"), false);
        assert_eq!(supports_tls("ioxxoj[asdfgh]zxcvbn"), true);
    }

    #[test]
    fn test_find_aba_indexes() {
        assert_eq!(
            find_aba_indexes("aba[bab]xyz", &[3..=7]),
            vec![(0, 'a', 'b')]
        );
    }

    #[test]
    fn test_has_bab_sequence() {
        assert!(has_bab_sequence("aba[bab]xyz", 'a', 'b', &[3..=7]));
    }
}
