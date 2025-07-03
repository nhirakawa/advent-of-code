use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

use crate::common::parse::{finish, unsigned_number};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let instructions = parse(input)?;

    let mut grid = HashSet::new();

    for instruction in instructions {
        let action = instruction.action;

        for coordinate in instruction.all_coordinates() {
            match action {
                Action::TurnOn => {
                    grid.insert(coordinate);
                }
                Action::TurnOff => {
                    grid.remove(&coordinate);
                }
                Action::Toggle => {
                    if grid.contains(&coordinate) {
                        grid.remove(&coordinate);
                    } else {
                        grid.insert(coordinate);
                    }
                }
            }
        }
    }

    Ok(grid.len())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let instructions = parse(input)?;

    let mut grid = Grid::new();

    for instruction in instructions {
        let action = instruction.action;

        for coordinate in instruction.all_coordinates() {
            match action {
                Action::TurnOn => {
                    grid.turn_on(coordinate);
                }
                Action::TurnOff => {
                    grid.turn_off(coordinate);
                }
                Action::Toggle => {
                    grid.toggle(coordinate);
                }
            }
        }
    }

    Ok(grid.brightness())
}

struct Grid {
    inner: HashMap<Coordinate, u8>,
}

impl Grid {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn turn_on(&mut self, coordinate: Coordinate) {
        let entry = self.inner.entry(coordinate).or_insert(0);
        *entry += 1;
    }

    fn turn_off(&mut self, coordinate: Coordinate) {
        let entry = self.inner.entry(coordinate).or_insert(0);
        if *entry > 0 {
            *entry -= 1;
        }
    }

    fn toggle(&mut self, coordinate: Coordinate) {
        let entry = self.inner.entry(coordinate).or_insert(0);
        *entry += 2;
    }

    fn brightness(&self) -> usize {
        self.inner.values().map(|&v| v as usize).sum()
    }
}

type Coordinate = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    action: Action,
    start: Coordinate,
    end: Coordinate,
}

impl Instruction {
    fn new(action: Action, start: Coordinate, end: Coordinate) -> Self {
        Self { action, start, end }
    }

    fn all_coordinates(&self) -> impl IntoIterator<Item = Coordinate> {
        let (start_x, start_y) = self.start;
        let (end_x, end_y) = self.end;
        (start_x..=end_x).flat_map(move |x| (start_y..=end_y).map(move |y| (x, y)))
    }
}

type Instructions = Vec<Instruction>;

fn parse(i: &str) -> anyhow::Result<Instructions> {
    finish(instructions, i)
}

fn instructions(i: &str) -> IResult<&str, Instructions> {
    separated_list1(tag("\n"), instruction).parse(i)
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        (action, tag(" "), coordinate, tag(" through "), coordinate),
        |(action, _, start, _, end)| Instruction::new(action, start, end),
    )
    .parse(i)
}

fn coordinate(i: &str) -> IResult<&str, Coordinate> {
    separated_pair(unsigned_number, tag(","), unsigned_number).parse(i)
}

fn action(i: &str) -> IResult<&str, Action> {
    alt((turn_on, turn_off, toggle)).parse(i)
}

fn turn_on(i: &str) -> IResult<&str, Action> {
    value(Action::TurnOn, tag("turn on")).parse(i)
}

fn turn_off(i: &str) -> IResult<&str, Action> {
    value(Action::TurnOff, tag("turn off")).parse(i)
}

fn toggle(i: &str) -> IResult<&str, Action> {
    value(Action::Toggle, tag("toggle")).parse(i)
}
