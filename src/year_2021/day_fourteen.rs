use std::collections::HashMap;
use std::time::SystemTime;
use crate::common::answer::*;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::multispace0,
    combinator::{all_consuming, into},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-14.txt");
    let (template, rules) = parse_reaction(input);

    let part_one = part_one(&template, &rules);
    let part_two = part_two(&template, &rules);

    Ok((part_one, part_two))
}

fn part_one(template: &str, rules: &Rules) -> PartAnswer {
    let start = SystemTime::now();

    let polymer = react(template, rules, 10);

    let max_count = polymer.most_common_character_count();
    let min_count = polymer.least_common_character_count();

    let solution = max_count - min_count;

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two(template: &str, rules: &Rules) -> PartAnswer {
    let start = SystemTime::now();

    let polymer = react(template, rules, 40);

    let max_count = polymer.most_common_character_count();
    let min_count = polymer.least_common_character_count();

    let solution = max_count - min_count;

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn react(template: &str, rules: &Rules, count: usize) -> CompactPolymer {
    let mut polymer = CompactPolymer::new(template);

    for _ in 0..count {
        polymer = polymer.react_with_rules(rules);
    }

    polymer
}

#[derive(Debug, PartialEq)]
struct CompactPolymer {
    // only keep the counts of each pair of elements
    polymer: HashMap<String, usize>,
    last_character: char,
}

impl CompactPolymer {
    fn new(longform: &str) -> CompactPolymer {
        let mut polymer = HashMap::new();

        for i in 0..longform.len() - 1 {
            let substring = &longform[i..i + 2];
            *polymer.entry(substring.to_string()).or_default() += 1;
        }

        let last_character = longform.chars().nth_back(0).unwrap();

        CompactPolymer {
            polymer,
            last_character,
        }
    }

    fn react_with_rules(&self, rules: &Rules) -> CompactPolymer {
        let mut polymer = HashMap::new();

        for (input, count) in &self.polymer {
            let output = &rules.rules[input];

            let output_1 = vec![
                input.chars().next().unwrap().to_string(),
                output.to_string(),
            ]
            .join("");
            let output_2 = vec![
                output.to_string(),
                input.chars().nth(1).unwrap().to_string(),
            ]
            .join("");

            *polymer.entry(output_1).or_default() += *count;
            *polymer.entry(output_2).or_default() += *count;
        }

        CompactPolymer {
            polymer,
            last_character: self.last_character,
        }
    }

    fn most_common_character_count(&self) -> usize {
        *self.character_counts().values().max().unwrap()
    }

    fn least_common_character_count(&self) -> usize {
        *self.character_counts().values().min().unwrap()
    }

    fn character_counts(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();

        for (chunk, count) in &self.polymer {
            if let Some(c) = chunk.chars().next() {
                *counts.entry(c).or_default() += *count;
            }
        }

        *counts.entry(self.last_character).or_default() += 1;

        counts
    }
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
    fn test_react_with_rules() {
        let rules = rules("CH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C").unwrap().1;

        let polymer = CompactPolymer::new("NNCB");
        assert_eq!(
            polymer.react_with_rules(&rules),
            CompactPolymer::new("NCNBCHB")
        );

        let polymer = CompactPolymer::new("NCNBCHB");
        assert_eq!(
            polymer.react_with_rules(&rules),
            CompactPolymer::new("NBCCNBBBCBHCB")
        );

        let polymer = CompactPolymer::new("NBCCNBBBCBHCB");
        assert_eq!(
            polymer.react_with_rules(&rules),
            CompactPolymer::new("NBBBCNCCNBBNBNBBCHBHHBCHB")
        );

        let polymer = CompactPolymer::new("NBBBCNCCNBBNBNBBCHBHHBCHB");
        assert_eq!(
            polymer.react_with_rules(&rules),
            CompactPolymer::new("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB")
        );
    }

    #[test]
    fn test_short_example() {
        let rules = rules("CH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C").unwrap().1;

        let polymer = react("NNCB", &rules, 10);

        let answer = polymer.most_common_character_count() - polymer.least_common_character_count();
        assert_eq!(answer, 1588);
    }

    #[test]
    fn test_long_example() {
        let rules = rules("CH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C").unwrap().1;

        let polymer = react("NNCB", &rules, 40);

        let answer = polymer.most_common_character_count() - polymer.least_common_character_count();
        assert_eq!(answer, 2188189693529);
    }
}
