use std::fmt::Display;

use common::{parse::unsigned_number, prelude::*};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, into, map, value},
    multi::{many0, many1},
    sequence::terminated,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-18.txt");
    let symbols = parse_symbols(input);

    let part_one = part_one(&symbols);
    let part_two = part_two(&symbols);

    Ok((part_one, part_two))
}

fn part_one(numbers: &[Number]) -> PartAnswer {
    PartAnswer::default()
}

fn part_two(symbols: &[Number]) -> PartAnswer {
    PartAnswer::default()
}

fn add(first: &Number, second: &Number) -> Number {
    // add 2 to combined lengths for open + close brackets
    let new_length = first.len() + second.len() + 2;

    let mut number = Vec::with_capacity(new_length);

    number.push(Symbol::OpenBracket);

    number.extend(first.symbols.iter().copied());
    number.extend(second.symbols.iter().copied());

    number.push(Symbol::CloseBracket);

    number.into()
}

fn explode(number: &Number) -> Number {
    let mut result = Vec::new();

    let mut current_depth = 0;

    let mut carryover = 0;

    for (index, symbol) in number.symbols.iter().enumerate() {
        if let Symbol::OpenBracket = symbol {
            current_depth += 1;
            if current_depth <= 4 {
                result.push(*symbol);
            }
        } else if let Symbol::CloseBracket = symbol {
            current_depth -= 1;
            result.push(*symbol);
        } else if let Symbol::Number(current_number) = symbol {
            if current_depth > 4 {
                println!("searching for left number");

                let mut new_left_number = 0;
                let mut stack = Vec::new();

                // look back to find a number
                while let Some(popped) = result.pop() {
                    println!("checking {}", popped);
                    if let Symbol::Number(left_number) = popped {
                        new_left_number = left_number + current_number;
                        println!(
                            "found left number {}, new combined number is {}",
                            left_number, new_left_number
                        );
                        stack.push(Symbol::Number(new_left_number));
                        break;
                    } else {
                        let current_depth_modifier = match popped {
                            Symbol::OpenBracket => -1,
                            Symbol::CloseBracket => 1,
                            _ => 0,
                        };

                        current_depth += current_depth_modifier;
                        stack.push(popped);
                    }
                }

                // replay the symbols
                while let Some(popped) = stack.pop() {
                    result.push(popped);

                    let current_depth_modifier = match popped {
                        Symbol::OpenBracket => 1,
                        Symbol::CloseBracket => -1,
                        _ => 0,
                    };

                    current_depth += current_depth_modifier;
                }

                result.push(Symbol::Number(new_left_number));

                if let Some(Symbol::Number(right_number)) = number.symbols.get(index + 1).copied() {
                    carryover = right_number;
                }
            } else {
                let current_number = current_number + carryover;
                carryover = 0;

                result.push(Symbol::Number(current_number));
            }
        }
    }

    result.into()
}

fn split(number: &Number) -> Number {
    let mut result = Vec::with_capacity(number.len() + 10);

    for symbol in &number.symbols {
        if let Symbol::Number(number) = symbol {
            let number = *number;

            if number >= 10 {
                result.push(Symbol::OpenBracket);

                let first = (number as f32 / 2.0).floor() as u8;
                let second = (number as f32 / 2.0).ceil() as u8;

                result.push(Symbol::Number(first));
                result.push(Symbol::Number(second));

                result.push(Symbol::CloseBracket);
            } else {
                result.push(*symbol);
            }
        } else {
            result.push(*symbol);
        }
    }

    result.into()
}

#[derive(PartialEq)]
struct Number {
    symbols: Vec<Symbol>,
}

impl Number {
    fn new(symbols: Vec<Symbol>) -> Number {
        Number { symbols }
    }

    fn len(&self) -> usize {
        self.symbols.len()
    }
}

impl core::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .symbols
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{}", out)
    }
}

impl From<Vec<Symbol>> for Number {
    fn from(symbols: Vec<Symbol>) -> Number {
        Number::new(symbols)
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Symbol {
    OpenBracket,
    CloseBracket,
    Number(u8),
}

impl core::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenBracket => write!(f, "["),
            Self::CloseBracket => write!(f, "]"),
            Self::Number(number) => write!(f, "{}", number),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn parse_symbols(i: &str) -> Vec<Number> {
    all_consuming(numbers)(i).unwrap().1
}

fn numbers(i: &str) -> IResult<&str, Vec<Number>> {
    many1(number)(i)
}

fn number(i: &str) -> IResult<&str, Number> {
    into(many1(symbol))(i)
}

fn symbol(i: &str) -> IResult<&str, Symbol> {
    terminated(alt((open_bracket, regular_number, close_bracket)), junk)(i)
}

fn open_bracket(i: &str) -> IResult<&str, Symbol> {
    value(Symbol::OpenBracket, tag("["))(i)
}

fn regular_number(i: &str) -> IResult<&str, Symbol> {
    map(unsigned_number, Symbol::Number)(i)
}

fn close_bracket(i: &str) -> IResult<&str, Symbol> {
    value(Symbol::CloseBracket, tag("]"))(i)
}

fn junk(i: &str) -> IResult<&str, Vec<&str>> {
    many0(alt((tag(","), tag(" "), tag("\n"))))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        assert_eq!(
            number("[1,2]"),
            Ok((
                "",
                vec![
                    Symbol::OpenBracket,
                    Symbol::Number(1),
                    Symbol::Number(2),
                    Symbol::CloseBracket
                ]
                .into()
            ))
        );
    }

    #[test]
    fn test_add_number() {
        let first = vec![
            Symbol::OpenBracket,
            Symbol::Number(1),
            Symbol::Number(2),
            Symbol::CloseBracket,
        ]
        .into();
        let second = vec![
            Symbol::OpenBracket,
            Symbol::Number(3),
            Symbol::Number(4),
            Symbol::CloseBracket,
        ]
        .into();

        let added = add(&first, &second);

        assert_eq!(
            added,
            vec![
                Symbol::OpenBracket,
                Symbol::OpenBracket,
                Symbol::Number(1),
                Symbol::Number(2),
                Symbol::CloseBracket,
                Symbol::OpenBracket,
                Symbol::Number(3),
                Symbol::Number(4),
                Symbol::CloseBracket,
                Symbol::CloseBracket
            ]
            .into()
        )
    }

    #[test]
    fn test_split() {
        let parsed = number("[[[[0,7],4],[15,[0,13]]],[1,1]]").unwrap().1;

        let after = split(&parsed);
        let expected = number("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]").unwrap().1;

        assert_eq!(after, expected);
    }

    #[test]
    fn test_explode() {
        let parsed = number("[[[[[9,8],1],2],3],4]").unwrap().1;

        let after = explode(&parsed);

        let expected = number("[[[[0,9],2],3],4]").unwrap().1;

        assert_eq!(after, expected);
    }
}
