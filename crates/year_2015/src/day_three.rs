use anyhow::bail;
use std::collections::HashSet;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let visited = visit_houses(directions(input)?);
    Ok(visited.len())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let directions = directions(input)?;

    let visited_by_santa = visit_houses(directions.iter().step_by(2).copied());
    let visited_by_robot_santa = visit_houses(directions.iter().skip(1).step_by(2).copied());

    let visited = visited_by_santa
        .union(&visited_by_robot_santa)
        .copied()
        .collect::<HashSet<_>>();

    Ok(visited.len())
}

fn visit_houses<D: IntoIterator<Item = Direction>>(directions: D) -> HashSet<Position> {
    let mut visited = HashSet::new();

    let mut current = (0, 0);
    visited.insert(current);

    for direction in directions {
        current = direction.apply(&current);
        visited.insert(current);
    }

    visited
}

fn directions(input: &str) -> anyhow::Result<Vec<Direction>> {
    let mut directions = Vec::new();

    for c in input.trim().chars() {
        if let Some(direction) = match c {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        } {
            directions.push(direction);
        } else {
            bail!("Invalid character in input: {}", c);
        }
    }

    Ok(directions)
}

type Position = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn apply(&self, position: &Position) -> Position {
        match self {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Right => (position.0 + 1, position.1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
        }
    }
}
