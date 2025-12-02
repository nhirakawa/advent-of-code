use std::ops::RangeInclusive;

use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let ranges = parse_ranges(input)?;

    let mut sum = 0;

    for range in ranges {
        for n in range {
            if is_repeated_sequence(n) {
                sum += n;
            }
        }
    }

    Ok(sum)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let ranges = parse_ranges(input)?;

    let mut sum = 0;

    for range in ranges {
        for n in range {
            if has_repeating_sequence(n) {
                sum += n;
            }
        }
    }

    Ok(sum)
}

/// Returns true if `u` contains 2 concatenated halves (e.g. "11", "6464", but not "101"), false otherwise
fn is_repeated_sequence(u: u64) -> bool {
    let s = u.to_string();
    if s.len() % 2 != 0 {
        return false;
    }

    let (first, second) = s.split_at(s.len() / 2);
    first == second
}

fn has_repeating_sequence(u: u64) -> bool {
    let s = u.to_string();

    for length in 1..s.len() {
        if s.len() % length != 0 {
            // Only check substrings that cover the original string
            continue;
        }

        let chunks = s.as_bytes().chunks(length).collect_vec();

        let first = chunks[0];

        if chunks.iter().skip(1).all(|chunk| chunk == &first) {
            return true;
        }
    }

    false
}

fn parse_ranges(input: &str) -> anyhow::Result<Vec<RangeInclusive<u64>>> {
    let mut ranges = Vec::new();

    for id_range in input.trim().split(",") {
        let (prefix, suffix) = id_range
            .split_once("-")
            .ok_or(anyhow::anyhow!("Could not split on '-'"))?;

        let low = prefix.parse()?;
        let high = suffix.parse()?;

        ranges.push(low..=high);
    }

    Ok(ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_repeated_sequence() {
        assert_eq!(is_repeated_sequence(55), true);
        assert_eq!(is_repeated_sequence(6464), true);
        assert_eq!(is_repeated_sequence(123123), true);
        assert_eq!(is_repeated_sequence(101), false);

        assert_eq!(is_repeated_sequence(11), true);
        assert_eq!(is_repeated_sequence(22), true);
        assert_eq!(is_repeated_sequence(99), true);
        assert_eq!(is_repeated_sequence(1010), true);
    }

    #[test]
    fn test_has_repeated_sequence() {
        assert_eq!(has_repeating_sequence(12341234), true);
        assert_eq!(has_repeating_sequence(123123123), true);
        assert_eq!(has_repeating_sequence(1212121212), true);
        assert_eq!(has_repeating_sequence(1111111), true);
        assert_eq!(has_repeating_sequence(11), true);

        assert_eq!(has_repeating_sequence(999), true);
        assert_eq!(has_repeating_sequence(1010), true);
    }
}
