use crate::common::parse::griderator;
use anyhow::{anyhow, bail};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::successors;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let cells: HashSet<Position> = cells_from_str(input).collect();

    let mut iterations = 0;
    let mut seen = HashSet::new();

    for round in successors(Some(cells), |cells| Some(step(cells))) {
        if iterations >= 10_000 {
            break;
        }

        iterations += 1;

        let biodiversity = biodiversity(&round);

        if !seen.insert(biodiversity) {
            return Ok(biodiversity);
        }
    }

    bail!("No solution found")
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let _cells: HashSet<RecursivePosition> = cells_from_str(input).collect();

    // successors(Some(cells), |cells| Some(step(cells)))
    //     .nth(200)
    //     .map(|cells| cells.len())
    //     .ok_or(anyhow!("No solution found"))

    Err::<u32, _>(anyhow!("Not implemented"))
}

fn step<C: Cell>(bugs: &HashSet<C>) -> HashSet<C> {
    let mut neighbor_counts: HashMap<_, usize> = HashMap::new();

    for bug in bugs {
        for adjacent in bug.adjacent() {
            *neighbor_counts.entry(adjacent).or_default() += 1;
        }
    }

    neighbor_counts
        .into_iter()
        .filter(|(cell, count)| {
            if bugs.contains(cell) {
                *count == 1
            } else {
                *count == 1 || *count == 2
            }
        })
        .map(|(cell, _)| cell)
        .collect()
}

fn biodiversity(cells: &HashSet<Position>) -> u32 {
    cells
        .iter()
        .fold(0, |accumulator, element| accumulator | element.as_bits())
}

trait Cell: Eq + Hash + Copy + Clone + From<(isize, isize)> {
    fn adjacent(&self) -> Vec<Self>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position([isize; 2]);

impl Position {
    #[cfg(test)]
    fn new(x: isize, y: isize) -> Self {
        Self([x, y])
    }

    fn as_bits(&self) -> u32 {
        // Each octet represents a row (with 3 unused bits)
        // Each bit in an octet represents a column in that row (with 3 unused bits)
        // Representation is little-endian
        1 << ((self.0[1] * 5) + self.0[0])
    }
}

impl From<(isize, isize)> for Position {
    fn from((x, y): (isize, isize)) -> Self {
        Self([x, y])
    }
}

impl Cell for Position {
    fn adjacent(&self) -> Vec<Self> {
        let [x, y] = self.0;
        [
            Position([x + 1, y]),
            Position([x - 1, y]),
            Position([x, y + 1]),
            Position([x, y - 1]),
        ]
        .into_iter()
        .filter(|p| (0..5).contains(&p.0[0]) && (0..5).contains(&p.0[1]))
        .collect()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct RecursivePosition([isize; 3]);

impl Cell for RecursivePosition {
    fn adjacent(&self) -> Vec<Self> {
        todo!()
    }
}

impl From<(isize, isize)> for RecursivePosition {
    fn from((x, y): (isize, isize)) -> Self {
        Self([x, y, 0])
    }
}

fn cells_from_str<C: Cell>(s: &str) -> impl Iterator<Item = C> + '_ {
    griderator(s)
        .filter(|(_, c)| *c == '#')
        .map(|(position, _)| position.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_as_bits() {
        let position = Position::new(1, 3);
        assert_eq!(position.as_bits(), 1 << 8);

        let position = Position::new(4, 4);
        assert_eq!(position.as_bits(), 1 << 24);
    }
}
