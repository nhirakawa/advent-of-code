use crate::common::parse::{finish, unsigned_number};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, tab},
    combinator::value,
    multi::separated_list1,
    IResult, Parser,
};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let rows = parse(input);
    let mut sum = 0;

    for row in rows {
        let max = row.iter().max().unwrap();
        let min = row.iter().min().unwrap();

        let difference = max - min;

        sum += difference;
    }

    Ok(sum)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let rows = parse(input);
    let mut sum = 0;

    for row in rows {
        let mut found_match = false;

        for first in &row {
            for second in &row {
                if first == second {
                    continue;
                }

                if found_match {
                    continue;
                }

                if first > second && first % second == 0 {
                    let division = first / second;
                    sum += division;
                    found_match = true;
                } else if second > first && second % first == 0 {
                    let division = second / first;
                    sum += division;
                    found_match = true;
                }
            }
        }
    }

    Ok(sum)
}

fn parse(i: &str) -> Vec<Vec<u32>> {
    finish(rows, i).unwrap()
}

fn rows(i: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(tag("\n"), row).parse(i)
}

fn row(i: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(separator, unsigned_number).parse(i)
}

fn separator(i: &str) -> IResult<&str, ()> {
    let tabs = value((), tab);
    let spaces = value((), space1);
    alt((tabs, spaces)).parse(i)
}
