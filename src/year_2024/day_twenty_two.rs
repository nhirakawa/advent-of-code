use std::{
    collections::{HashMap, HashSet},
    convert::{TryFrom, TryInto},
    iter::successors,
    ops::{BitXor, Rem, Sub},
};

use anyhow::{anyhow, bail};
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let secret_numbers = parse(input)?;

    let mut sum = 0;

    for secret_number in secret_numbers {
        sum += secret_number_sequence(secret_number).last().unwrap();
    }

    Ok(sum)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let secret_numbers = parse(input)?;

    let mut sequence_sums: HashMap<(i8, i8, i8, i8), usize> = HashMap::new();

    for secret_number in &secret_numbers {
        let price_sequence = price_sequence(*secret_number).collect_vec();
        let price_difference_sequence = price_difference_sequence(*secret_number).collect_vec();

        let mut seen_sequences = HashSet::new();

        for (idx, (s1, s2, s3, s4)) in price_difference_sequence.iter().tuple_windows().enumerate()
        {
            let price_idx = idx + 4;
            let sequence = (*s1, *s2, *s3, *s4);

            if seen_sequences.insert(sequence) {
                let sequence_sum = sequence_sums.entry(sequence).or_insert(0);

                let price = price_sequence.get(price_idx).copied().unwrap();
                let price: i8 = price.into();
                *sequence_sum += price as usize;
            }
        }
    }

    sequence_sums
        .values()
        .max()
        .copied()
        .ok_or(anyhow!("No max sum found"))
}

fn secret_number_sequence(start: u128) -> impl Iterator<Item = u128> {
    successors(Some(start), |secret_number| {
        let to_mix = *secret_number * 64;

        let secret_number = prune(mix(*secret_number, to_mix));

        let to_mix = secret_number / 32;

        let secret_number = prune(mix(secret_number, to_mix));

        let to_mix = secret_number * 2048;

        Some(prune(mix(secret_number, to_mix)))
    })
    .take(2001)
}

// TODO make this more idiomatic
fn price_sequence(start: u128) -> impl Iterator<Item = Digit> {
    secret_number_sequence(start)
        .skip(1)
        .map(|u| ones_digit(u).unwrap())
        .take(2000)
}

fn price_difference_sequence(start: u128) -> impl Iterator<Item = i8> {
    price_sequence(start)
        .tuple_windows()
        .map(|(first, second)| second - first)
}

fn mix(secret_number: u128, value: u128) -> u128 {
    secret_number.bitxor(value)
}

fn prune(secret_number: u128) -> u128 {
    secret_number.rem(16777216)
}

fn ones_digit(value: u128) -> anyhow::Result<Digit> {
    value.rem(10).try_into()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<Digit> for i8 {
    fn from(digit: Digit) -> i8 {
        match digit {
            Digit::Zero => 0,
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }
}

impl From<Digit> for i32 {
    fn from(digit: Digit) -> i32 {
        i8::from(digit) as i32
    }
}

impl Sub for Digit {
    type Output = i8;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs: i8 = self.into();
        let rhs: i8 = rhs.into();
        lhs - rhs
    }
}

impl TryFrom<u128> for Digit {
    type Error = anyhow::Error;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            9 => Ok(Self::Nine),
            _ => bail!("Invalid digit: {}", value),
        }
    }
}

fn parse(input: &str) -> anyhow::Result<Vec<u128>> {
    input
        .lines()
        .map(|line| {
            line.parse::<u128>()
                .map_err(|_| anyhow::anyhow!("Failed to parse line: {}", line))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 37), 15);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_next_ten_secret_numbers() {
        assert_eq!(
            secret_number_sequence(123).skip(1).take(10).collect_vec(),
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254
            ]
        );
    }

    #[test]
    fn test_nth_secret_number() {
        assert_eq!(secret_number_sequence(1).last().unwrap(), 8685429);
        assert_eq!(secret_number_sequence(10).last().unwrap(), 4700978);
        assert_eq!(secret_number_sequence(100).last().unwrap(), 15273692);
        assert_eq!(secret_number_sequence(2024).last().unwrap(), 8667524);
    }

    #[test]
    fn test_price_sequence() {
        let prices = price_sequence(123).take(10).collect_vec();

        assert_eq!(
            prices,
            vec![
                Digit::Three,
                Digit::Zero,
                Digit::Six,
                Digit::Five,
                Digit::Four,
                Digit::Four,
                Digit::Six,
                Digit::Four,
                Digit::Four,
                Digit::Two
            ]
        )
    }

    #[test]
    fn test_price_difference_sequence() {
        let price_differences = price_difference_sequence(123).take(9).collect_vec();

        assert_eq!(price_differences, vec![-3, 6, -1, -1, 0, 2, -2, 0, -2]);
    }
}
