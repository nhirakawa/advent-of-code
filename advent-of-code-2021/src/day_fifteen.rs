use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use common::{parse::unsigned_number, prelude::*};
use log::debug;
use nom::{
    bytes::complete::{tag, take},
    character::complete::multispace0,
    combinator::{all_consuming, into, map_parser},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-15.txt");
    let grid = parse_grid(input);

    let part_one = part_one(&grid);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(grid: &Grid) -> PartAnswer {
    let start = SystemTime::now();

    let distances = dijkstra(grid);

    let max_coordinate = *grid.vertices().iter().max().unwrap();

    let distance = distances.get(&max_coordinate);

    PartAnswer::new(distance, start.elapsed().unwrap())
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn dijkstra(grid: &Grid) -> Distances {
    let mut vertex_set = grid.vertices();

    let mut distances = HashMap::new();

    for vertex in &vertex_set {
        debug!("{:} has initial distance {}", vertex, usize::MAX);
        distances.insert(*vertex, usize::MAX);
    }

    distances.insert(Vertex::new(0, 0), 0);
    debug!("{} set to distance {}", Vertex::new(0, 0), 0);

    while !vertex_set.is_empty() {
        let min_distance_vertex = &vertex_set
            .iter()
            .min_by_key(|v| distances.get(v).copied().unwrap_or(usize::MAX))
            .copied()
            .unwrap();

        vertex_set.remove(min_distance_vertex);

        debug!("min-distance-vertex is {}", min_distance_vertex);

        let min_distance = distances.get(min_distance_vertex).copied().unwrap();

        for (neighbor, weight) in grid.neighbors(min_distance_vertex) {
            let alternate_distance = min_distance + weight;

            distances.entry(neighbor).and_modify(|current_distance| {
                *current_distance = min(*current_distance, alternate_distance)
            });
        }
    }

    Distances { d: distances }
}

fn min(first: usize, second: usize) -> usize {
    first.min(second)
}

#[derive(Debug, PartialEq)]
struct Grid {
    grid: HashMap<Vertex, usize>,
}

impl Grid {
    fn value(&self, source: &Vertex) -> Option<usize> {
        self.grid.get(source).copied()
    }

    fn vertices(&self) -> HashSet<Vertex> {
        self.grid.keys().into_iter().copied().collect()
    }

    fn neighbors(&self, source: &Vertex) -> Vec<(Vertex, usize)> {
        let mut neighbors = Vec::new();

        for potential_neighbor in source.neighbors() {
            if let Some(neighbor_weight) = self.grid.get(&potential_neighbor) {
                neighbors.push((potential_neighbor, *neighbor_weight));
            }
        }

        neighbors
    }

    fn closest_neighbor(&self, source: &Vertex) -> (Vertex, usize) {
        self.neighbors(source)
            .iter()
            .min_by_key(|(_, weight)| weight)
            .copied()
            .unwrap()
    }
}

impl From<Vec<Vec<usize>>> for Grid {
    fn from(raw: Vec<Vec<usize>>) -> Grid {
        let mut grid = HashMap::new();

        for (y, row) in raw.into_iter().enumerate() {
            for (x, risk_level) in row.into_iter().enumerate() {
                grid.insert((x, y).into(), risk_level);
            }
        }
        Grid { grid }
    }
}

#[derive(Debug, PartialEq)]
struct Distances {
    d: HashMap<Vertex, usize>,
}

impl Distances {
    fn get(&self, vertex: &Vertex) -> usize {
        self.d.get(vertex).copied().unwrap_or(usize::MAX)
    }
}

#[derive(Debug, PartialEq)]
struct Graph {
    matrix: HashMap<Vertex, HashMap<Vertex, usize>>,
}

impl Graph {
    fn vertices(&self) -> HashSet<Vertex> {
        self.matrix.keys().copied().collect()
    }

    fn adjacent(&self, source: &Vertex) -> Vec<Vertex> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Edge {
    source: Vertex,
    target: Vertex,
    weight: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Vertex {
    x: usize,
    y: usize,
}

impl Vertex {
    fn new(x: usize, y: usize) -> Vertex {
        Vertex { x, y }
    }

    fn neighbors(&self) -> Vec<Vertex> {
        let mut neighbors = vec![self.down(), self.right()];

        if let Some(up) = self.up() {
            neighbors.push(up);
        }

        if let Some(left) = self.left() {
            neighbors.push(left);
        }

        neighbors
    }

    fn up(&self) -> Option<Vertex> {
        if self.y == 0 {
            None
        } else {
            Some(Vertex {
                x: self.x,
                y: self.y - 1,
            })
        }
    }

    fn down(&self) -> Vertex {
        Vertex {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Option<Vertex> {
        if self.x == 0 {
            None
        } else {
            Some(Vertex {
                x: self.x - 1,
                y: self.y,
            })
        }
    }

    fn right(&self) -> Vertex {
        Vertex {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl From<(usize, usize)> for Vertex {
    fn from(tuple: (usize, usize)) -> Vertex {
        let (x, y) = tuple;
        Vertex::new(x, y)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn parse_grid(i: &str) -> Grid {
    all_consuming(terminated(grid, multispace0))(i).unwrap().1
}

fn grid(i: &str) -> IResult<&str, Grid> {
    into(separated_list1(tag("\n"), row))(i)
}

fn row(i: &str) -> IResult<&str, Vec<usize>> {
    many1(risk_level)(i)
}

fn risk_level(i: &str) -> IResult<&str, usize> {
    map_parser(take(1_usize), unsigned_number)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_dijkstra() {
        let grid = Grid::from(vec![vec![1, 3], vec![2, 4]]);
        let distances = dijkstra(&grid);

        assert_eq!(distances.get(&Vertex::new(1, 0)), 3);
        assert_eq!(distances.get(&Vertex::new(0, 1)), 2);
        assert_eq!(distances.get(&Vertex::new(1, 1)), 6);
    }
}
