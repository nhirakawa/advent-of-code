use std::collections::{HashSet, VecDeque};

use common::prelude::*;
use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list1, sequence::tuple, IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-18.txt");

    let part_one = part_one(input);
    let part_two = part_two(input);

    Ok((part_one, part_two))
}

fn part_one(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let coordinates = parse(input);

    let mut surface_area = 0;

    for coordinate in &coordinates {
        let neighbors = coordinate.neighbors();

        let connected_neighbors: Vec<Coordinate> =
            coordinates.intersection(&neighbors).copied().collect();

        surface_area += 6 - connected_neighbors.len();
    }

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(surface_area, elapsed)
}

fn part_two(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let coordinates = parse(input);

    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;

    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;

    let mut min_z = isize::MAX;
    let mut max_z = isize::MIN;

    for coordinate in &coordinates {
        min_x = min_x.min(coordinate.x);
        max_x = max_x.max(coordinate.x);

        min_y = min_y.min(coordinate.y);
        max_y = max_y.max(coordinate.y);

        min_z = min_z.min(coordinate.z);
        max_z = max_z.max(coordinate.z);
    }

    min_x = min_x - 10;
    max_x = max_x + 10;

    min_y = min_y - 10;
    max_y = max_y + 10;

    min_z = min_z - 10;
    max_z = max_z + 10;

    println!("Bounding box min ({min_x},{min_y},{min_z}), max ({max_x},{max_y},{max_z})");

    let mut coordinates_to_test = HashSet::new();

    let min = (min_x, min_y, min_z);

    let mut queue = VecDeque::new();
    queue.push_back(min);

    let mut seen = HashSet::new();

    while let Some(next) = queue.pop_front() {
        let (x, y, z) = next;

        if !seen.insert(next) {
            continue;
        }

        if !(min_x..=max_x).contains(&x)
            || !(min_y..=max_y).contains(&y)
            || !(min_z..=max_z).contains(&z)
        {
            continue;
        }

        let coordinate = Coordinate::new(x, y, z);
        let neighbors = coordinate.neighbors();

        for neighbor in neighbors {
            if coordinates.contains(&neighbor) {
                coordinates_to_test.insert(neighbor);
            } else {
                queue.push_back((neighbor.x, neighbor.y, neighbor.z));
            }
        }
    }

    println!(
        "Testing {}/{} coordinates",
        coordinates_to_test.len(),
        coordinates.len()
    );

    let mut surface_area = 0;

    for coordinate in &coordinates_to_test {
        let neighbors = coordinate.neighbors();

        let connected_neighbors: Vec<Coordinate> =
            coordinates.intersection(&neighbors).copied().collect();

        surface_area += 6 - connected_neighbors.len();
    }

    let elapsed = start.elapsed().unwrap();

    // 2621 is too high
    PartAnswer::new(surface_area, elapsed)
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Neighbors {
    positive_x: Coordinate,
    negative_x: Coordinate,
    positive_y: Coordinate,
    negative_y: Coordinate,
    positive_z: Coordinate,
    negative_z: Coordinate,
}

impl From<&Coordinate> for Neighbors {
    fn from(coordinate: &Coordinate) -> Neighbors {
        let positive_x = Coordinate::new(coordinate.x + 1, coordinate.y, coordinate.z);
        let negative_x = Coordinate::new(coordinate.x - 1, coordinate.y, coordinate.z);

        let positive_y = Coordinate::new(coordinate.x, coordinate.y + 1, coordinate.z);
        let negative_y = Coordinate::new(coordinate.x, coordinate.y - 1, coordinate.z);

        let positive_z = Coordinate::new(coordinate.x, coordinate.y, coordinate.z + 1);
        let negative_z = Coordinate::new(coordinate.x, coordinate.y, coordinate.z - 1);

        Neighbors {
            positive_x,
            negative_x,
            positive_y,
            negative_y,
            positive_z,
            negative_z,
        }
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
