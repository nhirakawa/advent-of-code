use std::collections::HashMap;

use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-5.txt");

    let part_one = part_one();
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one() -> PartAnswer {
    PartAnswer::default()
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

struct Crane {
    crates: HashMap<usize, Vec<char>>,
}

impl Crane {
    fn new() -> Crane {
        let crates = HashMap::new();
        Crane { crates }
    }

    fn apply(&mut self, instruction: &Instruction) {
        todo!()
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
