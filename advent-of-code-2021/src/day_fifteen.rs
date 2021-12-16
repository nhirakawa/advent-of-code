use std::collections::{HashMap, HashSet};

use common::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let part_one = part_one();
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one() -> PartAnswer {
    PartAnswer::default()
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn dijkstra(grid: &Grid) -> Distances {
    let mut vertex_set = grid.vertices();

    let mut distances = HashMap::new();

    for vertex in &vertex_set {
        distances.insert(*vertex, usize::MAX);
    }

    while !vertex_set.is_empty() {
        let min_distance_vertex = &vertex_set
            .iter()
            .min_by_key(|v| distances.get(v).copied().unwrap_or(usize::MAX))
            .copied()
            .unwrap();

        vertex_set.remove(min_distance_vertex);

        let min_distance = distances.get(min_distance_vertex).copied().unwrap();

        for (neighbor, weight) in grid.neighbors(min_distance_vertex) {
            let alternate_distance = min_distance + weight;
        }

        for vertex in &vertex_set {
            todo!()
        }
        todo!()
    }

    Distances { d: distances }
}

#[derive(Debug, PartialEq)]
struct Grid {
    grid: HashMap<Vertex, usize>,
    max_x: usize,
    max_y: usize,
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
    x: u32,
    y: u32,
}

impl Vertex {
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
