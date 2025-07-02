use crate::common::{
    math::triangular_number,
    parse::{finish, number},
};
use anyhow::anyhow;
use nom::{bytes::complete::tag, combinator::map, sequence::separated_pair, IResult, Parser};
use std::ops::RangeInclusive;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let (horizontal_range, vertical_range) = parse(input)?;
    let hit_velocities = find_valid_initial_velocities(&horizontal_range, &vertical_range);

    let max_y_velocity = hit_velocities
        .iter()
        .map(|(_, y)| y)
        .max()
        .copied()
        .ok_or(anyhow!("No max y velocity found"))?;

    Ok(triangular_number(max_y_velocity as usize).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let (horizontal_range, vertical_range) = parse(input)?;
    let hit_velocities = find_valid_initial_velocities(&horizontal_range, &vertical_range);

    Ok(hit_velocities.len().to_string())
}

fn find_valid_initial_velocities(
    horizontal_range: &RangeInclusive<i64>,
    vertical_range: &RangeInclusive<i64>,
) -> Vec<(i64, i64)> {
    let mut hit_velocities = Vec::new();

    for x in 0..=250 {
        for y in -200..=200 {
            let probe = Probe::new(x, y);

            if hits_target_area(probe, horizontal_range, vertical_range) {
                hit_velocities.push((x, y));
            }
        }
    }

    hit_velocities
}

fn hits_target_area(
    mut probe: Probe,
    horizontal_range: &RangeInclusive<i64>,
    vertical_range: &RangeInclusive<i64>,
) -> bool {
    loop {
        probe.step();

        if horizontal_range.contains(&probe.x_position)
            && vertical_range.contains(&probe.y_position)
        {
            return true;
        }

        // if we're too far right
        if probe.x_position > *horizontal_range.end() {
            return false;
        }

        // if we're too far down
        if probe.y_position < *vertical_range.start() {
            return false;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Probe {
    x_position: i64,
    y_position: i64,
    x_velocity: i64,
    y_velocity: i64,
}

impl Probe {
    fn new(x_velocity: i64, y_velocity: i64) -> Probe {
        Probe {
            x_position: 0,
            y_position: 0,
            x_velocity,
            y_velocity,
        }
    }

    fn step(&mut self) {
        self.x_position += self.x_velocity;
        self.y_position += self.y_velocity;

        self.x_velocity -= self.x_velocity.signum();
        self.y_velocity -= 1;
    }
}

fn parse(input: &str) -> anyhow::Result<(RangeInclusive<i64>, RangeInclusive<i64>)> {
    finish(ranges, input)
}

fn ranges(input: &str) -> IResult<&str, (RangeInclusive<i64>, RangeInclusive<i64>)> {
    map(
        (tag("target area: x="), range, tag(", y="), range),
        |(_, lower, _, upper)| (lower, upper),
    )
    .parse(input)
}

fn range(i: &str) -> IResult<&str, RangeInclusive<i64>> {
    map(
        separated_pair(number, tag(".."), number),
        |(lower, upper)| lower..=upper,
    )
    .parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe() {
        let mut probe = Probe::new(7, 2);
        probe.step();

        assert_eq!((probe.x_position, probe.y_position), (7, 2));
        assert_eq!((probe.x_velocity, probe.y_velocity), (6, 1));

        probe.step();

        assert_eq!((probe.x_position, probe.y_position), (13, 3));
        assert_eq!((probe.x_velocity, probe.y_velocity), (5, 0));
    }

    #[test]
    fn test_probe_iterated() {
        let horizontal_range = 20..=30;
        let vertical_range = -10..=-5;

        let probe = Probe::new(7, 2);
        assert!(hits_target_area(probe, &horizontal_range, &vertical_range));

        let probe = Probe::new(6, 3);
        assert!(hits_target_area(probe, &horizontal_range, &vertical_range));

        let probe = Probe::new(9, 0);
        assert!(hits_target_area(probe, &horizontal_range, &vertical_range));

        let probe = Probe::new(17, -4);
        assert!(!hits_target_area(probe, &horizontal_range, &vertical_range));
    }

    #[test]
    fn test_find_velocities() {
        let horizontal_range = 20..=30;
        let vertical_range = -10..=-5;

        assert_eq!(
            find_valid_initial_velocities(&horizontal_range, &vertical_range).len(),
            112
        );
    }
}
