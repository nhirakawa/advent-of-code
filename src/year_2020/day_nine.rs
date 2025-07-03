use anyhow::bail;

const WINDOW_SIZE: usize = 25;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let numbers = parse_integers(input)?;
    Ok(find_target_without_sum_in_window(&numbers))
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let numbers = parse_integers(input)?;

    let part_one_solution = find_target_without_sum_in_window(&numbers);

    for i in 2..50 {
        for window in numbers.windows(i) {
            let sum: u64 = window.iter().sum();

            if sum == part_one_solution {
                let mut min = u64::MAX;
                let mut max = 0;

                for number in window {
                    min = min.min(*number);
                    max = max.max(*number);
                }

                return Ok(min + max);
            }
        }
    }

    bail!("No solution found")
}

fn find_target_without_sum_in_window(numbers: &[u64]) -> u64 {
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
    for (outer_index, outer) in window.iter().enumerate() {
        for (inner_index, inner) in window.iter().enumerate() {
            if *outer + *inner == target && outer_index != inner_index {
                return true;
            }
        }
    }

    false
}

fn parse_integers(i: &str) -> anyhow::Result<Vec<u64>> {
    let mut numbers = Vec::new();

    for line in i.split('\n') {
        if line.is_empty() {
            continue;
        }

        let number = line
            .parse::<u64>()
            .map_err(anyhow::Error::from)
            .unwrap_or_else(|_| panic!("cannot parse {} as int", line));

        numbers.push(number)
    }

    Ok(numbers)
}
