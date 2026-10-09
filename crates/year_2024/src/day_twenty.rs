use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use anyhow::{anyhow, bail};
use log::debug;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let RaceTrack { start, end, track } = parse(input)?;

    let distances = bfs(&end, &start, &track)?;

    let shortcuts = find_shortcuts(
        &RaceTrack { start, end, track },
        &distances,
        100,
        Cheat::ExactlyTwoSteps,
    )?;

    Ok(shortcuts)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let RaceTrack { start, end, track } = parse(input)?;

    let distances = bfs(&end, &start, &track)?;

    let shortcuts = find_shortcuts(
        &RaceTrack { start, end, track },
        &distances,
        100,
        Cheat::UpToTwentySteps,
    )?;

    Ok(shortcuts)
}

fn find_shortcuts(
    race_track: &RaceTrack,
    distances_to_end: &HashMap<Coordinate, usize>,
    minimum_distance: usize,
    cheat: Cheat,
) -> anyhow::Result<usize> {
    let RaceTrack {
        start: _start,
        end: _end,
        track,
    } = race_track;

    let mut count = 0;

    for segment in track {
        let current_distance_to_end = distances_to_end.get(segment).copied().ok_or(anyhow!(
            "Coordinate {segment:?} not found in distances_to_end"
        ))?;

        for other in track {
            if segment == other {
                continue;
            }

            let distance_to_other = manhattan_distance(segment, other);

            if !cheat.is_distance_ok(distance_to_other) {
                continue;
            }

            let other_distance_to_end = distances_to_end.get(other).copied().ok_or(anyhow!(
                "Coordinate {other:?} not found in distances_to_end"
            ))?;

            if other_distance_to_end > current_distance_to_end {
                continue;
            }

            if current_distance_to_end - minimum_distance - 1 == other_distance_to_end {
                continue;
            }

            if current_distance_to_end - other_distance_to_end - distance_to_other
                < minimum_distance
            {
                continue;
            }

            debug!(
                "Shortcut from {segment:?} to {other:?} saves {} steps",
                current_distance_to_end - other_distance_to_end - 2
            );

            count += 1;
        }
    }

    Ok(count)
}

fn manhattan_distance(first: &Coordinate, second: &Coordinate) -> usize {
    ((first.0 - second.0).abs() + (first.1 - second.1).abs()) as usize
}

fn bfs(
    start: &Coordinate,
    end: &Coordinate,
    track: &HashSet<Coordinate>,
) -> anyhow::Result<HashMap<Coordinate, usize>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();

    queue.push_back((0, *start));

    while let Some((distance, coordinate)) = queue.pop_back() {
        if !visited.insert(coordinate) {
            continue;
        }

        // println!("Visiting {:?}", coordinate);

        if coordinate == *end {
            distances.insert(coordinate, distance);
            return Ok(distances);
        }

        distances.insert(coordinate, distance);

        for direction in Direction::values() {
            let next = advance(&coordinate, &direction);

            if visited.contains(&next) {
                continue;
            }

            if track.contains(&next) || start == &next || end == &next {
                queue.push_back((distance + 1, next));
            }
        }
    }

    bail!("No path found")
}

fn advance(coordinate: &Coordinate, direction: &Direction) -> Coordinate {
    match direction {
        Direction::Up => (coordinate.0, coordinate.1 - 1),
        Direction::Right => (coordinate.0 + 1, coordinate.1),
        Direction::Down => (coordinate.0, coordinate.1 + 1),
        Direction::Left => (coordinate.0 - 1, coordinate.1),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Cheat {
    ExactlyTwoSteps,
    UpToTwentySteps,
}

impl Cheat {
    fn is_distance_ok(&self, distance: usize) -> bool {
        if distance <= 1 {
            false
        } else {
            match self {
                Cheat::ExactlyTwoSteps => distance == 2,
                Cheat::UpToTwentySteps => distance <= 20,
            }
        }
    }
}

type Coordinate = (isize, isize);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn values() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RaceTrack {
    start: Coordinate,
    end: Coordinate,
    track: HashSet<Coordinate>,
}

impl RaceTrack {
    fn new(start: Coordinate, end: Coordinate, track: HashSet<Coordinate>) -> Self {
        Self { start, end, track }
    }
}

fn parse(input: &str) -> anyhow::Result<RaceTrack> {
    let mut start = None;
    let mut end = None;
    let mut track = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                track.insert((x as isize, y as isize));
            } else if c == 'S' {
                if start.is_some() {
                    bail!("Multiple start positions found");
                }
                start = Some((x as isize, y as isize));
                track.insert((x as isize, y as isize));
            } else if c == 'E' {
                if end.is_some() {
                    bail!("Multiple end positions found");
                }
                end = Some((x as isize, y as isize));
                track.insert((x as isize, y as isize));
            }
        }
    }

    if start.is_none() {
        bail!("No start position found");
    }

    if end.is_none() {
        bail!("No end position found");
    }

    Ok(RaceTrack::new(start.unwrap(), end.unwrap(), track))
}
