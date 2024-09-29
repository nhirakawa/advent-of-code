use std::{
    collections::{HashMap, VecDeque},
    iter,
};
use std::time::SystemTime;
use crate::common::answer::*;
use log::debug;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use crate::common::parse::{finish, unsigned_number};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-11.txt");
    let monkeys = parse(input);

    let part_one = part_one(monkeys.clone());
    let part_two = part_two(monkeys);

    Ok((part_one, part_two))
}

fn part_one(monkeys: Vec<Monkey>) -> PartAnswer {
    let start = SystemTime::now();

    let mut game = KeepAwayGame::new(monkeys, true);

    for _ in 0..20 {
        game.play_round();
    }

    let mut values: Vec<usize> = game.inspected_items_by_monkey.values().cloned().collect();

    values.sort_unstable();

    let answer = values
        .into_iter()
        .rev()
        .take(2)
        .reduce(|first, second| first * second)
        .unwrap();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn part_two(monkeys: Vec<Monkey>) -> PartAnswer {
    let start = SystemTime::now();

    let mut game = KeepAwayGame::new(monkeys, false);

    for _ in 0..10_000 {
        game.play_round();
    }

    let mut values: Vec<usize> = game.inspected_items_by_monkey.values().cloned().collect();

    values.sort_unstable();

    let answer = values
        .into_iter()
        .rev()
        .take(2)
        .reduce(|first, second| first * second)
        .unwrap();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct KeepAwayGame {
    monkey_ids: Vec<usize>,
    monkeys: Vec<Monkey>,
    round: usize,
    inspected_items_by_monkey: HashMap<usize, usize>,
    reduce_worry_level: bool,
    modulo: usize,
}

/**
 * For part 2, the numbers get large very quickly
 * The key observation (from Reddit) is that the monkeys "divisible-by" test integers are all prime
 * Multiplying them together gives a safe modulus, so we can always "reduce" the worry level
 * modulo that modulus and end up giving the item to the same monkey as without taking the modulus
 */
impl KeepAwayGame {
    fn new(monkeys: Vec<Monkey>, reduce_worry_level: bool) -> KeepAwayGame {
        let monkey_ids = monkeys.iter().map(|monkey| monkey.id.clone()).collect();
        let mut inspected_items_by_monkey = HashMap::new();

        let mut modulo = 1;

        for monkey in monkeys.iter() {
            inspected_items_by_monkey.insert(monkey.id, 0);

            modulo *= monkey.test.divisible_by;
        }

        KeepAwayGame {
            monkey_ids,
            monkeys,
            round: 0,
            inspected_items_by_monkey,
            reduce_worry_level,
            modulo,
        }
    }

    fn play_round(&mut self) {
        for id in &self.monkey_ids {
            let mut new_items_for_monkeys: Vec<Vec<usize>> =
                iter::repeat(vec![]).take(self.monkeys.len()).collect();

            if let Some(monkey) = self.monkeys.get_mut(*id) {
                let mut items_inspected = 0;

                while let Some(item) = monkey.items.pop_front() {
                    items_inspected += 1;

                    let new_item_value = monkey.get_new_value(item);

                    let new_item_value = if self.reduce_worry_level {
                        new_item_value / 3
                    } else {
                        new_item_value % self.modulo
                    };

                    let next_monkey_id = monkey.get_next_monkey(new_item_value);

                    new_items_for_monkeys[next_monkey_id].push(new_item_value);
                }

                if let Some(count) = self.inspected_items_by_monkey.get_mut(&monkey.id) {
                    *count += items_inspected;
                }
            }

            for monkey in self.monkeys.iter_mut() {
                monkey.items.extend(&new_items_for_monkeys[monkey.id]);
            }
        }

        self.round += 1;

        if self.round == 1 || self.round == 20 || self.round % 1000 == 0 {
            debug!("== After round {} ==", self.round);
            for id in &self.monkey_ids {
                let inspected_items = self.inspected_items_by_monkey[id];
                debug!("Monkey {id} inspected {inspected_items} items",)
            }

            debug!("\n");
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
    operation_type: OperationType,
    term: Term,
}

impl Operation {
    fn new(operation_type: OperationType, term: Term) -> Operation {
        Operation {
            operation_type,
            term,
        }
    }

    fn apply(&self, value: usize) -> usize {
        match (self.operation_type, self.term) {
            (OperationType::Add, Term::Old) => value + value,
            (OperationType::Add, Term::Constant(constant)) => value + constant,
            (OperationType::Multiply, Term::Old) => value * value,
            (OperationType::Multiply, Term::Constant(constant)) => value * constant,
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

    fn apply(&self, value: usize) -> usize {
        if value % self.divisible_by == 0 {
            self.true_monkey_id
        } else {
            self.false_monkey_id
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

    fn get_new_value(&self, value: usize) -> usize {
        self.operation.apply(value)
    }

    fn get_next_monkey(&self, value: usize) -> usize {
        self.test.apply(value)
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
            tag("  Operation: new = old "),
            tuple((operation_type, tag(" "), term)),
            tag("\n"),
        ),
        |(operation_type, _, term)| Operation::new(operation_type, term),
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
        assert_eq!(false_test("    If false: throw to monkey 7"), Ok(("", 7)));
    }
}
