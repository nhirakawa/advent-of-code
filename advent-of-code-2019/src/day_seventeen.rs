use std::collections::{HashMap, HashSet};
use std::thread;

use common::prelude::AdventOfCodeResult;

use common::prelude::*;
use itertools::Itertools;

use crate::computer::Computer;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-17.txt");

    let part_one = part_one(input);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(program: &str) -> PartAnswer {
    let start = SystemTime::now();

    let mut computer = Computer::from_program(program);

    computer.step_until_halt();

    let mut map = HashMap::new();

    /*
     * Coordinate system
     * top-left is (0,0)
     * Moving right is positive x
     * Moving down is positive y
     */

    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut max_x = 0;
    let mut max_y = 0;

    while let Some(output) = computer.get_output() {
        let status = output as u8 as char;

        if status == '\n' {
            y += 1;
            x = 0;
            continue;
        } else {
            map.insert((x, y), status);
            x += 1;
        }

        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    let mut intersections = HashSet::new();

    // intersections cannot be at borders
    for x in 1..max_x - 1 {
        for y in 1..max_y - 1 {
            let status = map.get(&(x, y)).unwrap();

            if *status != '#' {
                continue;
            }

            let neighbors = &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .iter()
                .map(|c| map.get(c))
                .flatten()
                .filter(|c| **c == '#')
                .count();

            if *neighbors == 4 {
                println!("({x}, {y}) -> {}", x * y);
                intersections.insert((x, y));
            }
        }
    }

    for y in 0..=max_y {
        for x in 0..max_x {
            if intersections.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{}", map.get(&(x, y)).unwrap());
            }
        }
        println!();
    }

    thread::sleep(Duration::from_millis(100));

    let alignment_sum = sum_alignment_parameters(intersections);

    PartAnswer::new(alignment_sum, start.elapsed().unwrap())
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn sum_alignment_parameters<I>(intersections: I) -> usize
where
    I: IntoIterator<Item = (usize, usize)>,
{
    intersections.into_iter().map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_alignment_parameters() {
        assert_eq!(
            sum_alignment_parameters([(2, 2), (2, 4), (6, 4), (10, 4)]),
            76
        );
    }
}
