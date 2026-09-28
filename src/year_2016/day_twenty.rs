use std::ops::RangeInclusive;

use anyhow::anyhow;
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let ranges = ranges(input)?;
    let ranges = ranges
        .into_iter()
        .sorted_by_key(|range| *range.start())
        .collect_vec();

    for (index, range) in ranges.iter().enumerate() {
        println!("Range#{index}: {range:?}");
    }

    println!();

    for ((first_index, first), (second_index, second)) in ranges.iter().enumerate().tuple_windows()
    {
        if first.contains(second.start()) {
            println!("Range#{first_index} overlaps with Range#{second_index}");
        }
        if first.end() + 1 == *second.start() {
            println!("Range#{first_index} can be extended with Range#{second_index}");
        }
    }

    Err::<usize, _>(anyhow!("Not implemented"))
}
pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

fn ranges(s: &str) -> anyhow::Result<Vec<RangeInclusive<u32>>> {
    let mut ranges = Vec::new();

    for line in s.lines() {
        let range = range(line)?;
        ranges.push(range);
    }

    Ok(ranges)
}

fn range(s: &str) -> anyhow::Result<RangeInclusive<u32>> {
    let (start, end) = s
        .split_once("-")
        .ok_or(anyhow!("Could not split IP address range {s} on '-'"))?;

    let start = start.parse()?;
    let end = end.parse()?;

    Ok(start..=end)
}
