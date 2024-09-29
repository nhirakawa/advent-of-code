use std::time::SystemTime;
use crate::common::answer::*;
use crate::year_2019::computer::{self, Computer};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-9.txt");

    let part_one = part_one(input);
    let part_two = part_two(input);

    Ok((part_one, part_two))
}

fn part_one(i: &str) -> PartAnswer {
    let start = SystemTime::now();

    let output = run_computer(i, 1);

    PartAnswer::new(output, start.elapsed().unwrap())
}

fn part_two(i: &str) -> PartAnswer {
    let start = SystemTime::now();

    let output = run_computer(i, 2);

    PartAnswer::new(output, start.elapsed().unwrap())
}

fn run_computer(i: &str, input: computer::Data) -> computer::Data {
    let mut computer = Computer::from_program(i);
    computer.push_input(input);
    computer.step_until_halt();
    let outputs = computer.get_outputs();
    for i in 0..outputs.len() - 1 {
        if outputs[i] != 0 {
            panic!("found abnormal output in {:?}", outputs);
        }
    }

    outputs.last().cloned().unwrap()
}
