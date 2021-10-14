use std::collections::HashSet;

use common::{parse::unsigned_number, prelude::*};
use multiset::HashMultiSet;
use nom::{
    bytes::complete::tag,
    combinator::{into, map, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let start = SystemTime::now();
    let claims = parse_claims(include_str!("../input/day-3.txt"));
    let parse_duration = start.elapsed().unwrap();

    let part_one = part_one(&claims, &parse_duration);
    let part_two = part_two(&claims, &parse_duration);

    Ok((part_one, part_two))
}

fn part_one(claims: &[Claim], parse_duration: &Duration) -> PartAnswer {
    let start = SystemTime::now();
    let mut all_coordinates = HashMultiSet::new();

    for claim in claims {
        for coordinates in &claim.covering_coordinates {
            all_coordinates.insert(coordinates.clone());
        }
    }

    let mut overlapping = 0;
    for key in all_coordinates.distinct_elements() {
        if all_coordinates.count_of(key) > 1 {
            overlapping += 1;
        }
    }

    PartAnswer::new(overlapping, start.elapsed().unwrap() + *parse_duration)
}

fn part_two(claims: &[Claim], parse_duration: &Duration) -> PartAnswer {
    let start = SystemTime::now();
    let mut all_coordinates = HashMultiSet::new();

    for claim in claims {
        for coordinates in &claim.covering_coordinates {
            all_coordinates.insert(coordinates.clone());
        }
    }

    for claim in claims {
        let mut has_overlap = false;
        for coordinates in &claim.covering_coordinates {
            if all_coordinates.count_of(coordinates) > 1 {
                has_overlap = true;
                break;
            }
        }
        if !has_overlap {
            return PartAnswer::new(claim.id, start.elapsed().unwrap() + *parse_duration);
        }
    }

    PartAnswer::default()
}

#[derive(PartialEq, Debug)]
struct Claim {
    id: usize,
    top_left_corner: TopLeftCorner,
    width_height: WidthHeight,
    covering_coordinates: HashSet<(usize, usize)>,
}

impl Claim {
    fn new(id: usize, top_left_corner: TopLeftCorner, width_height: WidthHeight) -> Self {
        let initial_x = top_left_corner.x;
        let final_x = top_left_corner.x + width_height.width;

        let initial_y = top_left_corner.y;
        let final_y = top_left_corner.y + width_height.height;

        let mut covering_coordinates = HashSet::new();

        for x in initial_x..final_x {
            for y in initial_y..final_y {
                covering_coordinates.insert((x, y));
            }
        }

        Self {
            id,
            top_left_corner,
            width_height,
            covering_coordinates,
        }
    }
}

#[derive(PartialEq, Debug)]
struct TopLeftCorner {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for TopLeftCorner {
    fn from(coordinates: (usize, usize)) -> Self {
        let (x, y) = coordinates;
        Self { x, y }
    }
}

#[derive(PartialEq, Debug)]
struct WidthHeight {
    width: usize,
    height: usize,
}

impl From<(usize, usize)> for WidthHeight {
    fn from(width_height: (usize, usize)) -> Self {
        let (width, height) = width_height;

        Self { width, height }
    }
}

fn parse_claims(i: &str) -> Vec<Claim> {
    claims(i).unwrap().1
}

fn claims(i: &str) -> IResult<&str, Vec<Claim>> {
    separated_list1(tag("\n"), claim)(i)
}

fn claim(i: &str) -> IResult<&str, Claim> {
    map(
        tuple((
            claim_id,
            value((), tag(" @ ")),
            top_left_corner,
            value((), tag(": ")),
            width_height,
        )),
        |(id, _, top_left_corner, _, width_height)| Claim::new(id, top_left_corner, width_height),
    )(i)
}

fn claim_id(i: &str) -> IResult<&str, usize> {
    preceded(tag("#"), unsigned_number)(i)
}

fn top_left_corner(i: &str) -> IResult<&str, TopLeftCorner> {
    into(separated_pair(unsigned_number, tag(","), unsigned_number))(i)
}

fn width_height(i: &str) -> IResult<&str, WidthHeight> {
    into(separated_pair(unsigned_number, tag("x"), unsigned_number))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claim() {
        assert_eq!(
            claim("#123 @ 44,76: 98x22"),
            Ok((
                "",
                Claim::new(
                    123,
                    TopLeftCorner::from((44, 76)),
                    WidthHeight::from((98, 22))
                )
            ))
        );
    }

    #[test]
    fn test_claim_id() {
        assert_eq!(claim_id("#123"), Ok(("", 123)));
    }

    #[test]
    fn test_top_left_corner() {
        assert_eq!(
            top_left_corner("33,11"),
            Ok(("", TopLeftCorner::from((33, 11))))
        );
    }

    #[test]
    fn test_width_height() {
        assert_eq!(width_height("23x89"), Ok(("", WidthHeight::from((23, 89)))));
    }
}
