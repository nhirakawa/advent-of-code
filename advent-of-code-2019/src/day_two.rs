use crate::computer;
use common::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let part_one = part_one();
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one() -> PartAnswer {
    let start = SystemTime::now();
    let solution = run_with_initial_memory(12, 2);
    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two() -> PartAnswer {
    let start = SystemTime::now();

    for i in 0..100 {
        for j in 0..100 {
            let output = run_with_initial_memory(i, j);

            if output == 19690720 {
                let solution = (100 * i) + j;

                return PartAnswer::new(solution, start.elapsed().unwrap());
            }
        }
    }

    panic!()
}

fn run_with_initial_memory(arg1: i32, arg2: i32) -> i32 {
    let input = include_str!("../input/day-2.txt");
    let mut computer = computer::Computer::from(input);
    computer.set(1, arg1);
    computer.set(2, arg2);
    computer.step_until_halt();

    computer[0]
}
