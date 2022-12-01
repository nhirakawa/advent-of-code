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
    let part_two = part_two(&list_of_calories);

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

fn part_two(list_of_calories: &[Vec<u64>]) -> PartAnswer {
    let start = SystemTime::now();

    let mut sorted_calories_sums: Vec<u64> =
        list_of_calories.iter().map(|l| l.iter().sum()).collect();
    sorted_calories_sums.sort();
    let top_3_summed_calories: u64 = sorted_calories_sums.iter().rev().take(3).sum();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(top_3_summed_calories, elapsed)
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
