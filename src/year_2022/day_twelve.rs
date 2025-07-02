use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::not_line_ending,
    combinator::{map, map_opt, map_parser, value},
    multi::{many1, separated_list1},
    IResult, Parser,
};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let elevation_map = parse(input);
    Ok(bfs(&elevation_map.start, &elevation_map).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let elevation_map = parse(input);

    let mut distance_to_end = usize::MAX;

    for (coordinate, elevation) in &elevation_map.map {
        if let Elevation::Height(height) = elevation {
            if *height == 0 {
                let candidate_distance = bfs(coordinate, &elevation_map);
                distance_to_end = distance_to_end.min(candidate_distance);
            }
        }
    }

    Ok(distance_to_end.to_string())
}

fn bfs(start: &(usize, usize), elevation_map: &ElevationMap) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((*start, 0));

    let mut explored = HashSet::new();

    while let Some((current, distance)) = queue.pop_front() {
        // pr0intln!("Checking {:?} at distance {}", current, distance);
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

    usize::MAX
}

fn neighbors(coordinate: &(usize, usize), elevation_map: &ElevationMap) -> Vec<(usize, usize)> {
    let (x, y) = *coordinate;

    let current_elevation = elevation_map.map.get(coordinate).unwrap();

    vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .into_iter()
        .filter(|c| {
            if let Some(elevation) = elevation_map.map.get(c) {
                match (current_elevation, elevation) {
                    (Elevation::Start, Elevation::Start) => unreachable!(),
                    (Elevation::End, Elevation::End) => unreachable!(),
                    (Elevation::End, Elevation::Height(_value)) => true,
                    (Elevation::Start, Elevation::End) | (Elevation::End, Elevation::Start) => {
                        false
                    }
                    (Elevation::Start, Elevation::Height(value)) => *value <= 1,
                    (Elevation::Height(_value), Elevation::Start) => true,
                    (Elevation::Height(current), Elevation::Height(next)) => current + 1 >= *next,
                    (Elevation::Height(value), Elevation::End) => *value == 24 || *value == 25,
                }
            } else {
                false
            }
        })
        .collect()
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

        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, row) in all_elevations.iter().enumerate() {
            for (x, column) in row.iter().enumerate() {
                // pr0intln!("Inserting {:?} at {x},{y}", column);
                map.insert((x, y), *column);

                if *column == Elevation::Start {
                    start = (x, y);
                } else if *column == Elevation::End {
                    end = (x, y);
                }
            }
        }

        ElevationMap { map, start, end }
    }
}

fn parse(i: &str) -> ElevationMap {
    map(all_elevations, ElevationMap::from).parse(i).unwrap().1
}

fn all_elevations(i: &str) -> IResult<&str, Vec<Vec<Elevation>>> {
    separated_list1(tag("\n"), row).parse(i)
}

fn row(i: &str) -> IResult<&str, Vec<Elevation>> {
    map_parser(not_line_ending, many1(elevation)).parse(i)
}

fn elevation(i: &str) -> IResult<&str, Elevation> {
    alt((start, end, height)).parse(i)
}

fn start(i: &str) -> IResult<&str, Elevation> {
    value(Elevation::Start, tag("S")).parse(i)
}

fn height(i: &str) -> IResult<&str, Elevation> {
    map(
        map_opt(take(1_usize), |s: &str| {
            s.chars().next().map(|c| c as usize - 97)
        }),
        Elevation::Height,
    )
    .parse(i)
}

fn end(i: &str) -> IResult<&str, Elevation> {
    value(Elevation::End, tag("E")).parse(i)
}
