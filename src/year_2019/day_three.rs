use std::collections::HashSet;
use std::time::SystemTime;
use crate::common::answer::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_opt, value},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-3.txt");

    let (first, second) = parse(input);

    let part_one = run_part_one(&first, &second);
    let part_two = run_part_two(&first, &second);

    Ok((part_one, part_two))
}

fn run_part_one(first: &[Step], second: &[Step]) -> PartAnswer {
    let start = SystemTime::now();
    let intersections = get_intersections(first, second);

    let solution = intersections
        .iter()
        .map(|(x, y)| (x.abs() + y.abs()) as u32)
        .min()
        .unwrap();

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn run_part_two(first: &[Step], second: &[Step]) -> PartAnswer {
    let start = SystemTime::now();
    let intersections = get_intersections(first, second);

    let first_path = expand_points(first);
    let second_path = expand_points(second);

    let mut min_number_of_combined_steps = u32::MAX;

    for intersection in intersections {
        let steps_along_first_path = get_number_of_steps_to_point(&intersection, &first_path);
        let steps_along_second_path = get_number_of_steps_to_point(&intersection, &second_path);

        min_number_of_combined_steps =
            min_number_of_combined_steps.min(steps_along_first_path + steps_along_second_path);
    }

    PartAnswer::new(min_number_of_combined_steps, start.elapsed().unwrap())
}

fn get_number_of_steps_to_point(point: &(i32, i32), path: &[(i32, i32)]) -> u32 {
    let mut counter = 0;
    for step in path {
        if step == point {
            return counter;
        } else {
            counter += 1;
        }
    }

    panic!()
}

// todo @nhirakawa - solve this more elegantly using line segment intersection?
fn get_intersections(first: &[Step], second: &[Step]) -> HashSet<(i32, i32)> {
    let first_points: HashSet<(i32, i32)> = expand_points(first).into_iter().collect();
    let second_points = expand_points(second).into_iter().collect();

    let intersections = first_points.intersection(&second_points);

    intersections
        .filter_map(|(x, y)| {
            if *x == 0 && *y == 0 {
                None
            } else {
                Some((*x, *y))
            }
        })
        .collect()
}

fn expand_points(first: &[Step]) -> Vec<(i32, i32)> {
    let mut start = (0, 0);
    let mut all_points = Vec::new();
    for path in first {
        let points = path.expand(start);
        all_points.extend(points);
        start = path.get_endpoint(start, path.length as i32);
    }
    all_points
}

type Coordinate = (i32, i32);

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone, Debug)]
struct Step {
    direction: Direction,
    length: u32,
}

impl Step {
    fn new(direction: Direction, length: u32) -> Step {
        Step { direction, length }
    }

    fn expand(&self, start: Coordinate) -> Vec<Coordinate> {
        let mut coordinates = Vec::with_capacity(self.length as usize);

        for delta in 0..self.length {
            let delta = delta as i32;

            let next = self.get_endpoint(start, delta);

            coordinates.push(next);
        }

        coordinates
    }

    fn get_endpoint(&self, coordinate: Coordinate, delta: i32) -> Coordinate {
        match self.direction {
            Direction::Up => (coordinate.0, coordinate.1 + delta),
            Direction::Right => (coordinate.0 + delta, coordinate.1),
            Direction::Down => (coordinate.0, coordinate.1 - delta),
            Direction::Left => (coordinate.0 - delta, coordinate.1),
        }
    }
}

fn parse(i: &str) -> (Vec<Step>, Vec<Step>) {
    steps(i).unwrap().1
}

fn steps(i: &str) -> IResult<&str, (Vec<Step>, Vec<Step>)> {
    separated_pair(whole_path, tag("\n"), whole_path)(i)
}

fn whole_path(i: &str) -> IResult<&str, Vec<Step>> {
    separated_list1(tag(","), step)(i)
}

fn step(i: &str) -> IResult<&str, Step> {
    map(tuple((direction, number)), |(direction, number)| {
        Step::new(direction, number)
    })(i)
}

fn direction(i: &str) -> IResult<&str, Direction> {
    let up = value(Direction::Up, tag("U"));
    let right = value(Direction::Right, tag("R"));
    let down = value(Direction::Down, tag("D"));
    let left = value(Direction::Left, tag("L"));

    alt((up, right, down, left))(i)
}

fn number(i: &str) -> IResult<&str, u32> {
    map_opt(digit1, |s: &str| s.parse::<u32>().ok())(i)
}
