use std::collections::HashMap;

use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-5.txt");

    let (stacks, ids, instructions) = parse(input);

    let part_one = part_one(stacks.clone(), ids.clone(), instructions.clone());
    let part_two = part_two(stacks, ids, instructions);

    Ok((part_one, part_two))
}

fn part_one(
    stacks: Vec<Vec<CrateId>>,
    ids: Vec<usize>,
    instructions: Vec<Instruction>,
) -> PartAnswer {
    let start = SystemTime::now();

    let mut crane = Crane::new(stacks, ids);

    for instruction in instructions {
        crane.apply_in_series(&instruction);
    }

    let answer = crane.get_top_of_stacks();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn part_two(
    stacks: Vec<Vec<CrateId>>,
    ids: Vec<usize>,
    instructions: Vec<Instruction>,
) -> PartAnswer {
    let start = SystemTime::now();

    let mut crane = Crane::new(stacks, ids);

    for instruction in instructions {
        crane.apply_in_parallel(&instruction);
    }

    let answer = crane.get_top_of_stacks();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

struct Crane {
    crates: HashMap<usize, Vec<char>>,
    ids: Vec<usize>,
}

impl Crane {
    fn new(stacks: Vec<Vec<CrateId>>, ids: Vec<usize>) -> Crane {
        let mut crates = HashMap::new();

        for id in &ids {
            crates.insert(*id, vec![]);
        }

        for row in stacks.iter().rev() {
            let mut index = 0;

            for column in row {
                if let CrateId::Value(value) = column {
                    crates.get_mut(&ids[index]).unwrap().push(*value);
                }

                index += 1;
            }
        }
        Crane { crates, ids }
    }

    fn get_top_of_stacks(&self) -> String {
        self.ids
            .iter()
            .map(|id| self.crates[id].iter().rev().next().unwrap())
            .cloned()
            .collect()
    }

    fn apply_in_series(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.quantity {
            let item = self
                .crates
                .get_mut(&instruction.source)
                .unwrap()
                .pop()
                .unwrap();

            self.crates
                .get_mut(&instruction.destination)
                .unwrap()
                .push(item);
        }
    }

    fn apply_in_parallel(&mut self, instruction: &Instruction) {
        let mut batch = vec![];

        for _ in 0..instruction.quantity {
            let item = self
                .crates
                .get_mut(&instruction.source)
                .unwrap()
                .pop()
                .unwrap();

            batch.push(item);
        }

        batch.reverse();

        self.crates
            .get_mut(&instruction.destination)
            .unwrap()
            .append(&mut batch);
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Instruction {
    quantity: usize,
    source: usize,
    destination: usize,
}

impl Instruction {
    fn new(quantity: usize, source: usize, destination: usize) -> Instruction {
        Instruction {
            quantity,
            source,
            destination,
        }
    }
}

impl From<(usize, usize, usize)> for Instruction {
    fn from(raw: (usize, usize, usize)) -> Instruction {
        let (quantity, source, destination) = raw;
        Instruction::new(quantity, source, destination)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum CrateId {
    Value(char),
    Empty,
}

fn parse(i: &str) -> (Vec<Vec<CrateId>>, Vec<usize>, Vec<Instruction>) {
    map(
        separated_pair(stacks_and_ids, tag("\n\n"), instructions),
        |((stacks, ids), instructions)| (stacks, ids, instructions),
    )(i)
    .unwrap()
    .1
}

fn stacks_and_ids(i: &str) -> IResult<&str, (Vec<Vec<CrateId>>, Vec<usize>)> {
    separated_pair(stacks, tag("\n"), stack_ids)(i)
}

// crates

fn stacks(i: &str) -> IResult<&str, Vec<Vec<CrateId>>> {
    separated_list1(tag("\n"), rows)(i)
}

fn rows(i: &str) -> IResult<&str, Vec<CrateId>> {
    separated_list1(tag(" "), crate_id)(i)
}

fn crate_id(i: &str) -> IResult<&str, CrateId> {
    alt((crate_id_value, empty))(i)
}

fn crate_id_value(i: &str) -> IResult<&str, CrateId> {
    map(delimited(tag("["), anychar, tag("]")), CrateId::Value)(i)
}

fn empty(i: &str) -> IResult<&str, CrateId> {
    value(CrateId::Empty, tag("   "))(i)
}

// stacks

fn stack_ids(i: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(" "), stack_id)(i)
}

fn stack_id(i: &str) -> IResult<&str, usize> {
    delimited(tag(" "), unsigned_number, tag(" "))(i)
}

// instructions

fn instructions(i: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(tag("\n"), instruction)(i)
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            tag("move "),
            unsigned_number,
            tag(" from "),
            unsigned_number,
            tag(" to "),
            unsigned_number,
        )),
        |(_, quantity, _, source, _, destination)| Instruction::new(quantity, source, destination),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crane() {
        let first_row = vec![CrateId::Empty, CrateId::Value('D'), CrateId::Empty];
        let second_row = vec![CrateId::Value('N'), CrateId::Value('C'), CrateId::Empty];
        let third_row = vec![
            CrateId::Value('Z'),
            CrateId::Value('M'),
            CrateId::Value('P'),
        ];

        let mut crane = Crane::new(vec![first_row, second_row, third_row], vec![1, 2, 3]);

        assert_eq!(crane.crates[&1], vec!['Z', 'N']);
        assert_eq!(crane.crates[&2], vec!['M', 'C', 'D']);
        assert_eq!(crane.crates[&3], vec!['P']);

        crane.apply_in_series(&Instruction::new(1, 2, 1));

        assert_eq!(crane.crates[&1], vec!['Z', 'N', 'D']);
    }
}
