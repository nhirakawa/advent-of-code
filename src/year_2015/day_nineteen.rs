use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (rules, molecule) = parse::parse(input)?;

    let produced_molecules = do_one_replacement(molecule, &rules);

    Ok(produced_molecules.len())
}

// TODO - rewrite to use CYK algorithm for rigor
pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (_rules, molecule) = parse::parse(input)?;

    // Taken from https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4etju/

    let number_of_elements = molecule.chars().filter(|c| c.is_uppercase()).count();
    let count_of_rn = molecule.matches("Rn").count();
    let count_of_ar = molecule.matches("Ar").count();
    let count_of_y = molecule.matches("Y").count();

    println!("number of elements: {}", number_of_elements);
    println!("count_of_rn: {}", count_of_rn);
    println!("count_of_ar: {}", count_of_ar);
    println!("count_of_y: {}", count_of_y);

    let result = number_of_elements - count_of_rn - count_of_ar - (2 * count_of_y) - 1;

    Ok(result)
}

fn do_one_replacement(molecule: &str, rules: &[Rule]) -> HashSet<String> {
    let from_lengths = rules
        .iter()
        .map(|rule| rule.from.len())
        .unique()
        .sorted()
        .collect::<Vec<_>>();

    let mut produced_molecules = HashSet::new();

    for from_length in &from_lengths {
        for idx in 0..=molecule.len() - from_length {
            let start = idx;
            let end = idx + from_length;

            if end >= molecule.len() {
                continue;
            }

            let from = &molecule[start..end];

            for rule in rules {
                if rule.from == from {
                    let mut produced_molecule = String::new();
                    produced_molecule.push_str(&molecule[..idx]);
                    produced_molecule.push_str(rule.to);
                    produced_molecule.push_str(&molecule[idx + from_length..]);
                    produced_molecules.insert(produced_molecule);
                }
            }
        }
    }

    produced_molecules
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Rule<'a> {
    from: &'a str,
    to: &'a str,
}

impl<'a> Rule<'a> {
    pub fn new(from: &'a str, to: &'a str) -> Self {
        Self { from, to }
    }
}

mod parse {
    use anyhow::anyhow;

    use super::Rule;

    pub fn parse(input: &str) -> anyhow::Result<(Vec<Rule>, &str)> {
        let (rules, molecule) = input.split_once("\n\n").ok_or(anyhow!("Invalid input"))?;

        let rules = parse_rules(rules)?;

        Ok((rules, molecule))
    }

    fn parse_rules(input: &str) -> anyhow::Result<Vec<Rule>> {
        input
            .lines()
            .map(|line| {
                line.split_once(" => ")
                    .ok_or(anyhow!("Invalid rule"))
                    .map(|(from, to)| Rule::new(from, to))
            })
            .collect::<anyhow::Result<Vec<_>>>()
    }
}
