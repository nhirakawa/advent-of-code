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

fn dijkstra(graph: &Graph) -> Distances {
    let vertex_set = graph.vertices();

    let mut distances = HashMap::new();

    for vertex in &vertex_set {
        distances.insert(*vertex, usize::MAX);
    }

    let start = Vertex { x: 0, y: 0 };

    *distances.entry(start).or_default() = 0;

    while !vertex_set.is_empty() {}

    Distances { d: distances }
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
