use std::collections::HashMap;

use common::prelude::*;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::multispace0,
    combinator::{all_consuming, into},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-14.txt");
    let (template, rules) = parse_reaction(input);

    let part_one = part_one(&template, &rules);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(template: &str, rules: &Rules) -> PartAnswer {
    let start = SystemTime::now();

    let mut polymer = template.to_string();

    for _ in 0..10 {
        polymer = react(&polymer, rules);
    }

    let mut counts: HashMap<char, usize> = HashMap::new();

    for c in polymer.chars() {
        *counts.entry(c).or_default() += 1;
    }

    let max_count = *counts.values().max().unwrap();
    let min_count = *counts.values().min().unwrap();

    let solution = max_count - min_count;

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn react(polymer: &str, rules: &Rules) -> String {
    let mut parts = Vec::new();

    for i in 0..(polymer.len() - 1) {
        let substring = &polymer[i..(i + 2)];
        let output = &rules.rules[substring];
        if i == 0 {
            parts.push(substring.chars().nth(0).unwrap().to_string());
        }

        parts.push(output.to_string());
        parts.push(substring.chars().nth(1).unwrap().to_string());
    }

    parts.join("")
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

#[derive(Debug, PartialEq)]
struct Rules {
    rules: HashMap<String, String>,
}

impl From<Vec<Rule>> for Rules {
    fn from(list: Vec<Rule>) -> Rules {
        let mut rules = HashMap::new();

        for rule in list {
            rules.insert(rule.input, rule.output);
        }

        Rules { rules }
    }
}

#[derive(Debug, PartialEq)]
struct Rule {
    input: String,
    output: String,
}

impl From<(&str, &str)> for Rule {
    fn from(tuple: (&str, &str)) -> Rule {
        let (input, output) = tuple;

        let input = input.into();
        let output = output.into();

        Rule { input, output }
    }
}

fn parse_reaction(i: &str) -> (String, Rules) {
    reaction(i).unwrap().1
}

fn reaction(i: &str) -> IResult<&str, (String, Rules)> {
    all_consuming(terminated(
        separated_pair(template, tag("\n"), rules),
        multispace0,
    ))(i)
}

fn template(i: &str) -> IResult<&str, String> {
    into(terminated(elements, tag("\n")))(i)
}

fn rules(i: &str) -> IResult<&str, Rules> {
    into(separated_list1(tag("\n"), rule))(i)
}

fn rule(i: &str) -> IResult<&str, Rule> {
    into(separated_pair(elements, tag(" -> "), elements))(i)
}

fn elements(i: &str) -> IResult<&str, &str> {
    take_while1(|c| ('A'..='Z').contains(&c))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_react() {
        let rules = rules("CH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C").unwrap().1;

        assert_eq!(react("NNCB", &rules), "NCNBCHB");
        assert_eq!(react("NCNBCHB", &rules), "NBCCNBBBCBHCB");
        assert_eq!(react("NBCCNBBBCBHCB", &rules), "NBBBCNCCNBBNBNBBCHBHHBCHB");
        assert_eq!(
            react("NBBBCNCCNBBNBNBBCHBHHBCHB", &rules),
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );
    }
}
