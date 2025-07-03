use crate::common::parse::unsigned_number;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult, Parser,
};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let commands = parse_commands(input);

    let mut current = (0, 0);

    for command in commands {
        current = match command {
            Command::Forward(unit) => (current.0 + unit, current.1),
            Command::Down(unit) => (current.0, current.1 + unit),
            Command::Up(unit) => (current.0, current.1 - unit),
        }
    }

    Ok(current.0 * current.1)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let commands = parse_commands(input);

    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;

    for command in commands {
        match command {
            Command::Down(unit) => aim += unit,
            Command::Up(unit) => aim -= unit,
            Command::Forward(unit) => {
                x += unit;
                y += unit * aim;
            }
        };
    }

    Ok(x * y)
}

enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

fn parse_commands(i: &str) -> Vec<Command> {
    commands(i).unwrap().1
}

fn commands(i: &str) -> IResult<&str, Vec<Command>> {
    all_consuming(terminated(separated_list1(tag("\n"), command), tag("\n"))).parse(i)
}

fn command(i: &str) -> IResult<&str, Command> {
    alt((forward, down, up)).parse(i)
}

fn forward(i: &str) -> IResult<&str, Command> {
    map(preceded(tag("forward "), unsigned_number), Command::Forward).parse(i)
}

fn down(i: &str) -> IResult<&str, Command> {
    map(preceded(tag("down "), unsigned_number), Command::Down).parse(i)
}

fn up(i: &str) -> IResult<&str, Command> {
    map(preceded(tag("up "), unsigned_number), Command::Up).parse(i)
}
