use crate::common::parse::finish;
use anyhow::bail;
use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::many1, IResult, Parser};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let list_of_parens = parse(input);
    Ok(list_of_parens.iter().map(Parens::value).sum::<i64>())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let list_of_parens = parse(input);
    let mut sum: i64 = 0;

    for (index, parens) in list_of_parens.iter().enumerate() {
        sum += parens.value();
        if sum < 0 {
            return Ok(index + 1);
        }
    }

    bail!("No answer found")
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Parens {
    Open,
    Close,
}

impl Parens {
    fn value(&self) -> i64 {
        match self {
            Parens::Open => 1,
            Parens::Close => -1,
        }
    }
}

fn parse(i: &str) -> Vec<Parens> {
    finish(list_of_parens, i).unwrap()
}

fn list_of_parens(i: &str) -> IResult<&str, Vec<Parens>> {
    many1(parens).parse(i)
}

fn parens(i: &str) -> IResult<&str, Parens> {
    alt((open, close)).parse(i)
}

fn open(i: &str) -> IResult<&str, Parens> {
    value(Parens::Open, tag("(")).parse(i)
}

fn close(i: &str) -> IResult<&str, Parens> {
    value(Parens::Close, tag(")")).parse(i)
}
