use crate::AdventOfCodeError;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> Result<(u32, u32), AdventOfCodeError> {
    let trees = parse()?;
    let part_one_answer = part_one(&trees);
    let part_two_answer = part_two(&trees);

    Ok((part_one_answer, part_two_answer))
}

fn part_one(trees: &Trees) -> u32 {
    count_the_trees(trees, (3, 1))
}

fn part_two(trees: &Trees) -> u32 {
    let first = count_the_trees(trees, (1, 1));
    let second = count_the_trees(trees, (3, 1));
    let third = count_the_trees(trees, (5, 1));
    let fourth = count_the_trees(trees, (7, 1));
    let fifth = count_the_trees(trees, (1, 2));

    first * second * third * fourth * fifth
}

fn count_the_trees(trees: &Trees, slope: (u32, u32)) -> u32 {
    let path = Path::new(slope, trees.bounds);

    let mut counter = 0;

    for coordinate in path {
        if trees.has_tree(coordinate) {
            counter += 1;
        }
    }

    counter
}

struct Path {
    slope: (u32, u32),
    lower_right_corner: (u32, u32),
    current: (u32, u32),
}

impl Path {
    pub fn new(slope: (u32, u32), lower_right_corner: (u32, u32)) -> Self {
        Path {
            slope,
            lower_right_corner,
            current: (0, 0),
        }
    }
}

impl Iterator for Path {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.1 >= self.lower_right_corner.1 {
            None
        } else {
            let next_x = self.current.0 + self.slope.0;
            let next_y = self.current.1 + self.slope.1;

            self.current = (next_x, next_y);

            Some((next_x, next_y))
        }
    }
}

struct Trees {
    trees: HashSet<(u32, u32)>,
    bounds: (u32, u32),
}

impl Trees {
    pub fn new(trees: HashSet<(u32, u32)>, bounds: (u32, u32)) -> Trees {
        Trees { trees, bounds }
    }

    pub fn has_tree(&self, coordinate: (u32, u32)) -> bool {
        let coordinate = (coordinate.0 % self.bounds.0, coordinate.1);

        self.trees.contains(&coordinate)
    }
}

fn parse() -> Result<Trees, AdventOfCodeError> {
    let input = include_str!("../input/day-3.txt");

    let mut set = HashSet::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for line in input.split("\n") {
        let (width, tree_coordinates_in_line) = parse_string(line.into(), max_y);

        set.extend(tree_coordinates_in_line);

        max_x = max_x.max(width);
        max_y += 1;
    }

    Ok(Trees::new(set, (max_x, max_y)))
}

fn parse_string(s: String, y: u32) -> (u32, HashSet<(u32, u32)>) {
    let mut set = HashSet::new();
    let mut x = 0;
    for c in s.chars() {
        if c == '#' {
            set.insert((x, y));
        }

        x += 1;
    }

    (x, set)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let (_, actual) = parse_string("..##.......".into(), 0);
        let mut expected = HashSet::new();
        expected.insert((2, 0));
        expected.insert((3, 0));

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_path() {
        let path = Path::new((2, 1), (3, 3));

        let actual: Vec<(u32, u32)> = path.into_iter().collect();

        let expected = vec![(2, 1), (4, 2), (6, 3)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_answers() {
        let answers = run().unwrap();
        assert_eq!(answers.0, 184);
        assert_eq!(answers.1, 2431272960);
    }
}
