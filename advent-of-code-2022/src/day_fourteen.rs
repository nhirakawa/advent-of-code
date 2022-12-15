use std::collections::HashSet;

use common::prelude::*;
use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list1, sequence::separated_pair,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-14.txt");

    let mut falling_sand = parse(input);

    let part_one = part_one(&mut falling_sand);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(falling_sand: &mut FallingSand) -> PartAnswer {
    let start = SystemTime::now();

    let answer = falling_sand.add_sand_until_flowing();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn part_two() -> PartAnswer {
    let start = SystemTime::now();
    let elapsed = start.elapsed().unwrap();

    PartAnswer::default()
}

#[derive(Debug)]
struct FallingSand {
    rocks: HashSet<(usize, usize)>,
    sand: HashSet<(usize, usize)>,
    greatest_y: usize,
}

impl FallingSand {
    fn new(rocks: HashSet<(usize, usize)>) -> FallingSand {
        let greatest_y = rocks.iter().map(|(_, y)| y).max().cloned().unwrap();

        let sand = HashSet::new();

        FallingSand {
            rocks,
            sand,
            greatest_y,
        }
    }

    fn add_sand_until_flowing(&mut self) -> usize {
        let mut count = 0;

        let mut current_count = 0;

        loop {
            self.add_sand();

            if self.sand.len() > current_count {
                current_count = self.sand.len();
                count += 1;
            } else {
                return count;
            }
        }
    }

    fn add_sand(&mut self) {
        let mut current = (500, 0);

        while current.1 <= self.greatest_y {
            // println!("current {current:?}");
            let to_check = (current.0, current.1 + 1);

            if self.rocks.contains(&to_check) || self.sand.contains(&to_check) {
                // next downward position is occupied

                let down_and_to_the_left = (to_check.0 - 1, to_check.1);

                // println!("encountered obstacle, checking down-left");
                if self.rocks.contains(&down_and_to_the_left)
                    || self.sand.contains(&down_and_to_the_left)
                {
                    // down-left is blocked
                    let down_and_to_the_right = (to_check.0 + 1, to_check.1);

                    // println!("encountered obstacle, checking down-right");
                    if self.rocks.contains(&down_and_to_the_right)
                        || self.sand.contains(&down_and_to_the_right)
                    {
                        // down-left and down-right are both blocked

                        // println!("settled");
                        self.sand.insert(current);
                        return;
                    } else {
                        current = down_and_to_the_right;
                    }
                } else {
                    current = down_and_to_the_left;
                }
            } else {
                // next space is not occupied, advance the sand
                // println!("advancing");
                current = to_check;
            }
        }
    }
}

fn parse(i: &str) -> FallingSand {
    finish(falling_sand)(i).unwrap().1
}

fn falling_sand(i: &str) -> IResult<&str, FallingSand> {
    map(rock_lines, FallingSand::new)(i)
}

fn rock_lines(i: &str) -> IResult<&str, HashSet<(usize, usize)>> {
    map(separated_list1(tag("\n"), rock_line), |sets| {
        sets.into_iter().flat_map(|set| set.into_iter()).collect()
    })(i)
}

fn rock_line(i: &str) -> IResult<&str, HashSet<(usize, usize)>> {
    map(separated_list1(tag(" -> "), coordinate), |corners| {
        corners
            .windows(2)
            .map(|window| {
                let first = window[0];
                let second = window[1];

                transform_to_set_of_points(first, second)
            })
            .flat_map(|set| set.into_iter())
            .collect()
    })(i)
}

fn transform_to_set_of_points(
    first: (usize, usize),
    second: (usize, usize),
) -> HashSet<(usize, usize)> {
    let (first_x, first_y) = first;
    let (second_x, second_y) = second;

    if first_x == second_x {
        let min_y = usize::min(first_y, second_y);
        let max_y = usize::max(first_y, second_y);

        (min_y..=max_y).map(|y| (first_x, y)).collect()
    } else if first_y == second_y {
        let min_x = usize::min(first_x, second_x);
        let max_x = usize::max(first_x, second_x);

        (min_x..=max_x).map(|x| (x, first_y)).collect()
    } else {
        panic!()
    }
}

fn coordinate(i: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(unsigned_number, tag(","), unsigned_number)(i)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_transform_to_set_of_points() {
        let points = transform_to_set_of_points((498, 6), (496, 6));

        assert_eq!(
            points,
            vec![(498, 6), (497, 6), (496, 6)].into_iter().collect()
        );
    }

    #[test]
    fn test_add_sand() {
        let mut falling_sand = parse("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9");

        let expected_rocks: HashSet<(usize, usize)> = vec![
            (496, 6),
            (497, 6),
            (498, 6),
            (498, 5),
            (498, 4),
            (494, 9),
            (495, 9),
            (496, 9),
            (497, 9),
            (498, 9),
            (499, 9),
            (500, 9),
            (501, 9),
            (502, 9),
            (502, 8),
            (502, 7),
            (502, 6),
            (502, 5),
            (502, 4),
            (503, 4),
        ]
        .into_iter()
        .collect();

        assert_eq!(falling_sand.rocks.len(), expected_rocks.len());
        assert_eq!(falling_sand.rocks, expected_rocks);

        falling_sand.add_sand();
        assert_eq!(falling_sand.sand, vec![(500, 8)].into_iter().collect());

        falling_sand.add_sand();
        assert_eq!(
            falling_sand.sand,
            vec![(500, 8), (499, 8)].into_iter().collect()
        );

        falling_sand.add_sand();
        falling_sand.add_sand();
        falling_sand.add_sand();

        assert_eq!(
            falling_sand.sand,
            vec![(500, 8), (499, 8), (498, 8), (501, 8), (500, 7)]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn test_add_sand_until_flowing() {
        let mut falling_sand = parse("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9");

        let count = falling_sand.add_sand_until_flowing();

        assert_eq!(count, 24);
    }
}
