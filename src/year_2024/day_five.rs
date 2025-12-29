use anyhow::bail;
use nom::{
    IResult, Parser, bytes::complete::tag, multi::separated_list1, sequence::separated_pair,
};

use crate::common::parse::{finish, unsigned_number};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (rules, updates) = parse(input)?;

    let mut sum = 0;

    for update in updates {
        if follows_rules(&update, &rules) {
            sum += find_middle(&update)?;
        }
    }

    Ok(sum)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (rules, updates) = parse(input)?;

    let mut sum = 0;

    for mut update in updates {
        let follows_rules = follows_rules(&update, &rules);
        if !follows_rules {
            update.sort_by(|a, b| {
                if let Some((left, right)) = find_rule(*a, *b, &rules) {
                    if *a == left && *b == right {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                } else {
                    panic!("No rule found for {} and {}", a, b);
                }
            });

            sum += find_middle(&update)?;
        }
    }

    Ok(sum)
}

fn follows_rules(update: &Update, rules: &Rules) -> bool {
    for i in 0..update.len() - 1 {
        for j in i + 1..update.len() {
            let a = update[i];
            let b = update[j];

            if let Some((left, right)) = find_rule(a, b, rules) {
                if b == left && a == right {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    true
}

fn find_rule(a: u32, b: u32, rules: &Rules) -> Option<Rule> {
    for rule in rules {
        let (first, second) = rule;
        if a == *first && b == *second {
            return Some(*rule);
        }
        if a == *second && b == *first {
            return Some(*rule);
        }
    }
    None
}

fn find_middle(nums: &[u32]) -> anyhow::Result<u32> {
    if nums.len().is_multiple_of(2) {
        bail!("Length must be odd ({})", nums.len());
    }

    let middle = nums.len() / 2;
    Ok(nums[middle])
}

type Rule = (u32, u32);
type Rules = Vec<Rule>;
type Update = Vec<u32>;
type Updates = Vec<Update>;

fn parse(i: &str) -> anyhow::Result<(Rules, Updates)> {
    finish(rules_and_updates, i)
}

fn rules_and_updates(i: &str) -> IResult<&str, (Rules, Updates)> {
    separated_pair(rules, tag("\n\n"), updates).parse(i)
}

fn rules(i: &str) -> IResult<&str, Rules> {
    separated_list1(tag("\n"), rule).parse(i)
}

fn rule(i: &str) -> IResult<&str, Rule> {
    separated_pair(unsigned_number, tag("|"), unsigned_number).parse(i)
}

fn update(i: &str) -> IResult<&str, Update> {
    separated_list1(tag(","), unsigned_number).parse(i)
}

fn updates(i: &str) -> IResult<&str, Updates> {
    separated_list1(tag("\n"), update).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_middle_element() {
        assert_eq!(find_middle(&[75, 47, 61, 53, 29]).unwrap(), 61);
    }
}
