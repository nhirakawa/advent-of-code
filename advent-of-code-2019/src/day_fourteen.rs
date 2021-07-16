use std::collections::{HashMap, VecDeque};
use std::ops::Mul;

use common::parse::unsigned_number;
use common::prelude::*;
use multimap::MultiMap;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::{all_consuming, into, value};
use nom::multi::{self, separated_list1};
use nom::sequence::{separated_pair, terminated};
use nom::IResult;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-14.txt");

    let reactions = parse_reactions(input);

    let part_one = part_one(&reactions);
    let part_two = part_two();
    Ok((part_one, part_two))
}

fn part_one(reactions: &[Reaction]) -> PartAnswer {
    let start = SystemTime::now();

    let mut reactions_by_output_name = HashMap::new();

    for reaction in reactions {
        if reactions_by_output_name.contains_key(&reaction.output.name) {
            panic!("index already contains {:?}", reaction);
        }

        reactions_by_output_name.insert(reaction.output.name.clone(), reaction.clone());
    }

    if !reactions_by_output_name.contains_key("FUEL") {
        panic!("FUEL is missing from {:?}", reactions_by_output_name);
    }

    let mut queue = VecDeque::new();
    queue.push_back("FUEL");

    PartAnswer::default()
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

#[derive(Debug, PartialEq, Clone)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

impl Reaction {
    fn new(inputs: Vec<Chemical>, output: Chemical) -> Reaction {
        Reaction { inputs, output }
    }

    fn ensure_output(&self, min_output: usize) -> Reaction {
        let multiplier = (min_output as f64 / self.output.quantity as f64).ceil() as usize;

        if multiplier <= 1 {
            return self.clone();
        }

        let inputs = self.inputs.iter().map(|c| c * multiplier).collect();
        let output = &self.output * multiplier;

        Reaction::new(inputs, output)
    }
}

impl From<(Vec<Chemical>, Chemical)> for Reaction {
    fn from(tuple: (Vec<Chemical>, Chemical)) -> Reaction {
        let (inputs, output) = tuple;
        Reaction::new(inputs, output)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Chemical {
    quantity: usize,
    name: String,
}

impl Chemical {
    fn new<S: ToString>(quantity: usize, name: S) -> Chemical {
        let name = name.to_string();

        Chemical { quantity, name }
    }
}

impl Mul<usize> for &Chemical {
    type Output = Chemical;

    fn mul(self, rhs: usize) -> Chemical {
        Chemical {
            quantity: self.quantity * rhs,
            name: self.name.clone(),
        }
    }
}

impl<S: ToString> From<(usize, S)> for Chemical {
    fn from(tuple: (usize, S)) -> Chemical {
        let (quantity, name) = tuple;
        Chemical::new(quantity, name)
    }
}

fn parse_reactions(i: &str) -> Vec<Reaction> {
    all_consuming(reactions)(i).unwrap().1
}

fn reactions(i: &str) -> IResult<&str, Vec<Reaction>> {
    terminated(separated_list1(tag("\n"), reaction), multispace0)(i)
}

fn reaction(i: &str) -> IResult<&str, Reaction> {
    into(separated_pair(chemicals, reaction_separator, chemical))(i)
}

fn chemicals(i: &str) -> IResult<&str, Vec<Chemical>> {
    separated_list1(chemical_separator, chemical)(i)
}

fn chemical(i: &str) -> IResult<&str, Chemical> {
    into(separated_pair(unsigned_number, tag(" "), alpha1))(i)
}

fn chemical_separator(i: &str) -> IResult<&str, ()> {
    value((), tag(", "))(i)
}

fn reaction_separator(i: &str) -> IResult<&str, ()> {
    value((), tag(" => "))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reaction() {
        assert_eq!(
            reaction("10 ORE => 10 A"),
            Ok((
                "",
                Reaction::new(vec![Chemical::new(10, "ORE")], Chemical::new(10, "A"))
            ))
        )
    }

    #[test]
    fn test_scale_reaction() {
        let reaction = Reaction::new(
            vec![Chemical::new(1, "A"), Chemical::new(2, "B")],
            Chemical::new(3, "C"),
        );

        let scaled = reaction.ensure_output(4);

        assert_eq!(
            scaled,
            Reaction::new(
                vec![Chemical::new(2, "A"), Chemical::new(4, "B")],
                Chemical::new(6, "C")
            )
        );

        assert_eq!(reaction.ensure_output(1), reaction);
    }
}
