use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::not_line_ending,
    combinator::{into, map, map_opt, map_parser, map_res, value},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-12.txt");

    let elevation_map = parse(input);

    println!("{}", elevation_map.map.len());

    let part_one = part_one(&elevation_map);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(elevation_map: &ElevationMap) -> PartAnswer {
    let start = SystemTime::now();

    let distance_to_end = bfs(elevation_map);
    println!("{distance_to_end}");

    let elapsed = start.elapsed().unwrap();
    PartAnswer::default()
}

fn part_two() -> PartAnswer {
    let start = SystemTime::now();
    let elapsed = start.elapsed().unwrap();
    PartAnswer::default()
}

fn bfs(elevation_map: &ElevationMap) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((elevation_map.start, 0));

    let mut explored = HashSet::new();

    while let Some((current, distance)) = queue.pop_front() {
        println!("Checking {:?} at distance {}", current, distance);
        if current == elevation_map.end {
            return distance;
        }

        explored.insert(current);

        for neighbor in neighbors(&current, elevation_map) {
            if explored.insert(neighbor) {
                queue.push_back((neighbor, distance + 1));
            }
        }
    }

    unreachable!()
}

fn neighbors(coordinate: &(usize, usize), elevation_map: &ElevationMap) -> Vec<(usize, usize)> {
    let (x, y) = *coordinate;

    let current_elevation = elevation_map.map.get(coordinate).unwrap();

    vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .into_iter()
        .filter(|c| {
            if let Some(elevation) = elevation_map.map.get(c) {
                let are_neighbors = match (current_elevation, elevation) {
                    (Elevation::Start, Elevation::Start) => unreachable!(),
                    (Elevation::End, Elevation::End) => unreachable!(),
                    (Elevation::End, Elevation::Height(_value)) => true,
                    (Elevation::Start, Elevation::End) | (Elevation::End, Elevation::Start) => false,
                    (Elevation::Start, Elevation::Height(value)) => *value <= 1,
                    (Elevation::Height(_value), Elevation::Start) => true,
                    (Elevation::Height(current), Elevation::Height(next)) => {
                        current + 1 >= *next
                    }
                    (Elevation::Height(value), Elevation::End) => *value == 24 || *value == 25,
                };

                println!("Checking {coordinate:?} [{current_elevation:?}] and {c:?} [{elevation:?}] for neighbor status => {are_neighbors}");

                are_neighbors
            } else {
                false
            }
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct CoordinateWithDistance {
    coordinate: (usize, usize),
    distance: usize,
}

impl CoordinateWithDistance {
    fn new(coordinate: (usize, usize), distance: usize) -> CoordinateWithDistance {
        CoordinateWithDistance {
            coordinate,
            distance,
        }
    }
}

impl PartialOrd for CoordinateWithDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CoordinateWithDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance
            .cmp(&other.distance)
            .then(self.coordinate.cmp(&other.coordinate))
            .reverse()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Elevation {
    Start,
    Height(usize),
    End,
}

struct ElevationMap {
    map: HashMap<(usize, usize), Elevation>,
    start: (usize, usize),
    end: (usize, usize),
}

impl From<Vec<Vec<Elevation>>> for ElevationMap {
    fn from(all_elevations: Vec<Vec<Elevation>>) -> ElevationMap {
        let mut map = HashMap::new();

        let mut y = 0;

        let mut start = (0, 0);
        let mut end = (0, 0);

        for row in all_elevations {
            let mut x = 0;

            for column in row {
                // println!("Inserting {:?} at {x},{y}", column);
                map.insert((x, y), column);

                if column == Elevation::Start {
                    start = (x, y);
                } else if column == Elevation::End {
                    end = (x, y);
                }
                x += 1;
            }
            y += 1;
        }

        ElevationMap { map, start, end }
    }
}

fn parse(i: &str) -> ElevationMap {
    map(all_elevations, |elevations| ElevationMap::from(elevations))(i)
        .unwrap()
        .1
}

fn all_elevations(i: &str) -> IResult<&str, Vec<Vec<Elevation>>> {
    separated_list1(tag("\n"), row)(i)
}

fn row(i: &str) -> IResult<&str, Vec<Elevation>> {
    map_parser(not_line_ending, many1(elevation))(i)
}

fn elevation(i: &str) -> IResult<&str, Elevation> {
    alt((start, end, height))(i)
}

fn start(i: &str) -> IResult<&str, Elevation> {
    value(Elevation::Start, tag("S"))(i)
}

fn height(i: &str) -> IResult<&str, Elevation> {
    map(
        map_opt(take(1_usize), |s: &str| {
            s.chars().next().map(|c| c as usize - 97)
        }),
        Elevation::Height,
    )(i)
}

fn end(i: &str) -> IResult<&str, Elevation> {
    value(Elevation::End, tag("E"))(i)
}
