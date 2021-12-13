use std::collections::{HashMap, HashSet};

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
