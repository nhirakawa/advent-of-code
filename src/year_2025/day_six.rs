use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let expressions = Expressions::try_from(input)?;
    Ok(expressions.evaluate_all(NumberDirection::Horizontal))
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let expressions = Expressions::try_from(input)?;
    Ok(expressions.evaluate_all(NumberDirection::Vertical))
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum NumberDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn apply(&self, first: Term, second: Term) -> Term {
        match self {
            Operator::Add => first + second,
            Operator::Multiply => first * second,
        }
    }
}

impl TryFrom<&str> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(anyhow::anyhow!("Invalid operand: {value}")),
        }
    }
}

impl TryFrom<u8> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Operator::try_from(&value)
    }
}

impl TryFrom<&u8> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            b'+' => Ok(Operator::Add),
            b'*' => Ok(Operator::Multiply),
            _ => Err(anyhow::anyhow!("Invalid operator: {value:x}")),
        }
    }
}

type Term = u64;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Expression {
    /// The operator to apply to the terms
    operator: Operator,
    /// The terms when parsed as horizontal strings (left-to-right, top-to-bottom)
    horizontal_terms: Vec<Term>,
    /// The terms when parsed as vertical strings (right_to_left, top_to_bottom)
    vertical_terms: Vec<Term>,
}

impl Expression {
    fn new(
        operator: Operator,
        horizontal_terms: Vec<Term>,
        vertical_terms: Vec<Term>,
    ) -> Expression {
        Expression {
            operator,
            horizontal_terms,
            vertical_terms,
        }
    }

    fn evaluate(&self, direction: NumberDirection) -> Term {
        let terms = match direction {
            NumberDirection::Horizontal => &self.horizontal_terms,
            NumberDirection::Vertical => &self.vertical_terms,
        };

        terms
            .iter()
            .copied()
            .reduce(|accum, term| self.operator.apply(accum, term))
            .expect("Expression should contain at least 1 term")
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Expressions(Vec<Expression>);

impl Expressions {
    fn evaluate_all(&self, direction: NumberDirection) -> Term {
        self.0
            .iter()
            .map(|expression| expression.evaluate(direction))
            .sum()
    }
}

impl TryFrom<&str> for Expressions {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let operators = value
            .lines()
            .rev()
            .take(1)
            .next()
            .ok_or(anyhow::anyhow!("No lines"))?;

        let operators = parse_operators(operators)?;

        let operator_indexes = operators.iter().map(|(index, _)| *index).collect_vec();

        let operators: HashMap<usize, Operator> = operators
            .iter()
            .map(|(_, operator)| *operator)
            .enumerate()
            .collect();

        let mut columns = HashSet::new();
        let mut terms_by_column = HashMap::new();

        let mut lines = value.lines().peekable();

        while let Some(line) = lines.next() {
            if lines.peek().is_none() {
                // skip last line
                continue;
            }

            let tokens = extract_term_tokens(line, &operator_indexes);

            for (column, token) in tokens.iter().enumerate() {
                let tokens = terms_by_column.entry(column).or_insert(Vec::new());
                tokens.push(*token);
                columns.insert(column);
            }
        }

        let mut expressions = Vec::new();

        for column in columns {
            let operator = operators
                .get(&column)
                .copied()
                .ok_or(anyhow::anyhow!("No operator for column {column}"))?;

            let terms = terms_by_column
                .get(&column)
                .ok_or(anyhow::anyhow!("No terms for column {column}"))?;

            let horizontal_terms: Result<Vec<Term>, _> = terms
                .iter()
                .map(|term| term.trim().parse::<Term>())
                .collect();

            let horizontal_terms = horizontal_terms?;

            let vertical_terms = parse_vertical_numbers(terms);

            let expression = Expression::new(operator, horizontal_terms, vertical_terms);
            expressions.push(expression);
        }

        Ok(Expressions(expressions))
    }
}

/// Parses a list of (index, Operator) from s
/// Returns an error if any byte is not one of b' ', b'+', or b'*'
fn parse_operators(s: &str) -> anyhow::Result<Vec<(usize, Operator)>> {
    let mut operators = Vec::new();
    for (index, byte) in s.as_bytes().iter().enumerate() {
        if *byte == b' ' {
            continue;
        }

        let operator = Operator::try_from(byte)?;
        operators.push((index, operator));
    }
    Ok(operators)
}

/// Scans the given string and extracts the string slices between operators
fn extract_term_tokens<'a>(s: &'a str, operator_indexes: &[usize]) -> Vec<&'a str> {
    let mut tokens = Vec::new();

    let mut windows = operator_indexes.iter().tuple_windows().peekable();

    while let Some((start, end)) = windows.next() {
        let slice = &s[*start..*end - 1];
        tokens.push(slice);

        if windows.peek().is_none() {
            let slice = &s[*end..];
            tokens.push(slice);
        }
    }

    tokens
}

fn parse_vertical_numbers(tokens: &[&str]) -> Vec<Term> {
    let mut columns = HashSet::new();
    let mut digits = HashMap::new();

    for token in tokens {
        for (column, b) in token.as_bytes().iter().enumerate() {
            columns.insert(column);
            if *b == b' ' {
                continue;
            }

            digits.entry(column).or_insert(Vec::new()).push(*b - 0x30);
        }
    }

    let mut terms = Vec::new();

    let columns = columns.iter().sorted().rev().collect_vec();

    for column in columns {
        let digits = digits
            .get(column)
            .expect("No terms found for column {column}");

        let term = unify(digits);
        terms.push(term);
    }

    terms
}

fn unify(digits: &[u8]) -> Term {
    let mut sum = 0;

    for (index, digit) in digits.iter().enumerate() {
        sum += *digit as Term * 10u64.pow(digits.len() as u32 - index as u32 - 1)
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vertical_numbers() {
        assert_eq!(
            parse_vertical_numbers(&["123", " 45", "  6"]),
            vec![356, 24, 1]
        );
        assert_eq!(
            parse_vertical_numbers(&["64 ", "23 ", "314"]),
            vec![4, 431, 623]
        );
    }
}
