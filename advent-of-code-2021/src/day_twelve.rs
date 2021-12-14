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
    let part_two = part_two(&graph);

    Ok((part_one, part_two))
}

fn part_one(graph: &AdjacencyList) -> PartAnswer {
    let start = SystemTime::now();
    let all_paths = search(graph, false);

    for path in &all_paths {
        debug!("part 1 {}", path);
    }

    let number_of_paths = all_paths.len();

    PartAnswer::new(number_of_paths, start.elapsed().unwrap())
}

fn part_two(graph: &AdjacencyList) -> PartAnswer {
    let start = SystemTime::now();
    let all_paths = search(graph, true);

    for path in &all_paths {
        debug!("part 2 {}", path);
    }

    let number_of_paths = all_paths.len();

    PartAnswer::new(number_of_paths, start.elapsed().unwrap())
}

fn search(graph: &AdjacencyList, allow_multiple_visits_to_small_caves: bool) -> HashSet<String> {
    let mut queue = VecDeque::new();
    queue.push_back(Path::new(allow_multiple_visits_to_small_caves));

    let mut complete_paths = HashSet::new();

    while !queue.is_empty() {
        let next_path = queue.pop_front().unwrap();

        if next_path.is_complete() {
            complete_paths.insert(next_path.to_string());
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
    seen: HashMap<Vertex, usize>,
    last_vertex: Vertex,
    allow_multiple_visits_to_small_caves: bool,
}

impl Path {
    fn new(allow_multiple_visits_to_small_caves: bool) -> Path {
        Path {
            vertices: vec![Vertex::Start],
            seen: vec![(Vertex::Start, 1)].into_iter().collect(),
            last_vertex: Vertex::Start,
            allow_multiple_visits_to_small_caves,
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
                seen.insert(Vertex::End, 1);

                let last_vertex = Vertex::End;

                let allow_multiple_visits_to_small_caves =
                    self.allow_multiple_visits_to_small_caves;

                Some(Path {
                    vertices,
                    seen,
                    last_vertex,
                    allow_multiple_visits_to_small_caves,
                })
            }
            Vertex::SmallCave(_) => {
                if self.seen.contains_key(vertex) {
                    if !self.allow_multiple_visits_to_small_caves {
                        return None;
                    }

                    if self.seen.values().any(|count| *count > 1) {
                        return None;
                    }
                }
                let mut vertices = self.vertices.clone();
                vertices.push(vertex.clone());

                let mut seen = self.seen.clone();
                *seen.entry(vertex.clone()).or_default() += 1;

                let last_vertex = vertex.clone();

                let allow_multiple_visits_to_small_caves =
                    self.allow_multiple_visits_to_small_caves;

                Some(Path {
                    vertices,
                    seen,
                    last_vertex,
                    allow_multiple_visits_to_small_caves,
                })
            }
            Vertex::LargeCave(_) => {
                let mut vertices = self.vertices.clone();
                vertices.push(vertex.clone());

                let mut seen = self.seen.clone();
                // never increment count of large caves
                *seen.entry(vertex.clone()).or_default() = 1;

                let last_vertex = vertex.clone();

                let allow_multiple_visits_to_small_caves =
                    self.allow_multiple_visits_to_small_caves;
                Some(Path {
                    vertices,
                    seen,
                    last_vertex,
                    allow_multiple_visits_to_small_caves,
                })
            }
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.vertices
                .iter()
                .map(Vertex::to_string)
                .collect::<Vec<String>>()
                .join(",")
        )
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
    fn test_search_no_multiple_visits() {
        let graph = parse_adjacency_list("start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end\n");
        let paths = search(&graph, false);

        todo!()
    }

    #[test]
    fn test_search_multiple_visits() {
        let graph = parse_adjacency_list("start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end\n");
        let paths = search(&graph, true);
        assert_eq!(
            paths,
            vec![
                "start,A,b,A,b,A,c,A,end".to_string(),
                "start,A,b,A,b,A,end".to_string(),
                "start,A,b,A,b,end".to_string(),
                "start,A,b,A,c,A,b,A,end".to_string(),
                "start,A,b,A,c,A,b,end".to_string(),
                "start,A,b,A,c,A,c,A,end".to_string(),
                "start,A,b,A,c,A,end".to_string(),
                "start,A,b,A,end".to_string(),
                "start,A,b,d,b,A,c,A,end".to_string(),
                "start,A,b,d,b,A,end".to_string(),
                "start,A,b,d,b,end".to_string(),
                "start,A,b,end".to_string(),
                "start,A,c,A,b,A,b,A,end".to_string(),
                "start,A,c,A,b,A,b,end".to_string(),
                "start,A,c,A,b,A,c,A,end".to_string(),
                "start,A,c,A,b,A,end".to_string(),
                "start,A,c,A,b,d,b,A,end".to_string(),
                "start,A,c,A,b,d,b,end".to_string(),
                "start,A,c,A,b,end".to_string(),
                "start,A,c,A,c,A,b,A,end".to_string(),
                "start,A,c,A,c,A,b,end".to_string(),
                "start,A,c,A,c,A,end".to_string(),
                "start,A,c,A,end".to_string(),
                "start,A,end".to_string(),
                "start,b,A,b,A,c,A,end".to_string(),
                "start,b,A,b,A,end".to_string(),
                "start,b,A,b,end".to_string(),
                "start,b,A,c,A,b,A,end".to_string(),
                "start,b,A,c,A,b,end".to_string(),
                "start,b,A,c,A,c,A,end".to_string(),
                "start,b,A,c,A,end".to_string(),
                "start,b,A,end".to_string(),
                "start,b,d,b,A,c,A,end".to_string(),
                "start,b,d,b,A,end".to_string(),
                "start,b,d,b,end".to_string(),
                "start,b,end".to_string()
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_path_add_vertex() {
        let path = Path::new(false);

        let next_vertex = Vertex::SmallCave("asdf".to_string());

        let next_path = path.with_vertex(&next_vertex);
        assert!(next_path.is_some());

        let next_path = next_path.unwrap();
        assert!(next_path.seen.contains_key(&next_vertex));
        assert!(next_path.seen.contains_key(&Vertex::Start));
        assert_eq!(next_path.last_vertex, next_vertex);

        assert!(next_path.with_vertex(&Vertex::Start).is_none());

        assert!(next_path
            .with_vertex(&Vertex::SmallCave("asdf".to_string()))
            .is_none());
    }

    #[test]
    fn test_allow_multiple_visits_to_small_caves() {
        let path = Path::new(true);

        let next_vertex = Vertex::SmallCave("first".to_string());

        let next_path = path.with_vertex(&next_vertex);
        assert!(next_path.is_some());

        let next_path = next_path.unwrap();
        assert!(next_path.seen.contains_key(&next_vertex));
        assert!(next_path.seen.contains_key(&Vertex::Start));
        assert_eq!(next_path.last_vertex, next_vertex);

        assert!(next_path.with_vertex(&Vertex::Start).is_none());

        let next_path = next_path.with_vertex(&Vertex::LargeCave("second".to_string()));
        assert!(next_path.is_some());

        let next_path = next_path.unwrap();

        assert_eq!(
            next_path.vertices,
            vec![
                Vertex::Start,
                Vertex::SmallCave("first".to_string()),
                Vertex::LargeCave("second".to_string())
            ]
        );

        let next_path = next_path.with_vertex(&Vertex::SmallCave("first".to_string()));
        assert!(next_path.is_some());

        let next_path = next_path.unwrap();

        assert_eq!(
            next_path.vertices,
            vec![
                Vertex::Start,
                Vertex::SmallCave("first".to_string()),
                Vertex::LargeCave("second".to_string()),
                Vertex::SmallCave("first".to_string())
            ]
        );

        let empty_path = next_path.with_vertex(&Vertex::SmallCave("first".to_string()));
        assert!(empty_path.is_none());

        let next_path = next_path.with_vertex(&Vertex::SmallCave("third".to_string()));
        assert!(next_path.is_some());

        let next_path = next_path.unwrap();

        assert_eq!(
            next_path.vertices,
            vec![
                Vertex::Start,
                Vertex::SmallCave("first".to_string()),
                Vertex::LargeCave("second".to_string()),
                Vertex::SmallCave("first".to_string()),
                Vertex::SmallCave("third".to_string())
            ]
        );
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
