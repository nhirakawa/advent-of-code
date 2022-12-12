use std::collections::VecDeque;

use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-11.txt");
    let monkeys = parse(input);

    let part_one = part_one();
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one() -> PartAnswer {
    let start = SystemTime::now();
    let elapsed = start.elapsed().unwrap();

    PartAnswer::default()
}

fn part_two() -> PartAnswer {
    let start = SystemTime::now();
    let elapsed = start.elapsed().unwrap();

    PartAnswer::default()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct KeepAwayGame {
    monkeys: Vec<Monkey>,
    round: usize,
}

impl KeepAwayGame {
    fn new(monkeys: Vec<Monkey>) -> KeepAwayGame {
        KeepAwayGame { monkeys, round: 0 }
    }

    fn play_round(&mut self) {
        for monkey in self.monkeys.iter_mut() {
            for item in &monkey.items {
                let item = *item;
                todo!()
            }
            todo!()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Term {
    Old,
    Constant(usize),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Operation {
    first: Term,
    operation_type: OperationType,
    second: Term,
}

impl Operation {
    fn new(first: Term, operation_type: OperationType, second: Term) -> Operation {
        Operation {
            first,
            operation_type,
            second,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum OperationType {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Test {
    divisible_by: usize,
    true_monkey_id: usize,
    false_monkey_id: usize,
}

impl Test {
    fn new(divisible_by: usize, true_monkey_id: usize, false_monkey_id: usize) -> Test {
        Test {
            divisible_by,
            true_monkey_id,
            false_monkey_id,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Monkey {
    id: usize,
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn new(id: usize, items: Vec<usize>, operation: Operation, test: Test) -> Monkey {
        let items = items.into_iter().collect();

        Monkey {
            id,
            items,
            operation,
            test,
        }
    }
}

fn parse(i: &str) -> Vec<Monkey> {
    finish(monkeys)(i).unwrap().1
}

fn monkeys(i: &str) -> IResult<&str, Vec<Monkey>> {
    many1(monkey)(i)
}

fn monkey(i: &str) -> IResult<&str, Monkey> {
    terminated(
        map(
            tuple((monkey_id, starting_items, operation, test)),
            |(id, items, operation, test)| Monkey::new(id, items, operation, test),
        ),
        multispace0,
    )(i)
}

fn monkey_id(i: &str) -> IResult<&str, usize> {
    delimited(tag("Monkey "), unsigned_number, tag(":\n"))(i)
}

fn starting_items(i: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        tag("  Starting items: "),
        separated_list1(tag(", "), unsigned_number),
        tag("\n"),
    )(i)
}

fn operation(i: &str) -> IResult<&str, Operation> {
    map(
        delimited(
            tag("  Operation: new = "),
            tuple((term, tag(" "), operation_type, tag(" "), term)),
            tag("\n"),
        ),
        |(first, _, operation_type, _, second)| Operation::new(first, operation_type, second),
    )(i)
}

fn term(i: &str) -> IResult<&str, Term> {
    alt((old_term, constant_term))(i)
}

fn old_term(i: &str) -> IResult<&str, Term> {
    value(Term::Old, tag("old"))(i)
}

fn constant_term(i: &str) -> IResult<&str, Term> {
    map(unsigned_number, Term::Constant)(i)
}

fn operation_type(i: &str) -> IResult<&str, OperationType> {
    alt((add_operation_type, multiply_operation_type))(i)
}

fn add_operation_type(i: &str) -> IResult<&str, OperationType> {
    value(OperationType::Add, tag("+"))(i)
}

fn multiply_operation_type(i: &str) -> IResult<&str, OperationType> {
    value(OperationType::Multiply, tag("*"))(i)
}

fn test(i: &str) -> IResult<&str, Test> {
    map(
        terminated(tuple((divisible_by, true_test, false_test)), tag("\n")),
        |(divisible_by, true_monkey_id, false_monkey_id)| {
            Test::new(divisible_by, true_monkey_id, false_monkey_id)
        },
    )(i)
}

fn divisible_by(i: &str) -> IResult<&str, usize> {
    delimited(tag("  Test: divisible by "), unsigned_number, tag("\n"))(i)
}

fn true_test(i: &str) -> IResult<&str, usize> {
    delimited(
        tag("    If true: throw to monkey "),
        unsigned_number,
        tag("\n"),
    )(i)
}

fn false_test(i: &str) -> IResult<&str, usize> {
    preceded(tag("    If false: throw to monkey "), unsigned_number)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisible_by() {
        assert_eq!(divisible_by("  Test: divisible by 7\n"), Ok(("", 7)));
    }

    #[test]
    fn test_true_test_() {
        assert_eq!(true_test("    If true: throw to monkey 6\n"), Ok(("", 6)));
    }

    #[test]
    fn test_false_test() {
        assert_eq!(false_test("    If false: throw to monkey 7\n"), Ok(("", 7)));
    }
}
