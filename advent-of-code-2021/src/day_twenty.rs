use std::collections::HashMap;

use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    multi::{many1, separated_list1},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
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

struct ScannerOutput {
    image_input: HashMap<(usize, usize), Pixel>,
    enhancement_algorithm: Vec<Pixel>,
}

#[derive(Debug, Clone, Copy)]
enum Pixel {
    Dark,
    Light,
}

fn image_input(i: &str) -> IResult<&str, HashMap<(usize, usize), Pixel>> {
    map(separated_list1(tag("\n"), many1(pixel)), |rows| todo!())(i)
}

fn image_enhancement_algorithm(i: &str) -> IResult<&str, Vec<Pixel>> {
    many1(pixel)(i)
}

fn pixel(i: &str) -> IResult<&str, Pixel> {
    alt((dark, light))(i)
}

fn dark(i: &str) -> IResult<&str, Pixel> {
    value(Pixel::Dark, tag("."))(i)
}

fn light(i: &str) -> IResult<&str, Pixel> {
    value(Pixel::Light, tag("#"))(i)
}
