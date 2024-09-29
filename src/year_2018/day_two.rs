use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use crate::common::answer::*;
use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let parse_start = SystemTime::now();
    let checksums = parse(include_str!("input/day-2.txt"));
    let parsed_elapsed = parse_start.elapsed().unwrap();

    let part_one = part_one(&checksums, &parsed_elapsed);
    let part_two = part_two(&checksums, &parsed_elapsed);

    Ok((part_one, part_two))
}

fn part_one(checksums: &[String], parse_duration: &Duration) -> PartAnswer {
    let start = SystemTime::now();

    let mut number_of_doubles = 0;
    let mut number_of_triples = 0;

    for checksum in checksums {
        let mut counts = HashMap::new();

        for character in checksum.chars() {
            *counts.entry(character).or_insert(0) += 1;
        }

        let mut has_double = false;
        let mut has_triple = false;

        for count in counts.values() {
            if *count == 2 {
                has_double = true;
            } else if *count == 3 {
                has_triple = true;
            }
        }

        if has_double {
            number_of_doubles += 1;
        }

        if has_triple {
            number_of_triples += 1;
        }
    }

    PartAnswer::new(
        number_of_doubles * number_of_triples,
        start.elapsed().unwrap() + *parse_duration,
    )
}

fn part_two(checksums: &[String], parse_duration: &Duration) -> PartAnswer {
    let start = SystemTime::now();

    for outer in checksums {
        for inner in checksums {
            let mut differences = 0;
            let mut same = Vec::new();

            for i in 0..inner.len() {
                let outer_char = outer.get(i..=i).unwrap();
                let inner_char = inner.get(i..=i).unwrap();

                if outer_char != inner_char {
                    differences += 1;
                } else {
                    same.push(inner_char.to_string());
                }
            }

            if differences == 1 {
                return PartAnswer::new(same.join(""), start.elapsed().unwrap() + *parse_duration);
            }
        }
    }

    panic!()
}

fn parse(i: &str) -> Vec<String> {
    all_consuming(checksums)(i).unwrap().1
}

fn checksums(i: &str) -> IResult<&str, Vec<String>> {
    terminated(separated_list1(tag("\n"), checksum), tag("\n"))(i)
}

fn checksum(i: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string())(i)
}
