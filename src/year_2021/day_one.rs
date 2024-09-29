use std::time::SystemTime;
use crate::common::{parse::unsigned_number, answer::*};
use nom::{bytes::complete::tag, multi::separated_list1};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-1.txt");
    let depths = parse(input);

    let part_one = part_one(&depths);
    let part_two = part_two(&depths);

    Ok((part_one, part_two))
}

fn part_one(depths: &[usize]) -> PartAnswer {
    let start = SystemTime::now();
    let mut increases = 0;
    for i in 0..depths.len() - 1 {
        if depths[i + 1] > depths[i] {
            increases += 1;
        }
    }

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(increases, elapsed)
}

fn part_two(depths: &[usize]) -> PartAnswer {
    let start = SystemTime::now();

    let mut increases = 0;

    for i in 0..depths.len() - 3 {
        let first_window = depths[i] + depths[i + 1] + depths[i + 2];
        let second_window = depths[i + 1] + depths[i + 2] + depths[i + 3];

        if second_window > first_window {
            increases += 1;
        }
    }

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(increases, elapsed)
}

fn parse(i: &str) -> Vec<usize> {
    separated_list1(tag("\n"), unsigned_number)(i).unwrap().1
}
