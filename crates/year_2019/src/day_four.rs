use anyhow::anyhow;
use anyhow::Context;
use itertools::Itertools;
use multiset::HashMultiSet;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (low, high) = parse(input)?;
    Ok((low..high).filter(|n| is_valid_part_one(*n)).count())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (low, high) = parse(input)?;
    Ok((low..high).filter(|n| is_valid_part_two(*n)).count())
}

fn is_valid_part_one(n: u32) -> bool {
    let split = split(n);

    let mut map = HashMultiSet::new();

    let mut last_seen = 0;

    for num in &split {
        if *num < last_seen {
            return false;
        }

        map.insert(*num);

        last_seen = *num;
    }

    let mut found_double = false;

    for num in split {
        let count = map.count_of(&num);

        if count >= 2 {
            found_double = true;
        }
    }

    if !found_double {
        return false;
    }

    true
}

fn is_valid_part_two(n: u32) -> bool {
    let split = split(n);

    let mut map = HashMultiSet::new();

    let mut last_seen = 0;

    for num in &split {
        if *num < last_seen {
            return false;
        }

        map.insert(*num);

        last_seen = *num;
    }

    let mut found_double = false;

    for num in split {
        let count = map.count_of(&num);

        if count == 2 {
            found_double = true;
        }
    }

    if !found_double {
        return false;
    }

    true
}

fn split(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn parse(input: &str) -> anyhow::Result<(u32, u32)> {
    if let Some((low, high)) = input.split("-").collect_tuple() {
        let low = low
            .parse()
            .with_context(|| format!("Could not parse {low} as u32"))?;
        let high = high
            .parse()
            .with_context(|| format!("Could not parse {high} as u32"))?;

        Ok((low, high))
    } else {
        Err(anyhow!("Could not split {input} into two numbers"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert_eq!(split(123), vec![1, 2, 3]);
    }

    #[test]
    fn test_valid_part_one() {
        assert!(is_valid_part_one(111111));
        assert!(!is_valid_part_one(223450));
        assert!(!is_valid_part_one(123789));
    }

    #[test]
    fn test_valid_part_two() {
        assert!(is_valid_part_two(112233));
        assert!(!is_valid_part_two(123444));
        assert!(is_valid_part_two(111122));
    }
}
