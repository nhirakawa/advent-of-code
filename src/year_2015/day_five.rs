use std::collections::HashMap;

use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let is_nice_string = |s: &str| -> bool {
        contains_three_vowels(s) && contains_repeated_letter(s) && does_not_contain_bad_strings(s)
    };

    let count = count_nice_strings(input, is_nice_string);

    Ok(count.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let is_nice_string = |s: &str| -> bool {
        contains_duplicated_pair(s) && contains_repeated_letter_with_one_between(s)
    };

    let count = count_nice_strings(input, is_nice_string);

    Ok(count.to_string())
}

fn count_nice_strings(input: &str, rule: fn(&str) -> bool) -> usize {
    input.trim().split("\n").filter(|s| rule(s)).count()
}

// Part one rules

fn contains_three_vowels(s: &str) -> bool {
    s.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
}

fn contains_repeated_letter(s: &str) -> bool {
    s.chars().tuple_windows().any(|(a, b)| a == b)
}

fn does_not_contain_bad_strings(s: &str) -> bool {
    let bad_strings = vec!["ab", "cd", "pq", "xy"];
    for bad_string in bad_strings {
        if s.contains(bad_string) {
            return false;
        }
    }
    true
}

// Part two rules

fn contains_duplicated_pair(s: &str) -> bool {
    // contains map of pairs to list of their starting indexes
    let mut map: HashMap<(char, char), Vec<usize>> = HashMap::new();

    for (index, pair) in s.chars().tuple_windows().enumerate() {
        map.entry(pair).or_default().push(index);
    }

    for indexes in map.values() {
        if indexes.len() < 2 {
            continue;
        }

        if indexes.iter().tuple_windows().any(|(a, b)| b - a > 1) {
            return true;
        }
    }

    false
}

fn contains_repeated_letter_with_one_between(s: &str) -> bool {
    let chars = s.chars().collect_vec();

    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }

    false
}
