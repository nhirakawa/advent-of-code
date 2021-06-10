use crate::computer::Computer;
use common::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-5.txt");

    let mut computer: Computer = input.into();

    let part_one = part_one(&mut computer);

    computer.reset();

    let part_two = part_two(&mut computer);

    Ok((part_one, part_two))
}

fn part_one(computer: &mut Computer) -> PartAnswer {
    let start = SystemTime::now();

    let solution = run_computer(computer, 1);

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two(computer: &mut Computer) -> PartAnswer {
    let start = SystemTime::now();

    let solution = run_computer(computer, 5);

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn run_computer(computer: &mut Computer, input: i32) -> i32 {
    let mut outputs = Vec::new();

    computer.step_until_halt(Some(input), |o| outputs.push(o));

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
