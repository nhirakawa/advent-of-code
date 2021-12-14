use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

use common::prelude::*;
use log::debug;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::multispace0,
    combinator::{all_consuming, into, map, value},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-12.txt");
    let graph = parse_adjacency_list(input);

    let part_one = part_one(&graph);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(graph: &AdjacencyList) -> PartAnswer {
    let start = SystemTime::now();
    let all_paths = search(graph);

    for path in &all_paths {
        debug!("{}", path);
    }

    let number_of_paths = all_paths.len();

    PartAnswer::new(number_of_paths, start.elapsed().unwrap())
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn search(graph: &AdjacencyList) -> Vec<Path> {
    let mut queue = VecDeque::new();
    queue.push_back(Path::new());

    let mut complete_paths = Vec::new();

    while !queue.is_empty() {
        let next_path = queue.pop_front().unwrap();

        if next_path.is_complete() {
            complete_paths.push(next_path);
        } else {
            for neighbor in &graph.graph[&next_path.last_vertex] {
                if let Some(with_neighbor) = next_path.with_vertex(neighbor) {
                    queue.push_back(with_neighbor);
                }
            }
        }
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

    fn with_vertex(&self, vertex: &Vertex) -> Option<Path> {
        if self.last_vertex == Vertex::End {
            return None;
        }

        match vertex {
            Vertex::Start => None,
            Vertex::End => {
                let mut vertices = self.vertices.clone();
                vertices.push(Vertex::End);

                let mut seen = self.seen.clone();
                seen.insert(Vertex::End);

                let last_vertex = Vertex::End;

                Some(Path {
                    vertices,
                    seen,
                    last_vertex,
                })
            }
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

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for vertex in &self.vertices {
            write!(f, "{} ", vertex)?;
        }

        Ok(())
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

impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            Vertex::Start => "start",
            Vertex::End => "end",
            Vertex::SmallCave(name) | Vertex::LargeCave(name) => name,
        };

        write!(f, "{}", s)
    }
}

fn parse_adjacency_list(i: &str) -> AdjacencyList {
    adjacency_list(i).unwrap().1
}

fn adjacency_list(i: &str) -> IResult<&str, AdjacencyList> {
    into(all_consuming(terminated(edges, multispace0)))(i)
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
    map(take_while1(|c| ('a'..='z').contains(&c)), |small: &str| {
        Vertex::SmallCave(small.to_string())
    })(i)
}

fn large_cave(i: &str) -> IResult<&str, Vertex> {
    map(take_while1(|c| ('A'..='Z').contains(&c)), |large: &str| {
        Vertex::LargeCave(large.to_string())
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_add_vertex() {
        let path = Path::new();

        let next_vertex = Vertex::SmallCave("asdf".to_string());

        let next_path = path.with_vertex(&next_vertex);
        assert!(next_path.is_some());

        let next_path = next_path.unwrap();
        assert!(next_path.seen.contains(&next_vertex));
        assert!(next_path.seen.contains(&Vertex::Start));
        assert_eq!(next_path.last_vertex, next_vertex);

        assert!(next_path.with_vertex(&Vertex::Start).is_none());

        assert!(next_path
            .with_vertex(&Vertex::SmallCave("asdf".to_string()))
            .is_none());
    }

    #[test]
    fn test_small_cave() {
        assert_eq!(
            vertex("asdf"),
            Ok(("", Vertex::SmallCave("asdf".to_string())))
        );
    }

    #[test]
    fn test_large_cave() {
        assert_eq!(
            vertex("ASDF"),
            Ok(("", Vertex::LargeCave("ASDF".to_string())))
        );
    }

    #[test]
    fn test_start() {
        assert_eq!(vertex("start"), Ok(("", Vertex::Start)));
    }

    #[test]
    fn test_end() {
        assert_eq!(vertex("end"), Ok(("", Vertex::End)));
    }

    #[test]
    fn test_parse_edge() {
        assert_eq!(
            edge("start-A"),
            Ok(("", (Vertex::Start, Vertex::LargeCave("A".to_string()))))
        );
    }
}
