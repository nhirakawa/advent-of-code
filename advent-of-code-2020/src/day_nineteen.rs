use std::collections::HashMap;

use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space0, space1},
    combinator::{map, map_res},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};
use regex::Regex;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-19.txt");
    let parse_start = SystemTime::now();
    let rules_and_messages = parse_rules_and_messages(input);

    let parse_duration = parse_start.elapsed().unwrap();

    let part_one = part_one(&rules_and_messages, parse_duration);
    let part_two = part_two(&rules_and_messages, parse_duration);

    Ok((part_one, part_two))
}

fn part_one(rules_and_messages: &RulesAndMessages, parse_duration: Duration) -> PartAnswer {
    let start = SystemTime::now();

    let regexes_by_index = build_regular_expressions(&rules_and_messages.rules);

    let regex = format!("^{}$", &regexes_by_index[&0]);
    let regex = Regex::new(&regex).unwrap();

    let counter = count_matches(&rules_and_messages.messages, &regex);

    let elapsed = start.elapsed().unwrap();

    (counter, elapsed + parse_duration).into()
}

fn part_two(rules_and_messages: &RulesAndMessages, parse_duration: Duration) -> PartAnswer {
    let start = SystemTime::now();

    let regexes_by_index = build_regular_expressions(&rules_and_messages.rules);

    let rule_forty_two = &regexes_by_index[&42];
    let rule_thirty_one = &regexes_by_index[&31];

    let new_rule_eleven = (1..=4)
        .into_iter()
        .map(|i| {
            format!(
                "(({}{{{}}})({}{{{}}}))",
                rule_forty_two, i, rule_thirty_one, i
            )
        })
        .collect::<Vec<String>>()
        .join("|");
    let new_rule_eleven = format!("({})", new_rule_eleven);

    let new_rule_eight = format!("({}+)", rule_forty_two);

    let new_rule_zero = format!("^({})({})$", new_rule_eight, new_rule_eleven);

    let regex = Regex::new(&new_rule_zero).unwrap();

    let counter = count_matches(&rules_and_messages.messages, &regex);

    let elapsed = start.elapsed().unwrap();

    (counter, elapsed + parse_duration).into()
}

fn build_regular_expressions(rules: &[Rule]) -> HashMap<usize, String> {
    // first populate the terminal rules
    // then populate the rules that only reference the initial terminal rules
    // continue until all rules are populated
    let mut terminating_rules = HashMap::new();

    while terminating_rules.len() != rules.len() {
        for rule in rules {
            match &rule.rule_type {
                RuleType::Terminal(s) => {
                    terminating_rules.insert(rule.index, s.clone());
                }
                RuleType::Referencing(references) => {
                    let all_reference_terminating_rule = references
                        .iter()
                        .flatten()
                        .all(|index| terminating_rules.contains_key(&index));

                    if all_reference_terminating_rule {
                        let mut regex_groups = Vec::new();

                        for list_of_references in references {
                            let mut s = String::new();
                            s += "(";

                            for reference in list_of_references {
                                s += &terminating_rules[reference];
                            }

                            s += ")";

                            regex_groups.push(s);
                        }

                        let regex_group = format!("({})", regex_groups.join("|"));

                        terminating_rules.insert(rule.index, regex_group);
                    }
                }
            }
        }
    }

    terminating_rules
}

fn count_matches(messages: &[String], regex: &Regex) -> u64 {
    let mut counter = 0;

    for message in messages {
        if regex.is_match(message) {
            counter += 1;
        }
    }

    counter
}

#[derive(Debug, PartialEq, Clone)]
struct RulesAndMessages {
    rules: Vec<Rule>,
    messages: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
struct Rule {
    rule_type: RuleType,
    index: usize,
}

#[derive(Debug, PartialEq, Clone)]
enum RuleType {
    Terminal(String),
    Referencing(Vec<Vec<usize>>),
}

fn parse_rules_and_messages(i: &str) -> RulesAndMessages {
    rules_and_messages(i)
        .map(|(_, rules_and_messages)| rules_and_messages)
        .unwrap()
}

fn rules_and_messages(i: &str) -> IResult<&str, RulesAndMessages> {
    map(
        separated_pair(rules, tag("\n\n"), messages),
        |(rules, messages)| RulesAndMessages { rules, messages },
    )(i)
}

fn rules(i: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(tag("\n"), rule)(i)
}

fn rule(i: &str) -> IResult<&str, Rule> {
    let (remaining, (index, _, rule_type)) =
        tuple((number, tag(": "), alt((terminal_rule, referencing_rule))))(i)?;

    Ok((remaining, Rule { rule_type, index }))
}

fn referencing_rule(i: &str) -> IResult<&str, RuleType> {
    let parser = separated_list1(space1, number);
    let parser = preceded(space0, parser);
    let parser = terminated(parser, space0);

    let parser = separated_list0(tag("|"), parser);

    let mut parser = map(parser, RuleType::Referencing);

    parser(i)
}

fn number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse())(i)
}

fn terminal_rule(i: &str) -> IResult<&str, RuleType> {
    map(delimited(tag("\""), alpha1, tag("\"")), |s: &str| {
        RuleType::Terminal(s.into())
    })(i)
}

fn messages(i: &str) -> IResult<&str, Vec<String>> {
    separated_list1(tag("\n"), message)(i)
}

fn message(i: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.into())(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_referencing_rule() {
        assert_eq!(
            referencing_rule("4 1 5"),
            Ok(("", RuleType::Referencing(vec![vec![4, 1, 5]])))
        );
        assert_eq!(
            referencing_rule("2 3 | 3 2"),
            Ok(("", RuleType::Referencing(vec![vec![2, 3], vec![3, 2]])))
        );
    }

    #[test]
    fn test_terminal_rule() {
        assert_eq!(
            terminal_rule("\"a\""),
            Ok(("", RuleType::Terminal("a".into())))
        );
    }

    #[test]
    fn test_rule() {
        assert_eq!(
            rule("5: \"b\""),
            Ok((
                "",
                Rule {
                    index: 5,
                    rule_type: RuleType::Terminal("b".into())
                }
            ))
        );

        assert_eq!(
            rule("0: 4 1 5"),
            Ok((
                "",
                Rule {
                    index: 0,
                    rule_type: RuleType::Referencing(vec![vec![4, 1, 5]])
                }
            ))
        );
    }

    #[test]
    fn test_rules() {
        assert_eq!(
            rules("0: 4 1 4\n1: 2 3 | 3 2"),
            Ok((
                "",
                vec![
                    Rule {
                        index: 0,
                        rule_type: RuleType::Referencing(vec![vec![4, 1, 4]])
                    },
                    Rule {
                        index: 1,
                        rule_type: RuleType::Referencing(vec![vec![2, 3], vec![3, 2]])
                    }
                ]
            ))
        );
    }
}
