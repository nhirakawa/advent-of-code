use crate::prelude::*;

const WINDOW_SIZE: usize = 25;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-9.txt");

    let numbers = parse_integers(input)?;

    // println!("{:?}", numbers);

    let part_one = part_one(&numbers);

    let part_two = part_one
        .clone()
        .and_then(|(solution, _)| part_two(&numbers, solution));

    Ok((part_one, part_two))
}

fn part_one(numbers: &Vec<u64>) -> Result<AnswerWithTiming, AdventOfCodeError> {
    let start = SystemTime::now();

    for window in numbers.windows(WINDOW_SIZE + 1) {
        let (window, target) = window.split_at(WINDOW_SIZE);
        let target = target[0];

        // println!("{} - {:?}", target, window);

        let has_sum = has_sum_in_window(window, target);

        if !has_sum {
            let elapsed = start.elapsed().unwrap();
            return Ok((target, elapsed));
        }
    }

    Err(AdventOfCodeError::NoAnswerFoundPartOne)
}

fn part_two(
    numbers: &Vec<u64>,
    part_one_solution: u64,
) -> Result<AnswerWithTiming, AdventOfCodeError> {
    let start = SystemTime::now();

    for i in 2..50 {
        for window in numbers.windows(i) {
            let sum: u64 = window.into_iter().sum();

            if sum == part_one_solution {
                let mut min = u64::MAX;
                let mut max = 0;

                for number in window {
                    min = min.min(*number);
                    max = max.max(*number);
                }

                let elapsed = start.elapsed().unwrap();

                return Ok((min + max, elapsed));
            }
        }
    }

    Err(AdventOfCodeError::NoAnswerFoundPartTwo)
}

fn has_sum_in_window(window: &[u64], target: u64) -> bool {
    for (outer_index, outer) in window.into_iter().enumerate() {
        for (inner_index, inner) in window.into_iter().enumerate() {
            if *outer as u64 + *inner as u64 == target as u64 && outer_index != inner_index {
                return true;
            }
        }
    }

    false
}

fn parse_integers(i: &str) -> Result<Vec<u64>, AdventOfCodeError> {
    let mut numbers = Vec::new();

    for line in i.split("\n") {
        if line == "" {
            continue;
        }

        let number = line
            .parse::<u64>()
            .map_err(AdventOfCodeError::CannotParseInteger)
            .expect(format!("cannot parse {} as int", line).as_str());

        numbers.push(number)
    }

    Ok(numbers)
}
