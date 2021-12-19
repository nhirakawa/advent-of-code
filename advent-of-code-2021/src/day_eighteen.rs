use std::ops::Add;

use common::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let part_one = part_one();
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one() -> PartAnswer {
    PartAnswer::default()
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

#[derive(Debug)]
struct Pair {
    left: SnailfishNumber,
    right: SnailfishNumber,
}

#[derive(Debug)]
enum SnailfishNumber {
    Regular(u32),
    Pair(Box<Pair>),
}

impl Add for &Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
