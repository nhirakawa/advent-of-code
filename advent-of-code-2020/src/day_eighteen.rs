use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{map, map_res, value},
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-18.txt");
    let parse_start = SystemTime::now();
    let tokens = parse_tokenized_expressions(input);
    let parse_elapsed = parse_start.elapsed().unwrap();

    let part_one = part_one(&tokens, parse_elapsed);
    let part_two = part_two(&tokens, parse_elapsed);

    Ok((part_one, part_two))
}

fn part_one(expressions: &[TokenizedExpression], parse_duration: Duration) -> PartAnswer {
    let start = SystemTime::now();

    let mut sum = 0;
    for expression in expressions {
        let evaluated = evaluate_expression(expression, false);
        sum += evaluated;
    }

    let elapsed = start.elapsed().unwrap();

    (sum, elapsed + parse_duration).into()
}

fn part_two(expressions: &[TokenizedExpression], parse_duration: Duration) -> PartAnswer {
    let start = SystemTime::now();

    let mut sum = 0;
    for expression in expressions {
        let evaluated = evaluate_expression(expression, true);
        sum += evaluated;
    }

    let elapsed = start.elapsed().unwrap();

    (sum, elapsed + parse_duration).into()
}

type ReversePolishExpression = Vec<StackElement>;
type TokenizedExpression = Vec<Token>;

#[derive(Debug, PartialEq, Copy, Clone)]
enum StackElement {
    Operand(u64),
    Operator(Operator),
}

fn evaluate_expression(expression: &[Token], addition_has_higher_precedence: bool) -> u64 {
    evaluate_reverse_polish_expression(shunting_yard(expression, addition_has_higher_precedence))
}

fn shunting_yard(
    tokens: &[Token],
    addition_has_higher_precedence: bool,
) -> ReversePolishExpression {
    let mut output = Vec::new();
    let mut operator_stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(number) => {
                output.push(StackElement::Operand(*number));
            }
            Token::Add | Token::Multiply => {
                if addition_has_higher_precedence
                    && operator_stack.last() == Some(&Token::Multiply)
                    && *token == Token::Add
                {
                    operator_stack.push(*token);
                } else {
                    while !operator_stack.is_empty()
                        && (!addition_has_higher_precedence
                            || operator_stack.last() == Some(&Token::Add)
                                && *token == Token::Multiply)
                    {
                        let last_operator = operator_stack[operator_stack.len() - 1];
                        if last_operator == Token::OpenParens {
                            break;
                        }

                        let last_operator = operator_stack.pop();

                        let last_operator = last_operator.map(|operator| match operator {
                            Token::Add => Operator::Add,
                            Token::Multiply => Operator::Multiply,
                            _ => unreachable!(),
                        });
                        let last_operator = last_operator.map(StackElement::Operator);

                        if let Some(last_operator) = last_operator {
                            output.push(last_operator);
                        }
                    }
                    operator_stack.push(*token);
                }
            }
            Token::OpenParens => {
                operator_stack.push(Token::OpenParens);
            }
            Token::CloseParens => {
                while operator_stack.last() != Some(&Token::OpenParens) {
                    let operator_to_move_to_output = operator_stack.pop().unwrap();

                    match operator_to_move_to_output {
                        Token::Add => output.push(StackElement::Operator(Operator::Add)),
                        Token::Multiply => output.push(StackElement::Operator(Operator::Multiply)),
                        _ => unreachable!(),
                    }
                }

                if operator_stack.last() == Some(&Token::OpenParens) {
                    operator_stack.pop();
                }
            }
        }
    }

    while !operator_stack.is_empty() {
        let element = operator_stack.pop().unwrap();
        let element = match element {
            Token::Add => StackElement::Operator(Operator::Add),
            Token::Multiply => StackElement::Operator(Operator::Multiply),
            _ => unreachable!(),
        };

        output.push(element);
    }

    output
}

fn evaluate_reverse_polish_expression(expression: ReversePolishExpression) -> u64 {
    let mut evaluation_stack = Vec::new();

    // todo terminology
    for stack_element in expression {
        match stack_element {
            StackElement::Operand(number) => evaluation_stack.push(number),
            StackElement::Operator(operator) => {
                let first = evaluation_stack.pop().unwrap();
                let second = evaluation_stack.pop().unwrap();

                let result = match operator {
                    Operator::Add => first + second,
                    Operator::Multiply => first * second,
                };

                evaluation_stack.push(result);
            }
        }
    }

    evaluation_stack.pop().unwrap()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Number(u64),
    Add,
    Multiply,
    OpenParens,
    CloseParens,
}

fn parse_tokenized_expressions(input: &str) -> Vec<TokenizedExpression> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(tokenize)
        .collect()
}

fn tokenize(i: &str) -> TokenizedExpression {
    let (_, tokens) = tokens(i).unwrap();

    tokens
}

fn tokens(i: &str) -> IResult<&str, TokenizedExpression> {
    many1(token_stripping_whitespace)(i)
}

fn token_stripping_whitespace(i: &str) -> IResult<&str, Token> {
    let parser = preceded(space, token);
    let mut parser = terminated(parser, space);

    parser(i)
}

fn token(i: &str) -> IResult<&str, Token> {
    alt((number, add, multiply, open_parens, close_parens))(i)
}

fn number(i: &str) -> IResult<&str, Token> {
    map(map_res(digit1, |s: &str| s.parse::<u64>()), Token::Number)(i)
}

fn add(i: &str) -> IResult<&str, Token> {
    value(Token::Add, tag("+"))(i)
}

fn multiply(i: &str) -> IResult<&str, Token> {
    value(Token::Multiply, tag("*"))(i)
}

fn open_parens(i: &str) -> IResult<&str, Token> {
    value(Token::OpenParens, tag("("))(i)
}

fn close_parens(i: &str) -> IResult<&str, Token> {
    value(Token::CloseParens, tag(")"))(i)
}

fn space(i: &str) -> IResult<&str, ()> {
    value((), multispace0)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_part_one() {
        let parse_and_evaluate = |input: &str| {
            let expression = tokenize(input);
            evaluate_expression(&expression, false)
        };

        assert_eq!(parse_and_evaluate("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(parse_and_evaluate("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(parse_and_evaluate("2 * 3 + (4 * 5)"), 26);
        assert_eq!(parse_and_evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            parse_and_evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            parse_and_evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_evaluate_part_two() {
        let parse_and_evaluate = |input: &str| {
            let expression = tokenize(input);
            evaluate_expression(&expression, true)
        };

        assert_eq!(parse_and_evaluate("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(parse_and_evaluate("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(parse_and_evaluate("2 * 3 + (4 * 5)"), 46);
        assert_eq!(parse_and_evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            parse_and_evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            parse_and_evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }

    #[test]
    fn test_evaluate_reverse_polish_notation() {
        let expression = vec![
            StackElement::Operand(2),
            StackElement::Operand(3),
            StackElement::Operand(4),
            StackElement::Operator(Operator::Add),
            StackElement::Operator(Operator::Add),
        ];

        assert_eq!(evaluate_reverse_polish_expression(expression), 9);
    }

    #[test]
    fn test_shunting_yard() {
        assert_eq!(
            shunting_yard(&vec![Token::Number(3), Token::Add, Token::Number(4)], false),
            vec![
                StackElement::Operand(3),
                StackElement::Operand(4),
                StackElement::Operator(Operator::Add)
            ]
        );

        assert_eq!(
            shunting_yard(
                &vec![
                    Token::Number(3),
                    Token::Add,
                    Token::Number(4),
                    Token::Multiply,
                    Token::Number(2)
                ],
                false
            ),
            vec![
                StackElement::Operand(3),
                StackElement::Operand(4),
                StackElement::Operator(Operator::Add),
                StackElement::Operand(2),
                StackElement::Operator(Operator::Multiply)
            ]
        );
    }

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("3 + 4 * 2"),
            vec![
                Token::Number(3),
                Token::Add,
                Token::Number(4),
                Token::Multiply,
                Token::Number(2)
            ]
        );
    }
}
