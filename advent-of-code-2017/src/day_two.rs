use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, not_line_ending, space1, tab},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-2.txt");

    let rows = parse(input);

    let part_one = part_one(&rows);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(rows: &[Vec<u32>]) -> PartAnswer {
    let start = SystemTime::now();

    let mut sum = 0;

    for row in rows {
        let max = row.iter().max().unwrap();
        let min = row.iter().min().unwrap();

        let difference = max - min;

        println!("{max}-{min}={difference}");

        sum += difference;
    }

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(sum, elapsed)
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn parse(i: &str) -> Vec<Vec<u32>> {
    finish(rows)(i).unwrap().1
}

fn rows(i: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(tag("\n"), row)(i)
}

fn row(i: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(separator, unsigned_number)(i)
}

fn separator(i: &str) -> IResult<&str, String> {
    alt((
        map(tab, |c: char| c.to_string()),
        map(space1, |s: &str| s.into()),
    ))(i)
}
