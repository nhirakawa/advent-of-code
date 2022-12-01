use common::parse::unsigned_number;
use common::prelude::*;
use nom::bytes::complete::tag;
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::IResult;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-1.txt");

    let list_of_calories = parse(input);

    let part_one = part_one(&list_of_calories);
    let part_two = PartAnswer::default();

    Ok((part_one, part_two))
}

fn part_one(list_of_calories: &[Vec<u64>]) -> PartAnswer {
    let start = SystemTime::now();

    let answer: u64 = list_of_calories
        .iter()
        .map(|l| l.iter().sum())
        .max()
        .unwrap();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn parse(i: &str) -> Vec<Vec<u64>> {
    all_consuming(elves_list)(i).unwrap().1
}

fn elves_list(i: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(tag("\n\n"), calories_list)(i)
}

fn calories_list(i: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag("\n"), unsigned_number)(i)
}
