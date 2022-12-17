use std::collections::HashMap;

use common::prelude::*;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::map,
    multi::separated_list1, sequence::tuple, IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-16.txt");

    let valves = parse(input);

    let mut flow_rates_by_label = HashMap::new();

    for valve in &valves {
        flow_rates_by_label.insert(valve.label.clone(), valve.flow_rate);
    }

    let mut graph_parts = vec![];

    graph_parts.push("graph Valves {".to_string());

    for valve in valves {
        for connecting_valve in &valve.connecting_tunnels {
            graph_parts.push(format!(
                "\t{}_{} -- {}_{};",
                valve.label,
                valve.flow_rate,
                connecting_valve,
                flow_rates_by_label[connecting_valve]
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
    valves: &[Valve],
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

struct ValveSystem {
    valves: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valve {
    label: String,
    flow_rate: usize,
    connecting_tunnels: Vec<String>,
}

impl Valve {
    fn new(label: String, flow_rate: usize, connecting_tunnels: Vec<String>) -> Valve {
        Valve {
            label,
            flow_rate,
            connecting_tunnels,
        }
    }
}

fn parse(i: &str) -> Vec<Valve> {
    let valves = finish(valves)(i).unwrap().1;

    valves
}

fn valves(i: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(tag("\n"), valve)(i)
}

fn valve(i: &str) -> IResult<&str, Valve> {
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
            Valve::new(label, flow_rate, connecting_tunnels)
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
                Valve::new(
                    "DD".to_string(),
                    20,
                    vec!["CC".to_string(), "AA".to_string(), "EE".to_string()]
                )
            ))
        );

        assert_eq!(
            valve("Valve HH has flow rate=22; tunnel leads to valve GG"),
            Ok(("", Valve::new("HH".to_string(), 22, vec!["GG".to_string()])))
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
