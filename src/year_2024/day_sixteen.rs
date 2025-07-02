use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Ok;
use log::debug;

use crate::common::base::Day;
use crate::common::base::Year;
use crate::common::debug;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let (start, end, tiles, _x_range, _y_range) = parse_map(input)?;

    let scores = dijkstra(start, vec![Direction::East], end, tiles)?;

    scores
        .iter()
        .filter(|(key, _)| end == key.0)
        .map(|(_, value)| *value)
        .min()
        .map(|score| score.to_string())
        .ok_or(anyhow!("No path found"))
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let (start, end, tiles, x_range, y_range) = parse_map(input)?;

    let forward_scores = dijkstra(start, vec![Direction::East], end, tiles.clone())?;

    let end_score = forward_scores
        .iter()
        .filter(|(key, _)| end == key.0)
        .map(|(_, value)| *value)
        .min()
        .ok_or(anyhow!("No path found"))?;

    let backward_scores = dijkstra(end.into(), Direction::values(), start.into(), tiles.clone())?;

    let mut tiles_on_fastest_paths = HashSet::new();

    for y in y_range.clone() {
        for x in x_range.clone() {
            let coordinate = (x, y).into();

            for direction in Direction::values() {
                let forward_score = forward_scores.get(&(coordinate, direction));
                let backward_score = backward_scores.get(&(coordinate, direction.opposite()));

                if forward_score.is_none() || backward_score.is_none() {
                    continue;
                }

                let forward_score = forward_score.unwrap();
                let backward_score = backward_score.unwrap();

                if forward_score + backward_score == end_score {
                    tiles_on_fastest_paths.insert(coordinate);
                }
            }
        }
    }

    let mut render = String::new();

    for y in y_range.clone() {
        for x in x_range.clone() {
            let coordinate = (x, y).into();
            if start == coordinate {
                render.push('S');
            } else if end == coordinate {
                render.push('E');
            } else if tiles_on_fastest_paths.contains(&coordinate) {
                render.push('O');
            } else if tiles.contains(&coordinate) {
                render.push('.');
            } else {
                render.push('#');
            }
        }
        render.push('\n');
    }

    let writer = debug::OutputWriter::new(Year::Year2024, Day::Day25);

    writer.write("render.txt", &render)?;

    Ok(tiles_on_fastest_paths.len().to_string())
}

fn dijkstra(
    start: Start,
    start_directions: Vec<Direction>,
    end: End,
    tiles: HashSet<Coordinate>,
) -> anyhow::Result<HashMap<(Coordinate, Direction), usize>> {
    let mut priority_queue = BinaryHeap::new();

    let mut scores = HashMap::new();

    for tile in &tiles {
        scores.insert((*tile, Direction::North), usize::MAX);
        scores.insert((*tile, Direction::East), usize::MAX);
        scores.insert((*tile, Direction::South), usize::MAX);
        scores.insert((*tile, Direction::West), usize::MAX);
    }

    for direction in &start_directions {
        scores.insert((start.into(), *direction), 0);
    }

    for direction in Direction::values() {
        if !start_directions.contains(&direction) {
            scores.insert((start.into(), direction), usize::MAX);
        }
    }

    scores.insert((end.into(), Direction::North), usize::MAX);
    scores.insert((end.into(), Direction::East), usize::MAX);
    scores.insert((end.into(), Direction::South), usize::MAX);
    scores.insert((end.into(), Direction::West), usize::MAX);

    for direction in &start_directions {
        priority_queue.push(SearchTile::new(0, start.into(), *direction));
    }

    while let Some(SearchTile {
        score,
        coordinate,
        direction,
    }) = priority_queue.pop()
    {
        debug!("Evaluating score={score}, coordinate={coordinate:?}, direction={direction:?}");

        if end == coordinate {
            debug!("Found path with score={score}");
            continue;
        }

        let advance = SearchTile::new(score + 1, coordinate.advance(&direction), direction);

        let left = match direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        let left = SearchTile::new(score + 1000, coordinate, left);

        let right = match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        let right = SearchTile::new(score + 1000, coordinate, right);

        for SearchTile {
            score: new_score,
            coordinate: new_coordinate,
            direction: new_direction,
        } in [advance, left, right]
        {
            if !tiles.contains(&new_coordinate) && end != new_coordinate && start != new_coordinate
            {
                debug!(
                    "Skipping tile with coordinate={new_coordinate:?} as it is not a valid tile"
                );
                continue;
            }

            #[allow(clippy::expect_fun_call)]
            let current_score = scores.get_mut(&(new_coordinate, new_direction)).expect(
                format!(
                    "No score found for coordinate={new_coordinate:?}, direction={new_direction:?}"
                )
                .as_str(),
            );

            if new_score < *current_score {
                debug!(
                    "Updating tile with coordinate={new_coordinate:?} from score={current_score} to score={new_score}"
                );

                *current_score = new_score;

                debug!(
                    "Enqueueing tile with score={new_score}, coordinate={new_coordinate:?}, direction={new_direction:?}"
                );
                priority_queue.push(SearchTile::new(new_score, new_coordinate, new_direction));
            } else {
                debug!(
                    "Skipping tile with coordinate={new_coordinate:?} because {new_score} > {current_score}"
                );
            }
        }
    }

    Ok(scores)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn values() -> Vec<Self> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct SearchTile {
    score: usize,
    coordinate: Coordinate,
    direction: Direction,
}

impl SearchTile {
    fn new(score: usize, coordinate: Coordinate, direction: Direction) -> Self {
        Self {
            score,
            coordinate,
            direction,
        }
    }
}

impl PartialOrd for SearchTile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchTile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coordinate((isize, isize));

impl Coordinate {
    fn advance(&self, direction: &Direction) -> Coordinate {
        let (x, y) = self.0;
        let (dx, dy) = match direction {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        };

        (x + dx, y + dy).into()
    }
}

impl From<(isize, isize)> for Coordinate {
    fn from(value: (isize, isize)) -> Self {
        Self(value)
    }
}

impl From<Start> for Coordinate {
    fn from(start: Start) -> Self {
        start.0
    }
}

impl From<End> for Coordinate {
    fn from(end: End) -> Self {
        end.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Start(Coordinate);

impl PartialEq<Coordinate> for Start {
    fn eq(&self, other: &Coordinate) -> bool {
        self.0 == *other
    }
}

impl From<End> for Start {
    fn from(end: End) -> Self {
        Self(end.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct End(Coordinate);

impl PartialEq<Coordinate> for End {
    fn eq(&self, other: &Coordinate) -> bool {
        self.0 == *other
    }
}

impl From<Start> for End {
    fn from(start: Start) -> Self {
        Self(start.0)
    }
}

#[allow(clippy::type_complexity)]
fn parse_map(
    input: &str,
) -> anyhow::Result<(
    Start,
    End,
    HashSet<Coordinate>,
    RangeInclusive<isize>,
    RangeInclusive<isize>,
)> {
    let mut start = None;
    let mut end = None;
    let mut tiles = HashSet::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().char_indices() {
            max_x = max_x.max(x as isize);
            max_y = max_y.max(y as isize);

            let coordinate = (x as isize, y as isize).into();
            match c {
                'S' => {
                    if start.is_some() {
                        bail!("Multiple starts found")
                    }
                    start = Some(Start(coordinate));
                }
                'E' => {
                    if end.is_some() {
                        bail!("Multiple ends found")
                    }
                    end = Some(End(coordinate));
                }
                '.' => {
                    tiles.insert(coordinate);
                }
                '#' => {}
                _ => bail!("Invalid character found: {}", c),
            }
        }
    }

    let start = start.ok_or(anyhow!("No start found"))?;
    let end = end.ok_or(anyhow!("No end found"))?;

    Ok((start, end, tiles, 0..=max_x, 0..=max_y))
}
