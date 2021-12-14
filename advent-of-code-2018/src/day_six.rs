use std::collections::HashSet;

use common::parse::*;
use common::prelude::*;
use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, into},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use multimap::MultiMap;

const BUFFER: isize = 100;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-6.txt");
    let targets = parse_coordinates(input);

    let part_one = part_one(&targets);
    let part_two = part_two(&targets);

    Ok((part_one, part_two))
}

fn part_one(targets: &HashSet<Coordinate>) -> PartAnswer {
    let start = SystemTime::now();
    let bounding_box = BoundingBox::new(targets);

    let mut regions_by_target = MultiMap::new();
    let mut infinite_regions = HashSet::new();

    for coordinate in bounding_box.all_coordinates() {
        if targets.contains(&coordinate) {
            continue;
        }

        let closest_target = targets
            .iter()
            .min_by_key(|target| coordinate.manhattan_distance(target))
            .unwrap();

        regions_by_target.insert(closest_target, coordinate);

        if coordinate.x == bounding_box.bottom_left.x
            || coordinate.y == bounding_box.bottom_left.y
            || coordinate.x == bounding_box.top_right.x
            || coordinate.y == bounding_box.top_right.y
        {
            infinite_regions.insert(closest_target);
        }
    }

    println!(
        "{} targets, {} infinite regions",
        targets.len(),
        infinite_regions.len()
    );
    println!("{:#?}", infinite_regions);

    let region_sizes: Vec<usize> = regions_by_target
        .iter_all()
        .filter(|(target, _)| !infinite_regions.contains(*target))
        .map(|(_, region)| region.len())
        .collect();

    let solution = region_sizes.iter().max().unwrap();

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two(targets: &HashSet<Coordinate>) -> PartAnswer {
    PartAnswer::default()
}

fn explore(targets: &HashSet<Coordinate>) {
    let bounding_box = BoundingBox::new(targets);

    for coordinate in bounding_box.all_coordinates() {
        println!("{:?}", coordinate);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Coordinate {
    fn from(pair: (isize, isize)) -> Self {
        let (x, y) = pair;
        Self { x, y }
    }
}

impl Coordinate {
    fn manhattan_distance(&self, other: &Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BoundingBox {
    top_right: Coordinate,
    bottom_left: Coordinate,
}

impl BoundingBox {
    fn new(targets: &HashSet<Coordinate>) -> Self {
        let top_right_x = targets.iter().map(|target| target.x).max().unwrap();
        let top_right_y = targets.iter().map(|target| target.y).max().unwrap();
        let bottom_left_x = targets.iter().map(|target| target.x).min().unwrap();
        let bottom_right_y = targets.iter().map(|target| target.y).min().unwrap();

        let top_right = (top_right_x, top_right_y).into();
        let bottom_left = (bottom_left_x, bottom_right_y).into();

        Self {
            top_right,
            bottom_left,
        }
    }

    fn all_coordinates(&self) -> Vec<Coordinate> {
        let mut targets = Vec::new();
        for x in self.bottom_left.x - BUFFER..self.top_right.x + BUFFER {
            for y in self.bottom_left.y - BUFFER..self.top_right.y + BUFFER {
                targets.push((x, y).into());
            }
        }

        targets
    }
}

fn parse_coordinates(i: &str) -> HashSet<Coordinate> {
    all_consuming(whitespace(targets))(i)
        .unwrap()
        .1
        .into_iter()
        .collect()
}

fn targets(i: &str) -> IResult<&str, Vec<Coordinate>> {
    separated_list1(tag("\n"), coordinate)(i)
}

fn coordinate(i: &str) -> IResult<&str, Coordinate> {
    into(separated_pair(unsigned_number, tag(", "), unsigned_number))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate() {
        assert_eq!(coordinate("275, 276"), Ok(("", (275, 276).into())))
    }

    #[test]
    fn test_new_bounding_box() {
        let targets = vec![(4, 4).into(), (0, 0).into(), (-2, -1).into()]
            .into_iter()
            .collect();

        let bounding_box = BoundingBox::new(&targets);

        assert_eq!(bounding_box.top_right, (4, 4).into());
        assert_eq!(bounding_box.bottom_left, (-2, -1).into());
    }

    #[test]
    fn test_manhattan_distance() {
        let first: Coordinate = (1, 2).into();
        let second = (6, 8).into();

        assert_eq!(first.manhattan_distance(&second), 11);
    }
}
