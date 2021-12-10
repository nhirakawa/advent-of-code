use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::{map, map_res},
    multi::{many1, separated_list0, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

use common::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-16.txt");
    let parse_start = SystemTime::now();
    let rules_and_tickets = parse_rules_and_tickets(input);
    let parse_duration = parse_start.elapsed().unwrap();

    let part_one = part_one(&rules_and_tickets, parse_duration);
    let part_two = part_two(&rules_and_tickets, parse_duration);

    Ok((part_one, part_two))
}

fn part_one(rules_and_tickets: &RulesAndTickets, parse_duration: Duration) -> PartAnswer {
    let start = SystemTime::now();
    let mut error_rate = 0;

    for ticket in &rules_and_tickets.nearby_tickets {
        for field in ticket {
            let mut has_valid_rule = false;

            for rule in &rules_and_tickets.rules.rules {
                if rule.is_field_valid(field) {
                    has_valid_rule = true;
                    break;
                }
            }

            if !has_valid_rule {
                error_rate += field;
            }
        }
    }

    let elapsed = start.elapsed().unwrap();

    (error_rate, elapsed + parse_duration).into()
}

fn part_two(rules_and_tickets: &RulesAndTickets, parse_duration: Duration) -> PartAnswer {
    let start = SystemTime::now();

    let assigned_fields_by_rule = assign_rules_to_fields(rules_and_tickets);

    let mut product = 1;

    for rule in &rules_and_tickets.rules.rules {
        if rule.name.starts_with("departure") {
            let assigned_field = assigned_fields_by_rule[&rule.index];

            let value = rules_and_tickets.my_ticket[assigned_field];

            product *= value;
        }
    }

    let elapsed = start.elapsed().unwrap();

    (product, elapsed + parse_duration).into()
}

fn assign_rules_to_fields(rules_and_tickets: &RulesAndTickets) -> HashMap<usize, usize> {
    let mut valid_tickets: Vec<Vec<u64>> = Vec::new();

    for ticket in &rules_and_tickets.nearby_tickets {
        if rules_and_tickets.rules.is_ticket_valid(ticket) {
            valid_tickets.push(ticket.clone());
        }
    }

    let mut valid_fields_by_rule: HashMap<usize, Vec<usize>> = HashMap::new();

    let number_of_rules = rules_and_tickets.rules.rules.len();

    for rule in &rules_and_tickets.rules.rules {
        valid_fields_by_rule.insert(rule.index, Vec::with_capacity(number_of_rules));

        for field_index in 0..number_of_rules {
            let mut is_valid_for_field = true;

            for ticket in &valid_tickets {
                let field = ticket[field_index];

                if !rule.is_field_valid(&field) {
                    is_valid_for_field = false;
                }
            }

            if is_valid_for_field {
                valid_fields_by_rule
                    .get_mut(&rule.index)
                    .unwrap()
                    .push(field_index);
            }
        }
    }

    let mut valid_fields_by_rule: Vec<(usize, Vec<usize>)> =
        valid_fields_by_rule.into_iter().collect();
    valid_fields_by_rule.sort_by_key(|(_, v)| v.len());

    let mut assigned_fields_by_rule: HashMap<usize, usize> = HashMap::new();
    let mut assigned_fields: HashSet<usize> = HashSet::new();

    for (rule_index, possible_field_indexes) in &valid_fields_by_rule {
        if assigned_fields_by_rule.contains_key(rule_index) {
            continue;
        }

        let mut valid_and_unassigned_field_indexes = Vec::new();

        for field_index in possible_field_indexes {
            if !assigned_fields.contains(field_index) {
                valid_and_unassigned_field_indexes.push(field_index);
            }
        }

        valid_and_unassigned_field_indexes.sort();

        if valid_and_unassigned_field_indexes.len() == 1 {
            let field_index = valid_and_unassigned_field_indexes
                .into_iter()
                .next()
                .unwrap();

            assigned_fields_by_rule.insert(*rule_index, *field_index);
            assigned_fields.insert(*field_index);
        }
    }

    assigned_fields_by_rule
}

#[derive(Debug, PartialEq)]
struct Rule {
    index: usize,
    name: String,
    ranges: Vec<RangeInclusive<u64>>,
}

impl Rule {
    pub fn is_field_valid(&self, field: &u64) -> bool {
        for range in &self.ranges {
            if range.contains(field) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, PartialEq)]
struct RulesAndTickets {
    rules: Rules,
    my_ticket: Ticket,
    nearby_tickets: Tickets,
}

type Ticket = Vec<u64>;
type Tickets = Vec<Ticket>;

#[derive(Debug, PartialEq)]
struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    pub fn is_ticket_valid(&self, ticket: &[u64]) -> bool {
        for field in ticket {
            let mut is_field_valid = false;

            for rule in &self.rules {
                if rule.is_field_valid(field) {
                    is_field_valid = true;
                }
            }

            if !is_field_valid {
                return false;
            }
        }

        true
    }
}

impl From<Vec<Rule>> for Rules {
    fn from(rules: Vec<Rule>) -> Rules {
        Rules { rules }
    }
}

fn parse_rules_and_tickets(input: &str) -> RulesAndTickets {
    rules_and_tickets(input)
        .map(|(_, rules_and_tickets)| rules_and_tickets)
        .unwrap()
}

fn rules_and_tickets(i: &str) -> IResult<&str, RulesAndTickets> {
    map(
        tuple((rules, tag("\n"), my_ticket, tag("\n"), nearby_tickets)),
        |(rules, _, my_ticket, _, nearby_tickets)| RulesAndTickets {
            rules,
            my_ticket,
            nearby_tickets,
        },
    )(i)
}

fn my_ticket(i: &str) -> IResult<&str, Ticket> {
    preceded(tag("your ticket:\n"), ticket)(i)
}

fn nearby_tickets(i: &str) -> IResult<&str, Vec<Ticket>> {
    preceded(
        tag("nearby tickets:\n"),
        map(many1(ticket), |tickets| {
            tickets.into_iter().filter(|v| !v.is_empty()).collect()
        }),
    )(i)
}

fn ticket(i: &str) -> IResult<&str, Vec<u64>> {
    terminated(separated_list0(tag(","), integer), tag("\n"))(i)
}

fn rules(i: &str) -> IResult<&str, Rules> {
    map(many1(rule), |rules| {
        rules
            .into_iter()
            .enumerate()
            .map(|(index, (name, ranges))| Rule {
                index,
                name,
                ranges,
            })
            .collect::<Vec<Rule>>()
            .into()
    })(i)
}

fn rule(i: &str) -> IResult<&str, (String, Vec<RangeInclusive<u64>>)> {
    map(
        terminated(tuple((rule_name, tag(": "), ranges)), tag("\n")),
        |(name, _, ranges)| (name, ranges),
    )(i)
}

fn rule_name(i: &str) -> IResult<&str, String> {
    map(take_until(":"), |s: &str| s.into())(i)
}

fn ranges(i: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(or_separator, range)(i)
}

fn range(i: &str) -> IResult<&str, RangeInclusive<u64>> {
    map(
        separated_pair(integer, tag("-"), integer),
        |(lower, upper)| lower..=upper,
    )(i)
}

fn integer(i: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse())(i)
}

fn or_separator(i: &str) -> IResult<&str, &str> {
    tag(" or ")(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        assert_eq!(range("1-3"), Ok(("", 1..=3)));
    }

    #[test]
    fn test_ranges() {
        assert_eq!(ranges("1-3 or 5-7"), Ok(("", vec![1..=3, 5..=7])));
    }

    #[test]
    fn test_rule() {
        assert_eq!(
            rule("class: 1-3 or 5-7\n"),
            Ok(("", ("class".into(), vec![1..=3, 5..=7])))
        );
    }

    #[test]
    fn test_rules() {
        assert_eq!(
            rules("class: 1-3 or 5-7\nrow: 6-11 or 33-44\n"),
            Ok((
                "",
                vec![
                    Rule {
                        index: 0,
                        name: "class".into(),
                        ranges: vec![1..=3, 5..=7]
                    },
                    Rule {
                        index: 1,
                        name: "row".into(),
                        ranges: vec![6..=11, 33..=44]
                    }
                ]
                .into()
            ))
        );
    }

    #[test]
    fn test_ticket() {
        assert_eq!(ticket("7,1,14\n"), Ok(("", vec![7, 1, 14])));
    }

    #[test]
    fn test_my_ticket() {
        assert_eq!(
            my_ticket("your ticket:\n7,1,14\n"),
            Ok(("", vec![7, 1, 14]))
        );
    }

    #[test]
    fn test_nearby_tickets() {
        assert_eq!(
            nearby_tickets("nearby tickets:\n7,3,47\n40,4,50\n"),
            Ok(("", vec![vec![7, 3, 47], vec![40, 4, 50]]))
        );
    }

    #[test]
    fn test_rule_is_field_valid() {
        let rule = Rule {
            name: "class".into(),
            index: 0,
            ranges: vec![1..=3, 5..=7],
        };

        assert_eq!(rule.is_field_valid(&1), true);
        assert_eq!(rule.is_field_valid(&2), true);
        assert_eq!(rule.is_field_valid(&3), true);
        assert_eq!(rule.is_field_valid(&4), false);
        assert_eq!(rule.is_field_valid(&5), true);
    }

    #[test]
    fn test_rules_is_ticket_valid() {
        let rules: Rules = vec![
            Rule {
                name: "class".into(),
                index: 0,
                ranges: vec![1..=3, 5..=7],
            },
            Rule {
                name: "row".into(),
                index: 1,
                ranges: vec![6..=1, 33..=44],
            },
            Rule {
                name: "seat".into(),
                index: 2,
                ranges: vec![13..=40, 45..=50],
            },
        ]
        .into();

        assert!(rules.is_ticket_valid(&vec![7, 3, 47]));
        assert!(!rules.is_ticket_valid(&vec![40, 4, 50]));
        assert!(!rules.is_ticket_valid(&vec![55, 2, 20]));
        assert!(!rules.is_ticket_valid(&vec![38, 6, 12]));
    }

    #[test]
    fn test_assign_rules_to_fields() {
        let rules_and_tickets = parse_rules_and_tickets("class: 0-1 or 4-19\nrow: 0-5 or 8-19\nseat: 0-13 or 16-19\n\nyour ticket:\n11,12,13\n\nnearby tickets:\n3,9,18\n15,1,5\n5,14,9\n");

        let assigned = assign_rules_to_fields(&rules_and_tickets);

        let expected = vec![(0, 1), (1, 0), (2, 2)].into_iter().collect();

        assert_eq!(assigned, expected);
    }
}
