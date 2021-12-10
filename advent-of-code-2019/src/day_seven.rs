use crate::computer;
use crate::computer::Computer;
use common::prelude::*;
use itertools::Itertools;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-7.txt");

    let part_one = part_one(input);
    let part_two = part_two(input);

    Ok((part_one, part_two))
}

fn part_one(i: &str) -> PartAnswer {
    let start = SystemTime::now();
    let initial_sequence = vec![0, 1, 2, 3, 4];

    let mut max_output = 0;

    for permutation in initial_sequence.into_iter().permutations(5) {
        let output = run_sequence_part_one(i, permutation);

        max_output = max_output.max(output);
    }

    PartAnswer::new(max_output, start.elapsed().unwrap())
}

fn part_two(i: &str) -> PartAnswer {
    let start = SystemTime::now();
    let initial_sequence = vec![5, 6, 7, 8, 9];

    let mut max_output = 0;

    for permutation in initial_sequence.into_iter().permutations(5) {
        let output = run_sequence_part_two(i, permutation);

        max_output = max_output.max(output);
    }

    PartAnswer::new(max_output, start.elapsed().unwrap())
}

fn run_sequence_part_one(i: &str, sequence: Vec<computer::Data>) -> computer::Data {
    let mut last_output = None;

    for phase_setting in sequence {
        let amplifier_input = last_output.unwrap_or(0);

        let inputs = vec![phase_setting, amplifier_input];
        let inputs = Box::new(inputs);

        let mut computer = Computer::from_program_and_input(i, inputs);

        computer.step_until_output();

        last_output = computer.get_outputs().last().cloned();
    }

    last_output.unwrap()
}

fn run_sequence_part_two(program: &str, sequence: Vec<computer::Data>) -> computer::Data {
    let mut computers = Vec::with_capacity(sequence.len());

    for (i, data) in sequence.iter().enumerate() {
        let mut computer = Computer::from_program(program);
        computer.push_input(*data);

        if i == 0 {
            computer.push_input(0);
        }

        computers.push(computer);
    }

    loop {
        let are_all_halted = computers.iter_mut().all(|c| c.is_halted());

        if are_all_halted {
            break;
        }

        let all_are_blocked_on_input = computers.iter_mut().all(|c| c.is_blocked_on_input());

        if all_are_blocked_on_input {
            panic!("all are blocked on input");
        }

        for index in 0..computers.len() {
            let computer = computers.get_mut(index).unwrap();

            if computer.is_blocked_on_input() {
                continue;
            }

            computer.step();

            if let Some(output) = computer.get_output() {
                let next_index = (index + 1) % sequence.len();
                let next = computers.get_mut(next_index);
                let next = next.unwrap();

                next.push_input(output);
            }
        }
    }

    let last_output = *computers
        .last()
        .expect("could not get last computer")
        .get_outputs()
        .last()
        .expect("could not get last output");

    last_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_sequence() {
        assert_eq!(
            run_sequence_part_one(
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
                vec![4, 3, 2, 1, 0]
            ),
            43210
        );

        assert_eq!(
            run_sequence_part_one(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
                vec![0, 1, 2, 3, 4]
            ),
            54321
        );

        assert_eq!(
            run_sequence_part_one(
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
                vec![1, 0, 4, 3, 2]
            ),
            65210
        );
    }

    #[test]
    fn test_sequence_part_two() {
        let program =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";

        assert_eq!(
            run_sequence_part_two(program, vec![9, 8, 7, 6, 5]),
            139629729
        );
    }
}
