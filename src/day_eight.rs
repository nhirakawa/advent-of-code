use std::collections::HashSet;

use crate::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, not_line_ending},
    combinator::{map, map_res, value},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};

type Instructions = Vec<Op>;

pub fn run() -> AdventOfCodeResult {
    let start = SystemTime::now();

    let input = include_str!("../input/day-8.txt");
    let instructions = parse_instructions(input)?;

    let parse_ms = start.elapsed().unwrap().as_millis();

    let part_one = part_one(&instructions, parse_ms);
    let part_two = part_two(&instructions, parse_ms)?;

    Ok((Ok(part_one), Ok(part_two)))
}

fn part_one(instructions: &Instructions, parse_ms: u128) -> AnswerWithTiming {
    let start = SystemTime::now();

    let (_, counter) = execute(instructions);

    let elapsed = start.elapsed().unwrap().as_millis() + parse_ms;
    let elapsed = Duration::from_millis(elapsed as u64);

    (counter as u64, elapsed)
}

fn part_two(
    instructions: &Instructions,
    parse_ms: u128,
) -> Result<AnswerWithTiming, AdventOfCodeError> {
    let mut start = SystemTime::now();

    let mut copy = instructions.clone();

    for (index, instruction) in instructions.into_iter().enumerate() {
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
                let elapsed = start.elapsed().unwrap().as_millis() + parse_ms;
                let elapsed = Duration::from_millis(elapsed as u64);
                return Ok((counter as u64, elapsed));
            }
            OperationResult::InfiniteLoop => {
                copy[index] = *instruction;
            }
        }
    }

    Err(AdventOfCodeError::NoAnswerFoundPartTwo)
}

fn execute(instructions: &Instructions) -> (OperationResult, i32) {
    let mut program_counter = 0;
    let mut accumulator = 0;
    let mut seen = HashSet::new();

    loop {
        if !seen.insert(program_counter) {
            return (OperationResult::InfiniteLoop, accumulator);
        }

        if program_counter >= instructions.len() {
            return (OperationResult::Success, accumulator);
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

fn parse_instructions(i: &str) -> Result<Instructions, AdventOfCodeError> {
    let (_, ops) = instructions(i).unwrap();

    Ok(ops)
}

fn instructions(i: &str) -> IResult<&str, Vec<Op>> {
    separated_list1(tag("\n"), instruction)(i)
}

fn instruction(i: &str) -> IResult<&str, Op> {
    alt((nop, acc, jmp))(i)
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Op {
    Nop { value: i32 },
    Acc { value: i32 },
    Jmp { value: i32 },
}

fn nop(i: &str) -> IResult<&str, Op> {
    map(preceded(tag("nop "), number), |n| Op::Nop { value: n })(i)
}

fn acc(i: &str) -> IResult<&str, Op> {
    map(preceded(tag("acc "), number), |n| Op::Acc { value: n })(i)
}

fn jmp(i: &str) -> IResult<&str, Op> {
    map(preceded(tag("jmp "), number), |n| Op::Jmp { value: n })(i)
}

fn number(i: &str) -> IResult<&str, i32> {
    map_res(not_line_ending, |s: &str| s.parse::<i32>())(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        let (part_one, part_two) = run().unwrap();
        let (part_one, _) = part_one.unwrap();
        let (part_two, _) = part_two.unwrap();

        assert_eq!(part_one, 1859);
        assert_eq!(part_two, 1235);
    }
}
