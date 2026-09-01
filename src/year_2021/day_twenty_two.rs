use anyhow::{Context, anyhow, bail};
use itertools::iproduct;
use std::collections::HashSet;
use std::fmt::Display;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let cuboids = Cuboids::from_str(input)?;

    let mut on: HashSet<(isize, isize, isize)> = HashSet::new();

    for cuboid in &cuboids.cuboids {
        if cuboid.is_out_of_bounds((-50, 50)) {
            continue;
        }

        for coords in cuboid {
            match cuboid.state {
                State::On => {
                    on.insert(coords);
                }
                State::Off => {
                    on.remove(&coords);
                }
            }
        }
    }

    Ok(on.len())
}

// Inspired by https://www.reddit.com/r/adventofcode/comments/rlxhmg/2021_day_22_solutions/hpizza8/
pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let cuboids = Cuboids::from_str(input)?;

    let mut signed_volumes: Vec<SignedVolume> = Vec::new();

    for cuboid in &cuboids.cuboids {
        let mut signed_volumes_update = Vec::new();

        for signed_volume in &signed_volumes {
            if let Some(intersection) = cuboid.volume.intersection(&signed_volume.volume) {
                signed_volumes_update
                    .push(SignedVolume::new(signed_volume.sign.flip(), intersection));
            }
        }

        if cuboid.state == State::On {
            signed_volumes_update.push(SignedVolume::new(Sign::Positive, cuboid.volume.clone()));
        }

        signed_volumes.append(&mut signed_volumes_update);
    }

    let result = signed_volumes
        .iter()
        .map(|v| v.volume.volume() * v.sign.value())
        .sum::<isize>();

    // 1197216533115020 is too low
    Ok(result)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    On,
    Off,
}

impl FromStr for State {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(State::On),
            "off" => Ok(State::Off),
            _ => Err(anyhow!("Invalid state: {}", s)),
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::On => write!(f, "on"),
            State::Off => write!(f, "off"),
        }
    }
}

struct Cuboids {
    cuboids: Vec<Cuboid>,
}

impl FromStr for Cuboids {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cuboids = vec![];
        for line in s.lines() {
            cuboids.push(Cuboid::from_str(line)?);
        }
        Ok(Self { cuboids })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Volume {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
    z: RangeInclusive<isize>,
}

impl Volume {
    fn new(x: RangeInclusive<isize>, y: RangeInclusive<isize>, z: RangeInclusive<isize>) -> Self {
        Self { x, y, z }
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.x.end() >= other.x.start()
            && self.x.start() <= other.x.end()
            && self.y.end() >= other.y.start()
            && self.y.start() <= other.y.end()
            && self.z.end() >= other.z.start()
            && self.z.start() <= other.z.end()
        {
            let min_x = isize::max(*self.x.start(), *other.x.start());
            let min_y = isize::max(*self.y.start(), *other.y.start());
            let min_z = isize::max(*self.z.start(), *other.z.start());

            let max_x = isize::min(*self.x.end(), *other.x.end());
            let max_y = isize::min(*self.y.end(), *other.y.end());
            let max_z = isize::min(*self.z.end(), *other.z.end());

            let x = min_x..=max_x;
            let y = min_y..=max_y;
            let z = min_z..=max_z;

            Some(Self { x, y, z })
        } else {
            None
        }
    }

    fn volume(&self) -> isize {
        let x_length = self.x.end() - self.x.start() + 1;
        let y_length = self.y.end() - self.y.start() + 1;
        let z_length = self.z.end() - self.z.start() + 1;
        x_length * y_length * z_length
    }
}

struct Cuboid {
    state: State,
    volume: Volume,
}

impl Cuboid {
    fn new(state: State, volume: Volume) -> Self {
        Self {
            state,
            volume: volume,
        }
    }

    fn is_out_of_bounds(&self, (min, max): (isize, isize)) -> bool {
        if *self.volume.x.start() < min || *self.volume.x.start() > max {
            return true;
        }

        if *self.volume.y.start() < min || *self.volume.y.start() > max {
            return true;
        }

        if *self.volume.z.start() < min || *self.volume.z.start() > max {
            return true;
        }

        false
    }

    fn intersection(&self, other: &Self) -> Option<Volume> {
        self.volume.intersection(&other.volume)
    }
}

impl FromStr for Cuboid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (state, rest) = s.split_once(" ").ok_or(anyhow!("Invalid input - no ' '"))?;
        let state = State::from_str(state)?;
        let volume = parse_volume(rest)?;

        Ok(Self::new(state, volume))
    }
}

impl<'a> IntoIterator for &'a Cuboid {
    type Item = (isize, isize, isize);
    // TODO: switch to `impl Iterator<Item = Self::Item>` once
    // `impl_trait_in_assoc_type` stabilizes, to avoid the Box.
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;
    fn into_iter(self) -> Self::IntoIter {
        Box::new(iproduct!(
            self.volume.x.clone(),
            self.volume.y.clone(),
            self.volume.z.clone()
        ))
    }
}

enum Sign {
    Positive,
    Negative,
}

impl Sign {
    fn flip(&self) -> Self {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }

    fn value(&self) -> isize {
        match self {
            Sign::Positive => 1,
            Sign::Negative => -1,
        }
    }
}

struct SignedVolume {
    sign: Sign,
    volume: Volume,
}

impl SignedVolume {
    fn new(sign: Sign, volume: Volume) -> Self {
        Self { sign, volume }
    }
}

fn parse_volume(s: &str) -> anyhow::Result<Volume> {
    let (first, rest) = s
        .split_once(",")
        .ok_or(anyhow!("Input does not contain ','"))?;
    let (second, third) = rest
        .split_once(",")
        .ok_or(anyhow!("Input does not contain second ','"))?;

    if third.contains(",") {
        bail!("Invalid input - remaining input contains ','");
    }

    let first = first
        .strip_prefix("x=")
        .ok_or(anyhow!("Invalid input - no x"))?;
    let second = second
        .strip_prefix("y=")
        .ok_or(anyhow!("Invalid input - no y"))?;
    let third = third
        .strip_prefix("z=")
        .ok_or(anyhow!("Invalid input - no z"))?;

    let x = parse_range(first)?;
    let y = parse_range(second)?;
    let z = parse_range(third)?;

    Ok(Volume::new(x, y, z))
}

fn parse_range(s: &str) -> anyhow::Result<RangeInclusive<isize>> {
    let (first, second) = s
        .split_once("..")
        .ok_or(anyhow!("Input does not contain '..'"))?;
    let first = first
        .parse()
        .with_context(|| format!("Input not a number: {}", first))?;
    let second = second
        .parse()
        .with_context(|| format!("Input not a number: {}", second))?;

    let min = isize::min(first, second);
    let max = isize::max(first, second);

    Ok(min..=max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_out_of_bounds_true_when_x_start_below_min() {
        let volume = Volume::new(-60..=-55, 0..=0, 0..=0);
        let cuboid = Cuboid::new(State::On, volume);
        assert!(cuboid.is_out_of_bounds((-50, 50)));
    }

    #[test]
    fn is_out_of_bounds_true_when_x_start_above_max() {
        let volume = Volume::new(55..=60, 0..=0, 0..=0);
        let cuboid = Cuboid::new(State::On, volume);
        assert!(cuboid.is_out_of_bounds((-50, 50)));
    }

    #[test]
    fn is_out_of_bounds_true_when_y_start_out_of_range() {
        let volume = Volume::new(0..=0, -60..=-55, 0..=0);
        let cuboid = Cuboid::new(State::On, volume);
        assert!(cuboid.is_out_of_bounds((-50, 50)));
    }

    #[test]
    fn is_out_of_bounds_true_when_z_start_out_of_range() {
        let volume = Volume::new(0..=0, 0..=0, 55..=60);
        let cuboid = Cuboid::new(State::On, volume);
        assert!(cuboid.is_out_of_bounds((-50, 50)));
    }

    #[test]
    fn is_out_of_bounds_false_when_fully_within_bounds() {
        let volume = Volume::new(-10..=10, -10..=10, -10..=10);
        let cuboid = Cuboid::new(State::On, volume);
        assert!(!cuboid.is_out_of_bounds((-50, 50)));
    }

    #[test]
    fn is_out_of_bounds_false_when_starts_in_bounds_but_end_exceeds_max() {
        // Documents current behavior: only the start of each axis is
        // checked, so a range extending past `max` is not flagged.
        let volume = Volume::new(45..=55, 0..=0, 0..=0);
        let cuboid = Cuboid::new(State::On, volume);
        assert!(!cuboid.is_out_of_bounds((-50, 50)));
    }

    #[test]
    fn ranges_intersection_none_when_disjoint_on_x() {
        let a = Volume::new(0..=10, 0..=10, 0..=10);
        let b = Volume::new(20..=30, 0..=10, 0..=10);
        assert_eq!(a.intersection(&b), None);
    }

    #[test]
    fn ranges_intersection_some_when_overlapping() {
        let a = Volume::new(0..=10, 0..=10, 0..=10);
        let b = Volume::new(5..=15, 5..=15, 5..=15);
        assert_eq!(
            a.intersection(&b),
            Some(Volume::new(5..=10, 5..=10, 5..=10))
        );
    }

    #[test]
    fn ranges_intersection_some_when_touching_at_single_point() {
        let a = Volume::new(0..=10, 0..=10, 0..=10);
        let b = Volume::new(10..=20, 10..=20, 10..=20);
        assert_eq!(
            a.intersection(&b),
            Some(Volume::new(10..=10, 10..=10, 10..=10))
        );
    }

    #[test]
    fn ranges_intersection_some_when_fully_contained() {
        let a = Volume::new(0..=10, 0..=10, 0..=10);
        let b = Volume::new(2..=4, 2..=4, 2..=4);
        assert_eq!(a.intersection(&b), Some(Volume::new(2..=4, 2..=4, 2..=4)));
    }

    #[test]
    fn volume_counts_inclusive_range_length() {
        let volume = Volume::new(10..=12, 0..=0, 0..=0);
        assert_eq!(volume.volume(), 3);
    }

    #[test]
    fn volume_of_single_point_is_one() {
        let volume = Volume::new(10..=10, 10..=10, 10..=10);
        assert_eq!(volume.volume(), 1);
    }

    #[test]
    fn ranges_intersection_is_symmetric() {
        let a = Volume::new(0..=10, 0..=10, 0..=10);
        let b = Volume::new(5..=15, 5..=15, 5..=15);
        assert_eq!(a.intersection(&b), b.intersection(&a));
    }
}
