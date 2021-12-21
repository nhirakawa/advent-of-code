use std::fmt::Debug;

use common::{
    parse::{number, unsigned_number},
    prelude::*,
};
use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{all_consuming, into, map, value},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
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

#[derive(Debug, PartialEq)]
struct ScannerView {
    id: u8,
    beacons: Vec<Coordinate>,
}

impl ScannerView {
    fn new(id: u8, beacons: Vec<Coordinate>) -> ScannerView {
        todo!()
    }
}

#[derive(PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    fn distance(&self, other: &Self) -> f64 {
        let delta_x_squared = (self.x - other.x).pow(2) as f64;
        let delta_y_squared = (self.y - other.y).pow(2) as f64;
        let delta_z_squared = (self.z - other.z).pow(2) as f64;

        (delta_x_squared + delta_y_squared + delta_z_squared).sqrt()
    }
}

impl From<(i32, i32, i32)> for Coordinate {
    fn from(tuple: (i32, i32, i32)) -> Coordinate {
        let (x, y, z) = tuple;
        Coordinate { x, y, z }
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

fn parse_scanners(i: &str) -> Vec<ScannerView> {
    all_consuming(terminated(scanners, multispace0))(i)
        .unwrap()
        .1
}

fn scanners(i: &str) -> IResult<&str, Vec<ScannerView>> {
    separated_list1(tag("\n\n"), scanner_view)(i)
}

fn scanner_view(i: &str) -> IResult<&str, ScannerView> {
    map(
        separated_pair(scanner_id, tag("\n"), coordinates),
        |(id, beacons)| ScannerView::new(id, beacons),
    )(i)
}

fn scanner_id(i: &str) -> IResult<&str, u8> {
    delimited(tag("--- scanner "), unsigned_number, tag(" ---"))(i)
}

fn coordinates(i: &str) -> IResult<&str, Vec<Coordinate>> {
    separated_list1(tag("\n"), coordinate)(i)
}

fn coordinate(i: &str) -> IResult<&str, Coordinate> {
    into(map(
        tuple((number, comma, number, comma, number)),
        |(x, _, y, _, z)| (x, y, z),
    ))(i)
}

fn comma(i: &str) -> IResult<&str, ()> {
    value((), tag(","))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coordinate() {
        assert_eq!(coordinate("-7,0,8"), Ok(("", (-7, 0, 8).into())))
    }
}
