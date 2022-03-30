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
    let part_two = part_two(&packets, &parse_elapsed);

    Ok((part_one, part_two))
}

fn part_one(packets: &[Packet], parse_duration: &Duration) -> PartAnswer {
    let start = SystemTime::now();

    let answer = sum_packet_versions(packets);

    let elapsed = start.elapsed().unwrap() + *parse_duration;

    PartAnswer::new(answer, elapsed)
}

fn part_two(packets: &[Packet], parse_duration: &Duration) -> PartAnswer {
    let start = SystemTime::now();

    let answer = evaluate(packets);

    let elapsed = start.elapsed().unwrap() + *parse_duration;

    PartAnswer::new(answer, elapsed)
}

fn sum_packet_versions(packets: &[Packet]) -> usize {
    packets.iter().map(Packet::version_sum).sum()
}

fn evaluate(packets: &[Packet]) -> usize {
    packets.iter().map(Packet::value).sum()
}

#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Literal {
        version: u8,
        literal: usize,
    },
    Sum {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    Product {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    Minimum {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    Maximum {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    GreaterThan {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    LessThan {
        version: u8,
        sub_packets: Vec<Packet>,
    },
    EqualTo {
        version: u8,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    fn version_sum(&self) -> usize {
        self.get_version() as usize
            + self
                .get_sub_packets()
                .iter()
                .map(Packet::version_sum)
                .sum::<usize>()
    }

    fn get_version(&self) -> u8 {
        *match &self {
            Packet::Literal { version, .. } => version,
            Packet::Sum { version, .. } => version,
            Packet::Product { version, .. } => version,
            Packet::Minimum { version, .. } => version,
            Packet::Maximum { version, .. } => version,
            Packet::GreaterThan { version, .. } => version,
            Packet::LessThan { version, .. } => version,
            Packet::EqualTo { version, .. } => version,
        }
    }

    fn get_sub_packets(&self) -> Vec<Packet> {
        match &self {
            Packet::Literal { .. } => vec![],
            Packet::Sum { sub_packets, .. } => sub_packets.clone(),
            Packet::Product { sub_packets, .. } => sub_packets.clone(),
            Packet::Minimum { sub_packets, .. } => sub_packets.clone(),
            Packet::Maximum { sub_packets, .. } => sub_packets.clone(),
            Packet::GreaterThan { sub_packets, .. } => sub_packets.clone(),
            Packet::LessThan { sub_packets, .. } => sub_packets.clone(),
            Packet::EqualTo { sub_packets, .. } => sub_packets.clone(),
        }
    }

    fn value(&self) -> usize {
        match &self {
            Packet::Literal { literal, .. } => *literal,
            Packet::Sum { sub_packets, .. } => sub_packets.iter().map(Packet::value).sum(),
            Packet::Product { sub_packets, .. } => sub_packets.iter().map(Packet::value).product(),
            Packet::Minimum { sub_packets, .. } => {
                sub_packets.iter().map(Packet::value).min().unwrap()
            }
            Packet::Maximum { sub_packets, .. } => {
                sub_packets.iter().map(Packet::value).max().unwrap()
            }
            Packet::GreaterThan { sub_packets, .. } => {
                let first = &sub_packets[0];
                let second = &sub_packets[1];

                if first.value() > second.value() {
                    1
                } else {
                    0
                }
            }
            Packet::LessThan { sub_packets, .. } => {
                let first = &sub_packets[0];
                let second = &sub_packets[1];

                if first.value() < second.value() {
                    1
                } else {
                    0
                }
            }
            Packet::EqualTo { sub_packets, .. } => {
                let first = &sub_packets[0];
                let second = &sub_packets[1];

                if first.value() == second.value() {
                    1
                } else {
                    0
                }
            }
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
    alt((
        sum_packet,
        product_packet,
        minimum_packet,
        maximum_packet,
        greater_than_packet,
        less_than_packet,
        equal_to_packet,
    ))(i)
}

fn sum_packet(i: &str) -> ParseResult<Packet> {
    map(
        tuple((packet_version, tag("000"), sub_packets)),
        |(version, _, sub_packets)| Packet::Sum {
            version,
            sub_packets,
        },
    )(i)
}

fn product_packet(i: &str) -> ParseResult<Packet> {
    map(
        tuple((packet_version, tag("001"), sub_packets)),
        |(version, _, sub_packets)| Packet::Product {
            version,
            sub_packets,
        },
    )(i)
}

fn minimum_packet(i: &str) -> ParseResult<Packet> {
    map(
        tuple((packet_version, tag("010"), sub_packets)),
        |(version, _, sub_packets)| Packet::Minimum {
            version,
            sub_packets,
        },
    )(i)
}

fn maximum_packet(i: &str) -> ParseResult<Packet> {
    map(
        tuple((packet_version, tag("011"), sub_packets)),
        |(version, _, sub_packets)| Packet::Maximum {
            version,
            sub_packets,
        },
    )(i)
}

fn greater_than_packet(i: &str) -> ParseResult<Packet> {
    map(
        tuple((packet_version, tag("101"), sub_packets)),
        |(version, _, sub_packets)| Packet::GreaterThan {
            version,
            sub_packets,
        },
    )(i)
}

fn less_than_packet(i: &str) -> ParseResult<Packet> {
    map(
        tuple((packet_version, tag("110"), sub_packets)),
        |(version, _, sub_packets)| Packet::LessThan {
            version,
            sub_packets,
        },
    )(i)
}

fn equal_to_packet(i: &str) -> ParseResult<Packet> {
    map(
        tuple((packet_version, tag("111"), sub_packets)),
        |(version, _, sub_packets)| Packet::EqualTo {
            version,
            sub_packets,
        },
    )(i)
}

fn sub_packets(i: &str) -> ParseResult<Vec<Packet>> {
    alt((length_based_sub_packet, count_based_sub_packet))(i)
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

fn all_hex(i: &str) -> ParseResult<String> {
    map(many1(hex_digit), |h| h.join(""))(i)
}

fn hex_digit(i: &str) -> ParseResult<String> {
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
        let expected = Packet::LessThan {
            version: 1,
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

        let expected = Packet::Maximum {
            version: 7,
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
