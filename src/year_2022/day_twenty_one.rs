use crate::common::answer::*;
use log::debug;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{map, value},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::{collections::HashMap, fmt::Debug};
use std::time::SystemTime;
use crate::common::parse::{finish, unsigned_number};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-21.txt");

    let part_one = part_one(input);
    let part_two = part_two(input);

    Ok((part_one, part_two))
}

fn part_one(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let mut equations = parse(input);

    equations.evaluate();

    let root_value = equations.root_value().unwrap_or(0);

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(root_value, elapsed)
}

fn part_two(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let mut lower: u128 = 1;

    let mut checked = vec![];

    let value_to_match = evaluate_and_return_values_at_root(input, 1).1;

    loop {
        checked.push(lower);

        let (left, right) = evaluate_and_return_values_at_root(input, lower);
        let (left_added, _) = evaluate_and_return_values_at_root(input, lower + 1000);

        debug!("{lower} -> {left} ... {left_added}");

        if left > right && left_added < left {
            // we're too big and we're decreasing as we increment
            lower *= 2;
        } else if left < right && left_added > left {
            // we're too small and we're increasing as we increment
            lower *= 2;
        } else {
            break;
        }
    }

    let mut upper = checked[checked.len() - 1];
    let mut lower = checked[checked.len() - 2];

    debug!("searching between {lower} and {upper} for {value_to_match}");

    let mut iterations = 0;

    while upper.abs_diff(lower) > 100 {
        if iterations > 100 {
            panic!();
        }

        let lower_value = evaluate_and_return_values_at_root(input, lower).0;
        let upper_value = evaluate_and_return_values_at_root(input, upper).0;

        debug!("{lower} - {lower_value} ... {upper} - {upper_value}");

        let midpoint = (lower + upper) / 2;

        if lower_value.abs_diff(value_to_match) > upper_value.abs_diff(value_to_match) {
            lower = midpoint - 1;
        } else {
            upper = midpoint + 1;
        }

        iterations += 1;
    }

    debug!("{lower} ... {upper}");

    for value in lower..=upper {
        let (left, right) = evaluate_and_return_values_at_root(input, value);

        debug!("{value} gives {left} = {right}");

        if left == right {
            let elapsed = start.elapsed().unwrap();

            // 3769668716710 is too high
            return PartAnswer::new(value, elapsed);
        }
    }

    PartAnswer::default()
}

fn evaluate_and_return_values_at_root(input: &str, humn_value: u128) -> (u128, u128) {
    let mut equations = parse(input);

    let root_equation = equations.root_equation();

    let lhs_equation = match &root_equation.expression {
        Expression::Constant { value: _ } => unreachable!(),
        Expression::Expression {
            lhs,
            operator: _,
            rhs: _,
        } => lhs,
    };

    let rhs_equation = match &root_equation.expression {
        Expression::Constant { value: _ } => unreachable!(),
        Expression::Expression {
            lhs: _,
            operator: _,
            rhs,
        } => rhs,
    };

    equations.results.insert("humn".to_string(), humn_value);
    equations.evaluate();

    let lhs_value = equations.results.get(lhs_equation).cloned().unwrap_or(0);
    let rhs_value = equations.results.get(rhs_equation).cloned().unwrap_or(0);

    (lhs_value, rhs_value)
}

struct Equations {
    equations: HashMap<String, Equation>,
    results: HashMap<String, u128>,
}

impl Equations {
    fn new(equations: Vec<Equation>) -> Equations {
        let mut results = HashMap::new();

        for equation in equations.iter() {
            match equation.expression {
                Expression::Constant { value } => {
                    results.insert(equation.result.clone(), value);
                }
                _ => {}
            }
        }

        let equations = equations
            .into_iter()
            .map(|equation| (equation.result.clone(), equation))
            .collect();

        Equations { equations, results }
    }

    fn root_value(&self) -> Option<u128> {
        self.results.get("root").copied()
    }

    fn root_equation(&self) -> Equation {
        self.equations.get("root").cloned().unwrap()
    }

    fn evaluate(&mut self) {
        while self.results.len() < self.equations.len() {
            let before_length = self.results.len();

            for equation in self.equations.values() {
                if self.results.contains_key(&equation.result) {
                    continue;
                }

                if !self.can_evaluate(equation) {
                    continue;
                }

                if let Some(result) = self.evaluate_expression(equation) {
                    self.results.insert(equation.result.clone(), result);
                }
            }

            let after_length = self.results.len();

            if before_length == after_length {
                break;
            }
        }
    }

    fn evaluate_expression(&self, equation: &Equation) -> Option<u128> {
        match &equation.expression {
            Expression::Constant { value } => Some(*value),
            Expression::Expression { lhs, operator, rhs } => {
                let lhs = self.results.get(lhs)?;

                let rhs = self.results.get(rhs)?;

                operator.apply(*lhs, *rhs)
            }
        }
    }

    fn can_evaluate(&self, equation: &Equation) -> bool {
        match &equation.expression {
            Expression::Constant { value: _ } => false,
            Expression::Expression {
                lhs,
                operator: _,
                rhs,
            } => self.results.contains_key(lhs) && self.results.contains_key(rhs),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Equation {
    result: String,
    expression: Expression,
}

impl Debug for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.result, self.expression)
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Expression {
    Constant {
        value: u128,
    },
    Expression {
        lhs: String,
        operator: Operator,
        rhs: String,
    },
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant { value } => write!(f, "{value}"),
            Self::Expression { lhs, operator, rhs } => write!(f, "{lhs} {operator:?} {rhs}"),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Operator {
    Multiply,
    Subtract,
    Add,
    Divide,
}

impl Operator {
    fn apply(&self, lhs: u128, rhs: u128) -> Option<u128> {
        match self {
            Operator::Multiply => lhs.checked_mul(rhs),
            Operator::Subtract => lhs.checked_sub(rhs),
            Operator::Add => lhs.checked_add(rhs),
            Operator::Divide => lhs.checked_div(rhs),
        }
    }
}

impl Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Multiply => write!(f, "*"),
            Self::Subtract => write!(f, "-"),
            Self::Add => write!(f, "+"),
            Self::Divide => write!(f, "."),
        }
    }
}

fn parse(i: &str) -> Equations {
    finish(equations)(i).unwrap().1
}

fn equations(i: &str) -> IResult<&str, Equations> {
    map(separated_list1(tag("\n"), equation), Equations::new)(i)
}

fn equation(i: &str) -> IResult<&str, Equation> {
    map(
        tuple((identifier, tag(": "), term)),
        |(result, _, expression)| Equation { result, expression },
    )(i)
}

fn term(i: &str) -> IResult<&str, Expression> {
    alt((expression, constant))(i)
}

fn expression(i: &str) -> IResult<&str, Expression> {
    map(
        tuple((identifier, tag(" "), operator, tag(" "), identifier)),
        |(lhs, _, operator, _, rhs)| Expression::Expression { lhs, operator, rhs },
    )(i)
}

fn identifier(i: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string())(i)
}

fn operator(i: &str) -> IResult<&str, Operator> {
    alt((add, subtract, multiply, divide))(i)
}

fn add(i: &str) -> IResult<&str, Operator> {
    value(Operator::Add, tag("+"))(i)
}

fn subtract(i: &str) -> IResult<&str, Operator> {
    value(Operator::Subtract, tag("-"))(i)
}

fn multiply(i: &str) -> IResult<&str, Operator> {
    value(Operator::Multiply, tag("*"))(i)
}

fn divide(i: &str) -> IResult<&str, Operator> {
    value(Operator::Divide, tag("/"))(i)
}

fn constant(i: &str) -> IResult<&str, Expression> {
    map(unsigned_number, |value| Expression::Constant { value })(i)
}
