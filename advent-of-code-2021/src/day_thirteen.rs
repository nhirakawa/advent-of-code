use std::collections::HashSet;

use common::{parse::unsigned_number, prelude::*};
use log::debug;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{all_consuming, into, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-13.txt");
    let (coordinates, instructions) = parse_coordinates(input);

    let part_one = part_one(&coordinates, &instructions);
    let part_two = part_two(coordinates, &instructions);

    Ok((part_one, part_two))
}

fn part_one(coordinates: &HashSet<Coordinate>, instructions: &[FoldInstruction]) -> PartAnswer {
    let start = SystemTime::now();

    let instruction = &instructions[0];

    let folded = fold(coordinates, instruction);

    PartAnswer::new(folded.len(), start.elapsed().unwrap())
}

fn part_two(coordinates: HashSet<Coordinate>, instructions: &[FoldInstruction]) -> PartAnswer {
    let start = SystemTime::now();

    let mut folded = coordinates;

    for instruction in instructions {
        folded = fold(&folded, instruction);
    }

    let max_x = folded.iter().map(|c| c.x).max().unwrap();
    let max_y = folded.iter().map(|c| c.y).max().unwrap();

    let mut parts = vec!["\n"];

    for y in 0..=max_y {
        for x in 0..=max_x {
            let coordinate = (x, y).into();
            let part = if folded.contains(&coordinate) {
                "\u{2588}"
            } else {
                " "
            };

            parts.push(part);
        }
        parts.push("\n");
    }

    let solution = parts.join("");

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn fold(coordinates: &HashSet<Coordinate>, instruction: &FoldInstruction) -> HashSet<Coordinate> {
    let mut result = HashSet::new();

    for coordinate in coordinates {
        let new_coordinate = match instruction {
            FoldInstruction::Horizontal(y) => {
                let absolute_distance = usize::max(*y, coordinate.y) - usize::min(*y, coordinate.y);
                debug!(
                    "distance: {}, y={}, point: {:?}",
                    absolute_distance, y, coordinate
                );
                let new_y = y - absolute_distance;
                Coordinate {
                    x: coordinate.x,
                    y: new_y,
                }
            }
            FoldInstruction::Vertical(x) => {
                let absolute_distance = usize::max(*x, coordinate.x) - usize::min(*x, coordinate.x);
                debug!(
                    "distance: {}, x={}, point: {:?}",
                    absolute_distance, x, coordinate
                );
                let new_x = x - absolute_distance;
                Coordinate {
                    x: new_x,
                    y: coordinate.y,
                }
            }
        };

        result.insert(new_coordinate);
    }

    result
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coordinate {
    fn from(tuple: (usize, usize)) -> Coordinate {
        let (x, y) = tuple;
        Coordinate { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum FoldInstruction {
    Horizontal(usize),
    Vertical(usize),
}

fn parse_coordinates(i: &str) -> (HashSet<Coordinate>, Vec<FoldInstruction>) {
    let coordinates = terminated(coordinates, tag("\n"));
    let fold_instructions = terminated(fold_instructions, multispace0);

    all_consuming(separated_pair(coordinates, tag("\n"), fold_instructions))(i)
        .unwrap()
        .1
}

fn coordinates(i: &str) -> IResult<&str, HashSet<Coordinate>> {
    map(separated_list1(tag("\n"), coordinate), |v| {
        v.into_iter().collect()
    })(i)
}

fn coordinate(i: &str) -> IResult<&str, Coordinate> {
    into(separated_pair(unsigned_number, tag(","), unsigned_number))(i)
}

fn fold_instructions(i: &str) -> IResult<&str, Vec<FoldInstruction>> {
    separated_list1(tag("\n"), fold_instruction)(i)
}

fn fold_instruction(i: &str) -> IResult<&str, FoldInstruction> {
    alt((horizontal, vertical))(i)
}

fn horizontal(i: &str) -> IResult<&str, FoldInstruction> {
    map(
        preceded(tag("fold along y="), unsigned_number),
        FoldInstruction::Horizontal,
    )(i)
}

fn vertical(i: &str) -> IResult<&str, FoldInstruction> {
    map(
        preceded(tag("fold along x="), unsigned_number),
        FoldInstruction::Vertical,
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_vertical() {
        let coordinates = vec![(0, 0), (4, 4)]
            .into_iter()
            .map(Coordinate::from)
            .collect();
        let folded = fold(&coordinates, &FoldInstruction::Vertical(3));
        assert_eq!(
            folded,
            vec![(0, 0), (2, 4)]
                .into_iter()
                .map(Coordinate::from)
                .collect()
        );
    }

    #[test]
    fn test_fold_horizontal() {
        let coordinates = vec![(0, 0), (4, 4)]
            .into_iter()
            .map(Coordinate::from)
            .collect();
        let folded = fold(&coordinates, &FoldInstruction::Horizontal(2));
        assert_eq!(
            folded,
            vec![(0, 0), (4, 0)]
                .into_iter()
                .map(Coordinate::from)
                .collect()
        );
    }

    #[test]
    fn test_example() {
        let (coordinates, instructions) = parse_coordinates("6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5");
        let mut coordinates = coordinates;
        for instruction in instructions {
            coordinates = fold(&coordinates, &instruction);
        }

        assert_eq!(
            coordinates,
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (1, 4),
                (2, 4),
                (3, 4),
                (4, 4),
                (4, 3),
                (4, 2),
                (4, 1),
                (4, 0),
                (3, 0),
                (2, 0),
                (1, 0)
            ]
            .into_iter()
            .map(Coordinate::from)
            .collect()
        );
    }
}
