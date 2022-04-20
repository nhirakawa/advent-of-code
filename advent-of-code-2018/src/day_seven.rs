use common::prelude::*;
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, map_opt},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use petgraph::{algo::toposort, graph::DiGraph, Directed, Graph};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-7.txt");
    let dependencies = parse_dependencies(input);

    let part_one = part_one(&dependencies);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(dependencies: &Graph<u8, ()>) -> PartAnswer {
    let topological_sort = toposort(dependencies, None).unwrap();

    for node in topological_sort {
        let node = dependencies[node];
    }
    PartAnswer::default()
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn parse_dependencies(i: &str) -> Graph<u8, ()> {
    let dependencies = dependencies(i).unwrap().1;

    let mut graph: Graph<u8, ()> = Graph::default();

    for (source, target) in dependencies {
        let source = graph.add_node(source);
        let target = graph.add_node(target);
        graph.add_edge(source, target, ());
    }

    graph
}

fn dependencies(i: &str) -> IResult<&str, Vec<(u8, u8)>> {
    separated_list1(tag("\n"), dependency)(i)
}

fn dependency(i: &str) -> IResult<&str, (u8, u8)> {
    map(
        tuple((
            tag("Step "),
            step,
            tag(" must be finished before step "),
            step,
            tag(" can begin."),
        )),
        |(_, first, _, second, _)| (first, second),
    )(i)
}

fn step(i: &str) -> IResult<&str, u8> {
    map_opt(anychar, |c| c.to_digit(26).map(|inner| inner as u8))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency() {
        assert_eq!(
            dependency("Step C must be finished before step A can begin."),
            Ok(("", ('C' as u8, 'A' as u8)))
        )
    }
}
