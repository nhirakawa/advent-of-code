use std::collections::{HashMap, HashSet, VecDeque};

use common::prelude::*;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, combinator::map,
    multi::separated_list1, sequence::tuple, IResult,
};

/**
 * A lot of inspiration and pseudocode taken from https://www.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/?sort=top
 */
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

    let valves = parse(input);

    let all_final_states = generate_all_final_states(&valves, 30);

    let answer = all_final_states
        .into_iter()
        .map(|state| state.current_score)
        .max()
        .unwrap();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn part_two(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let valves = parse(input);

    let all_final_states = generate_all_final_states(&valves, 26);

    let mut best = 0;

    for outer_state in &all_final_states {
        let outer_opened_set: HashSet<String> = outer_state.opened.iter().cloned().collect();

        for inner_state in &all_final_states {
            if outer_state == inner_state {
                continue;
            }

            let inner_opened_set: HashSet<String> = inner_state.opened.iter().cloned().collect();

            let number_of_common_items = outer_opened_set.intersection(&inner_opened_set).count();

            if number_of_common_items == 0 {
                best = best.max(outer_state.current_score + inner_state.current_score);
            }
        }
    }

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(best, elapsed)
}

fn generate_all_final_states(valves: &ValveSystem, time_budget: usize) -> Vec<SearchState> {
    let mut state_queue = VecDeque::new();
    state_queue.push_back(SearchState::new(1, "AA".to_string(), 0, vec![]));

    let mut seen_states = HashMap::new();

    let mut final_states = vec![];

    let mut number_of_final_states = 0;

    while let Some(current_state) = state_queue.pop_front() {
        println!("Checking state {current_state:?}");

        let mut opened_set: HashSet<String> = current_state.opened.iter().cloned().collect();

        let seen_state = (
            current_state.current_time,
            current_state.current_label.clone(),
        );

        let seen_state_score = seen_states.get(&seen_state).cloned().unwrap_or(-1_isize);

        if seen_state_score >= current_state.current_score {
            continue;
        } else {
            seen_states.insert(seen_state, current_state.current_score);
        }

        if current_state.current_time == time_budget {
            number_of_final_states += 1;
            final_states.push(current_state);
            continue;
        }

        let flow_rate = valves
            .flow_rates_by_label
            .get(&current_state.current_label)
            .unwrap();

        // pretend we just opened the current valve
        if *flow_rate > 0 && !opened_set.contains(&current_state.current_label) {
            opened_set.insert(current_state.current_label.clone());

            let new_score: isize = current_state.current_score
                + opened_set
                    .iter()
                    .filter_map(|valve| valves.flow_rates_by_label.get(valve).map(|u| *u as isize))
                    .sum::<isize>();

            let opened_list = opened_set.iter().cloned().collect::<Vec<String>>();
            let new_state = SearchState::new(
                current_state.current_time + 1,
                current_state.current_label.clone(),
                new_score,
                opened_list,
            );

            state_queue.push_back(new_state);
            opened_set.remove(&current_state.current_label);
        }

        // pretend we didn't just open the current valve
        let new_score: isize = current_state.current_score
            + opened_set
                .iter()
                .filter_map(|valve| valves.flow_rates_by_label.get(valve).map(|u| *u as isize))
                .sum::<isize>();

        if let Some(neighbors) = valves.graph.get(&current_state.current_label) {
            let opened_list = opened_set.iter().cloned().collect::<Vec<String>>();

            for (neighbor, distance) in neighbors.iter() {
                let next_time = current_state.current_time + distance;

                if next_time <= time_budget {
                    let new_state = SearchState::new(
                        next_time,
                        neighbor.clone(),
                        new_score,
                        opened_list.clone(),
                    );

                    state_queue.push_back(new_state);
                }
            }
        }
    }

    println!("Looked at {} final states", number_of_final_states);

    final_states
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct SearchState {
    current_time: usize,
    current_label: String,
    current_score: isize,
    opened: Vec<String>,
}

impl SearchState {
    fn new(
        current_time: usize,
        current_label: String,
        current_score: isize,
        opened: Vec<String>,
    ) -> SearchState {
        SearchState {
            current_time,
            current_label,
            current_score,
            opened,
        }
    }
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

        ValveSystem {
            graph,
            flow_rates_by_label,
        }
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
