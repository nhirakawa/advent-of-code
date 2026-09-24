use anyhow::{anyhow, bail};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let testing_facility = TestingFacility::from_str(input)?;
    minimum_moves(testing_facility).ok_or(anyhow!("No solution found"))
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let mut testing_facility = TestingFacility::from_str(input)?;
    let extra_components = vec![
        Component::Generator("elerium".to_owned()),
        Component::Microchip("elerium".to_owned()),
        Component::Generator("dilithium".to_owned()),
        Component::Microchip("dilithium".to_owned()),
    ];
    testing_facility.floors[0].extend(extra_components);
    minimum_moves(testing_facility).ok_or(anyhow!("No solution found"))
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Component {
    Microchip(String),
    Generator(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TestingFacility {
    elevator_floor: usize,
    floors: [Vec<Component>; 4],
}

impl TestingFacility {
    fn is_valid(&self) -> bool {
        self.floors.iter().all(|floor| {
            let has_generator = floor
                .iter()
                .any(|component| matches!(component, Component::Generator(_)));
            if !has_generator {
                return true;
            }
            floor.iter().all(|component| match component {
                Component::Microchip(element) => {
                    floor.contains(&Component::Generator(element.clone()))
                }
                Component::Generator(_) => true,
            })
        })
    }

    fn is_finished(&self) -> bool {
        self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
    }

    fn next_states(&self) -> Vec<TestingFacility> {
        let mut results = Vec::new();
        let current = self.elevator_floor;

        let mut carry_options: Vec<Vec<Component>> = Vec::new();
        for component in &self.floors[current] {
            carry_options.push(vec![component.clone()]);
        }
        for pair in self.floors[current].iter().combinations(2) {
            carry_options.push(pair.into_iter().cloned().collect());
        }

        let mut targets = Vec::new();
        if current > 0 {
            targets.push(current - 1);
        }
        if current < 3 {
            targets.push(current + 1);
        }

        for target in targets {
            for carry in &carry_options {
                let mut next = self.clone();
                next.elevator_floor = target;
                next.floors[current].retain(|component| !carry.contains(component));
                next.floors[target].extend(carry.iter().cloned());
                next.floors[current].sort();
                next.floors[target].sort();
                if next.is_valid() {
                    results.push(next);
                }
            }
        }

        results
    }
}

impl FromStr for TestingFacility {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        if lines.len() != 4 {
            bail!("Expected 4 lines, found {}", lines.len());
        }

        let first_floor = parse_floor(lines[0]);
        let second_floor = parse_floor(lines[1]);
        let third_floor = parse_floor(lines[2]);
        let fourth_floor = parse_floor(lines[3]);

        Ok(TestingFacility {
            elevator_floor: 0,
            floors: [first_floor, second_floor, third_floor, fourth_floor],
        })
    }
}

fn parse_floor(s: &str) -> Vec<Component> {
    s.split(" ")
        .tuple_windows()
        .filter_map(|(first, second)| {
            if second.contains("generator") {
                Some(Component::Generator(first.to_owned()))
            } else if second.contains("microchip") {
                let (microchip, _) = first.split_once("-")?;
                Some(Component::Microchip(microchip.to_owned()))
            } else {
                None
            }
        })
        .collect()
}

fn minimum_moves(initial: TestingFacility) -> Option<usize> {
    let mut start = initial;
    for floor in start.floors.iter_mut() {
        floor.sort();
    }

    let mut visited = HashSet::new();
    visited.insert(start.clone());

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((state, moves)) = queue.pop_front() {
        if state.is_finished() {
            return Some(moves);
        }
        for next in state.next_states() {
            if visited.insert(next.clone()) {
                queue.push_back((next, moves + 1));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_facility_takes_eleven_moves() {
        let facility = TestingFacility {
            elevator_floor: 0,
            floors: [
                vec![
                    Component::Microchip("hydrogen".to_string()),
                    Component::Microchip("lithium".to_string()),
                ],
                vec![Component::Generator("hydrogen".to_string())],
                vec![Component::Generator("lithium".to_string())],
                vec![],
            ],
        };

        assert_eq!(minimum_moves(facility), Some(11));
    }
}
