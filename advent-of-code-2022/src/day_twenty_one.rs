use common::prelude::*;
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

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-21.txt");

    let part_one = part_one(input);
    let part_two = part_two();

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

fn part_two() -> PartAnswer {
    let start = SystemTime::now();
    let _elapsed = start.elapsed().unwrap();
    PartAnswer::default()
}

struct Equations {
    equations: HashMap<String, Equation>,
    results: HashMap<String, usize>,
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

    fn root_value(&self) -> Option<usize> {
        self.results.get("root").copied()
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
                panic!()
            }
        }
    }

    fn evaluate_expression(&self, equation: &Equation) -> Option<usize> {
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
        value: usize,
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
    fn apply(&self, lhs: usize, rhs: usize) -> Option<usize> {
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
