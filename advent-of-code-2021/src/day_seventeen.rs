use std::ops::RangeInclusive;

use common::{math::triangular_number, prelude::*};

pub fn run() -> AdventOfCodeResult {
    let horizontal_range = 169..=206;
    let vertical_range = -108..=-68;

    let part_one = part_one(&horizontal_range, &vertical_range);
    let part_two = part_two(&horizontal_range, &vertical_range);

    Ok((part_one, part_two))
}

fn part_one(
    horizontal_range: &RangeInclusive<i64>,
    vertical_range: &RangeInclusive<i64>,
) -> PartAnswer {
    let start = SystemTime::now();

    let hit_velocities = find_valid_initial_velocities(horizontal_range, vertical_range);

    let max_y_velocity = hit_velocities
        .iter()
        .map(|(_, y)| y)
        .max()
        .copied()
        .unwrap();

    let solution = triangular_number(max_y_velocity as usize);

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two(
    horizontal_range: &RangeInclusive<i64>,
    vertical_range: &RangeInclusive<i64>,
) -> PartAnswer {
    let start = SystemTime::now();

    let hit_velocities = find_valid_initial_velocities(horizontal_range, vertical_range);

    let solution = hit_velocities.len();

    PartAnswer::new(solution, start.elapsed().unwrap())
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

    fn position(&self) -> (i64, i64) {
        (self.x_position, self.y_position)
    }

    fn velocity(&self) -> (i64, i64) {
        (self.x_velocity, self.y_velocity)
    }

    fn step(&mut self) {
        self.x_position += self.x_velocity;
        self.y_position += self.y_velocity;

        self.x_velocity -= self.x_velocity.signum();
        self.y_velocity -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe() {
        let mut probe = Probe::new(7, 2);
        probe.step();

        assert_eq!(probe.position(), (7, 2));
        assert_eq!(probe.velocity(), (6, 1));

        probe.step();

        assert_eq!(probe.position(), (13, 3));
        assert_eq!(probe.velocity(), (5, 0));
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
