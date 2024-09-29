use std::time::SystemTime;
use crate::year_2019::computer::{self, Computer};
use crate::common::answer::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-5.txt");

    let part_one = part_one(input);

    let part_two = part_two(input);

    Ok((part_one, part_two))
}

fn part_one(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let inputs = vec![1];
    let mut computer = Computer::from_program_and_input(input, inputs);

    let solution = run_computer(&mut computer);

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let inputs = vec![5];
    let mut computer = Computer::from_program_and_input(input, inputs);

    let solution = run_computer(&mut computer);

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn run_computer(computer: &mut Computer) -> computer::Data {
    computer.step_until_halt();

    let outputs = computer.get_outputs();

    if outputs.len() == 1 {
        return outputs[0];
    }

    for i in 0..outputs.len() - 2 {
        let output = outputs[i];
        if output != 0 {
            panic!("found non-zero output {} ({:?})", output, outputs);
        }
    }

    outputs[outputs.len() - 1]
}
