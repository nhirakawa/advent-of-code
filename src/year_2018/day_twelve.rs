use std::{
    collections::{HashSet, hash_set},
    ops::RangeInclusive,
};

use anyhow::anyhow;
use itertools::Itertools;

use crate::common::{
    base::{Day, Year},
    debug,
};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (initial_state, rules) = parse::parse(input)?;

    let sum = std::iter::successors(Some(initial_state), |current| {
        Some(next_state(current, &rules))
    })
    .nth(20)
    .unwrap()
    .sum();

    Ok(sum)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (initial_state, rules) = parse::parse(input)?;

    let pots = std::iter::successors(Some(initial_state), |current| {
        Some(next_state(current, &rules))
    })
    .take(1_000)
    .enumerate()
    .collect_vec();

    let csv = "index,sum\n".to_owned()
        + &pots
            .iter()
            .map(|(index, pots)| format!("{index},{}", pots.sum()))
            .join("\n");

    debug::write(Year::Year2018, Day::Day12, "sums.csv", csv)?;

    // After manual inspection, we know there's a point where every additional iteration adds a constant amount to the sum
    // Find that point and determine the sum

    let added_in_each_iteration = pots
        .iter()
        .map(|(_, pots)| pots)
        .tuple_windows()
        .filter_map(|(first, second, third, fourth, fifth)| {
            let first_first_order_difference = second.sum() - first.sum();
            let second_first_order_difference = third.sum() - second.sum();
            let third_first_order_difference = fourth.sum() - third.sum();
            let fourth_first_order_difference = fifth.sum() - fourth.sum();

            if first_first_order_difference == second_first_order_difference
                && second_first_order_difference == third_first_order_difference
                && third_first_order_difference == fourth_first_order_difference
            {
                Some(first_first_order_difference)
            } else {
                None
            }
        })
        .next()
        .ok_or(anyhow!("No cycle found"))?;

    let sum_after_1000 = pots
        .last()
        .map(|(_, pots)| pots.sum())
        .ok_or(anyhow!("Could not get last element"))?;

    // We calculated the sum over [0, 1000)
    // The +1 ensures we calculate the sum over [1000, 50_000_000_000]
    let sum_to_50_000_000_000 = (50_000_000_000 - 1000 + 1) * added_in_each_iteration;

    Ok(sum_after_1000 + sum_to_50_000_000_000)
}

fn next_state(state: &Pots, rules: &[Rule]) -> Pots {
    let mut next_state = HashSet::new();

    for index in state.get_bounds() {
        let pots = state.get_at(index);

        for rule in rules {
            if rule.matches(&pots) && rule.output == Pot::Plant {
                next_state.insert(index);
            }
        }
    }

    Pots(next_state)
}

#[derive(Debug, PartialEq, Eq)]
struct Pots(HashSet<isize>);

impl Pots {
    fn get_at(&self, center: isize) -> [Pot; 5] {
        [
            self.get_pot(center - 2),
            self.get_pot(center - 1),
            self.get_pot(center),
            self.get_pot(center + 1),
            self.get_pot(center + 2),
        ]
    }

    fn get_pot(&self, index: isize) -> Pot {
        if self.0.contains(&index) {
            Pot::Plant
        } else {
            Pot::Empty
        }
    }

    fn get_bounds(&self) -> RangeInclusive<isize> {
        let min_index = self.0.iter().min().copied().unwrap();
        let max_index = self.0.iter().max().copied().unwrap();
        (min_index - 4)..=(max_index + 4)
    }

    fn sum(&self) -> isize {
        self.0.iter().sum()
    }
}

impl<'a> IntoIterator for &'a Pots {
    type Item = &'a isize;

    type IntoIter = hash_set::Iter<'a, isize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Pot {
    Empty,
    Plant,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Rule {
    input: [Pot; 5],
    output: Pot,
}

impl Rule {
    fn matches(&self, input: &[Pot; 5]) -> bool {
        &self.input == input
    }
}

impl From<([Pot; 5], Pot)> for Rule {
    fn from((input, output): ([Pot; 5], Pot)) -> Self {
        Rule { input, output }
    }
}

mod parse {
    use std::collections::HashSet;

    use anyhow::Result;
    use nom::{
        Parser,
        branch::alt,
        bytes::complete::tag,
        combinator::{into, map, value},
        error::Error,
        multi::{many1, separated_list1},
        sequence::{preceded, separated_pair},
    };

    use crate::{
        common::parse::finish,
        year_2018::day_twelve::{Pot, Pots, Rule},
    };

    pub fn parse(input: &str) -> Result<(Pots, Vec<Rule>)> {
        finish(
            separated_pair(map(initial_state(), Pots), tag("\n\n"), rules()),
            input,
        )
    }

    fn initial_state<'a>()
    -> impl Parser<&'a str, Output = HashSet<isize>, Error = Error<&'a str>> + 'a {
        preceded(
            tag("initial state: "),
            map(many1(pot()), |pots| {
                pots.into_iter()
                    .enumerate()
                    .filter_map(|(index, pot)| match pot {
                        Pot::Empty => None,
                        Pot::Plant => Some(index as isize),
                    })
                    .collect()
            }),
        )
    }

    fn pot<'a>() -> impl Parser<&'a str, Output = Pot, Error = Error<&'a str>> + 'a {
        alt((empty(), plant()))
    }

    fn empty<'a>() -> impl Parser<&'a str, Output = Pot, Error = Error<&'a str>> + 'a {
        value(Pot::Empty, tag("."))
    }

    fn plant<'a>() -> impl Parser<&'a str, Output = Pot, Error = Error<&'a str>> + 'a {
        value(Pot::Plant, tag("#"))
    }

    fn rules<'a>() -> impl Parser<&'a str, Output = Vec<Rule>, Error = Error<&'a str>> + 'a {
        separated_list1(tag("\n"), rule())
    }

    fn rule<'a>() -> impl Parser<&'a str, Output = Rule, Error = Error<&'a str>> + 'a {
        into(separated_pair(input(), tag(" => "), pot()))
    }

    fn input<'a>() -> impl Parser<&'a str, Output = [Pot; 5], Error = Error<&'a str>> + 'a {
        map(
            (pot(), pot(), pot(), pot(), pot()),
            |(first, second, third, fourth, fifth)| [first, second, third, fourth, fifth],
        )
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_parses_rule() {
            assert_eq!(
                rule().parse("#.##. => .").unwrap().1,
                (
                    [Pot::Plant, Pot::Empty, Pot::Plant, Pot::Plant, Pot::Empty],
                    Pot::Empty
                )
                    .into()
            );
        }

        #[test]
        fn it_parses_initial_state() {
            assert_eq!(
                initial_state().parse("initial state: ##.###").unwrap().1,
                [0, 1, 3, 4, 5,].into()
            )
        }
    }
}
