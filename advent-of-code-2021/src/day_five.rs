use std::collections::{HashMap, HashSet};

use common::parse::unsigned_number;
use common::prelude::*;
use nom::bytes::complete::tag;
use nom::combinator::{all_consuming, into};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-5.txt");
    let lines = parse_lines(input);

    let part_one = part_one(&lines);
    let part_two = part_two(&lines);

    Ok((part_one, part_two))
}

fn part_one(lines: &[Line]) -> PartAnswer {
    let start = SystemTime::now();

    let mut counter = HashMap::new();

    for line in lines {
        match line.slope() {
            Slope::Horizontal | Slope::Vertical => {
                for point in line.points() {
                    if !counter.contains_key(&point) {
                        counter.insert(point, 0);
                    }

                    if let Some(count) = counter.get_mut(&point) {
                        *count += 1;
                    }
                }
            }
            _ => {}
        };
    }

    let solution = counter.values().filter(|count| **count >= 2).count();

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two(lines: &[Line]) -> PartAnswer {
    let start = SystemTime::now();

    let mut counter = HashMap::new();

    for line in lines {
        for point in line.points() {
            if !counter.contains_key(&point) {
                counter.insert(point, 0);
            }

            if let Some(count) = counter.get_mut(&point) {
                *count += 1;
            }
        }
    }

    let solution = counter.values().filter(|count| **count >= 2).count();

    PartAnswer::new(solution, start.elapsed().unwrap())
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn slope(&self) -> Slope {
        if self.start.y == self.end.y {
            Slope::Horizontal
        } else if self.start.x == self.end.x {
            Slope::Vertical
        } else {
            let slope = (self.end.y - self.start.y) / (self.end.x - self.start.x);
            let intercept = self.end.y - (slope * self.end.x);

            Slope::Diagonal { slope, intercept }
        }
    }

    fn points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();

        match &self.slope() {
            Slope::Horizontal => {
                let start_x = i64::min(self.start.x, self.end.x);
                let end_x = i64::max(self.start.x, self.end.x);
                for x in start_x..=end_x {
                    points.insert((x, self.start.y).into());
                }
            }
            Slope::Vertical => {
                let start_y = i64::min(self.start.y, self.end.y);
                let end_y = i64::max(self.start.y, self.end.y);

                for y in start_y..=end_y {
                    points.insert((self.start.x, y).into());
                }
            }
            Slope::Diagonal { slope, intercept } => {
                let start_x = i64::min(self.start.x, self.end.x);
                let end_x = i64::max(self.start.x, self.end.x);

                for x in start_x..=end_x {
                    let y = (x * slope) + intercept;

                    points.insert((x, y).into());
                }
            }
        }
        points
    }
}

impl From<(Point, Point)> for Line {
    fn from(tuple: (Point, Point)) -> Line {
        let (start, end) = tuple;
        Line { start, end }
    }
}

#[derive(Debug, PartialEq)]
enum Slope {
    Horizontal,
    Vertical,
    Diagonal { slope: i64, intercept: i64 },
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Point {
    fn from(tuple: (i64, i64)) -> Point {
        let (x, y) = tuple;
        Point { x, y }
    }
}

fn parse_lines(i: &str) -> Vec<Line> {
    all_consuming(terminated(lines, tag("\n")))(i).unwrap().1
}

fn lines(i: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(tag("\n"), line)(i)
}

fn line(i: &str) -> IResult<&str, Line> {
    into(separated_pair(point, tag(" -> "), point))(i)
}

fn point(i: &str) -> IResult<&str, Point> {
    into(separated_pair(unsigned_number, tag(","), unsigned_number))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slope() {
        let line: Line = ((1, 1).into(), (1, 3).into()).into();
        assert_eq!(line.slope(), Slope::Vertical);

        let line = Line::from((Point::from((1, 3)), Point::from((5, 3))));
        assert_eq!(line.slope(), Slope::Horizontal);

        let line: Line = ((9, 7).into(), (7, 9).into()).into();
        assert_eq!(
            line.slope(),
            Slope::Diagonal {
                slope: -1,
                intercept: 16
            }
        );
    }

    #[test]
    fn test_points() {
        let line: Line = ((1, 1).into(), (1, 3).into()).into();
        assert_eq!(
            line.points(),
            vec![(1, 1).into(), (1, 2).into(), (1, 3).into()]
                .into_iter()
                .collect()
        );

        let line: Line = ((5, 3).into(), (1, 3).into()).into();
        assert_eq!(
            line.points(),
            vec![(1, 3), (2, 3), (3, 3), (4, 3), (5, 3)]
                .into_iter()
                .map(|d| d.into())
                .collect()
        );

        let line: Line = ((9, 7).into(), (7, 9).into()).into();
        assert_eq!(
            line.points(),
            vec![(9, 7), (8, 8), (7, 9)]
                .into_iter()
                .map(|p| p.into())
                .collect()
        );
    }
}
