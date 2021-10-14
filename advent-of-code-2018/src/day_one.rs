use std::{collections::HashSet, ops::Neg};

use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map_res},
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let initial_start = SystemTime::now();
    let deltas = parse(include_str!("../input/day-1.txt"));

    let initial_elapsed = initial_start.elapsed().unwrap();

    let part_one = part_one(&deltas, &initial_elapsed);
    let part_two = part_two(&deltas, &initial_elapsed);

    Ok((part_one, part_two))
}

fn part_one(deltas: &[i32], pre_timing: &Duration) -> PartAnswer {
    let start = SystemTime::now();
    let solution: i32 = deltas.iter().sum();
    let elapsed = start.elapsed().unwrap() + *pre_timing;

    PartAnswer::new(solution, elapsed)
}

fn part_two(deltas: &[i32], pre_timing: &Duration) -> PartAnswer {
    let start = SystemTime::now();
    let mut seen = HashSet::new();
    let mut frequency = 0;

    for delta in deltas.iter().cycle() {
        frequency += delta;
        if !seen.insert(frequency) {
            return PartAnswer::new(frequency, start.elapsed().unwrap() + *pre_timing);
        }
    }

    panic!()
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
