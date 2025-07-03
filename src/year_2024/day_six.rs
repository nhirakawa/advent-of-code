use std::collections::{HashMap, HashSet};

use anyhow::bail;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (start, grid) = parse_grid(input)?;

    let positions = find_positions(start, &grid);

    Ok(positions.len())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (start, grid) = parse_grid(input)?;

    let count = find_positions(start, &grid)
        .par_iter()
        .filter(|position| {
            let mut updated_grid = grid.clone();
            updated_grid.insert(**position, PositionState::Obstacle);
            find_loop(start, &updated_grid)
        })
        .count();

    Ok(count)
}

fn find_loop(start: Coordinate, grid: &HashMap<Coordinate, PositionState>) -> bool {
    let mut position = start;
    let mut direction = Direction::Up;

    let mut seen = HashSet::new();

    loop {
        if !seen.insert((position, direction)) {
            return true;
        }

        let next_position = next_position(&position, &direction);

        if let Some(position_state) = grid.get(&next_position) {
            match position_state {
                PositionState::Empty => {
                    position = next_position;
                }
                PositionState::Obstacle => {
                    // Turn right
                    direction = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    };
                }
            }
        } else {
            return false;
        }
    }
}

fn find_positions(
    start: (isize, isize),
    grid: &HashMap<Coordinate, PositionState>,
) -> HashSet<Coordinate> {
    let mut position = start;
    let mut direction = Direction::Up;

    let mut positions = HashSet::new();

    loop {
        positions.insert(position);

        let next_position = next_position(&position, &direction);

        if let Some(position_state) = grid.get(&next_position) {
            match position_state {
                PositionState::Empty => {
                    position = next_position;
                }
                PositionState::Obstacle => {
                    // Turn right
                    direction = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    };
                }
            }
        } else {
            break;
        }
    }

    positions
}

fn next_position(current: &Coordinate, direction: &Direction) -> Coordinate {
    match direction {
        Direction::Up => (current.0, current.1 - 1),
        Direction::Right => (current.0 + 1, current.1),
        Direction::Down => (current.0, current.1 + 1),
        Direction::Left => (current.0 - 1, current.1),
    }
}

type Coordinate = (isize, isize);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy)]
enum PositionState {
    Empty,
    Obstacle,
}

fn parse_grid(input: &str) -> anyhow::Result<(Coordinate, HashMap<Coordinate, PositionState>)> {
    let mut grid = HashMap::new();

    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coordinate = (x as isize, y as isize);
            match c {
                '^' => {
                    if start != (0, 0) {
                        bail!("Multiple starting positions found");
                    }
                    start = coordinate;
                    grid.insert(coordinate, PositionState::Empty);
                }
                '#' => {
                    grid.insert(coordinate, PositionState::Obstacle);
                }
                '.' => {
                    grid.insert(coordinate, PositionState::Empty);
                }
                _ => bail!("Invalid character in input"),
            }
        }
    }

    Ok((start, grid))
}
