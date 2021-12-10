use common::prelude::*;
use multimap::MultiMap;
use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending},
    combinator::{all_consuming, map},
    multi::{many0, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::fs::File;
use std::io::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-6.txt");

    let orbits = parse(input);

    let part_one = part_one(&orbits);
    let part_two = part_two(&orbits);

    Ok((part_one, part_two))
}

fn part_one(orbits: &MultiMap<String, String>) -> PartAnswer {
    let start = SystemTime::now();

    let mut orbit_count = HashMap::new();
    orbit_count.insert("COM".to_string(), 0);

    let mut to_check = vec!["COM".to_string()];

    let mut seen = HashSet::new();

    while !to_check.is_empty() {
        let current = to_check.pop().unwrap();
        let current_count = orbit_count[&current];

        seen.insert(current.clone());

        let satellites = orbits.get_vec(&current);

        if satellites.is_none() {
            continue;
        }

        let satellites = satellites.unwrap();

        for satellite in satellites {
            if seen.contains(satellite) {
                continue;
            }

            orbit_count.insert(satellite.clone(), current_count + 1);
            to_check.push(satellite.clone());
        }
    }

    let solution: u32 = orbit_count.values().sum();

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two(orbits: &MultiMap<String, String>) -> PartAnswer {
    let start = SystemTime::now();

    let mut distance: HashMap<String, u32> = HashMap::new();
    let mut predecessor = HashMap::new();

    let mut to_check = HashSet::new();

    distance.insert("YOU".to_string(), 0);

    for each in orbits.keys() {
        to_check.insert(each.clone());
    }

    while !to_check.is_empty() {
        let node_with_min_distance = to_check
            .iter()
            .min_by_key(|p| match distance.get(*p) {
                Some(distance) => *distance,
                None => u32::MAX,
            })
            .unwrap()
            .clone();

        to_check.remove(&node_with_min_distance);

        for neighbor in orbits.get_vec(&node_with_min_distance).unwrap() {
            if to_check.contains(neighbor) {
                let new_distance = distance.get(&node_with_min_distance).unwrap() + 1;

                if new_distance < distance.get(neighbor).copied().unwrap_or(u32::MAX) {
                    distance.insert(neighbor.clone(), new_distance);
                    predecessor.insert(neighbor, node_with_min_distance.clone());
                }
            }
        }
    }

    let mut sequence = Vec::new();
    let mut current = Some("SAN".to_string());

    while current.is_some() {
        let this = current.unwrap();
        sequence.push(this.clone());
        current = predecessor.get(&this).cloned();
    }

    let solution = sequence.len() - 3; // remove YOU, SAN, and then count edges (not nodes)

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn parse(i: &str) -> MultiMap<String, String> {
    let newline = many0(line_ending);
    let orbits = all_consuming(terminated(orbits, newline))(i).unwrap().1;

    let mut neighbors = MultiMap::new();

    for (planet, satellite) in orbits {
        neighbors.insert(planet.clone(), satellite.clone());
        neighbors.insert(satellite.clone(), planet.clone());
    }

    neighbors
}

fn orbits(i: &str) -> IResult<&str, Vec<(String, String)>> {
    separated_list1(tag("\n"), orbit)(i)
}

fn orbit(i: &str) -> IResult<&str, (String, String)> {
    let sep = tag(")");
    separated_pair(planet, sep, planet)(i)
}

fn planet(i: &str) -> IResult<&str, String> {
    map(alphanumeric1, |s: &str| s.to_string())(i)
}

fn _write_dot(orbits: &[(String, String)]) {
    let mut file = File::create("2019_6_orbits.dot").unwrap();

    writeln!(file, "digraph orbits {{").unwrap();

    for (planet, satellite) in orbits {
        writeln!(file, "\t\"orbit_{}\" -> \"orbit_{}\";", planet, satellite).unwrap();
    }

    write!(file, "}}").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let orbits = parse("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");

        let answer = part_one(&orbits);

        assert_eq!(answer.get_answer(), "42");
    }

    #[test]
    fn test_part_two() {
        let orbits = parse("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");

        let answer = part_two(&orbits);

        assert_eq!(answer.get_answer(), "4");
    }
}
