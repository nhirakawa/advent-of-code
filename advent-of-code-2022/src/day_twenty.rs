use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    ops::Mul,
};

use common::prelude::*;
use log::debug;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-20.txt");

    let part_one = part_one(input);
    let part_two = part_two(input);

    Ok((part_one, part_two))
}

fn part_one(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let numbers = parse(input);

    let mixed = mix(&numbers, 1, 1);

    let groove_numbers = groove_numbers(&mixed);

    let sum: isize = groove_numbers.into_iter().sum();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(sum, elapsed)
}

fn part_two(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let numbers = parse(input);

    let mixed = mix(&numbers, 811589153, 10);

    let groove_numbers = groove_numbers(&mixed);

    let sum: isize = groove_numbers.into_iter().sum();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(sum, elapsed)
}

fn mix(
    numbers: &NumberAndOriginalIndices,
    multiplier: isize,
    number_of_rounds: usize,
) -> Vec<isize> {
    let mixed = numbers
        .numbers
        .iter()
        .map(|number| number * multiplier)
        .collect::<VecDeque<NumberAndOriginalIndex>>();

    let mut mixed = NumberAndOriginalIndices { numbers: mixed };

    let numbers = mixed.clone();

    for _ in 0..number_of_rounds {
        for index in 0..numbers.len() {
            let number_to_mix = numbers.number_at(index);
            mixed = mix_once(&mixed, number_to_mix);
        }
    }

    mixed.values()
}

fn mix_once(
    sequence: &NumberAndOriginalIndices,
    number_to_mix: &NumberAndOriginalIndex,
) -> NumberAndOriginalIndices {
    debug!("mixing {:?}", sequence);

    let current_index = sequence.index_of(number_to_mix);

    debug!("current index {}", current_index);

    let mut updated = sequence.numbers.clone();

    updated.rotate_left(current_index);

    debug!("rotated left {}", current_index);
    debug!("{:?}", updated);

    let front = updated.pop_front();

    debug!("{:?}", front);

    let amount_to_rotate = number_to_mix.number.rem_euclid(sequence.len() as isize - 1);

    updated.rotate_left(amount_to_rotate as usize);

    debug!("rotated left {}", amount_to_rotate);

    updated.push_front(*number_to_mix);

    debug!("{:?}", updated);

    NumberAndOriginalIndices::new(updated)
}

fn groove_numbers(sequence: &[isize]) -> VecDeque<isize> {
    let index_of_zero = index_of(sequence, 0);

    println!("Index of 0 - {index_of_zero}");

    let mut groove_numbers = VecDeque::with_capacity(3);

    let mut index = index_of_zero;
    let mut iterations = 0;

    while groove_numbers.len() != 3 {
        if iterations > 0 && iterations % 1000 == 0 {
            groove_numbers.push_back(sequence[index]);
        }

        index = (index + 1) % sequence.len();
        iterations += 1;
    }

    groove_numbers
}

fn index_of(sequence: &[isize], number_to_find: isize) -> usize {
    sequence
        .iter()
        .enumerate()
        .find_map(|(index, number)| {
            if *number == number_to_find {
                Some(index)
            } else {
                None
            }
        })
        .unwrap()
}

/*
 * Holds a number with its original index from the input
 * Used so that we can identify which numbers in a mixed slice correspond to numbers in the original
 */
#[derive(PartialEq, Eq, Copy, Clone)]
struct NumberAndOriginalIndex {
    number: isize,
    original_index: usize,
}

impl Mul<isize> for &NumberAndOriginalIndex {
    type Output = NumberAndOriginalIndex;

    fn mul(self, rhs: isize) -> Self::Output {
        let number = self.number * rhs;
        let original_index = self.original_index;

        NumberAndOriginalIndex {
            number,
            original_index,
        }
    }
}

impl Debug for NumberAndOriginalIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.original_index, self.number)
    }
}

impl Display for NumberAndOriginalIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl From<(usize, isize)> for NumberAndOriginalIndex {
    fn from(raw: (usize, isize)) -> NumberAndOriginalIndex {
        let (original_index, number) = raw;
        NumberAndOriginalIndex {
            number,
            original_index,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct NumberAndOriginalIndices {
    numbers: VecDeque<NumberAndOriginalIndex>,
}

impl Debug for NumberAndOriginalIndices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ")?;

        let parts: Vec<String> = self
            .numbers
            .iter()
            .map(|number| format!("{number:?}"))
            .collect();

        write!(f, "{}", parts.join(", "))?;

        write!(f, " ]")
    }
}

impl NumberAndOriginalIndices {
    fn new(numbers: VecDeque<NumberAndOriginalIndex>) -> NumberAndOriginalIndices {
        let length = numbers.len();

        let numbers = numbers
            .into_iter()
            .cycle()
            .skip_while(|number| number.number != 0)
            .take(length)
            .collect();

        NumberAndOriginalIndices { numbers }
    }

    fn len(&self) -> usize {
        self.numbers.len()
    }

    fn values(&self) -> Vec<isize> {
        self.numbers.iter().map(|number| number.number).collect()
    }

    fn number_at(&self, index: usize) -> &NumberAndOriginalIndex {
        self.numbers.get(index).unwrap()
    }

    fn index_of(&self, number: &NumberAndOriginalIndex) -> usize {
        self.numbers
            .iter()
            .enumerate()
            .find_map(|(index, number_and_original_index)| {
                if number_and_original_index == number {
                    Some(index)
                } else {
                    None
                }
            })
            .unwrap()
    }
}

impl From<Vec<isize>> for NumberAndOriginalIndices {
    fn from(vector: Vec<isize>) -> NumberAndOriginalIndices {
        let numbers = vector
            .into_iter()
            .enumerate()
            .map(|tuple| tuple.into())
            .collect();

        NumberAndOriginalIndices { numbers }
    }
}

fn parse(i: &str) -> NumberAndOriginalIndices {
    finish(numbers)(i).unwrap().1.into()
}

fn numbers(i: &str) -> IResult<&str, Vec<isize>> {
    separated_list1(tag("\n"), number)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_number_1() {
        assert_eq!(
            mix_once(&vec![1, 2, -3, 3, -2, 0, 4].into(), &(0, 1).into()),
            NumberAndOriginalIndices::new(
                vec![
                    (1, 2).into(),
                    (0, 1).into(),
                    (2, -3).into(),
                    (3, 3).into(),
                    (4, -2).into(),
                    (5, 0).into(),
                    (6, 4).into()
                ]
                .into()
            )
        );
    }

    #[test]
    fn test_mix_number_2() {
        assert_eq!(
            mix_once(
                &NumberAndOriginalIndices::new(
                    vec![
                        (1, 2).into(),
                        (0, 1).into(),
                        (2, -3).into(),
                        (3, 3).into(),
                        (4, -2).into(),
                        (5, 0).into(),
                        (6, 4).into()
                    ]
                    .into()
                ),
                &(1, 2).into()
            ),
            NumberAndOriginalIndices::new(
                vec![
                    (0, 1).into(),
                    (2, -3).into(),
                    (1, 2).into(),
                    (3, 3).into(),
                    (4, -2).into(),
                    (5, 0).into(),
                    (6, 4).into()
                ]
                .into()
            )
        );
    }

    #[test]
    fn test_mix_number_neg_3() {
        assert_eq!(
            mix_once(
                &NumberAndOriginalIndices::new(
                    vec![
                        (0, 1).into(),
                        (2, -3).into(),
                        (1, 2).into(),
                        (3, 3).into(),
                        (4, -2).into(),
                        (5, 0).into(),
                        (6, 4).into()
                    ]
                    .into()
                ),
                &(2, -3).into()
            ),
            NumberAndOriginalIndices::new(
                vec![
                    (0, 1).into(),
                    (1, 2).into(),
                    (3, 3).into(),
                    (4, -2).into(),
                    (2, -3).into(),
                    (5, 0).into(),
                    (6, 4).into()
                ]
                .into()
            )
        );
    }

    #[test]
    fn test_mix_number_3() {
        assert_eq!(
            mix_once(
                &NumberAndOriginalIndices::new(
                    vec![
                        (0, 1).into(),
                        (1, 2).into(),
                        (3, 3).into(),
                        (4, -2).into(),
                        (2, -3).into(),
                        (5, 0).into(),
                        (6, 4).into()
                    ]
                    .into()
                ),
                &(3, 3).into()
            ),
            NumberAndOriginalIndices::new(
                vec![
                    (0, 1).into(),
                    (1, 2).into(),
                    (4, -2).into(),
                    (2, -3).into(),
                    (5, 0).into(),
                    (3, 3).into(),
                    (6, 4).into()
                ]
                .into()
            )
        );
    }

    #[test]
    fn test_mix_number_0() {
        assert_eq!(
            mix_once(
                &NumberAndOriginalIndices::new(
                    vec![
                        (0, 1).into(),
                        (1, 2).into(),
                        (3, -3).into(),
                        (5, 0).into(),
                        (3, 3).into(),
                        (6, 4).into(),
                        (3, -2).into()
                    ]
                    .into()
                ),
                &(5, 0).into()
            ),
            NumberAndOriginalIndices::new(
                vec![
                    (0, 1).into(),
                    (1, 2).into(),
                    (3, -3).into(),
                    (5, 0).into(),
                    (3, 3).into(),
                    (6, 4).into(),
                    (3, -2).into()
                ]
                .into()
            )
        );
    }

    #[test]
    fn test_mix_number_4() {
        assert_eq!(
            mix_once(
                &NumberAndOriginalIndices::new(
                    vec![
                        (0, 1).into(),
                        (1, 2).into(),
                        (2, -3).into(),
                        (5, 0).into(),
                        (3, 3).into(),
                        (6, 4).into(),
                        (4, -2).into()
                    ]
                    .into()
                ),
                &(6, 4).into()
            ),
            NumberAndOriginalIndices::new(
                vec![
                    (0, 1).into(),
                    (1, 2).into(),
                    (2, -3).into(),
                    (6, 4).into(),
                    (5, 0).into(),
                    (3, 3).into(),
                    (4, -2).into()
                ]
                .into()
            )
        );
    }

    #[test]
    fn test_mix_number_neg_2() {
        assert_eq!(
            mix_once(
                &NumberAndOriginalIndices::new(
                    vec![
                        (0, 1).into(),
                        (1, 2).into(),
                        (4, -2).into(),
                        (2, -3).into(),
                        (5, 0).into(),
                        (3, 3).into(),
                        (6, 4).into()
                    ]
                    .into()
                ),
                &(4, -2).into()
            ),
            NumberAndOriginalIndices::new(
                vec![
                    (0, 1).into(),
                    (1, 2).into(),
                    (2, -3).into(),
                    (5, 0).into(),
                    (3, 3).into(),
                    (6, 4).into(),
                    (4, -2).into()
                ]
                .into()
            )
        )
    }

    #[test]
    fn test_mix() {
        assert_eq!(
            mix(&vec![1, 2, -3, 3, -2, 0, 4].into()),
            vec![0, 3, -2, 1, 2, -3, 4]
        );
    }

    #[test]
    fn test_groove_numbers() {
        assert_eq!(groove_numbers(&vec![1, 2, -3, 4, 0, 3, -2]), vec![4, -3, 2]);
    }

    #[test]
    fn test_rotate() {
        let mut rotated = VecDeque::new();

        rotated.push_back(1);
        rotated.push_back(2);
        rotated.push_back(3);

        rotated.rotate_left(4_usize.rem_euclid(rotated.len()));

        println!("{:?}", rotated);
    }

    #[test]
    fn test_mix_once() {
        let number_and_original_indices = NumberAndOriginalIndices::new(
            vec![
                (0, 1).into(),
                (1, 2).into(),
                (2, -3).into(),
                (3, 3).into(),
                (4, -2).into(),
                (5, 0).into(),
                (6, 4).into(),
            ]
            .into(),
        );

        let result = mix_once(&number_and_original_indices, &(0, 1).into());

        let expected: VecDeque<NumberAndOriginalIndex> = vec![
            (5, 0).into(),
            (6, 4).into(),
            (1, 2).into(),
            (0, 1).into(),
            (2, -3).into(),
            (3, 3).into(),
            (4, -2).into(),
        ]
        .into();

        assert_eq!(result.numbers, expected);
    }
}
