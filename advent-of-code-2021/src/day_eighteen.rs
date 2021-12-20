use std::fmt::Display;

use common::{parse::unsigned_number, prelude::*};
use log::debug;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, into, map, value},
    multi::{many0, many1, separated_list1},
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

fn iterated_add(numbers: &[Number]) -> Number {
    let first = &numbers[0];
    let second = &numbers[1];

    let mut out = add(first, second);

    for i in 2..numbers.len() {
        out = add(&out, &numbers[i]);
    }

    out
}

fn add(first: &Number, second: &Number) -> Number {
    // add 2 to combined lengths for open + close brackets
    let new_length = first.len() + second.len() + 2;

    let mut number = Vec::with_capacity(new_length);

    number.push(Symbol::OpenBracket);

    number.extend(first.symbols.iter().copied());
    number.push(Symbol::Comma);
    number.extend(second.symbols.iter().copied());

    number.push(Symbol::CloseBracket);

    let number: Number = number.into();

    let mut before = number.clone();

    loop {
        let after = explode(&before);
        let after = split(&after);

        if before == after {
            return before;
        }

        // println!("before {:?}", before);
        // println!("after: {:?}", after);

        before = after;
    }
}

fn explode(number: &Number) -> Number {
    let mut result = Vec::new();

    let mut current_depth = 0;
    let mut carryover = 0;
    let mut should_check_left = true;
    let mut last_placed = Symbol::Comma;

    for (index, symbol) in number.symbols.iter().enumerate() {
        if let Symbol::Comma = symbol {
            if last_placed != Symbol::Comma {
                result.push(*symbol);
                last_placed = Symbol::Comma;
            }
        } else if let Symbol::OpenBracket = symbol {
            current_depth += 1;
            if current_depth <= 4 {
                result.push(*symbol);
                last_placed = Symbol::OpenBracket;
            }
        } else if let Symbol::CloseBracket = symbol {
            debug!("current depth on close {}", current_depth);
            if current_depth <= 4 {
                result.push(*symbol);
                last_placed = Symbol::CloseBracket;
            }
            current_depth -= 1;
        } else if let Symbol::Number(current_number) = symbol {
            if current_depth > 4 {
                if should_check_left {
                    debug!("searching number to the left");

                    let mut stack = Vec::new();

                    // look back to find a number
                    while let Some(popped) = result.pop() {
                        debug!("checking {}", popped);
                        if let Symbol::Number(left_number) = popped {
                            let new_left_number = left_number + current_number;
                            debug!(
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

                    // push a 0 for current exploded pair
                    result.push(Symbol::Number(0));
                    if last_placed != Symbol::Comma {
                        result.push(Symbol::Comma);
                        last_placed = Symbol::Comma;
                    }

                    should_check_left = false;
                } else {
                    // carry current number to next number
                    carryover = *current_number;
                    should_check_left = true;
                }
            } else {
                let current_number = current_number + carryover;
                carryover = 0;

                result.push(Symbol::Number(current_number));
                last_placed = Symbol::Number(0);
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
                result.push(Symbol::Comma);
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

fn magnitude(number: &Number) -> u64 {
    let mut magnitude = 0;

    let mut symbol_multiplier = 1;

    for symbol in &number.symbols {
        match symbol {
            Symbol::OpenBracket => symbol_multiplier *= 3,
            Symbol::CloseBracket => symbol_multiplier /= 2,
            Symbol::Comma => symbol_multiplier = (symbol_multiplier * 2) / 3,
            Symbol::Number(number) => magnitude += *number as u64 * symbol_multiplier as u64,
        };
    }

    magnitude
}

#[derive(PartialEq, Clone)]
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
    Comma,
    Number(u8),
}

impl core::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenBracket => write!(f, "["),
            Self::CloseBracket => write!(f, "]"),
            Self::Comma => write!(f, ","),
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
    separated_list1(tag("\n"), number)(i)
}

fn number(i: &str) -> IResult<&str, Number> {
    into(many1(symbol))(i)
}

fn symbol(i: &str) -> IResult<&str, Symbol> {
    alt((open_bracket, regular_number, comma, close_bracket))(i)
}

fn open_bracket(i: &str) -> IResult<&str, Symbol> {
    value(Symbol::OpenBracket, tag("["))(i)
}

fn regular_number(i: &str) -> IResult<&str, Symbol> {
    map(unsigned_number, Symbol::Number)(i)
}

fn comma(i: &str) -> IResult<&str, Symbol> {
    value(Symbol::Comma, tag(","))(i)
}

fn close_bracket(i: &str) -> IResult<&str, Symbol> {
    value(Symbol::CloseBracket, tag("]"))(i)
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
    fn test_add_number_simple() {
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
    fn test_add_number_example() {
        let first = number("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap().1;
        let second = number("[1,1]").unwrap().1;

        let added = add(&first, &second);
        let expected = number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap().1;
        assert_eq!(added, expected);
    }

    #[test]
    fn test_magnitude() {
        let parsed = number("[[1,2],[[3,4],5]]").unwrap().1;
        assert_eq!(magnitude(&parsed), 143);

        let parsed = number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap().1;
        assert_eq!(magnitude(&parsed), 1384);

        let parsed = number("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap().1;
        assert_eq!(magnitude(&parsed), 445);

        let parsed = number("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap().1;
        assert_eq!(magnitude(&parsed), 791);

        let parsed = number("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap().1;
        assert_eq!(magnitude(&parsed), 1137);

        let parsed = number("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
            .unwrap()
            .1;
        assert_eq!(magnitude(&parsed), 3488);
    }

    #[test]
    fn test_example() {
        let numbers = parse_symbols("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]");
        println!("{} numbers", numbers.len());

        let sum = iterated_add(&numbers);
        let expected_sum = number("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
            .unwrap()
            .1;

        assert_eq!(sum, expected_sum);

        let magnitude = magnitude(&sum);

        assert_eq!(magnitude, 4140);
    }

    #[test]
    fn test_add_iterated() {
        let numbers = parse_symbols("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]");
        let sum = iterated_add(&numbers);
        let expected_sum = number("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap().1;
        assert_eq!(sum, expected_sum);
    }

    #[test]
    fn test_larger_example() {
        let first = number("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
            .unwrap()
            .1;
        let second = number("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]").unwrap().1;

        let sum = add(&first, &second);

        let expected_sum = number("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
            .unwrap()
            .1;

        assert_eq!(sum, expected_sum);
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

        let parsed = number("[7,[6,[5,[4,[3,2]]]]]").unwrap().1;
        let after = explode(&parsed);
        let expected = number("[7,[6,[5,[7,0]]]]").unwrap().1;
        assert_eq!(after, expected);

        let parsed = number("[[6,[5,[4,[3,2]]]],1]").unwrap().1;
        let after = explode(&parsed);
        let expected = number("[[6,[5,[7,0]]],3]").unwrap().1;
        assert_eq!(after, expected);

        let parsed = number("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap().1;
        let after = explode(&parsed);
        let expected = number("[[[[0,7],4],[15,[0,13]]],[1,1]]").unwrap().1;
        assert_eq!(after, expected);
    }
}
