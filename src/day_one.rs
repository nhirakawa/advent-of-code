use crate::answer::{AdventOfCodeError, AdventOfCodeResult, AnswerWithTiming};
use std::time::SystemTime;

pub fn run() -> AdventOfCodeResult {
    let expenses = read_expenses()?;
    let part_one_answer = part_one(&expenses);
    let part_two_answer = part_two(&expenses);

    Ok((part_one_answer, part_two_answer))
}

fn part_one(expenses: &Vec<u32>) -> Result<AnswerWithTiming, AdventOfCodeError> {
    let now = SystemTime::now();

    for (outer_index, outer) in expenses.iter().enumerate() {
        for (inner_index, inner) in expenses.iter().enumerate() {
            if inner_index != outer_index && outer + inner == 2020 {
                let elapsed = now.elapsed().unwrap();
                return Ok(((outer * inner) as u64, elapsed));
            }
        }
    }

    Err(AdventOfCodeError::NoAnswerFoundPartOne)
}

fn part_two(expenses: &Vec<u32>) -> Result<AnswerWithTiming, AdventOfCodeError> {
    let start = SystemTime::now();
    for (first_index, first) in expenses.iter().enumerate() {
        for (second_index, second) in expenses.iter().enumerate() {
            for (third_index, third) in expenses.iter().enumerate() {
                if first_index != second_index
                    && second_index != third_index
                    && first + second + third == 2020
                {
                    let elapsed = start.elapsed().unwrap();
                    return Ok(((first * second * third) as u64, elapsed));
                }
            }
        }
    }

    Err(AdventOfCodeError::NoAnswerFoundPartTwo)
}

fn read_expenses() -> Result<Vec<u32>, AdventOfCodeError> {
    let input = include_str!("../input/day-1.txt");

    let expenses = input
        .split("\n")
        .filter(|s| s != &"")
        .map(|s| {
            s.parse::<u32>()
                .map_err(AdventOfCodeError::CannotParseInteger)
        })
        .collect::<Result<Vec<u32>, AdventOfCodeError>>();

    expenses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answers() {
        let (part_one, part_two) = run().unwrap();
        let part_one = part_one.unwrap();
        let part_two = part_two.unwrap();

        assert_eq!(part_one.0, 1020099);
        assert_eq!(part_two.0, 49214880);
    }
}
