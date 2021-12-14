use std::collections::{HashMap, HashSet, VecDeque};

use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    combinator::{all_consuming, into, map, value},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

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

fn search(graph: &AdjacencyList) -> Vec<Path> {
    let mut queue = VecDeque::new();
    queue.push_back(Path::new());

    let mut complete_paths = Vec::new();

    while !queue.is_empty() {
        let next_path = queue.pop_back().unwrap();
        for neighbors in &graph.graph[&next_path.last_vertex] {
            todo!()
        }
        todo!()
    }

    complete_paths
}

struct Path {
    vertices: Vec<Vertex>,
    seen: HashSet<Vertex>,
    last_vertex: Vertex,
}

impl Path {
    fn new() -> Path {
        Path {
            vertices: vec![Vertex::Start],
            seen: vec![Vertex::Start].into_iter().collect(),
            last_vertex: Vertex::Start,
        }
    }

    fn is_complete(&self) -> bool {
        self.last_vertex == Vertex::End
    }

    fn add_vertex(&self, vertex: &Vertex) -> Option<Path> {
        match vertex {
            Vertex::Start => None,
            Vertex::End => None,
            Vertex::SmallCave(_) => {
                if self.seen.contains(vertex) {
                    None
                } else {
                    let mut vertices = self.vertices.clone();
                    vertices.push(vertex.clone());

                    let mut seen = self.seen.clone();
                    seen.insert(vertex.clone());

                    let last_vertex = vertex.clone();

                    Some(Path {
                        vertices,
                        seen,
                        last_vertex,
                    })
                }
            }
            Vertex::LargeCave(_) => {
                let mut vertices = self.vertices.clone();
                vertices.push(vertex.clone());

                let mut seen = self.seen.clone();
                seen.insert(vertex.clone());

                let last_vertex = vertex.clone();
                Some(Path {
                    vertices,
                    seen,
                    last_vertex,
                })
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct AdjacencyList {
    graph: HashMap<Vertex, HashSet<Vertex>>,
}

impl From<Vec<(Vertex, Vertex)>> for AdjacencyList {
    fn from(edges: Vec<(Vertex, Vertex)>) -> AdjacencyList {
        let mut graph: HashMap<Vertex, HashSet<Vertex>> = HashMap::new();

        for edge in edges {
            let (first, second) = edge;
            graph
                .entry(first.clone())
                .or_default()
                .insert(second.clone());
            graph.entry(second).or_default().insert(first);
        }

        AdjacencyList { graph }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Vertex {
    Start,
    End,
    SmallCave(String),
    LargeCave(String),
}

fn parse_adjacency_list(i: &str) -> AdjacencyList {
    adjacency_list(i).unwrap().1
}

fn adjacency_list(i: &str) -> IResult<&str, AdjacencyList> {
    into(all_consuming(terminated(edges, tag("\n"))))(i)
}

fn edges(i: &str) -> IResult<&str, Vec<(Vertex, Vertex)>> {
    separated_list1(tag("\n"), edge)(i)
}

fn edge(i: &str) -> IResult<&str, (Vertex, Vertex)> {
    separated_pair(vertex, tag("-"), vertex)(i)
}

fn vertex(i: &str) -> IResult<&str, Vertex> {
    alt((start, end, small_cave, large_cave))(i)
}

fn start(i: &str) -> IResult<&str, Vertex> {
    value(Vertex::Start, tag("start"))(i)
}

fn end(i: &str) -> IResult<&str, Vertex> {
    value(Vertex::End, tag("end"))(i)
}

fn small_cave(i: &str) -> IResult<&str, Vertex> {
    map(take_while(|c| ('a'..='z').contains(&c)), |small: &str| {
        Vertex::SmallCave(small.to_string())
    })(i)
}

fn large_cave(i: &str) -> IResult<&str, Vertex> {
    map(take_while(|c| ('A'..='Z').contains(&c)), |small: &str| {
        Vertex::SmallCave(small.to_string())
    })(i)
}
