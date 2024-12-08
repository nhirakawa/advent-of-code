use std::collections::HashMap;

use anyhow::anyhow;
use log::debug;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::common::parse::{finish, unsigned_number};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let operations: Vec<Operation> = parse(input)?;

    let values = evaluate_all_operations(&operations);

    values
        .get("a")
        .copied()
        .ok_or_else(|| anyhow!("No value for a"))
        .map(|value| value.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let operations: Vec<Operation> = parse(input)?;

    let values = evaluate_all_operations(&operations);

    let value_of_a = values
        .get("a")
        .copied()
        .ok_or_else(|| anyhow!("No value for a"))?;

    let mut rewritten_operations = vec![];

    for operation in operations {
        if operation.rhs == "b" {
            let rewritten_operation = Operation {
                lhs: Expression::Provides(Operand::Number(value_of_a)),
                rhs: operation.rhs,
            };

            rewritten_operations.push(rewritten_operation);
        } else {
            rewritten_operations.push(operation);
        }
    }

    let values = evaluate_all_operations(&rewritten_operations);

    values
        .get("a")
        .copied()
        .ok_or_else(|| anyhow!("No value for a"))
        .map(|value| value.to_string())
}

fn evaluate_all_operations(operations: &Operations) -> HashMap<String, u16> {
    let mut values: HashMap<String, u16> = HashMap::new();

    while values.len() < operations.len() {
        for operation in operations {
            if values.contains_key(&operation.rhs) {
                continue;
            }

            if let Ok(value) = evaluate(&operation.lhs, &values) {
                debug!("{} has value {}", operation.rhs, value);
                values.insert(operation.rhs.clone(), value);
            }
        }
    }

    values
}

fn evaluate(expression: &Expression, values: &HashMap<String, u16>) -> anyhow::Result<u16> {
    match expression {
        Expression::Provides(operand) => convert_to_value(operand, values),
        Expression::And(lhs, rhs) => {
            let lhs = convert_to_value(lhs, values)?;
            let rhs = convert_to_value(rhs, values)?;
            Ok(lhs & rhs)
        }
        Expression::Or(lhs, rhs) => {
            let lhs = convert_to_value(lhs, values)?;
            let rhs = convert_to_value(rhs, values)?;
            Ok(lhs | rhs)
        }
        Expression::Not(operand) => {
            let operand = convert_to_value(operand, values)?;
            Ok(!operand)
        }
        Expression::LeftShift(lhs, rhs) => {
            let lhs = convert_to_value(lhs, values)?;
            let rhs = convert_to_value(rhs, values)?;
            Ok(lhs << rhs)
        }
        Expression::RightShift(lhs, rhs) => {
            let lhs = convert_to_value(lhs, values)?;
            let rhs = convert_to_value(rhs, values)?;
            Ok(lhs >> rhs)
        }
    }
}

fn convert_to_value(operand: &Operand, values: &HashMap<String, u16>) -> anyhow::Result<u16> {
    match operand {
        Operand::Identifier(identifier) => values
            .get(identifier)
            .copied()
            .ok_or_else(|| anyhow!("No value for {}", identifier)),
        Operand::Number(number) => Ok(*number),
    }
}

#[derive(Debug, Clone)]
struct Operation {
    lhs: Expression,
    rhs: String,
}

#[derive(Debug, Clone)]
enum Expression {
    Provides(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Not(Operand),
    LeftShift(Operand, Operand),
    RightShift(Operand, Operand),
}

#[derive(Debug, Clone)]
enum Operand {
    Identifier(String),
    Number(u16),
}

type Operations = Vec<Operation>;

fn parse(i: &str) -> anyhow::Result<Operations> {
    finish(operations)(i)
        .map(|(_, operations)| operations)
        .map_err(|e| anyhow!(e.to_string()))
}

fn operations(i: &str) -> IResult<&str, Operations> {
    separated_list1(tag("\n"), operation)(i)
}

fn operation(i: &str) -> IResult<&str, Operation> {
    map(
        separated_pair(expression, tag(" -> "), identifier),
        |(lhs, rhs)| Operation { lhs, rhs },
    )(i)
}

fn expression(i: &str) -> IResult<&str, Expression> {
    alt((and, or, not, left_shift, right_shift, provides))(i)
}

fn provides(i: &str) -> IResult<&str, Expression> {
    map(operand, Expression::Provides)(i)
}

fn and(i: &str) -> IResult<&str, Expression> {
    map(
        separated_pair(operand, tag(" AND "), operand),
        |(lhs, rhs)| Expression::And(lhs, rhs),
    )(i)
}

fn or(i: &str) -> IResult<&str, Expression> {
    map(
        separated_pair(operand, tag(" OR "), operand),
        |(lhs, rhs)| Expression::Or(lhs, rhs),
    )(i)
}

fn not(i: &str) -> IResult<&str, Expression> {
    map(preceded(tag("NOT "), operand), Expression::Not)(i)
}

fn left_shift(i: &str) -> IResult<&str, Expression> {
    map(
        separated_pair(operand, tag(" LSHIFT "), operand),
        |(lhs, rhs)| Expression::LeftShift(lhs, rhs),
    )(i)
}

fn right_shift(i: &str) -> IResult<&str, Expression> {
    map(
        separated_pair(operand, tag(" RSHIFT "), operand),
        |(lhs, rhs)| Expression::RightShift(lhs, rhs),
    )(i)
}

fn operand(i: &str) -> IResult<&str, Operand> {
    alt((
        map(identifier, Operand::Identifier),
        map(number, Operand::Number),
    ))(i)
}

fn identifier(i: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string())(i)
}

fn number(i: &str) -> IResult<&str, u16> {
    unsigned_number(i)
}
