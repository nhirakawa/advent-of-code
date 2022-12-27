use std::fmt::{Debug, Display};

use common::prelude::*;
use log::debug;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-20.txt");

    let part_one = part_one(input);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let numbers = parse(input);

    let mixed = mix(&numbers);

    let groove_numbers = groove_numbers(&mixed);

    let sum: isize = groove_numbers.into_iter().sum();

    let elapsed = start.elapsed().unwrap();

    // 5346 is too low
    PartAnswer::new(sum, elapsed)
}

fn part_two() -> PartAnswer {
    let start = SystemTime::now();
    let _elapsed = start.elapsed().unwrap();
    PartAnswer::default()
}

fn mix(numbers: &NumberAndOriginalIndices) -> Vec<isize> {
    let mut mixed = numbers.clone();

    for index in 0..numbers.len() {
        let number_to_mix = numbers.number_at(index);
        mixed = mix_number(&mixed, number_to_mix);
    }

    mixed.values()
}

fn mix_number(
    sequence: &NumberAndOriginalIndices,
    number_to_mix: &NumberAndOriginalIndex,
) -> NumberAndOriginalIndices {
    let mut updated = Vec::with_capacity(sequence.len());

    let index_of_number_to_mix = sequence.index_of(number_to_mix);

    debug!(
        "Mixing {:?} with index {} in {:?}",
        number_to_mix, index_of_number_to_mix, sequence
    );

    let index_of_number_to_mix = index_of_number_to_mix as isize;

    let new_index = index_of_number_to_mix + number_to_mix.number;

    let new_index = if new_index > 0 {
        new_index as usize % sequence.len()
    } else if new_index < 0 {
        (new_index + (sequence.len() - 1) as isize) as usize % sequence.len()
    } else {
        sequence.len() - 1
    };

    // is this even right?
    let new_index = if new_index == 0 { 1 } else { new_index };

    if new_index as isize == index_of_number_to_mix {
        return sequence.clone();
    }

    debug!("  New index {}", new_index);

    let mut index = 0;

    for number_and_original_index in sequence.numbers.iter() {
        if number_and_original_index == number_to_mix {
            debug!("  skipping {:?} from input", number_and_original_index);
            continue;
        }

        debug!("  inserting {:?}", number_and_original_index);
        updated.push(*number_and_original_index);
        index += 1;

        if index == new_index {
            debug!("  inserting {:?} at index {}", number_to_mix, index);
            updated.push(*number_to_mix);
            index += 1;
        }
    }

    NumberAndOriginalIndices::new(updated)
}

fn groove_numbers(sequence: &[isize]) -> Vec<isize> {
    let index_of_zero = index_of(sequence, 0);

    println!("Index of 0 - {index_of_zero}");

    let mut groove_numbers = Vec::with_capacity(3);

    let mut index = index_of_zero;
    let mut iterations = 0;

    while groove_numbers.len() != 3 {
        if iterations > 0 && iterations % 1000 == 0 {
            groove_numbers.push(sequence[index]);
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct NumberAndOriginalIndices {
    numbers: Vec<NumberAndOriginalIndex>,
}

impl NumberAndOriginalIndices {
    fn new(numbers: Vec<NumberAndOriginalIndex>) -> NumberAndOriginalIndices {
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
            mix_number(&vec![1, 2, -3, 3, -2, 0, 4].into(), &(0, 1).into()),
            NumberAndOriginalIndices::new(vec![
                (1, 2).into(),
                (0, 1).into(),
                (2, -3).into(),
                (3, 3).into(),
                (4, -2).into(),
                (5, 0).into(),
                (6, 4).into()
            ])
        );
    }

    #[test]
    fn test_mix_number_2() {
        assert_eq!(
            mix_number(
                &NumberAndOriginalIndices::new(vec![
                    (1, 2).into(),
                    (0, 1).into(),
                    (2, -3).into(),
                    (3, 3).into(),
                    (4, -2).into(),
                    (5, 0).into(),
                    (6, 4).into()
                ]),
                &(1, 2).into()
            ),
            NumberAndOriginalIndices::new(vec![
                (0, 1).into(),
                (2, -3).into(),
                (1, 2).into(),
                (3, 3).into(),
                (4, -2).into(),
                (5, 0).into(),
                (6, 4).into()
            ])
        );
    }

    #[test]
    fn test_mix_number_neg_3() {
        assert_eq!(
            mix_number(
                &NumberAndOriginalIndices::new(vec![
                    (0, 1).into(),
                    (2, -3).into(),
                    (1, 2).into(),
                    (3, 3).into(),
                    (4, -2).into(),
                    (5, 0).into(),
                    (6, 4).into()
                ]),
                &(2, -3).into()
            ),
            NumberAndOriginalIndices::new(vec![
                (0, 1).into(),
                (1, 2).into(),
                (3, 3).into(),
                (4, -2).into(),
                (2, -3).into(),
                (5, 0).into(),
                (6, 4).into()
            ])
        );
    }

    #[test]
    fn test_mix_number_3() {
        assert_eq!(
            mix_number(
                &NumberAndOriginalIndices::new(vec![
                    (0, 1).into(),
                    (1, 2).into(),
                    (3, 3).into(),
                    (4, -2).into(),
                    (2, -3).into(),
                    (5, 0).into(),
                    (6, 4).into()
                ]),
                &(3, 3).into()
            ),
            NumberAndOriginalIndices::new(vec![
                (0, 1).into(),
                (1, 2).into(),
                (4, -2).into(),
                (2, -3).into(),
                (5, 0).into(),
                (3, 3).into(),
                (6, 4).into()
            ])
        );
    }

    #[test]
    fn test_mix_number_0() {
        // move 0
        assert_eq!(
            mix_number(
                &NumberAndOriginalIndices::new(vec![
                    (0, 1).into(),
                    (1, 2).into(),
                    (3, -3).into(),
                    (5, 0).into(),
                    (3, 3).into(),
                    (6, 4).into(),
                    (3, -2).into()
                ]),
                &(5, 0).into()
            ),
            NumberAndOriginalIndices::new(vec![
                (0, 1).into(),
                (1, 2).into(),
                (3, -3).into(),
                (5, 0).into(),
                (3, 3).into(),
                (6, 4).into(),
                (3, -2).into()
            ])
        );
    }

    #[test]
    fn test_mix_number_4() {
        assert_eq!(
            mix_number(
                &NumberAndOriginalIndices::new(vec![
                    (0, 1).into(),
                    (1, 2).into(),
                    (2, -3).into(),
                    (5, 0).into(),
                    (3, 3).into(),
                    (6, 4).into(),
                    (4, -2).into()
                ]),
                &(6, 4).into()
            ),
            NumberAndOriginalIndices::new(vec![
                (0, 1).into(),
                (1, 2).into(),
                (2, -3).into(),
                (6, 4).into(),
                (5, 0).into(),
                (3, 3).into(),
                (4, -2).into()
            ])
        );
    }

    #[test]
    fn test_mix_number_neg_2() {
        assert_eq!(
            mix_number(
                &NumberAndOriginalIndices::new(vec![
                    (0, 1).into(),
                    (1, 2).into(),
                    (4, -2).into(),
                    (2, -3).into(),
                    (5, 0).into(),
                    (3, 3).into(),
                    (6, 4).into()
                ]),
                &(4, -2).into()
            ),
            NumberAndOriginalIndices::new(vec![
                (0, 1).into(),
                (1, 2).into(),
                (2, -3).into(),
                (5, 0).into(),
                (3, 3).into(),
                (6, 4).into(),
                (4, -2).into()
            ])
        )
    }

    #[test]
    fn test_mix_number_custom() {
        // my own test case, not from example
        assert_eq!(
            mix_number(
                &NumberAndOriginalIndices::new(vec![
                    (0, 20).into(),
                    (1, 1).into(),
                    (2, 2).into(),
                    (3, 3).into(),
                    (4, 4).into(),
                    (5, 5).into(),
                    (6, 6).into(),
                    (7, 7).into(),
                    (8, 8).into(),
                    (9, 9).into()
                ]),
                &(0, 20).into()
            ),
            NumberAndOriginalIndices::new(vec![
                (0, 20).into(),
                (1, 1).into(),
                (2, 2).into(),
                (3, 3).into(),
                (4, 4).into(),
                (5, 5).into(),
                (6, 6).into(),
                (7, 7).into(),
                (8, 8).into(),
                (9, 9).into()
            ])
        );
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
}
