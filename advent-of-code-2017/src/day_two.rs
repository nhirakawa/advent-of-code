use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, tab},
    combinator::map,
    multi::separated_list1,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-2.txt");

    let rows = parse(input);

    let part_one = part_one(&rows);
    let part_two = part_two(&rows);

    Ok((part_one, part_two))
}

fn part_one(rows: &[Vec<u32>]) -> PartAnswer {
    let start = SystemTime::now();

    let mut sum = 0;

    for row in rows {
        let max = row.iter().max().unwrap();
        let min = row.iter().min().unwrap();

        let difference = max - min;

        sum += difference;
    }

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(sum, elapsed)
}

fn part_two(rows: &[Vec<u32>]) -> PartAnswer {
    let start = SystemTime::now();

    let mut sum = 0;

    for row in rows {
        let mut found_match = false;

        for first in row {
            for second in row {
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

    let elapsed = start.elapsed().unwrap();

    // 61414 too high
    // 428 too high
    PartAnswer::new(sum, elapsed)
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
