use common::{parse::ParseResult, prelude::*};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::multispace0,
    combinator::{all_consuming, map, map_res},
    multi::{length_count, length_value, many0, many1},
    sequence::{preceded, terminated, tuple},
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-16.txt");
    let parse_start = SystemTime::now();
    let packets = parse_packets(input);
    let parse_elapsed = parse_start.elapsed().unwrap();

    let part_one = part_one(&packets, &parse_elapsed);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(packets: &[Packet], parse_duration: &Duration) -> PartAnswer {
    let start = SystemTime::now();

    let answer = sum_packet_versions(packets);

    let elapsed = start.elapsed().unwrap() + *parse_duration;

    PartAnswer::new(answer, elapsed)
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn sum_packet_versions(packets: &[Packet]) -> usize {
    packets.iter().map(Packet::version_sum).sum()
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
        type_id: u8,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    fn version_sum(&self) -> usize {
        match &self {
            Packet::Literal {
                version,
                literal: _,
            } => *version as usize,
            Packet::Operator {
                version,
                type_id: _,
                sub_packets,
            } => *version as usize + sub_packets.iter().map(Packet::version_sum).sum::<usize>(),
        }
    }
}

fn parse_packets(i: &str) -> Vec<Packet> {
    let parsed_hex = all_consuming(terminated(all_hex, multispace0))(i)
        .unwrap()
        .1;

    let result = all_consuming(terminated(packets, many0(tag("0"))))(&parsed_hex);

    result.unwrap().1
}

fn packets(i: &str) -> ParseResult<Vec<Packet>> {
    many1(packet)(i)
}

fn packet(i: &str) -> ParseResult<Packet> {
    alt((literal_packet, operator_packet))(i)
}

fn literal_packet(i: &str) -> ParseResult<Packet> {
    map(
        tuple((packet_version, tag("100"), literal_number)),
        |(version, _, literal)| Packet::Literal { version, literal },
    )(i)
}

fn literal_number(i: &str) -> ParseResult<usize> {
    let continued = map(tuple((tag("1"), take(4_usize))), |(_, num)| num);
    let last = map(tuple((tag("0"), take(4_usize))), |(_, num)| num);

    map_res(tuple((many0(continued), last)), |(first, last)| {
        let mut combined = Vec::new();
        combined.extend(first);
        combined.push(last);
        usize::from_str_radix(&combined.join(""), 2)
    })(i)
}

fn operator_packet(i: &str) -> ParseResult<Packet> {
    let sub_packets = alt((length_based_sub_packet, count_based_sub_packet));

    let combined = tuple((packet_version, packet_type_id, sub_packets));

    map(combined, |(version, type_id, sub_packets)| {
        Packet::Operator {
            version,
            type_id,
            sub_packets,
        }
    })(i)
}

fn length_based_sub_packet(i: &str) -> ParseResult<Vec<Packet>> {
    // take 15 bits and extract to usize
    let length_parser = map_res(take(15_usize), |s: &str| usize::from_str_radix(s, 2));

    // check for leading 0, read number of bits from length_parser, parse into packets
    preceded(tag("0"), length_value(length_parser, many1(packet)))(i)
}

fn count_based_sub_packet(i: &str) -> ParseResult<Vec<Packet>> {
    // take 11 bits and extract to usize
    let count_parser = map_res(take(11_usize), |s: &str| usize::from_str_radix(s, 2));

    // check for leading 1, read number of packets from count_parser, parse into packets
    preceded(tag("1"), length_count(count_parser, packet))(i)
}

fn packet_version(i: &str) -> ParseResult<u8> {
    map_res(take(3_usize), |s: &str| u8::from_str_radix(s, 2))(i)
}

fn packet_type_id(i: &str) -> ParseResult<u8> {
    map_res(take(3_usize), |s: &str| u8::from_str_radix(s, 2))(i)
}

fn all_hex(i: &str) -> ParseResult<String> {
    map(many1(hex_digit), |h| h.join(""))(i)
}

fn hex_digit(i: &str) -> ParseResult<String> {
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
    use nom::{multi::many0, sequence::terminated};

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

    #[test]
    fn test_literal_packet() {
        assert_eq!(
            terminated(literal_packet, many0(tag("0")))("110100101111111000101000"),
            Ok((
                "",
                Packet::Literal {
                    version: 6,
                    literal: 2021
                }
            ))
        );

        assert_eq!(
            literal_packet("11010001010"),
            Ok((
                "",
                Packet::Literal {
                    version: 6,
                    literal: 10
                }
            ))
        );
        assert_eq!(
            literal_packet("0101001000100100"),
            Ok((
                "",
                Packet::Literal {
                    version: 2,
                    literal: 20
                }
            ))
        );
    }

    #[test]
    fn test_length_based_sub_packet() {
        let sub_packets = length_based_sub_packet("0000000000011011110100010100101001000100100")
            .unwrap()
            .1;

        assert_eq!(
            sub_packets,
            vec![
                Packet::Literal {
                    version: 6,
                    literal: 10,
                },
                Packet::Literal {
                    version: 2,
                    literal: 20,
                },
            ],
        );
    }

    #[test]
    fn test_operator_packet() {
        let mut parser = terminated(operator_packet, many0(tag("0")));
        let expected = Packet::Operator {
            version: 1,
            type_id: 6,
            sub_packets: vec![
                Packet::Literal {
                    version: 6,
                    literal: 10,
                },
                Packet::Literal {
                    version: 2,
                    literal: 20,
                },
            ],
        };

        assert_eq!(
            parser("00111000000000000110111101000101001010010001001000000000"),
            Ok(("", expected))
        );

        let expected = Packet::Operator {
            version: 7,
            type_id: 3,
            sub_packets: vec![
                Packet::Literal {
                    version: 2,
                    literal: 1,
                },
                Packet::Literal {
                    version: 4,
                    literal: 2,
                },
                Packet::Literal {
                    version: 1,
                    literal: 3,
                },
            ],
        };
        assert_eq!(
            parser("11101110000000001101010000001100100000100011000001100000"),
            Ok(("", expected))
        );
    }

    #[test]
    fn test_version_sum() {
        let packets = parse_packets("8A004A801A8002F478");
        assert_eq!(sum_packet_versions(&packets), 16);

        let packets = parse_packets("620080001611562C8802118E34");
        assert_eq!(sum_packet_versions(&packets), 12);

        let packets = parse_packets("C0015000016115A2E0802F182340");
        assert_eq!(sum_packet_versions(&packets), 23);

        let packets = parse_packets("A0016C880162017C3686B18A3D4780");
        assert_eq!(sum_packet_versions(&packets), 31);
    }
}
