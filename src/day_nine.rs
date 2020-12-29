use crate::prelude::*;

const WINDOW_SIZE: usize = 25;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-9.txt");

    let numbers = parse_integers(input)?;

    let part_one = part_one(&numbers);

    let part_two = part_two(&numbers);

    Ok((part_one, part_two))
}

fn part_one(numbers: &Vec<u64>) -> PartAnswer {
    let start = SystemTime::now();

    let target = find_target_without_sum_in_window(numbers);

    let elapsed = start.elapsed().unwrap();

    (target, elapsed)
}

fn part_two(numbers: &Vec<u64>) -> PartAnswer {
    let start = SystemTime::now();

    let part_one_solution = find_target_without_sum_in_window(numbers);

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

                return (min + max, elapsed);
            }
        }
    }

    (0, start.elapsed().unwrap())
}

fn find_target_without_sum_in_window(numbers: &Vec<u64>) -> u64 {
    for window in numbers.windows(WINDOW_SIZE + 1) {
        let (window, target) = window.split_at(WINDOW_SIZE);
        let target = target[0];

        let has_sum = has_sum_in_window(window, target);

        if !has_sum {
            return target;
        }
    }

    0
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answers() {
        let (part_one, part_two) = run().unwrap();
        let (part_one, _) = part_one;
        let (part_two, _) = part_two;

        assert_eq!(part_one, 1639024365);
        assert_eq!(part_two, 219202240);
    }
}
