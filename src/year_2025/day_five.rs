use std::ops::RangeInclusive;

use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (fresh_ingredients, ingredients) = parser::parse(input)?;
    let mut count = 0;
    for ingredient in ingredients {
        let is_fresh = fresh_ingredients
            .iter()
            .any(|range| range.contains(&ingredient));
        if is_fresh {
            count += 1;
        }
    }
    Ok(count)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (fresh_ingredient_ranges, _ingredients) = parser::parse(input)?;

    let ranges = merge_ranges(fresh_ingredient_ranges);

    let mut count = 0;

    for range in ranges {
        count += range.end() - range.start() + 1;
    }

    Ok(count)
}

fn merge_ranges(ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    let ranges = ranges
        .into_iter()
        .sorted_by_key(|range| (*range.start(), *range.end()))
        .collect_vec();

    let mut merged: Vec<RangeInclusive<usize>> = Vec::new();
    let mut current = ranges[0].clone();

    for range in ranges.into_iter().skip(1) {
        if *current.end() >= range.start().saturating_sub(1) {
            // add onto existing range
            let new_end = (*current.end()).max(*range.end());
            current = *current.start()..=new_end;
        } else {
            // start a new range
            merged.push(current);
            current = range;
        }
    }

    // push the range-in-progress
    merged.push(current);

    merged
}

mod parser {
    use std::ops::RangeInclusive;

    use nom::{
        IResult, Parser, bytes::complete::tag, combinator::map, multi::separated_list1,
        sequence::separated_pair,
    };

    use crate::common::parse::{finish, unsigned_number};

    pub fn parse(i: &str) -> anyhow::Result<(Vec<RangeInclusive<usize>>, Vec<usize>)> {
        finish(
            separated_pair(fresh_ingredient_ranges, tag("\n\n"), ingredients),
            i,
        )
    }

    fn fresh_ingredient_ranges(i: &str) -> IResult<&str, Vec<RangeInclusive<usize>>> {
        separated_list1(tag("\n"), fresh_ingredient_range).parse(i)
    }

    fn fresh_ingredient_range(i: &str) -> IResult<&str, RangeInclusive<usize>> {
        map(
            separated_pair(unsigned_number, tag("-"), unsigned_number),
            |(first, second)| first..=second,
        )
        .parse(i)
    }

    fn ingredients(i: &str) -> IResult<&str, Vec<usize>> {
        separated_list1(tag("\n"), ingredient).parse(i)
    }

    fn ingredient(i: &str) -> IResult<&str, usize> {
        unsigned_number(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_ranges() {
        assert_eq!(merge_ranges(vec![0..=1, 2..=3]), vec![0..=3]);
        assert_eq!(merge_ranges(vec![0..=1, 4..=5]), vec![0..=1, 4..=5]);
        assert_eq!(merge_ranges(vec![0..=5, 4..=5]), vec![0..=5]);
        assert_eq!(merge_ranges(vec![0..=3, 2..=7]), vec![0..=7]);
    }
}
