use std::str::FromStr;

use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-13.txt");

    let packet_pairs = parse(input);

    let part_one = part_one(&packet_pairs);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(packet_pairs: &[(PacketValue, PacketValue)]) -> PartAnswer {
    let start = SystemTime::now();

    let mut sum = 0;

    for (index, (left, right)) in packet_pairs.iter().enumerate() {
        if are_packets_ordered_correctly(left, right) == Comparison::CorrectOrder {
            sum += index + 1;
        }
    }

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(sum, elapsed)
}

fn part_two() -> PartAnswer {
    let start = SystemTime::now();
    let elapsed = start.elapsed().unwrap();

    PartAnswer::default()
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketValue {
    Integer(usize),
    List(Vec<PacketValue>),
}

fn are_packets_ordered_correctly(left: &PacketValue, right: &PacketValue) -> Comparison {
    match (left, right) {
        (PacketValue::Integer(left_value), PacketValue::Integer(right_value)) => {
            if left_value < right_value {
                Comparison::CorrectOrder
            } else if left_value > right_value {
                Comparison::IncorrectOrder
            } else {
                Comparison::Inconclusive
            }
        }
        (PacketValue::List(left_value), PacketValue::List(right_value)) => {
            println!("list and list");
            let mut i = 0;
            let mut j = 0;

            while i < left_value.len() && j < right_value.len() {
                let comparison = are_packets_ordered_correctly(&left_value[i], &right_value[j]);

                if comparison != Comparison::Inconclusive {
                    return comparison;
                }

                i += 1;
                j += 1;
            }

            println!(
                "End of loop - {i}/{}, {j}/{}",
                left_value.len(),
                right_value.len()
            );

            if i == left_value.len() && j == right_value.len() {
                return Comparison::Inconclusive;
            } else if i == left_value.len() {
                return Comparison::CorrectOrder;
            } else {
                return Comparison::IncorrectOrder;
            }
        }
        (PacketValue::List(_), PacketValue::Integer(_)) => {
            are_packets_ordered_correctly(left, &PacketValue::List(vec![right.clone()]))
        }
        (PacketValue::Integer(_), PacketValue::List(_)) => {
            are_packets_ordered_correctly(&PacketValue::List(vec![left.clone()]), right)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Comparison {
    CorrectOrder,
    IncorrectOrder,
    Inconclusive,
}

fn parse(i: &str) -> Vec<(PacketValue, PacketValue)> {
    finish(packets)(i).unwrap().1
}

fn packets(i: &str) -> IResult<&str, Vec<(PacketValue, PacketValue)>> {
    separated_list1(tag("\n\n"), packet_pair)(i)
}

fn packet_pair(i: &str) -> IResult<&str, (PacketValue, PacketValue)> {
    separated_pair(list, tag("\n"), list)(i)
}

fn integer_or_list(i: &str) -> IResult<&str, PacketValue> {
    alt((integer, list))(i)
}

fn integer(i: &str) -> IResult<&str, PacketValue> {
    map(unsigned_number, PacketValue::Integer)(i)
}

fn list(i: &str) -> IResult<&str, PacketValue> {
    delimited(
        tag("["),
        map(
            separated_list0(tag(","), integer_or_list),
            PacketValue::List,
        ),
        tag("]"),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_packets_ordered_correctly_integers() {
        let left = PacketValue::Integer(1);
        let right = PacketValue::Integer(1);
        assert_eq!(
            are_packets_ordered_correctly(&left, &right),
            Comparison::Inconclusive
        );

        let left = PacketValue::Integer(3);
        let right = PacketValue::Integer(5);
        assert_eq!(
            are_packets_ordered_correctly(&left, &right),
            Comparison::CorrectOrder
        );

        let left = PacketValue::Integer(9);
        let right = PacketValue::Integer(8);
        assert_eq!(
            are_packets_ordered_correctly(&left, &right),
            Comparison::IncorrectOrder
        );
    }

    #[test]
    fn test_are_packets_ordered_correctly_lists() {
        let left = list("[1,1,3,1,1]").unwrap().1;
        let right = list("[1,1,5,1,1]").unwrap().1;

        assert_eq!(
            are_packets_ordered_correctly(&left, &right),
            Comparison::CorrectOrder
        );
    }

    #[test]
    fn test_are_packets_ordered_correctly_examples() {
        let left = list("[9]").unwrap().1;
        let right = list("[[8,7,6]]").unwrap().1;
        assert_eq!(
            are_packets_ordered_correctly(&left, &right),
            Comparison::IncorrectOrder
        );

        let left = list("[[4,4],4,4]").unwrap().1;
        let right = list("[[4,4],4,4,4]").unwrap().1;
        assert_eq!(
            are_packets_ordered_correctly(&left, &right),
            Comparison::CorrectOrder
        );

        let left = list("[7,7,7,7]").unwrap().1;
        let right = list("[7,7,7]").unwrap().1;
        assert_eq!(
            are_packets_ordered_correctly(&left, &right),
            Comparison::IncorrectOrder
        );

        let left = list("[]").unwrap().1;
        let right = list("[3]").unwrap().1;
        assert_eq!(
            are_packets_ordered_correctly(&left, &right),
            Comparison::CorrectOrder
        );

        let left = list("[[[]]]").unwrap().1;
        let right = list("[[]]").unwrap().1;
        assert_eq!(
            are_packets_ordered_correctly(&left, &right),
            Comparison::IncorrectOrder
        );

        let left = list("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap().1;
        let right = list("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap().1;
        assert_eq!(
            are_packets_ordered_correctly(&left, &right),
            Comparison::IncorrectOrder
        );
    }

    #[test]
    fn test_integer() {
        assert_eq!(integer("1"), Ok(("", PacketValue::Integer(1))));
    }

    #[test]
    fn test_integer_list() {
        assert_eq!(
            list("[3,1,2]"),
            Ok((
                "",
                PacketValue::List(vec![
                    PacketValue::Integer(3),
                    PacketValue::Integer(1),
                    PacketValue::Integer(2)
                ])
            ))
        )
    }

    #[test]
    fn test_nested_list() {
        assert_eq!(
            list("[[2],[5]]"),
            Ok((
                "",
                PacketValue::List(vec![
                    PacketValue::List(vec![PacketValue::Integer(2)]),
                    PacketValue::List(vec![PacketValue::Integer(5)])
                ])
            ))
        );

        assert_eq!(
            list("[6,[4,1]]"),
            Ok((
                "",
                PacketValue::List(vec![
                    PacketValue::Integer(6),
                    PacketValue::List(vec![PacketValue::Integer(4), PacketValue::Integer(1)])
                ])
            ))
        );
    }

    #[test]
    fn test_packet_examples() {
        assert_eq!(
            list("[[1],[2,3,4]]"),
            Ok((
                "",
                PacketValue::List(vec![
                    PacketValue::List(vec![PacketValue::Integer(1)]),
                    PacketValue::List(vec![
                        PacketValue::Integer(2),
                        PacketValue::Integer(3),
                        PacketValue::Integer(4)
                    ])
                ])
            ))
        );

        assert_eq!(list("[]"), Ok(("", PacketValue::List(vec![]))));

        assert_eq!(
            list("[[]]"),
            Ok(("", PacketValue::List(vec![PacketValue::List(vec![])])))
        );
    }
}
