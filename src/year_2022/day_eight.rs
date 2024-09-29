use std::collections::{HashMap, HashSet};
use std::time::SystemTime;
use crate::common::answer::*;
use log::debug;
use nom::{
    bytes::complete::{tag, take},
    combinator::{into, map_res},
    multi::{many1, separated_list1},
    IResult,
};
use crate::common::parse::finish;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-8.txt");

    let grid = parse(input);

    let part_one = part_one(&grid);
    let part_two = part_two(&grid);

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

fn part_two(grid: &Grid) -> PartAnswer {
    let start = SystemTime::now();

    let answer = grid
        .all_coordinates()
        .iter()
        .map(|c| grid.score_visible_trees(c))
        .max()
        .unwrap_or(0);

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
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

    fn get_coordinates_up(&self, coordinate: &(usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = *coordinate;

        (0..y).rev().map(|check_y| (x, check_y)).collect()
    }

    fn get_coordinates_right(&self, coordinate: &(usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = *coordinate;
        (x + 1..=self.max_x).map(|check_x| (check_x, y)).collect()
    }

    fn get_coordinates_down(&self, coordinate: &(usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = *coordinate;

        (y + 1..=self.max_y).map(|check_y| (x, check_y)).collect()
    }

    fn get_coordinates_left(&self, coordinate: &(usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = *coordinate;

        (0..x).rev().map(|check_x| (check_x, y)).collect()
    }

    fn is_edge(&self, coordinate: &(usize, usize)) -> bool {
        let (x, y) = *coordinate;
        if x == 0 || x == self.max_x || y == 0 || y == self.max_y {
            true
        } else {
            false
        }
    }

    fn is_visible(&self, coordinate: &(usize, usize)) -> bool {
        debug!("Checking {:?}", coordinate);
        let current_value = *self.heights_by_coordinate.get(coordinate).unwrap();

        if self.is_edge(coordinate) {
            return true;
        }

        let max_to_left = self
            .get_coordinates_left(coordinate)
            .iter()
            .filter_map(|c| self.heights_by_coordinate.get(&c))
            .max()
            .cloned()
            .unwrap();

        debug!("max_to_left {max_to_left}, current_value {current_value}");

        if max_to_left < current_value {
            return true;
        }

        let max_to_right = self
            .get_coordinates_right(coordinate)
            .iter()
            .filter_map(|c| self.heights_by_coordinate.get(&c))
            .max()
            .cloned()
            .unwrap();

        debug!("max_to_right {max_to_right}, current_value {current_value}");

        if max_to_right < current_value {
            return true;
        }

        let max_above = self
            .get_coordinates_up(coordinate)
            .iter()
            .filter_map(|c| self.heights_by_coordinate.get(&c))
            .max()
            .cloned()
            .unwrap();

        debug!("max_above {max_above}, current_value {current_value}");

        if max_above < current_value {
            return true;
        }

        let max_below = self
            .get_coordinates_down(coordinate)
            .iter()
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

    fn score_visible_trees(&self, coordinate: &(usize, usize)) -> usize {
        let current_height = *self.heights_by_coordinate.get(coordinate).unwrap();

        debug!("checking {:?}, height {current_height}", coordinate);

        if self.is_edge(coordinate) {
            return 0;
        }

        let visible_trees_up =
            self.get_visible_tree_count(&self.get_coordinates_up(coordinate), current_height);

        debug!("visible_trees_up {visible_trees_up}");

        let visible_trees_right =
            self.get_visible_tree_count(&self.get_coordinates_right(coordinate), current_height);

        debug!("visible_trees_right {visible_trees_right}");

        let visible_trees_down =
            self.get_visible_tree_count(&self.get_coordinates_down(coordinate), current_height);

        debug!("visible_trees_down {visible_trees_down}");

        let visible_trees_left =
            self.get_visible_tree_count(&self.get_coordinates_left(coordinate), current_height);

        debug!("visible_trees_left {visible_trees_left}");

        visible_trees_up * visible_trees_right * visible_trees_down * visible_trees_left
    }

    fn get_visible_tree_count(
        &self,
        coordinates: &[(usize, usize)],
        current_height: usize,
    ) -> usize {
        let mut count = 0;

        for coordinate in coordinates {
            if let Some(height) = self.heights_by_coordinate.get(coordinate) {
                count += 1;

                if *height >= current_height {
                    break;
                }
            } else {
                break;
            }
        }

        count
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

    fn build_grid() -> Grid {
        parse("30373\n25512\n65332\n33549\n35390")
    }

    #[test]
    fn test_is_visible() {
        let grid = build_grid();

        assert_eq!(grid.is_visible(&(1, 1)), true); // top-left 5
        assert_eq!(grid.is_visible(&(2, 1)), true); // top-middle 5
        assert_eq!(grid.is_visible(&(2, 2)), false); // center 3
        assert_eq!(grid.is_visible(&(3, 1)), false); // top-right 1
        assert_eq!(grid.is_visible(&(3, 2)), true); // right-middle 3

        assert_eq!(grid.is_visible(&(0, 1)), true); // edge
        assert_eq!(grid.is_visible(&(2, 4)), true); // edge
    }

    #[test]
    fn test_score_visible_trees() {
        let grid = build_grid();

        assert_eq!(grid.score_visible_trees(&(2, 1)), 4);
        assert_eq!(grid.score_visible_trees(&(2, 3)), 8);
    }

    #[test]
    fn test_get_coordinates_up() {
        let grid = build_grid();

        assert_eq!(
            grid.get_coordinates_up(&(2, 3)),
            vec![(2, 2), (2, 1), (2, 0)]
        );
    }

    #[test]
    fn test_get_coordinates_right() {
        let grid = build_grid();

        assert_eq!(
            grid.get_coordinates_right(&(1, 1)),
            vec![(2, 1), (3, 1), (4, 1)]
        );
    }

    #[test]
    fn test_get_coordinates_down() {
        let grid = build_grid();

        assert_eq!(
            grid.get_coordinates_down(&(1, 0)),
            vec![(1, 1), (1, 2), (1, 3), (1, 4)]
        );
    }

    #[test]
    fn test_get_coordinates_left() {
        let grid = build_grid();

        assert_eq!(
            grid.get_coordinates_left(&(4, 4)),
            vec![(3, 4), (2, 4), (1, 4), (0, 4)]
        );
    }
}
