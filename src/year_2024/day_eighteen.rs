use std::collections::{HashSet, VecDeque};

use anyhow::{anyhow, bail};
use log::{debug, trace};

const START: (isize, isize) = (0, 0);
const END: (isize, isize) = (70, 70);
const BYTES: usize = 1024;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let coordinates = parse(input)?;

    let coordinates = coordinates.into_iter().take(BYTES).collect();

    bfs(START, END, &coordinates)
        .map(|distance| distance.to_string())
        .ok_or(anyhow!("Could not find a path from start to end"))
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let coordinates = parse(input)?;

    let mut start = BYTES + 1;
    let mut end = coordinates.len() - 1;

    while start <= end {
        if start == end {
            return coordinates
                .get(start)
                .map(|coordinate| format!("{},{}", coordinate.0, coordinate.1))
                .ok_or_else(|| anyhow!("Could not get coordinate at index {start}"));
        }

        let mid = start + (end - start) / 2;

        let coordinates = coordinates.iter().take(mid).copied().collect();

        if bfs(START, END, &coordinates).is_some() {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }

    bail!("Could not find byte that blocks the exit")
}

fn bfs(start: Coordinate, end: Coordinate, walls: &HashSet<Coordinate>) -> Option<usize> {
    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    while let Some((distance, coordinate)) = queue.pop_front() {
        if !visited.insert(coordinate) {
            continue;
        }

        if coordinate == end {
            return Some(distance);
        }

        let neighbors = vec![
            (coordinate.0 + 1, coordinate.1),
            (coordinate.0 - 1, coordinate.1),
            (coordinate.0, coordinate.1 + 1),
            (coordinate.0, coordinate.1 - 1),
        ];

        for neighbor in neighbors {
            if neighbor.0 < 0 || neighbor.1 < 0 {
                continue;
            }
            if neighbor.0 > end.0 || neighbor.1 > end.1 {
                continue;
            }

            if walls.contains(&neighbor) {
                trace!("Skipping coordinate {neighbor:?} because it is a wall");
                continue;
            }

            if visited.contains(&neighbor) {
                trace!("Skipping coordinate {neighbor:?} because it has already been visited");
                continue;
            }

            debug!("Checking neighbor {neighbor:?}");

            queue.push_back((distance + 1, neighbor));
        }
    }

    None
}

type Coordinate = (isize, isize);

fn parse(input: &str) -> anyhow::Result<Vec<Coordinate>> {
    let mut coordinates = Vec::new();

    for line in input.lines() {
        let (x, y) = line
            .split_once(",")
            .ok_or(anyhow!("Could not split '{line}' on comma"))?;

        let x = x.parse()?;
        let y = y.parse()?;

        coordinates.push((x, y));
    }

    Ok(coordinates)
}
