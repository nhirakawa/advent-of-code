use std::collections::HashMap;

use common::prelude::*;

pub fn run() -> AdventOfCodeResult<u64, u64> {
    let integers = parse_integers();

    let part_one = part_one(&integers);
    let part_two = part_two(&integers);

    Ok((part_one, part_two))
}

fn part_one(numbers: &Vec<u64>) -> PartAnswer<u64> {
    let start = SystemTime::now();

    let mut ones = 0;
    let mut threes = 0;

    for i in 0..numbers.len() - 1 {
        let lesser = numbers[i];
        let greater = numbers[i + 1];

        match greater - lesser {
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        }
    }

    let solution: u64 = ones * threes;

    let elapsed = start.elapsed().unwrap();

    (solution, elapsed).into()
}

fn part_two(numbers: &Vec<u64>) -> PartAnswer<u64> {
    let start = SystemTime::now();

    let mut graph = HashMap::new();

    for i in 0..numbers.len() {
        let mut neighbors = Vec::new();

        for j in (i + 1)..numbers.len() {
            if numbers[j] - numbers[i] <= 3 {
                neighbors.push(numbers[j]);
            }
        }

        graph.insert(numbers[i], neighbors);
    }

    let mut memoized: HashMap<u64, u64> = HashMap::new();

    let solution = traverse_recursive(&graph, 0, &mut memoized);

    let elapsed = start.elapsed().unwrap();

    (solution, elapsed).into()
}

fn traverse_recursive(
    graph: &HashMap<u64, Vec<u64>>,
    current: u64,
    memoized: &mut HashMap<u64, u64>,
) -> u64 {
    let current_cost = memoized.get(&current);

    if current_cost.is_some() {
        let current_cost = current_cost.unwrap();
        return *current_cost;
    }

    let neighbors = graph.get(&current);

    match neighbors {
        Some(neighbors) => {
            if neighbors.len() == 0 {
                memoized.insert(current, 1);
                return 1;
            }

            let mut value = 0;

            for neighbor in neighbors {
                value += traverse_recursive(graph, *neighbor, memoized);
            }

            memoized.insert(current, value);

            value
        }
        None => 0,
    }
}

fn parse_integers() -> Vec<u64> {
    let input = include_str!("../input/day-10.txt");

    let mut integers: Vec<u64> = input
        .split("\n")
        .into_iter()
        .map(|s| s.parse::<u64>())
        .flatten()
        .collect();

    integers.push(0); // the adapter in the charger

    sort(&mut integers);

    let max = integers[integers.len() - 1];
    integers.push(max + 3); // our phone's adapter

    integers
}

// TODO implement this with counting/radix sort
fn sort(numbers: &mut Vec<u64>) {
    numbers.sort()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answers() {
        let (part_one, part_two) = run().unwrap();

        assert_eq!(*part_one.get_answer(), 2240);
        assert_eq!(*part_two.get_answer(), 99214346656768);
    }
}
