use common::prelude::*;
use itertools::Itertools;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-16.txt");
    let ints = parse_input(input);

    println!("input is {} characters long", ints.len());

    let part_one = part_one(&ints);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(ints: &[i8]) -> PartAnswer {
    let start = SystemTime::now();
    let output = iterated_fft(ints, 100);
    let output = output[..8].into_iter().map(|d| d.to_string()).join("");
    PartAnswer::new(output, start.elapsed().unwrap())
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn iterated_fft(ints: &[i8], times: usize) -> Vec<i8> {
    let mut inner_ints = ints.into_iter().copied().collect_vec();

    for _ in 0..times {
        inner_ints = run_fft(&inner_ints);
    }

    inner_ints
}

fn run_fft(ints: &[i8]) -> Vec<i8> {
    let mut output = Vec::with_capacity(ints.len());

    for i in 0..ints.len() {
        let digit = fft(ints, generate_pattern(i + 1, ints.len()));
        output.push(digit);
    }

    output
}

fn fft(ints: &[i8], pattern: Vec<i8>) -> i8 {
    (ints
        .into_iter()
        .zip(pattern.into_iter().cycle())
        .map(|(first, second)| (first * second) as i128)
        .sum::<i128>()
        % 10)
        .abs() as i8
}

fn generate_pattern(index: usize, count: usize) -> Vec<i8> {
    [0_i8, 1_i8, 0_i8, -1_i8]
        .iter()
        .map(|digit| repeat_digits(*digit, index))
        .flat_map(|a| a.into_iter())
        .cycle()
        .skip(1)
        .take(count)
        .collect()
}

fn repeat_digits(digit: i8, times: usize) -> Vec<i8> {
    [digit].iter().copied().cycle().take(times).collect_vec()
}

fn parse_input(s: &str) -> Vec<i8> {
    s.trim()
        .split("")
        .filter(|c| c.len() > 0)
        .map(|c| {
            c.parse::<i8>()
                .unwrap_or_else(|_| panic!("could not parse {}", c))
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_fft() {
        assert_eq!(
            run_fft(&[1, 2, 3, 4, 5, 6, 7, 8]),
            vec![4, 8, 2, 2, 6, 1, 5, 8],
        );
    }

    #[test]
    fn test_iterated_fft_small() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(iterated_fft(&input, 1), vec![4, 8, 2, 2, 6, 1, 5, 8]);
        assert_eq!(iterated_fft(&input, 2), vec![3, 4, 0, 4, 0, 4, 3, 8]);
        assert_eq!(iterated_fft(&input, 3), vec![0, 3, 4, 1, 5, 5, 1, 8]);
        assert_eq!(iterated_fft(&input, 4), vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_iterated_fft_large() {
        let output = iterated_fft(
            &[
                8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8, 6, 4,
                5, 5, 9, 5,
            ],
            100,
        );
        assert_eq!(&output[..8], &[2, 4, 1, 7, 6, 1, 7, 6]);

        let output = iterated_fft(
            &[
                1, 9, 6, 1, 7, 8, 0, 4, 2, 0, 7, 2, 0, 2, 2, 0, 9, 1, 4, 4, 9, 1, 6, 0, 4, 4, 1, 8,
                9, 9, 1, 7,
            ],
            100,
        );
        assert_eq!(&output[..8], &[7, 3, 7, 4, 5, 4, 1, 8]);

        let output = iterated_fft(
            &[
                6, 9, 3, 1, 7, 1, 6, 3, 4, 9, 2, 9, 4, 8, 6, 0, 6, 3, 3, 5, 9, 9, 5, 9, 2, 4, 3, 1,
                9, 8, 7, 3,
            ],
            100,
        );
        assert_eq!(&output[..8], &[5, 2, 4, 3, 2, 1, 3, 3]);
    }

    #[test]
    fn test_fft() {
        let input_signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(fft(&input_signal, generate_pattern(1, 8)), 4);
        assert_eq!(fft(&input_signal, generate_pattern(2, 8)), 8);
        assert_eq!(fft(&input_signal, generate_pattern(3, 8)), 2);
        assert_eq!(fft(&input_signal, generate_pattern(4, 8)), 2);
        assert_eq!(fft(&input_signal, generate_pattern(5, 8)), 6);
        assert_eq!(fft(&input_signal, generate_pattern(6, 8)), 1);
        assert_eq!(fft(&input_signal, generate_pattern(7, 8)), 5);
        assert_eq!(fft(&input_signal, generate_pattern(8, 8)), 8);
    }

    #[test]
    fn test_repeat_digits() {
        assert_eq!(repeat_digits(1, 3), vec![1, 1, 1]);
        assert_eq!(repeat_digits(4, 4), vec![4, 4, 4, 4]);
    }

    #[test]
    fn test_multiply_pattern() {
        assert_eq!(generate_pattern(1, 8), vec![1, 0, -1, 0, 1, 0, -1, 0]);
        assert_eq!(generate_pattern(2, 8), vec![0, 1, 1, 0, 0, -1, -1, 0]);
        assert_eq!(generate_pattern(3, 8), vec![0, 0, 1, 1, 1, 0, 0, 0]);
        assert_eq!(generate_pattern(4, 8), vec![0, 0, 0, 1, 1, 1, 1, 0]);
        assert_eq!(generate_pattern(5, 8), vec![0, 0, 0, 0, 1, 1, 1, 1]);
        assert_eq!(generate_pattern(6, 8), vec![0, 0, 0, 0, 0, 1, 1, 1]);
        assert_eq!(generate_pattern(7, 8), vec![0, 0, 0, 0, 0, 0, 1, 1]);
        assert_eq!(generate_pattern(8, 8), vec![0, 0, 0, 0, 0, 0, 0, 1]);
    }
}
