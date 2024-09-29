use std::time::SystemTime;
use crate::common::{parse::unsigned_number, answer::*};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-2.txt");
    let commands = parse_commands(input);

    let part_one = part_one(&commands);
    let part_two = part_two(&commands);

    Ok((part_one, part_two))
}

fn part_one(commands: &[Command]) -> PartAnswer {
    let start = SystemTime::now();
    let mut current = (0, 0);

    for command in commands {
        current = match command {
            Command::Forward(unit) => (current.0 + unit, current.1),
            Command::Down(unit) => (current.0, current.1 + unit),
            Command::Up(unit) => (current.0, current.1 - unit),
        }
    }

    let solution = current.0 * current.1;
    let elapsed = start.elapsed().unwrap();
    PartAnswer::new(solution, elapsed)
}

fn part_two(commands: &[Command]) -> PartAnswer {
    let start = SystemTime::now();

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

    let solution = x * y;
    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(solution, elapsed)
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
    all_consuming(terminated(separated_list1(tag("\n"), command), tag("\n")))(i)
}

fn command(i: &str) -> IResult<&str, Command> {
    alt((forward, down, up))(i)
}

fn forward(i: &str) -> IResult<&str, Command> {
    map(preceded(tag("forward "), unsigned_number), Command::Forward)(i)
}

fn down(i: &str) -> IResult<&str, Command> {
    map(preceded(tag("down "), unsigned_number), Command::Down)(i)
}

fn up(i: &str) -> IResult<&str, Command> {
    map(preceded(tag("up "), unsigned_number), Command::Up)(i)
}
