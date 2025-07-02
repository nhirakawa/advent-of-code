use anyhow::bail;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::not_line_ending,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};
use std::collections::HashSet;

type Instructions = Vec<Op>;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let instructions = parse_instructions(input)?;

    let (_, counter) = execute(&instructions);

    Ok(counter.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let instructions = parse_instructions(input)?;

    let mut copy = instructions.to_owned();

    for (index, instruction) in instructions.iter().enumerate() {
        match instruction {
            Op::Jmp { value } => {
                let updated = Op::Nop { value: *value };
                copy[index] = updated;
            }
            Op::Nop { value } => {
                let updated = Op::Jmp { value: *value };
                copy[index] = updated;
            }
            _ => {}
        }

        let (result, counter) = execute(&copy);
        match result {
            OperationResult::Success => {
                return Ok(counter.to_string());
            }
            OperationResult::InfiniteLoop => {
                copy[index] = *instruction;
            }
        }
    }

    bail!("No answer found")
}

fn execute(instructions: &[Op]) -> (OperationResult, u32) {
    let mut program_counter = 0;
    let mut accumulator = 0;
    let mut seen = HashSet::new();

    loop {
        if !seen.insert(program_counter) {
            return (OperationResult::InfiniteLoop, accumulator as u32);
        }

        if program_counter >= instructions.len() {
            return (OperationResult::Success, accumulator as u32);
        }

        let current = &instructions[program_counter];

        match current {
            Op::Acc { value } => {
                accumulator += value;
                program_counter += 1
            }
            Op::Jmp { value } => {
                program_counter = ((program_counter as i32) + value) as usize;
            }
            Op::Nop { value: _ } => program_counter += 1,
        }
    }
}

#[derive(Debug, PartialEq)]
enum OperationResult {
    InfiniteLoop,
    Success,
}

fn parse_instructions(i: &str) -> anyhow::Result<Instructions> {
    instructions(i)
        .map(|(_, ops)| ops)
        .map_err(|e| anyhow::Error::from(e.to_owned()))
}

fn instructions(i: &str) -> IResult<&str, Vec<Op>> {
    separated_list1(tag("\n"), instruction).parse(i)
}

fn instruction(i: &str) -> IResult<&str, Op> {
    alt((nop, acc, jmp)).parse(i)
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Op {
    Nop { value: i32 },
    Acc { value: i32 },
    Jmp { value: i32 },
}

fn nop(i: &str) -> IResult<&str, Op> {
    map(preceded(tag("nop "), number), |n| Op::Nop { value: n }).parse(i)
}

fn acc(i: &str) -> IResult<&str, Op> {
    map(preceded(tag("acc "), number), |n| Op::Acc { value: n }).parse(i)
}

fn jmp(i: &str) -> IResult<&str, Op> {
    map(preceded(tag("jmp "), number), |n| Op::Jmp { value: n }).parse(i)
}

fn number(i: &str) -> IResult<&str, i32> {
    map_res(not_line_ending, |s: &str| s.parse::<i32>()).parse(i)
}
