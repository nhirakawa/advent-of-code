use common::prelude::*;
use nom::{
    bytes::complete::tag, character::complete::anychar, combinator::map, multi::separated_list1,
    sequence::tuple, IResult,
};
use petgraph::{graph::DiGraph, Graph};

pub fn run() -> AdventOfCodeResult {
    todo!()
}

fn part_one() -> PartAnswer {
    PartAnswer::default()
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn dependency_graph(i: &str) -> IResult<&str, DiGraph<char, ()>> {
    map(dependencies, |edges| DiGraph::from_edges(edges))(i)
}

fn dependencies(i: &str) -> IResult<&str, Vec<(char, char)>> {
    separated_list1(tag("\n"), dependency)(i)
}

fn dependency(i: &str) -> IResult<&str, (char, char)> {
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

fn step(i: &str) -> IResult<&str, char> {
    anychar(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency() {
        assert_eq!(
            dependency("Step C must be finished before step A can begin."),
            Ok(("", ('C', 'A')))
        );
    }
}
