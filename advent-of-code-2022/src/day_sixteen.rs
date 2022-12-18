use std::collections::HashMap;

use common::prelude::*;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::map,
    multi::separated_list1, sequence::tuple, IResult,
};
use petgraph::{adj::NodeIndex, visit::IntoNeighbors, Graph, Undirected};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-16.txt");

    let valves = parse(input);

    let mut graph_parts = vec![];

    graph_parts.push("graph Valves {".to_string());

    for (label, neighbors) in valves.graph {
        for (neighbor, weight) in neighbors {
            graph_parts.push(format!(
                "\t{} -- {} [ label=\"{}\"]",
                label, neighbor, weight
            ));
        }
    }

    graph_parts.push("}".to_string());

    write_dot("2022-16.dot", &graph_parts.join("\n"));

    let part_one = part_one(input);
    let part_two = part_two(input);

    Ok((part_one, part_two))
}

fn part_one(input: &str) -> PartAnswer {
    let start = SystemTime::now();
    let _elapsed = start.elapsed().unwrap();

    PartAnswer::default()
}

fn part_two(input: &str) -> PartAnswer {
    let start = SystemTime::now();
    let _elapsed = start.elapsed().unwrap();

    PartAnswer::default()
}

fn floyd_warshall(
    valves: &[ValveWithConnections],
) -> (
    HashMap<(String, String), usize>,
    HashMap<(String, String), String>,
) {
    for valve in valves {
        for connected_valve in &valve.connecting_tunnels {
            todo!()
        }
        todo!()
    }
    todo!()
}

#[derive(Debug, Clone)]
struct ValveSystem {
    graph: HashMap<String, HashMap<String, usize>>,
    flow_rates_by_label: HashMap<String, usize>,
}

impl ValveSystem {
    fn new(valves: Vec<ValveWithConnections>) -> ValveSystem {
        let mut graph = HashMap::new();
        let mut flow_rates_by_label = HashMap::new();

        // initialize
        for valve in &valves {
            graph.insert(valve.label.clone(), valve.connecting_tunnels.clone());
            flow_rates_by_label.insert(valve.label.clone(), valve.flow_rate);
        }

        for valve in &valves {
            if valve.flow_rate == 0 && valve.label != "AA" {
                // remove valve from flow rates and graph
                flow_rates_by_label.remove(&valve.label);

                // connect every pair of neighbors with combined weight
                for (neighbor, neighbor_weight) in &valve.connecting_tunnels {
                    if !graph.contains_key(neighbor) {
                        continue;
                    }

                    for (other_neighbor, other_neighbor_weight) in
                        graph.get_mut(&valve.label).cloned().unwrap()
                    {
                        if neighbor == &other_neighbor {
                            continue;
                        }

                        let combined_weight = *neighbor_weight + other_neighbor_weight;

                        if let Some(mutable_valve) = graph.get_mut(neighbor) {
                            println!("Removing {} as neighbor of {}", valve.label, neighbor);
                            mutable_valve.remove(&valve.label);
                            println!(
                                "Adding {} as neighbor of {} with weight {}",
                                other_neighbor, neighbor, combined_weight
                            );
                            mutable_valve.insert(other_neighbor.clone(), combined_weight);
                        }

                        if let Some(mutable_valve) = graph.get_mut(&other_neighbor) {
                            println!("Removing {} as neighbor of {}", valve.label, neighbor);
                            mutable_valve.remove(&valve.label);

                            println!(
                                "Adding {} as neighbor of {} with weight {}",
                                neighbor, other_neighbor, combined_weight
                            );
                            mutable_valve.insert(neighbor.clone(), combined_weight);
                        }
                    }
                }

                println!("Removing {} from graph", valve.label);
                graph.remove(&valve.label);

                println!("{:?}", graph);

                println!();
            }
        }

        ValveSystem {
            graph,
            flow_rates_by_label,
        }
    }

    fn order_vertices(first: &str, second: &str) -> (String, String) {
        (first.min(second).to_string(), first.max(second).to_string())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valve {
    label: String,
    flow_rate: usize,
}

impl Valve {
    fn new(label: String, flow_rate: usize) -> Valve {
        Valve { label, flow_rate }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ValveWithConnections {
    label: String,
    flow_rate: usize,
    connecting_tunnels: HashMap<String, usize>,
}

impl ValveWithConnections {
    fn new(
        label: String,
        flow_rate: usize,
        connecting_tunnels: Vec<String>,
    ) -> ValveWithConnections {
        let connecting_tunnels = connecting_tunnels.into_iter().map(|s| (s, 1)).collect();
        ValveWithConnections {
            label,
            flow_rate,
            connecting_tunnels,
        }
    }
}

fn parse(i: &str) -> ValveSystem {
    let valves = finish(valves)(i).unwrap().1;

    valves
}

fn valves(i: &str) -> IResult<&str, ValveSystem> {
    map(separated_list1(tag("\n"), valve), ValveSystem::new)(i)
}

fn valve(i: &str) -> IResult<&str, ValveWithConnections> {
    map(
        tuple((
            tag("Valve "),
            map(alpha1, |s: &str| s.to_string()),
            tag(" has flow rate="),
            unsigned_number,
            tag("; "),
            alt((tag("tunnels lead"), tag("tunnel leads"))),
            alt((tag(" to valves "), tag(" to valve "))),
            connections,
        )),
        |(_, label, _, flow_rate, _, _, _, connecting_tunnels)| {
            ValveWithConnections::new(label, flow_rate, connecting_tunnels)
        },
    )(i)
}

fn connections(i: &str) -> IResult<&str, Vec<String>> {
    alt((multiple_connections, single_connection))(i)
}

fn multiple_connections(i: &str) -> IResult<&str, Vec<String>> {
    separated_list1(tag(", "), map(alpha1, |s: &str| s.to_string()))(i)
}

fn single_connection(i: &str) -> IResult<&str, Vec<String>> {
    map(alpha1, |s: &str| vec![s.to_string()])(i)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_valve() {
        assert_eq!(
            valve("Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE"),
            Ok((
                "",
                ValveWithConnections::new(
                    "DD".to_string(),
                    20,
                    vec!["CC".to_string(), "AA".to_string(), "EE".to_string()]
                )
            ))
        );

        assert_eq!(
            valve("Valve HH has flow rate=22; tunnel leads to valve GG"),
            Ok((
                "",
                ValveWithConnections::new("HH".to_string(), 22, vec!["GG".to_string()])
            ))
        );
    }

    #[test]
    fn test_connections() {
        assert_eq!(
            connections("EE, GG"),
            Ok(("", vec!["EE".to_string(), "GG".to_string()]))
        );

        assert_eq!(connections("GG"), Ok(("", vec!["GG".to_string()])));
    }

    #[test]
    fn test_multiple_connections() {
        assert_eq!(
            multiple_connections("EE, GG"),
            Ok(("", vec!["EE".to_string(), "GG".to_string()]))
        );
    }

    #[test]
    fn test_single_connection() {
        assert_eq!(single_connection("GG"), Ok(("", vec!["GG".to_string()])));
    }
}
