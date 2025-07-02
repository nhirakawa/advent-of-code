use crate::common::parse::unsigned_number;
use nom::{bytes::complete::tag, multi::separated_list1, Parser};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let depths = parse(input);

    let mut increases = 0;
    for i in 0..depths.len() - 1 {
        if depths[i + 1] > depths[i] {
            increases += 1;
        }
    }

    Ok(increases.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let depths = parse(input);

    let mut increases = 0;

    for i in 0..depths.len() - 3 {
        let first_window = depths[i] + depths[i + 1] + depths[i + 2];
        let second_window = depths[i + 1] + depths[i + 2] + depths[i + 3];

        if second_window > first_window {
            increases += 1;
        }
    }

    Ok(increases.to_string())
}

fn parse(i: &str) -> Vec<usize> {
    separated_list1(tag("\n"), unsigned_number)
        .parse(i)
        .unwrap()
        .1
}
