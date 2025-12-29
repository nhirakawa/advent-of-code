use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use anyhow::anyhow;
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let lan_party = parse(input)?;

    let components_of_three = lan_party
        .computers
        .iter()
        .tuple_combinations()
        .filter(|(first, second, third)| {
            first.starts_with_t() || second.starts_with_t() || third.starts_with_t()
        })
        .filter(|(first, second, third)| {
            let first = *first;
            let second = *second;
            let third = *third;

            if let Some(first_connections) = lan_party.connections.get(first)
                && (!first_connections.contains(second) || !first_connections.contains(third))
            {
                return false;
            }

            if let Some(second_connections) = lan_party.connections.get(second)
                && (!second_connections.contains(first) || !second_connections.contains(third))
            {
                return false;
            }

            if let Some(third_connections) = lan_party.connections.get(third)
                && (!third_connections.contains(first) || !third_connections.contains(second))
            {
                return false;
            }

            true
        })
        .count();

    Ok(components_of_three)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let lan_party = parse(input)?;

    let mut seen = HashSet::new();
    let mut current_clique = Vec::new();
    let mut largest_clique = Vec::new();

    for computer in &lan_party.computers {
        if seen.insert(*computer) {
            current_clique.clear();
            current_clique.push(*computer);
        }

        if let Some(connections) = lan_party.connections.get(computer) {
            for connection in connections {
                let mut connected_to_current_clique = true;
                for member in &current_clique {
                    if !lan_party.connections[member].contains(connection) {
                        connected_to_current_clique = false;
                        break;
                    }
                }

                if connected_to_current_clique {
                    // We don't need to check `seen`
                    // If we had seen any neighbors of `computer`, then we would have already seen `computer` itself
                    seen.insert(*connection);
                    current_clique.push(*connection);
                }
            }
        }

        if current_clique.len() > largest_clique.len() {
            largest_clique = current_clique.clone();
        }
    }

    let largest_clique = largest_clique.into_iter().unique().sorted().join(",");

    Ok(largest_clique)
}

#[derive(PartialEq, Eq, Clone)]
struct LanParty {
    computers: HashSet<Computer>,
    connections: HashMap<Computer, HashSet<Computer>>,
}

impl LanParty {
    fn new(
        computers: HashSet<Computer>,
        connections: HashMap<Computer, HashSet<Computer>>,
    ) -> Self {
        Self {
            computers,
            connections,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Computer {
    first: char,
    second: char,
}

impl Computer {
    fn new(first: char, second: char) -> Self {
        Self { first, second }
    }

    fn starts_with_t(&self) -> bool {
        self.first == 't'
    }

    fn from_str(input: &str) -> anyhow::Result<Self> {
        let mut chars = input.chars();
        let first = chars
            .next()
            .ok_or(anyhow!("Could not parse first character"))?;
        let second = chars
            .next()
            .ok_or(anyhow!("Could not parse second character"))?;

        Ok(Self::new(first, second))
    }

    #[allow(dead_code)]
    fn as_index(&self) -> usize {
        (self.first as usize - 'a' as usize) * 26 + (self.second as usize - 'a' as usize)
    }
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.first, self.second)
    }
}

fn parse(input: &str) -> anyhow::Result<LanParty> {
    let mut computers = HashSet::new();
    let mut connections = HashMap::new();

    for (line_number, line) in input.lines().enumerate() {
        let (first, second) = line
            .split_once("-")
            .ok_or(anyhow!("Could not parse line {}", line_number))?;

        let first = Computer::from_str(first)?;
        let second = Computer::from_str(second)?;

        computers.insert(first);
        computers.insert(second);

        let first_connections = connections.entry(first).or_insert(HashSet::new());
        first_connections.insert(second);

        let second_connections = connections.entry(second).or_insert(HashSet::new());
        second_connections.insert(first);
    }

    Ok(LanParty::new(computers, connections))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_index() {
        let computer = Computer::new('a', 'a');
        assert_eq!(computer.as_index(), 0);

        let computer = Computer::new('a', 'b');
        assert_eq!(computer.as_index(), 1);

        let computer = Computer::new('b', 'a');
        assert_eq!(computer.as_index(), 26);

        let computer = Computer::new('z', 'z');
        assert_eq!(computer.as_index(), 675);
    }

    #[test]
    fn test_to_string() {
        let computer = Computer::new('a', 'a');
        assert_eq!(computer.to_string(), "aa");

        let computer = Computer::new('a', 'b');
        assert_eq!(computer.to_string(), "ab");

        let computer = Computer::new('b', 'a');
        assert_eq!(computer.to_string(), "ba");

        let computer = Computer::new('z', 'z');
        assert_eq!(computer.to_string(), "zz");
    }
}
