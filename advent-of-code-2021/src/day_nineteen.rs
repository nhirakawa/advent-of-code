use std::{collections::HashSet, fmt::Debug, ops::Sub};

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
    let input = include_str!("../input/day-19.txt");
    let scanners = parse_scanners(input);

    let part_one = part_one(&scanners);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(scanners: &[ScannerView]) -> PartAnswer {
    let scanner_0 = scanners
        .iter()
        .filter(|scanner| scanner.id == 0)
        .next()
        .cloned()
        .unwrap();

    let scanner_1 = scanners
        .iter()
        .filter(|scanner| scanner.id == 1)
        .next()
        .cloned()
        .unwrap();

    let mut known_coordinates = HashSet::new();
    known_coordinates.extend(&scanner_0.beacons);

    // fingerprints don't change under rotation
    // figure out if there are enough matching fingerprints, then find the rotation
    if do_fingerprints_match(&scanner_0, &scanner_1) {
        println!(
            "scanner {} matches scanner {} - finding transformation",
            scanner_1.id, scanner_0.id
        );
    }

    if let Some((scanner_1_location, scanner_1_absolute_coordinates)) =
        find_scanner_position_and_true_beacon_locations(&known_coordinates, &scanner_0, &scanner_1)
    {
        println!("scanner {} is at {:?}", scanner_1.id, scanner_1_location);
        known_coordinates.extend(&scanner_1_absolute_coordinates);
    }

    PartAnswer::default()
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn do_fingerprints_match(scanner_0: &ScannerView, scanner_1: &ScannerView) -> bool {
    let mut matches = 0;
    for scanner_0_fingerprint in scanner_0.fingerprints() {
        for scanner_1_fingerprint in scanner_1.fingerprints() {
            if scanner_0_fingerprint.fingerprint == scanner_1_fingerprint.fingerprint {
                matches += 1;
            }
        }
    }

    matches >= 12
}

fn find_scanner_position_and_true_beacon_locations(
    known_coordinates: &HashSet<Coordinate>,
    scanner_0: &ScannerView,
    other: &ScannerView,
) -> Option<(Coordinate, Vec<Coordinate>)> {
    // check all rotations
    for rotation in Rotation::all() {
        let current_scanner = other.rotate(&rotation);

        for scanner_0_coordinate in &scanner_0.beacons {
            for scanner_1_coordinate in &current_scanner.beacons {
                let offset = scanner_1_coordinate - scanner_0_coordinate;

                let mut matches = 0;
                let mut offset_scanner_1_coordinates = Vec::new();

                for scanner_1_coordinate in &current_scanner.beacons {
                    let adjusted = scanner_1_coordinate - &offset;
                    if known_coordinates.contains(&adjusted) {
                        // println!("scanner 1 coordinate {:?} matches scanner 0 coordinate {:?} with offset {:?}", scanner_1_coordinate, &adjusted, offset);
                        matches += 1;
                    }
                    offset_scanner_1_coordinates.push(adjusted);
                }

                if matches >= 12 {
                    let scanner_1_location = &Coordinate::new(0, 0, 0) - &offset;
                    return Some((scanner_1_location, offset_scanner_1_coordinates));
                }
            }
        }
    }

    None
}

#[derive(Debug, PartialEq, Clone)]
struct ScannerView {
    id: u8,
    beacons: Vec<Coordinate>,
}

impl ScannerView {
    fn new(id: u8, beacons: Vec<Coordinate>) -> ScannerView {
        ScannerView { id, beacons }
    }

    fn fingerprints(&self) -> Vec<SegmentAndFingerprint> {
        let mut fingerprints = Vec::new();

        for (outer_index, outer) in self.beacons.iter().enumerate() {
            for (inner_index, inner) in self.beacons.iter().enumerate() {
                if inner_index <= outer_index {
                    continue;
                }

                let fingerprint =
                    SegmentAndFingerprint::new(*outer, *inner, Fingerprint::new(outer, inner));
                fingerprints.push(fingerprint);
            }
        }

        fingerprints
    }

    fn rotate(&self, rotation: &Rotation) -> ScannerView {
        let beacons = self
            .beacons
            .iter()
            .map(|coordinate| coordinate.rotate(rotation))
            .collect();

        ScannerView {
            id: self.id,
            beacons,
        }
    }
}

#[derive(Debug, PartialEq)]
struct SegmentAndFingerprint {
    c1: Coordinate,
    c2: Coordinate,
    fingerprint: Fingerprint,
}

impl SegmentAndFingerprint {
    fn new(c1: Coordinate, c2: Coordinate, fingerprint: Fingerprint) -> SegmentAndFingerprint {
        SegmentAndFingerprint {
            c1,
            c2,
            fingerprint,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Fingerprint {
    l1_norm: i32,
    l1_max: i32,
}

impl Fingerprint {
    fn new(c1: &Coordinate, c2: &Coordinate) -> Fingerprint {
        let l1_x = (c1.x - c2.x).abs();
        let l1_y = (c1.y - c2.y).abs();
        let l1_z = (c1.z - c2.z).abs();

        let l1_norm = l1_x + l1_y + l1_z;
        let l1_max = l1_x.max(l1_y).max(l1_z);

        Fingerprint { l1_norm, l1_max }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32, z: i32) -> Coordinate {
        Coordinate { x, y, z }
    }

    fn rotate(&self, rotation: &Rotation) -> Coordinate {
        let mut x = self.x;
        let mut y = self.y;
        let mut z = self.z;

        match rotation.negation {
            Negation::XYZ => {}
            Negation::XYNegZ => z *= -1,
            Negation::XNegYZ => y *= -1,
            Negation::XNegYNegZ => {
                y *= -1;
                z *= -1;
            }
            Negation::NegXYZ => x *= -1,
            Negation::NegXYNegZ => {
                x *= -1;
                z *= -1;
            }
            Negation::NegXNegYZ => {
                x *= -1;
                y *= -1;
            }
            Negation::NegXNegYNegZ => {
                x *= -1;
                y *= -1;
                z *= -1;
            }
        };

        match rotation.coordinate_order {
            CoordinateOrder::XYZ => (x, y, z),
            CoordinateOrder::XZY => (x, z, y),
            CoordinateOrder::YXZ => (y, x, z),
            CoordinateOrder::YZX => (y, z, x),
            CoordinateOrder::ZXY => (z, x, y),
            CoordinateOrder::ZYX => (z, y, x),
        }
        .into()
    }
}

impl Sub<&Coordinate> for &Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: &Coordinate) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;

        Coordinate::new(x, y, z)
    }
}

impl From<(i32, i32, i32)> for Coordinate {
    fn from(tuple: (i32, i32, i32)) -> Coordinate {
        let (x, y, z) = tuple;
        Coordinate::new(x, y, z)
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Rotation {
    negation: Negation,
    coordinate_order: CoordinateOrder,
}

impl Rotation {
    fn new(negation: Negation, coordinate_order: CoordinateOrder) -> Rotation {
        Rotation {
            negation,
            coordinate_order,
        }
    }

    fn all() -> Vec<Rotation> {
        let mut rotations = Vec::new();

        for negation in Negation::values() {
            for coordinate_order in CoordinateOrder::values() {
                rotations.push(Rotation {
                    negation,
                    coordinate_order,
                });
            }
        }

        rotations
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Negation {
    XYZ,
    XYNegZ,
    XNegYZ,
    XNegYNegZ,
    NegXYZ,
    NegXYNegZ,
    NegXNegYZ,
    NegXNegYNegZ,
}

impl Negation {
    fn values() -> Vec<Negation> {
        vec![
            Negation::XYZ,
            Negation::XYNegZ,
            Negation::XNegYZ,
            Negation::XNegYNegZ,
            Negation::NegXYZ,
            Negation::NegXYNegZ,
            Negation::NegXNegYZ,
            Negation::NegXNegYNegZ,
        ]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum CoordinateOrder {
    XYZ,
    XZY,
    YXZ,
    YZX,
    ZXY,
    ZYX,
}

impl CoordinateOrder {
    fn values() -> Vec<CoordinateOrder> {
        vec![
            CoordinateOrder::XYZ,
            CoordinateOrder::XZY,
            CoordinateOrder::YXZ,
            CoordinateOrder::YZX,
            CoordinateOrder::ZXY,
            CoordinateOrder::ZYX,
        ]
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

    #[test]
    fn test_rotation() {
        let coordinate = Coordinate::new(1, 2, 3);

        let rotation = Rotation::new(Negation::XYZ, CoordinateOrder::XZY);

        let rotated = coordinate.rotate(&rotation);

        assert_eq!(rotated, Coordinate::new(1, 3, 2));
    }
}
