use std::collections::HashSet;

use common::prelude::*;
use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list1, sequence::tuple, IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-18.txt");

    let part_one = part_one(input);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let coordinates = parse(input);

    let mut surface_area = 0;

    for coordinate in &coordinates {
        let neighbors = coordinate.neighbors();

        let connected_neighbors = coordinates.intersection(&neighbors).count();

        surface_area += 6 - connected_neighbors;
    }

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(surface_area, elapsed)
}

fn part_two() -> PartAnswer {
    let start = SystemTime::now();
    let _elapsed = start.elapsed().unwrap();
    PartAnswer::default()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

impl Coordinate {
    fn new(x: isize, y: isize, z: isize) -> Coordinate {
        Coordinate { x, y, z }
    }

    fn neighbors(&self) -> HashSet<Coordinate> {
        vec![
            Coordinate::new(self.x + 1, self.y, self.z),
            Coordinate::new(self.x - 1, self.y, self.z),
            Coordinate::new(self.x, self.y + 1, self.z),
            Coordinate::new(self.x, self.y - 1, self.z),
            Coordinate::new(self.x, self.y, self.z + 1),
            Coordinate::new(self.x, self.y, self.z - 1),
        ]
        .into_iter()
        .collect()
    }
}

fn parse(i: &str) -> HashSet<Coordinate> {
    finish(coordinates)(i).unwrap().1
}

fn coordinates(i: &str) -> IResult<&str, HashSet<Coordinate>> {
    map(separated_list1(tag("\n"), coordinate), |v| {
        v.into_iter().collect()
    })(i)
}

fn coordinate(i: &str) -> IResult<&str, Coordinate> {
    map(
        tuple((
            unsigned_number,
            tag(","),
            unsigned_number,
            tag(","),
            unsigned_number,
        )),
        |(x, _, y, _, z)| Coordinate::new(x, y, z),
    )(i)
}
