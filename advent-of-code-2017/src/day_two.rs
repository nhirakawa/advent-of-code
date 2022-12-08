use common::prelude::*;
use nom::{
    bytes::complete::tag, character::complete::multispace1, multi::separated_list1, IResult,
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

        sum += max - min;
    }

    let elapsed = start.elapsed().unwrap();

    // 5626 too low
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
    separated_list1(multispace1, unsigned_number)(i)
}
