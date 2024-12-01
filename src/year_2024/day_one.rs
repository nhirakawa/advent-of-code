use anyhow::bail;
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let (mut left, mut right) = parse_numbers(input)?;

    left.sort();
    right.sort();

    let answer = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| u32::abs_diff(*l, *r))
        .sum::<u32>()
        .to_string();

    Ok(answer)
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let (left, right) = parse_numbers(input)?;

    let answer = left
        .iter()
        .map(|u| u * count_occurrences(u, &right))
        .sum::<u32>()
        .to_string();

    Ok(answer)
}

fn count_occurrences<'a, I: IntoIterator<Item = &'a u32>>(target: &u32, numbers: I) -> u32 {
    numbers.into_iter().filter(|n| *n == target).count() as u32
}

fn parse_numbers(input: &str) -> anyhow::Result<(Vec<u32>, Vec<u32>)> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let line = line.trim();
        let tokens = line.split_whitespace().collect_vec();

        if tokens.len() != 2 {
            bail!("Could not parse line: {}", line);
        }

        let left_token = tokens[0].parse::<u32>().map_err(anyhow::Error::from)?;
        let right_token = tokens[1].parse::<u32>().map_err(anyhow::Error::from)?;

        left.push(left_token);
        right.push(right_token);
    }

    Ok((left, right))
}
