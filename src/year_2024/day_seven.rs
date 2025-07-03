use crate::common::parse::{finish, unsigned_number};
use nom::{
    bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult, Parser,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let equations = parse(input)?;

    let mut sum = 0;

    for equation in equations {
        let (test_value, operands) = equation;
        if is_equation_true_recursive(
            test_value,
            operands[0],
            1,
            &operands,
            OperatorMode::AddMultiply,
        ) {
            sum += test_value;
        }
    }

    Ok(sum)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let equations = parse(input)?;

    let sum = equations
        .into_par_iter()
        .filter(|(test_value, operands)| {
            is_equation_true_recursive(
                *test_value,
                operands[0],
                1,
                operands,
                OperatorMode::AddMultiplyConcatenate,
            )
        })
        .map(|(test_value, _)| test_value)
        .sum::<u64>();

    Ok(sum)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OperatorMode {
    AddMultiply,
    AddMultiplyConcatenate,
}

fn is_equation_true_recursive(
    test_value: TestValue,
    current_value: Operand,
    index: usize,
    operands: &Operands,
    operator_mode: OperatorMode,
) -> bool {
    if current_value > test_value {
        return false;
    }

    if index == operands.len() {
        return current_value == test_value;
    }

    let next_value = operands[index];

    if is_equation_true_recursive(
        test_value,
        current_value + next_value,
        index + 1,
        operands,
        operator_mode,
    ) {
        return true;
    }
    if is_equation_true_recursive(
        test_value,
        current_value * next_value,
        index + 1,
        operands,
        operator_mode,
    ) {
        return true;
    }

    if operator_mode == OperatorMode::AddMultiplyConcatenate
        && is_equation_true_recursive(
            test_value,
            concatenate(current_value, next_value),
            index + 1,
            operands,
            operator_mode,
        )
    {
        return true;
    }

    false
}

fn concatenate(a: Operand, b: Operand) -> Operand {
    format!("{}{}", a, b).parse().unwrap()
}

type TestValue = u64;
type Operand = u64;
type Operands = Vec<Operand>;
type Equation = (TestValue, Operands);
type Equations = Vec<Equation>;

fn parse(i: &str) -> anyhow::Result<Equations> {
    finish(equations, i)
}

fn equations(i: &str) -> IResult<&str, Equations> {
    separated_list1(tag("\n"), equation).parse(i)
}

fn equation(i: &str) -> IResult<&str, Equation> {
    separated_pair(test_value, tag(": "), operands).parse(i)
}

fn test_value(i: &str) -> IResult<&str, TestValue> {
    unsigned_number(i)
}

fn operands(i: &str) -> IResult<&str, Operands> {
    separated_list1(tag(" "), operand).parse(i)
}

fn operand(i: &str) -> IResult<&str, Operand> {
    unsigned_number(i)
}
