use std::collections::HashMap;

use anyhow::anyhow;
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let (towels, patterns) = parse(input)?;

    Ok(patterns
        .iter()
        .filter(|pattern| count_possibilities(pattern, &towels, &mut HashMap::new()) > 0)
        .count()
        .to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let (towels, patterns) = parse(input)?;

    let mut count = 0;

    for pattern in patterns {
        let mut memo = HashMap::new();
        let possibilities = count_possibilities(&pattern, &towels, &mut memo);
        count += possibilities;
    }

    Ok(count.to_string())
}

fn count_possibilities<'a>(
    pattern: &'a str,
    towels: &'a [String],
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(possibilities) = memo.get(pattern) {
        return *possibilities;
    }

    let mut possibilities = 0;

    for towel in towels {
        if pattern.starts_with(towel) {
            let rest = &pattern[towel.len()..];
            let possibilities_for_towel = count_possibilities(rest, towels, memo);
            possibilities += possibilities_for_towel;
        }
    }

    memo.insert(pattern, possibilities);

    possibilities
}

fn parse(input: &str) -> anyhow::Result<(Vec<String>, Vec<String>)> {
    let available_towels = input
        .lines()
        .next()
        .map(|line| line.split(", ").map(|s| s.to_string()).collect_vec())
        .ok_or(anyhow!("Could not parse available towels"))?;

    let requested_patterns = input
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|s| s.to_string())
        .collect_vec();

    Ok((available_towels, requested_patterns))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_possible() {
        // Towels: r, wr, b, g, bwu, rb, gb, br

        let towels = vec![
            "r".to_string(),
            "wr".to_string(),
            "b".to_string(),
            "g".to_string(),
            "bwu".to_string(),
            "rb".to_string(),
            "gb".to_string(),
            "br".to_string(),
        ];

        assert!(count_possibilities("brwrr", &towels, &mut HashMap::new()) > 0);
        assert!(count_possibilities("bggr", &towels, &mut HashMap::new()) > 0);
        assert!(count_possibilities("gbbr", &towels, &mut HashMap::new()) > 0);
        assert!(count_possibilities("rrbgbr", &towels, &mut HashMap::new()) > 0);
        assert!(count_possibilities("ubwu", &towels, &mut HashMap::new()) == 0);
        assert!(count_possibilities("bwurrg", &towels, &mut HashMap::new()) > 0);
        assert!(count_possibilities("brgr", &towels, &mut HashMap::new()) > 0);
        assert!(count_possibilities("bbrgwb", &towels, &mut HashMap::new()) == 0);
    }
}
