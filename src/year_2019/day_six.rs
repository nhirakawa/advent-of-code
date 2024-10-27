use anyhow::anyhow;
use multimap::MultiMap;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending},
    combinator::{all_consuming, map},
    multi::{many0, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let orbits = parse(input);

    let mut orbit_count = HashMap::new();
    orbit_count.insert("COM".to_string(), 0);

    let mut to_check = vec!["COM".to_string()];

    let mut seen = HashSet::new();

    while !to_check.is_empty() {
        let current = to_check.pop().ok_or(anyhow!("No more to check"))?;
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

    Ok(orbit_count.values().sum::<i32>().to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let orbits = parse(input);

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
            .cloned()
            .ok_or(anyhow!("No more to check"))?;

        to_check.remove(&node_with_min_distance);

        for neighbor in orbits
            .get_vec(&node_with_min_distance)
            .ok_or(anyhow!("No neighbors"))?
        {
            if to_check.contains(neighbor) {
                let new_distance = distance
                    .get(&node_with_min_distance)
                    .ok_or(anyhow!("No node found for node {node_with_min_distance}"))?
                    + 1;

                if new_distance < distance.get(neighbor).copied().unwrap_or(u32::MAX) {
                    distance.insert(neighbor.clone(), new_distance);
                    predecessor.insert(neighbor, node_with_min_distance.clone());
                }
            }
        }
    }

    let mut sequence = Vec::new();
    let mut current = Some("SAN".to_string());

    // TODO use while-let
    while current.is_some() {
        let this = current.unwrap();
        sequence.push(this.clone());
        current = predecessor.get(&this).cloned();
    }

    Ok((sequence.len() - 3).to_string()) // remove YOU, SAN, and then count edges (not nodes)
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
        let answer = part_one("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        assert_eq!(answer.unwrap().to_string(), "42");
    }

    #[test]
    fn test_part_two() {
        let answer =
            part_two("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN");

        assert_eq!(answer.unwrap().to_string(), "4");
    }
}
