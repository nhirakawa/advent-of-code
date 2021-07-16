use common::parse::unsigned_number;
use common::prelude::*;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::{all_consuming, into, value};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-14.txt");

    let reactions = parse_reactions(input);

    let part_one = part_one();
    let part_two = part_two();
    Ok((part_one, part_two))
}

fn part_one() -> PartAnswer {
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
}

impl From<(Vec<Chemical>, Chemical)> for Reaction {
    fn from(tuple: (Vec<Chemical>, Chemical)) -> Reaction {
        let (inputs, output) = tuple;
        Reaction::new(inputs, output)
    }
}

#[derive(Debug, PartialEq, Clone)]
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
}
