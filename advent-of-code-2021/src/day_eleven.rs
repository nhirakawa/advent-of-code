use std::collections::HashMap;

use common::{parse::unsigned_number, prelude::*};
use nom::{
    bytes::complete::{tag, take},
    combinator::{all_consuming, into, map_parser},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-11.txt");

    let part_one = part_one();
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one() -> PartAnswer {
    PartAnswer::default()
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

#[derive(Debug, PartialEq)]
struct Grid {
    grid: HashMap<(usize, usize), u8>,
}

impl Grid {
    fn step(&mut self) {
        todo!()
    }
}

impl From<Vec<Vec<u8>>> for Grid {
    fn from(matrix: Vec<Vec<u8>>) -> Grid {
        let mut grid = HashMap::new();

        for (y, row) in matrix.iter().enumerate() {
            for (x, octopus) in row.iter().enumerate() {
                grid.insert((x, y), *octopus);
            }
        }

        Grid { grid }
    }
}

fn parse_grid(i: &str) -> Grid {
    all_consuming(terminated(grid, tag("\n")))(i).unwrap().1
}

fn grid(i: &str) -> IResult<&str, Grid> {
    into(separated_list1(tag("\n"), row))(i)
}

fn row(i: &str) -> IResult<&str, Vec<u8>> {
    many1(octopus)(i)
}

fn octopus(i: &str) -> IResult<&str, u8> {
    map_parser(take(1_usize), unsigned_number)(i)
}
