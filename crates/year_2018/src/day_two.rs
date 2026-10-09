use anyhow::bail;
use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::terminated,
    IResult, Parser,
};
use std::collections::HashMap;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let checksums = parse(input);
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

    Ok(number_of_doubles * number_of_triples)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let checksums = parse(input);
    for outer in &checksums {
        for inner in &checksums {
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
                return Ok(same.join(""));
            }
        }
    }

    bail!("No answer found")
}

fn parse(i: &str) -> Vec<String> {
    all_consuming(checksums).parse(i).unwrap().1
}

fn checksums(i: &str) -> IResult<&str, Vec<String>> {
    terminated(separated_list1(tag("\n"), checksum), tag("\n")).parse(i)
}

fn checksum(i: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string()).parse(i)
}
