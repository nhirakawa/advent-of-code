use anyhow::bail;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res},
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};
use std::{collections::HashSet, ops::Neg};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let deltas = parse(input);
    Ok(deltas.iter().sum::<i32>().to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let deltas = parse(input);
    let mut seen = HashSet::new();
    let mut frequency = 0;

    for delta in deltas.iter().cycle() {
        frequency += delta;
        if !seen.insert(frequency) {
            return Ok(frequency.to_string());
        }
    }

    bail!("No answer found")
}

fn parse(i: &str) -> Vec<i32> {
    all_consuming(deltas)(i).unwrap().1
}

fn deltas(i: &str) -> IResult<&str, Vec<i32>> {
    many1(delta)(i)
}

fn delta(i: &str) -> IResult<&str, i32> {
    terminated(alt((positive, negative)), tag("\n"))(i)
}

fn positive(i: &str) -> IResult<&str, i32> {
    map_res(preceded(tag("+"), digit1), |s: &str| s.parse::<i32>())(i)
}

fn negative(i: &str) -> IResult<&str, i32> {
    map_res(preceded(tag("-"), digit1), |s: &str| {
        s.parse::<i32>().map(|i| i.neg())
    })(i)
}
