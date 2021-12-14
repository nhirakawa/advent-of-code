use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display};
use std::ops::{Add, Mul, Sub};

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

    let part_one = part_one(&reactions);
    let part_two = part_two();
    Ok((part_one, part_two))
}

fn part_one(reactions: &[Reaction]) -> PartAnswer {
    let start = SystemTime::now();

    let mut reactor = Reactor::with_initial_ore(reactions, Quantity::Unlimited);

    reactor.produce_fuel();

    PartAnswer::new(reactor.used_reactants["ORE"], start.elapsed().unwrap())
}

fn index_reactions_by_output_name(reactions: &[Reaction]) -> HashMap<String, Reaction> {
    let mut reactions_by_output_name = HashMap::new();

    for reaction in reactions {
        if reactions_by_output_name.contains_key(&reaction.output.name) {
            panic!("index already contains {:?}", reaction);
        }

        reactions_by_output_name.insert(reaction.output.name.clone(), reaction.clone());
    }

    reactions_by_output_name
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

struct Reactor {
    reactions_by_output_name: HashMap<String, Reaction>,
    available_reactants: HashMap<String, Quantity>,
    used_reactants: HashMap<String, usize>,
}

impl Reactor {
    fn with_initial_ore(reactions: &[Reaction], ore: Quantity) -> Reactor {
        let mut available_reactants = HashMap::new();
        available_reactants.insert("ORE".to_string(), ore);

        let reactions_by_output_name = index_reactions_by_output_name(reactions);

        Reactor {
            reactions_by_output_name,
            available_reactants,
            used_reactants: HashMap::new(),
        }
    }

    /*
     * returns true if fuel can be produced, false otherwise
     */
    fn produce_fuel(&mut self) -> bool {
        let amount_before_reaction = self
            .available_reactants
            .get("FUEL")
            .cloned()
            .unwrap_or(Quantity::Limited(0));

        self.produce("FUEL", 1);

        let amount_after_reaction = self.available_reactants.get("FUEL").unwrap();

        amount_after_reaction > &amount_before_reaction
    }

    fn produce(&mut self, output_name: &str, amount: usize) -> bool {
        // do we already have some output
        let available_quantity = self
            .available_reactants
            .get(output_name)
            .cloned()
            .unwrap_or(Quantity::Limited(0));

        if available_quantity.has_required_amount(amount) {
            println!("{} {} already available", amount, output_name);
            return true;
        }

        // if not enough ore, terminate recursion
        if output_name == "ORE" {
            return false;
        }

        // todo - this is safe, but it should be more obvious
        let amount_needed = available_quantity.get_amount_missing(amount).unwrap();

        println!(
            "{} {} needed and {:?} {} already exists - need {} {}",
            amount, output_name, available_quantity, output_name, amount_needed, output_name
        );

        let reaction = self
            .reactions_by_output_name
            .get(output_name)
            .expect(format!("could not get reaction for {}", output_name).as_str())
            .ensure_output(amount_needed);

        for input in reaction.inputs.iter() {
            println!("producing {} as input for {}", input.name, output_name);
            let enough_input = self.produce(&input.name, input.quantity);

            if !enough_input {
                return false;
            }
        }

        for input in reaction.inputs.iter() {
            self.consume(&input.name, input.quantity);
        }

        // this is wrong - we may produce more than what we need
        let new_available_reactant = &available_quantity + reaction.output.quantity;
        println!(
            "{:?} {} was just produced",
            new_available_reactant, output_name
        );
        self.available_reactants
            .insert(output_name.to_string(), new_available_reactant);

        return true;
    }

    fn consume(&mut self, name: &str, amount: usize) {
        println!("{:?}", self.available_reactants);
        let available_quantity = self.available_reactants.get(name).cloned().unwrap();

        println!(
            "consuming {} {}, {:?} {} available",
            amount, name, available_quantity, name
        );

        let new_available_reactant = &available_quantity - amount;
        let new_used_reactant = self.used_reactants.get(name).cloned().unwrap_or(0) + amount;

        self.available_reactants
            .insert(name.clone().to_string(), new_available_reactant);
        self.used_reactants
            .insert(name.clone().to_string(), new_used_reactant);
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Quantity {
    Unlimited,
    Limited(usize),
}

impl PartialOrd for &Quantity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for &Quantity {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Quantity::Unlimited, Quantity::Unlimited) => Ordering::Equal,
            (Quantity::Unlimited, Quantity::Limited(_)) => Ordering::Greater,
            (Quantity::Limited(_), Quantity::Unlimited) => Ordering::Less,
            (Quantity::Limited(first), Quantity::Limited(second)) => first.cmp(second),
        }
    }
}

impl Add<usize> for &Quantity {
    type Output = Quantity;

    fn add(self, rhs: usize) -> Self::Output {
        match self {
            Quantity::Unlimited => Quantity::Unlimited,
            Quantity::Limited(amount) => Quantity::Limited(*amount + rhs),
        }
    }
}

impl Sub<usize> for &Quantity {
    type Output = Quantity;
    fn sub(self, rhs: usize) -> Self::Output {
        match self {
            Quantity::Unlimited => Quantity::Unlimited,
            Quantity::Limited(amount) => Quantity::Limited(amount - rhs),
        }
    }
}

impl Debug for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quantity::Unlimited => write!(f, "Unlimited"),
            Quantity::Limited(amount) => write!(f, "{}", amount),
        }
    }
}

impl Quantity {
    fn has_required_amount(&self, required: usize) -> bool {
        match self {
            Quantity::Unlimited => true,
            Quantity::Limited(available) => *available >= required,
        }
    }

    fn get_amount_missing(&self, required: usize) -> Option<usize> {
        if self.has_required_amount(required) {
            None
        } else {
            // we always have enough when we have unlimited
            // if we don't have enough, it must be limited

            if let Quantity::Limited(current) = self {
                let current = *current;
                if required <= current {
                    None
                } else {
                    // println!("current {}, required {}", current, required);
                    Some(required - current)
                }
            } else {
                panic!()
            }
        }
    }
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

impl Display for Reaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inputs = self
            .inputs
            .iter()
            .map(|f| format!("{}", f))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{} => {}", inputs, self.output)
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

impl Display for Chemical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.quantity, self.name)
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

    #[test]
    fn test_amount_missing() {
        let quantity = Quantity::Limited(10);

        assert_eq!(quantity.get_amount_missing(11), Some(1));
        assert_eq!(quantity.get_amount_missing(9), None);

        let quantity = Quantity::Unlimited;

        assert_eq!(quantity.get_amount_missing(1), None);
    }

    #[test]
    fn test_produce() {
        let reactions = vec![Reaction::new(
            vec![Chemical::new(10, "ORE")],
            Chemical::new(1, "A"),
        )];

        let mut reactor = Reactor::with_initial_ore(&reactions, Quantity::Limited(11));

        assert!(reactor.produce("A", 1));
        assert_eq!(reactor.available_reactants["ORE"], Quantity::Limited(1));
        assert_eq!(reactor.available_reactants["A"], Quantity::Limited(1));
        assert_eq!(reactor.used_reactants["ORE"], 10);
        assert_eq!(reactor.used_reactants.get("A"), None);
        assert!(reactor.produce("A", 1));

        reactor.consume("A", 1);

        assert_eq!(reactor.produce("A", 1), false);
        assert_eq!(
            reactor.available_reactants.get("ORE").cloned(),
            Some(Quantity::Limited(1))
        );
        assert_eq!(
            reactor.available_reactants.get("A").cloned(),
            Some(Quantity::Limited(0))
        );
        assert_eq!(reactor.used_reactants.get("ORE").cloned(), Some(10));
        assert_eq!(reactor.used_reactants.get("A").cloned(), Some(1));
    }

    #[test]
    fn test_reaction_one() {
        let reactions = "9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL\n";
        let reactions = parse_reactions(reactions);

        let required_ore = part_one(&reactions);

        assert_eq!(required_ore.get_answer(), "165");
    }

    #[test]
    fn test_reaction_two() {
        let reactions = "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT\n";
        let reactions = parse_reactions(reactions);

        let required_ore = part_one(&reactions);

        assert_eq!(required_ore.get_answer(), "13312");
    }

    #[test]
    fn test_reaction_three() {
        let reactions = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n17 NVRVD, 3 JNWZP => 8 VPVL\n53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n22 VJHF, 37 MNCFX => 5 FWMGM\n139 ORE => 4 NVRVD\n144 ORE => 7 JNWZP\n5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n145 ORE => 6 MNCFX\n1 NVRVD => 8 CXFTF\n1 VJHF, 6 MNCFX => 4 RFSQX\n176 ORE => 6 VJHF\n";
        let reactions = parse_reactions(reactions);

        let required_ore = part_one(&reactions);

        assert_eq!(required_ore.get_answer(), "180697");
    }
}
