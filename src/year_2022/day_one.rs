use crate::common::parse::unsigned_number;
use anyhow::anyhow;
use nom::bytes::complete::tag;
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::IResult;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let list_of_calories = parse(input);

    list_of_calories
        .iter()
        .map(|l| l.iter().sum::<u64>())
        .max()
        .map(|u| u.to_string())
        .ok_or(anyhow!("No max found"))
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let list_of_calories = parse(input);

    let mut sorted_calories_sums: Vec<u64> =
        list_of_calories.iter().map(|l| l.iter().sum()).collect();
    sorted_calories_sums.sort_unstable();

    Ok(sorted_calories_sums
        .iter()
        .rev()
        .take(3)
        .sum::<u64>()
        .to_string())
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
