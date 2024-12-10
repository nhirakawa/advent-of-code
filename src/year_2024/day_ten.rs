use anyhow::bail;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let (trailheads, map) = parse_grid(input)?;

    let mut sum = 0;

    for trailhead in trailheads {
        let (unique_summits, _) = count_summits(trailhead, &map)?;
        sum += unique_summits
    }

    Ok(sum.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let (trailheads, map) = parse_grid(input)?;

    let mut sum = 0;

    for trailhead in trailheads {
        let (_, unique_paths) = count_summits(trailhead, &map)?;
        sum += unique_paths
    }

    Ok(sum.to_string())
}

fn count_summits(trailhead: Coordinate, map: &Grid) -> anyhow::Result<(usize, usize)> {
    let mut to_search = VecDeque::new();
    to_search.push_back(trailhead);

    let mut summits = HashSet::new();
    let mut summit_paths = 0;

    while let Some(current) = to_search.pop_front() {
        let next_step = find_next_step(&current, map)?;

        if let Some(next_step) = next_step {
            for next in next_step {
                to_search.push_back(next);
            }
        } else {
            summits.insert(current);
            summit_paths += 1;
        }
    }

    Ok((summits.len(), summit_paths))
}

fn find_next_step(current: &Coordinate, map: &Grid) -> anyhow::Result<Option<Vec<Coordinate>>> {
    let current_height = map
        .get(current)
        .ok_or_else(|| anyhow::anyhow!("Invalid coordinate: {current:?}"))?;

    if let Some(next_height) = current_height.next() {
        let up = (current.0, current.1 - 1);
        let down = (current.0, current.1 + 1);
        let left = (current.0 - 1, current.1);
        let right = (current.0 + 1, current.1);

        Ok(Some(
            [up, down, left, right]
                .iter()
                .filter(|coordinate| {
                    if let Some(height) = map.get(coordinate) {
                        height == &next_height
                    } else {
                        false
                    }
                })
                .copied()
                .collect(),
        ))
    } else {
        Ok(None)
    }
}

type Coordinate = (usize, usize);
type Trailheads = HashSet<Coordinate>;
type Grid = HashMap<Coordinate, Height>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Height {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Height {
    fn next(&self) -> Option<Height> {
        match self {
            Height::Zero => Some(Height::One),
            Height::One => Some(Height::Two),
            Height::Two => Some(Height::Three),
            Height::Three => Some(Height::Four),
            Height::Four => Some(Height::Five),
            Height::Five => Some(Height::Six),
            Height::Six => Some(Height::Seven),
            Height::Seven => Some(Height::Eight),
            Height::Eight => Some(Height::Nine),
            Height::Nine => None,
        }
    }
}

fn parse_grid(input: &str) -> anyhow::Result<(Trailheads, Grid)> {
    let mut grid = HashMap::new();
    let mut trailheads = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let height = match c {
                '0' => Height::Zero,
                '1' => Height::One,
                '2' => Height::Two,
                '3' => Height::Three,
                '4' => Height::Four,
                '5' => Height::Five,
                '6' => Height::Six,
                '7' => Height::Seven,
                '8' => Height::Eight,
                '9' => Height::Nine,
                _ => bail!("Invalid height found at line {}, column {}: '{}'", y, x, c),
            };

            grid.insert((x, y), height);
            if let Height::Zero = height {
                trailheads.insert((x, y));
            }
        }
    }

    Ok((trailheads, grid))
}
