use crate::error::AdventOfCodeError;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() -> Result<(u32, u32), AdventOfCodeError> {
    let expenses = read_expenses()?;
    let part_one_answer = part_one(&expenses)?;
    let part_two_answer = part_two(&expenses)?;

    Ok((part_one_answer, part_two_answer))
}

fn part_one(expenses: &Vec<u32>) -> Result<u32, AdventOfCodeError> {
    for (outer_index, outer) in expenses.iter().enumerate() {
        for (inner_index, inner) in expenses.iter().enumerate() {
            if inner_index != outer_index && outer + inner == 2020 {
                return Ok(outer * inner);
            }
        }
    }

    Err(AdventOfCodeError::NoAnswerFoundPartOne)
}

fn part_two(expenses: &Vec<u32>) -> Result<u32, AdventOfCodeError> {
    for (first_index, first) in expenses.iter().enumerate() {
        for (second_index, second) in expenses.iter().enumerate() {
            for (third_index, third) in expenses.iter().enumerate() {
                if first_index != second_index
                    && second_index != third_index
                    && first + second + third == 2020
                {
                    return Ok(first * second * third);
                }
            }
        }
    }

    Err(AdventOfCodeError::NoAnswerFoundPartTwo)
}

fn read_expenses() -> Result<Vec<u32>, AdventOfCodeError> {
    let file = File::open("input/day-1-part-1.txt").map_err(AdventOfCodeError::from)?;

    let reader = BufReader::new(file);

    let expenses = reader
        .lines()
        .map(|result| result.map_err(AdventOfCodeError::from))
        .map(|result| {
            result.and_then(|s| {
                s.parse::<u32>()
                    .map_err(AdventOfCodeError::CannotParseInteger)
            })
        })
        .collect::<Result<Vec<u32>, AdventOfCodeError>>();

    expenses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let nums = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part_one(&nums), Ok(514579));
    }

    #[test]
    fn test_part_two() {
        let nums = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part_two(&nums), Ok(241861950));
    }
}
