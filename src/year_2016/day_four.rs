use std::collections::HashMap;

use anyhow::bail;
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::alpha1, combinator::map, multi::separated_list1,
    IResult, Parser,
};

use crate::common::parse::finish;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut sum_of_sector_ids = 0;

    for line in input.trim().lines() {
        match parse_room(line) {
            Ok(room) => {
                if room.is_real() {
                    sum_of_sector_ids += room.sector_id;
                }
            }
            Err(e) => {
                bail!("Failed to parse room: {}", e);
            }
        }
    }

    Ok(sum_of_sector_ids)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let rooms = input
        .trim()
        .lines()
        .filter_map(|line| parse_room(line).ok())
        .collect_vec();

    for room in rooms {
        let decrypted_name = room
            .encrypted_name
            .chars()
            .map(|c| {
                if c == '-' {
                    ' '
                } else {
                    let c = c as u8 - b'a';
                    let c = (c as u32 + room.sector_id) % 26;
                    (c + b'a' as u32) as u8 as char
                }
            })
            .collect::<String>();

        if decrypted_name == "northpole object storage" {
            return Ok(room.sector_id);
        }
    }

    bail!("Could not find room with name 'northpole object storage'");
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Room {
    encrypted_name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn is_real(&self) -> bool {
        let mut char_counts: HashMap<_, usize> = HashMap::new();

        for c in self.encrypted_name.chars() {
            if c == '-' {
                continue;
            }

            *char_counts.entry(c).or_default() += 1;
        }

        let mut char_counts: Vec<_> = char_counts.into_iter().collect();
        char_counts.sort_by(|(first_char, first_count), (second_char, second_count)| {
            first_count
                .cmp(second_count)
                .reverse()
                .then(first_char.cmp(second_char))
        });

        let expected_checksum = char_counts
            .iter()
            .take(5)
            .map(|(c, _)| c.to_string())
            .collect::<String>();

        expected_checksum == self.checksum
    }
}

fn parse_room(i: &str) -> anyhow::Result<Room> {
    finish(room, i)
}

fn room(i: &str) -> IResult<&str, Room> {
    map(
        (
            encrypted_name,
            tag("-"),
            sector_id,
            tag("["),
            checksum,
            tag("]"),
        ),
        |(encrypted_name, _, sector_id, _, checksum, _)| Room {
            encrypted_name,
            sector_id,
            checksum,
        },
    )
    .parse(i)
}

fn checksum(i: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string()).parse(i)
}

fn sector_id(i: &str) -> IResult<&str, u32> {
    nom::character::complete::u32(i)
}

fn encrypted_name(i: &str) -> IResult<&str, String> {
    map(separated_list1(tag("-"), alpha1), |parts| parts.join("-")).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_room() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]";
        let expected = Room {
            encrypted_name: "aaaaa-bbb-z-y-x".to_string(),
            sector_id: 123,
            checksum: "abxyz".to_string(),
        };

        assert_eq!(parse_room(input).unwrap(), expected);
    }

    #[test]
    fn test_room_is_real() {
        let room = Room {
            encrypted_name: "aaaaa-bbb-z-y-x".to_string(),
            sector_id: 123,
            checksum: "abxyz".to_string(),
        };

        assert!(room.is_real());
    }
}
