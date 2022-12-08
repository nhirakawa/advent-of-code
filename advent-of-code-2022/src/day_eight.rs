use std::collections::{HashMap, HashSet};

use common::prelude::*;
use log::debug;
use nom::{
    bytes::complete::{tag, take},
    combinator::{into, map_res},
    multi::{many1, separated_list1},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-8.txt");

    let grid = parse(input);

    let part_one = part_one(&grid);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(grid: &Grid) -> PartAnswer {
    let start = SystemTime::now();

    let mut count = 0;

    for coordinate in grid.all_coordinates() {
        if grid.is_visible(&coordinate) {
            count += 1;
        }
    }

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(count, elapsed)
}

fn part_two() -> PartAnswer {
    let start = SystemTime::now();

    let elapsed = start.elapsed().unwrap();
    PartAnswer::default()
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    heights_by_coordinate: HashMap<(usize, usize), usize>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    fn all_coordinates(&self) -> HashSet<(usize, usize)> {
        let mut all_coordinates = HashSet::new();

        for x in 0..=self.max_x {
            for y in 0..=self.max_y {
                all_coordinates.insert((x, y));
            }
        }

        all_coordinates
    }
    fn is_visible(&self, coordinate: &(usize, usize)) -> bool {
        debug!("Checking {:?}", coordinate);
        let current_value = *self.heights_by_coordinate.get(coordinate).unwrap();

        let (x, y) = *coordinate;

        if x == 0 || x == self.max_x || y == 0 || y == self.max_y {
            return true;
        }

        let max_to_left = (0..x)
            .map(|check_x| (check_x, y))
            .filter_map(|c| self.heights_by_coordinate.get(&c))
            .max()
            .cloned()
            .unwrap();

        debug!("max_to_left {max_to_left}, current_value {current_value}");

        if max_to_left < current_value {
            return true;
        }

        let max_to_right = (x + 1..=self.max_x)
            .map(|check_x| (check_x, y))
            .filter_map(|c| self.heights_by_coordinate.get(&c))
            .max()
            .cloned()
            .unwrap();

        debug!("max_to_right {max_to_right}, current_value {current_value}");

        if max_to_right < current_value {
            return true;
        }

        let max_above = (0..y)
            .map(|check_y| (x, check_y))
            .filter_map(|c| self.heights_by_coordinate.get(&c))
            .max()
            .cloned()
            .unwrap();

        debug!("max_above {max_above}, current_value {current_value}");

        if max_above < current_value {
            return true;
        }

        let max_below = (y + 1..=self.max_y)
            .map(|check_y| (x, check_y))
            .filter_map(|c| self.heights_by_coordinate.get(&c))
            .max()
            .cloned()
            .unwrap();

        debug!("max_below {max_below}, current_value {current_value}");

        if max_below < current_value {
            return true;
        }

        false
    }
}

impl From<Vec<Vec<usize>>> for Grid {
    fn from(raw: Vec<Vec<usize>>) -> Grid {
        let mut heights_by_coordinate = HashMap::new();

        let mut x = 0;
        let mut y = 0;

        let mut max_x = 0;
        let mut max_y = 0;

        for row in raw {
            for column in row {
                debug!("value {column}");
                heights_by_coordinate.insert((x, y), column);

                max_x = max_x.max(x);

                x += 1;
            }

            max_y = max_y.max(y);

            x = 0;
            y += 1;
        }

        debug!("max_x {max_x}, max_y {max_y}");

        Grid {
            heights_by_coordinate,
            max_x,
            max_y,
        }
    }
}

fn parse(i: &str) -> Grid {
    finish(grid)(i).unwrap().1
}

fn grid(i: &str) -> IResult<&str, Grid> {
    into(separated_list1(tag("\n"), row))(i)
}

fn row(i: &str) -> IResult<&str, Vec<usize>> {
    many1(height)(i)
}

fn height(i: &str) -> IResult<&str, usize> {
    map_res(take(1usize), |s: &str| s.parse::<usize>())(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_visible() {
        let grid = parse("30373\n25512\n65332\n33549\n35390");

        assert_eq!(grid.is_visible(&(1, 1)), true); // top-left 5
        assert_eq!(grid.is_visible(&(2, 1)), true); // top-middle 5
        assert_eq!(grid.is_visible(&(2, 2)), false); // center 3
        assert_eq!(grid.is_visible(&(3, 1)), false); // top-right 1
        assert_eq!(grid.is_visible(&(3, 2)), true); // right-middle 3

        assert_eq!(grid.is_visible(&(0, 1)), true); // edge
        assert_eq!(grid.is_visible(&(2, 4)), true); // edge
    }
}
