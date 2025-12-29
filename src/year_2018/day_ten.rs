use std::{
    fmt::Display,
    ops::{Add, RangeInclusive},
};

use itertools::Itertools;

use crate::common::constants;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let points = parse::parse(input)?;
    let (_, result_points) = simulate(points);
    Ok(result_points.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let points = parse::parse(input)?;
    let (time, _) = simulate(points);
    Ok(time)
}

fn simulate(points: Points) -> (usize, Points) {
    // TODO(optimization) - only keep 3 instances of Points (so we can see the inflection point), instead of all instances
    let many_seconds = std::iter::successors(Some(points), |current| Some(current.advance()))
        .take(15_000)
        .collect_vec();

    let (min_idx, min_bounding_box_points) = many_seconds
        .iter()
        .enumerate()
        .min_by_key(|(_, points)| points.bounding_box().area().unwrap_or(0))
        .unwrap();

    (min_idx, min_bounding_box_points.clone())
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Position {
    fn from((x, y): (isize, isize)) -> Self {
        Position { x, y }
    }
}

impl Add<Velocity> for Position {
    type Output = Point;

    fn add(self, rhs: Velocity) -> Self::Output {
        let Position { x, y } = self;
        let Velocity { vx, vy } = rhs;
        let position = (x + vx, y + vy).into();
        let velocity = (vx, vy).into();
        Point { position, velocity }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Velocity {
    vx: isize,
    vy: isize,
}

impl From<(isize, isize)> for Velocity {
    fn from((vx, vy): (isize, isize)) -> Self {
        Velocity { vx, vy }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Point {
    position: Position,
    velocity: Velocity,
}

impl From<(Position, Velocity)> for Point {
    fn from((position, velocity): (Position, Velocity)) -> Self {
        Point { position, velocity }
    }
}

impl Point {
    fn advance(&self) -> Point {
        self.position + self.velocity
    }
}

#[derive(Debug, Clone)]
struct Points(Vec<Point>);

impl From<Vec<Point>> for Points {
    fn from(value: Vec<Point>) -> Self {
        Points(value)
    }
}

impl Points {
    fn advance(&self) -> Points {
        let points = self.0.iter().map(Point::advance).collect_vec();
        points.into()
    }

    /// Calculates the bounding box that contains these points
    /// The first value in the tuple is the (x, y) coordinate of the top left
    /// The second value in the tuple is the (x, y) coordinate of the bottom right
    fn bounding_box(&self) -> BoundingBox {
        let (_min_x_idx, min_x) = self
            .0
            .iter()
            .enumerate()
            .min_by_key(|point| point.1.position.x)
            .map(|(idx, point)| (idx, point.position.x))
            .unwrap();

        let (_max_x_idx, max_x) = self
            .0
            .iter()
            .enumerate()
            .max_by_key(|point| point.1.position.x)
            .map(|(idx, point)| (idx, point.position.x))
            .unwrap();

        let (_min_y_idx, min_y) = self
            .0
            .iter()
            .enumerate()
            .min_by_key(|point| point.1.position.y)
            .map(|(idx, point)| (idx, point.position.y))
            .unwrap();

        let (_max_y_idx, max_y) = self
            .0
            .iter()
            .enumerate()
            .max_by_key(|point| point.1.position.y)
            .map(|(idx, point)| (idx, point.position.y))
            .unwrap();

        BoundingBox::new(min_x, min_y, max_x, max_y)
    }

    fn contains(&self, position: Position) -> bool {
        self.0.iter().any(|point| point.position == position)
    }
}

impl Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bounding_box = self.bounding_box();

        let mut rows = Vec::new();
        rows.push("\n".to_owned());
        for y in bounding_box.y_span() {
            let mut row = Vec::new();
            for x in bounding_box.x_span() {
                let pixel = if self.contains((x, y).into()) {
                    constants::solid_square()
                } else {
                    constants::empty_square()
                };
                row.push(pixel);
            }
            rows.push(row.into_iter().collect::<String>());
        }
        rows.push("\n".to_owned());

        write!(f, "{}", rows.iter().join("\n"))
    }
}

// TODO(debt) - extract to crate::common::math
struct BoundingBox {
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
}

impl BoundingBox {
    fn new(min_x: isize, min_y: isize, max_x: isize, max_y: isize) -> BoundingBox {
        BoundingBox {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    fn x_span(&self) -> RangeInclusive<isize> {
        self.min_x..=self.max_x
    }

    fn y_span(&self) -> RangeInclusive<isize> {
        self.min_y..=self.max_y
    }

    fn area(&self) -> Option<usize> {
        self.min_x
            .abs_diff(self.max_x)
            .checked_mul(self.min_y.abs_diff(self.max_y))
    }
}

mod parse {
    use nom::{
        Parser,
        bytes::complete::tag,
        combinator::into,
        error::Error,
        multi::separated_list1,
        sequence::{preceded, separated_pair, terminated},
    };

    use crate::{
        common::parse::{finish, number, whitespace},
        year_2018::day_ten::{Point, Points, Position, Velocity},
    };

    pub fn parse(input: &str) -> anyhow::Result<Points> {
        finish(points(), input)
    }

    fn points<'a>() -> impl Parser<&'a str, Output = Points, Error = Error<&'a str>> + 'a {
        into(separated_list1(tag("\n"), point()))
    }

    fn point<'a>() -> impl Parser<&'a str, Output = Point, Error = Error<&'a str>> + 'a {
        into(separated_pair(position(), tag(" "), velocity()))
    }

    fn velocity<'a>() -> impl Parser<&'a str, Output = Velocity, Error = Error<&'a str>> + 'a {
        into(preceded(tag("velocity="), vector()))
    }

    fn position<'a>() -> impl Parser<&'a str, Output = Position, Error = Error<&'a str>> + 'a {
        into(preceded(tag("position="), vector()))
    }

    fn vector<'a>() -> impl Parser<&'a str, Output = (isize, isize), Error = Error<&'a str>> + 'a {
        separated_pair(
            preceded(tag("<"), whitespace(number)),
            tag(", "),
            terminated(whitespace(number), tag(">")),
        )
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_parses_vector() {
            assert_eq!(
                vector().parse("< 10775, -31651>"),
                Ok(("", (10775, -31651)))
            )
        }
    }
}
