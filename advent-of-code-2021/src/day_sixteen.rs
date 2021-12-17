use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{map, map_opt, map_res, value},
    multi::many1,
    sequence::tuple,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let part_one = part_one();
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one() -> PartAnswer {
    PartAnswer::default()
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Bit {
    One,
    Zero,
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal {
        version: u8,
        literal: usize,
    },
    Operator {
        version: u8,
        type_id: usize,
        sub_packets: Vec<Packet>,
    },
}

fn literal_packet(i: &str) -> IResult<&str, Packet> {
    map(
        tuple((packet_version, tag("100"), literal_number)),
        |(version, _, literal)| Packet::Literal { version, literal },
    )(i)
}

fn literal_number(i: &str) -> IResult<&str, usize> {
    let continued = map(tuple((tag("1"), take(4_usize))), |(_, num)| num);
    let last = map(tuple((tag("0"), take(4_usize))), |(_, num)| num);

    map_res(tuple((many1(continued), last)), |(first, last)| {
        let mut combined = Vec::new();
        combined.extend(first);
        combined.push(last);
        usize::from_str_radix(&combined.join(""), 2)
    })(i)
}

fn operator_packet(i: &str) -> IResult<&str, &str> {
    todo!()
}

fn packet_version(i: &str) -> IResult<&str, u8> {
    map_res(take(3_usize), |s: &str| u8::from_str_radix(s, 2))(i)
}

fn packet_type_id(i: &str) -> IResult<&str, u8> {
    map_res(take(3_usize), |s: &str| u8::from_str_radix(s, 2))(i)
}

fn all_hex(i: &str) -> IResult<&str, String> {
    map(many1(hex_digit), |h| h.join(""))(i)
}

fn hex_digit(i: &str) -> IResult<&str, String> {
    // let zero = value(vec![Bit::Zero, Bit::Zero, Bit::Zero, Bit::Zero], tag("0"));
    // let one = value(vec![Bit::Zero, Bit::Zero, Bit::Zero, Bit::One], tag("1"));
    // let two = value(vec![Bit::Zero, Bit::Zero, Bit::One, Bit::Zero], tag("2"));
    // let three = value(vec![Bit::Zero, Bit::Zero, Bit::One, Bit::One], tag("3"));
    // let four = value(vec![Bit::Zero, Bit::One, Bit::Zero, Bit::Zero], tag("4"));
    // let five = value(vec![Bit::Zero, Bit::One, Bit::Zero, Bit::One], tag("5"));
    // let six = value(vec![Bit::Zero, Bit::One, Bit::One, Bit::Zero], tag("6"));
    // let seven = value(vec![Bit::Zero, Bit::One, Bit::One, Bit::One], tag("7"));
    // let eight = value(vec![Bit::One, Bit::Zero, Bit::Zero, Bit::Zero], tag("8"));
    // let nine = value(vec![Bit::One, Bit::Zero, Bit::Zero, Bit::One], tag("9"));
    // let a = value(vec![Bit::One, Bit::Zero, Bit::One, Bit::Zero], tag("A"));
    // let b = value(vec![Bit::One, Bit::Zero, Bit::One, Bit::One], tag("B"));
    // let c = value(vec![Bit::One, Bit::One, Bit::Zero, Bit::Zero], tag("C"));
    // let d = value(vec![Bit::One, Bit::One, Bit::Zero, Bit::One], tag("D"));
    // let e = value(vec![Bit::One, Bit::One, Bit::One, Bit::Zero], tag("E"));
    // let f = value(vec![Bit::One, Bit::One, Bit::One, Bit::One], tag("F"));

    map(
        map_res(take(1_usize), |s: &str| u8::from_str_radix(s, 16)),
        |int| format!("{:04b}", int),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_digit() {
        assert_eq!(hex_digit("0"), Ok(("", "0000".into())));
        assert_eq!(hex_digit("1"), Ok(("", "0001".into())));
        assert_eq!(hex_digit("2"), Ok(("", "0010".into())));
        assert_eq!(hex_digit("3"), Ok(("", "0011".into())));
        assert_eq!(hex_digit("4"), Ok(("", "0100".into())));
        assert_eq!(hex_digit("5"), Ok(("", "0101".into())));
        assert_eq!(hex_digit("6"), Ok(("", "0110".into())));
        assert_eq!(hex_digit("7"), Ok(("", "0111".into())));
        assert_eq!(hex_digit("8"), Ok(("", "1000".into())));
        assert_eq!(hex_digit("9"), Ok(("", "1001".into())));
        assert_eq!(hex_digit("A"), Ok(("", "1010".into())));
        assert_eq!(hex_digit("B"), Ok(("", "1011".into())));
        assert_eq!(hex_digit("C"), Ok(("", "1100".into())));
        assert_eq!(hex_digit("D"), Ok(("", "1101".into())));
        assert_eq!(hex_digit("E"), Ok(("", "1110".into())));
        assert_eq!(hex_digit("F"), Ok(("", "1111".into())));
    }

    #[test]
    fn test_all_hex() {
        assert_eq!(
            all_hex("D2FE28"),
            Ok(("", "110100101111111000101000".into()))
        );
        assert_eq!(
            all_hex("38006F45291200"),
            Ok((
                "",
                "00111000000000000110111101000101001010010001001000000000".into()
            ))
        );
        assert_eq!(
            all_hex("EE00D40C823060"),
            Ok((
                "",
                "11101110000000001101010000001100100000100011000001100000".into()
            ))
        );
    }
}
