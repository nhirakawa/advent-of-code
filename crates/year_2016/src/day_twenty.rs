use std::ops::RangeInclusive;

use anyhow::{anyhow, bail};
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let ranges = ranges(input)?;
    let merged = merge_ranges(&ranges);

    let Some(first) = merged.first() else {
        bail!("No first merged range")
    };

    Ok(first.end() + 1)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let ranges = ranges(input)?;
    let merged = merge_ranges(&ranges);

    let blacklisted_count: u32 = merged
        .into_iter()
        .map(|range| *range.end() - *range.start() + 1)
        .sum();

    Ok(u32::MAX - blacklisted_count + 1)
}

fn merge_ranges(ranges: &[RangeInclusive<u32>]) -> Vec<RangeInclusive<u32>> {
    let mut merged: Vec<RangeInclusive<u32>> = Vec::new();

    for range in ranges.iter().sorted_by_key(|range| *range.start()) {
        if let Some(last) = merged.last_mut() {
            // Deref so this calls RangeInclusive::contains rather than the
            // consuming Itertools::contains on the &mut iterator.
            let overlaps = (*last).contains(range.start());
            let adjacent = last.end().checked_add(1) == Some(*range.start());

            if overlaps || adjacent {
                *last = *last.start()..=*last.end().max(range.end());
                continue;
            }
        }

        merged.push(range.clone());
    }

    merged
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_ranges() {
        // Example from the puzzle: overlapping, adjacent, and unsorted ranges.
        let ranges = vec![5..=8, 0..=2, 4..=7];
        assert_eq!(merge_ranges(&ranges), vec![0..=2, 4..=8]);

        // Adjacent ranges merge; a contained range doesn't shrink the end.
        let ranges = vec![0..=2, 3..=10, 4..=6];
        assert_eq!(merge_ranges(&ranges), vec![0..=10]);

        // A gap of one value keeps ranges separate.
        let ranges = vec![0..=2, 4..=6];
        assert_eq!(merge_ranges(&ranges), vec![0..=2, 4..=6]);

        // No overflow at the top of the u32 range.
        let ranges = vec![0..=u32::MAX, 5..=10];
        assert_eq!(merge_ranges(&ranges), vec![0..=u32::MAX]);
    }
}
