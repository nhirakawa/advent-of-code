use common::prelude::*;
use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::many1, IResult};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-1.txt");
    let list_of_parens = parse(input);

    let part_one = part_one(&list_of_parens);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(list_of_parens: &[Parens]) -> PartAnswer {
    let start = SystemTime::now();

    let answer: isize = list_of_parens.iter().map(Parens::value).sum();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Parens {
    Open,
    Close,
}

impl Parens {
    fn value(&self) -> isize {
        match self {
            Parens::Open => 1,
            Parens::Close => -1,
        }
    }
}

fn parse(i: &str) -> Vec<Parens> {
    finish(list_of_parens)(i).unwrap().1
}

fn list_of_parens(i: &str) -> IResult<&str, Vec<Parens>> {
    many1(parens)(i)
}

fn parens(i: &str) -> IResult<&str, Parens> {
    alt((open, close))(i)
}

fn open(i: &str) -> IResult<&str, Parens> {
    value(Parens::Open, tag("("))(i)
}

fn close(i: &str) -> IResult<&str, Parens> {
    value(Parens::Close, tag(")"))(i)
}
