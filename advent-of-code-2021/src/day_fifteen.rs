use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

use common::{parse::unsigned_number, prelude::*};
use log::{debug, info};
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
    let part_two = part_two(&grid);

    Ok((part_one, part_two))
}

fn part_one(grid: &Grid) -> PartAnswer {
    let start = SystemTime::now();

    let distance = min_distance(grid);

    PartAnswer::new(distance, start.elapsed().unwrap())
}

fn part_two(grid: &Grid) -> PartAnswer {
    let start = SystemTime::now();

    let grid = scale(grid, 5);

    let distance = min_distance(&grid);

    PartAnswer::new(distance, start.elapsed().unwrap())
}

fn scale(grid: &Grid, scalar: usize) -> Grid {
    let mut new_grid = HashMap::new();

    for (vertex, weight) in grid.grid.iter() {
        for x_multiplier in 0..scalar {
            let x_step = vertex.x + (grid.x_size * x_multiplier);

            for y_multiplier in 0..scalar {
                let y_step = vertex.y + (grid.y_size * y_multiplier);

                debug!(
                    "scaling {} with x_multiplier {}, y_multiplier: {}, max_x: {}, max_y: {}, x_step: {}, y_step: {}",
                    vertex, x_multiplier, y_multiplier, grid.x_size, grid.y_size, x_step, y_step
                );

                let new_vertex = Vertex::new(x_step, y_step);
                let new_weight = *weight + x_multiplier + y_multiplier;
                let new_weight = if new_weight >= 10 {
                    (new_weight + 1) % 10
                } else {
                    new_weight
                };

                debug!("new vertex {}, new weight {}", new_vertex, new_weight);

                new_grid.insert(new_vertex, new_weight);
            }
        }
    }

    Grid::new(new_grid)
}

fn min_distance(grid: &Grid) -> usize {
    let distances = dijkstra(grid);

    let lower_right = Vertex {
        x: grid.x_size - 1,
        y: grid.y_size - 1,
    };

    distances.get(&lower_right)
}

fn dijkstra(grid: &Grid) -> Distances {
    let mut vertex_set = grid.vertices();

    let mut distances = HashMap::new();

    for WeightedVertex { vertex, weight: _ } in &vertex_set {
        let initial_distance = if vertex.is_start() { 0 } else { usize::MAX };
        debug!("{} has initial distance {}", vertex, initial_distance);
        distances.insert(*vertex, initial_distance);
    }

    while let Some(WeightedVertex { vertex, weight }) = vertex_set.pop() {
        debug!("min-distance-vertex is {}, weight is {}", vertex, weight);

        for (neighbor, neighbor_weight) in grid.neighbors(&vertex) {
            let alternate_distance = weight + neighbor_weight;

            let current_neighbor_distance = distances[&neighbor];

            debug!(
                "{} currently has distance {}",
                neighbor, current_neighbor_distance
            );

            if alternate_distance < current_neighbor_distance {
                debug!(
                    "updating distance for {} to {}",
                    neighbor, alternate_distance
                );

                vertex_set.push(WeightedVertex {
                    vertex: neighbor,
                    weight: alternate_distance,
                });
                distances.insert(neighbor, alternate_distance);
            }
        }
    }

    info!("{:?}", distances);

    Distances { d: distances }
}

fn min(first: usize, second: usize) -> usize {
    first.min(second)
}

#[derive(Debug, PartialEq)]
struct Grid {
    grid: HashMap<Vertex, usize>,
    x_size: usize,
    y_size: usize,
}

impl Grid {
    fn new(grid: HashMap<Vertex, usize>) -> Grid {
        let mut max_x = 0;
        let mut max_y = 0;

        for vertex in grid.keys() {
            max_x = max_x.max(vertex.x);
            max_y = max_y.max(vertex.y);
        }

        let x_size = max_x + 1;
        let y_size = max_y + 1;

        Grid {
            grid,
            x_size,
            y_size,
        }
    }

    fn value(&self, source: &Vertex) -> Option<usize> {
        self.grid.get(source).copied()
    }

    fn vertices(&self) -> BinaryHeap<WeightedVertex> {
        self.grid
            .iter()
            .map(|(vertex, weight)| {
                let weight = if vertex.is_start() { 0 } else { *weight };

                WeightedVertex {
                    vertex: *vertex,
                    weight,
                }
            })
            .collect()
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
}

impl From<Vec<Vec<usize>>> for Grid {
    fn from(raw: Vec<Vec<usize>>) -> Grid {
        let mut grid = HashMap::new();

        for (y, row) in raw.into_iter().enumerate() {
            for (x, risk_level) in row.into_iter().enumerate() {
                grid.insert((x, y).into(), risk_level);
            }
        }

        Grid::new(grid)
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Vertex {
    x: usize,
    y: usize,
}

impl Vertex {
    fn new(x: usize, y: usize) -> Vertex {
        Vertex { x, y }
    }

    fn is_start(&self) -> bool {
        self.x == 0 && self.y == 0
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct WeightedVertex {
    vertex: Vertex,
    weight: usize,
}

impl From<(Vertex, usize)> for WeightedVertex {
    fn from(tuple: (Vertex, usize)) -> WeightedVertex {
        let (vertex, weight) = tuple;
        WeightedVertex { vertex, weight }
    }
}

impl PartialOrd for WeightedVertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WeightedVertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight).reverse()
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

        debug!("{:#?}", distances);

        assert_eq!(distances.get(&Vertex::new(1, 0)), 3);
        assert_eq!(distances.get(&Vertex::new(0, 1)), 2);
        assert_eq!(distances.get(&Vertex::new(1, 1)), 6);
    }

    #[test]
    fn test_scale_grid() {
        let grid = Grid::from(vec![vec![1, 3], vec![7, 9]]);

        let scaled = scale(&grid, 2);

        assert_eq!(scaled.value(&Vertex::new(0, 0)), Some(1));
        assert_eq!(scaled.value(&Vertex::new(0, 2)), Some(2));
        assert_eq!(scaled.value(&Vertex::new(3, 1)), Some(1));
        assert_eq!(scaled.value(&Vertex::new(2, 3)), Some(9));
    }
}
