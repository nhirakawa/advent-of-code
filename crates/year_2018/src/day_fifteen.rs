use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, bail};
use common::parse::griderator;

pub fn part_one(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position(i64, i64);

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
        Self(x, y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Debug, PartialEq, Eq)]
struct Unit {
    unit_type: UnitType,
    position: Position,
    attack: u32,
    health: u32,
}

impl Unit {
    fn new(unit_type: UnitType, position: Position) -> Self {
        Self {
            unit_type,
            position,
            attack: 3,
            health: 200,
        }
    }

    fn new_goblin(position: Position) -> Self {
        Self::new(UnitType::Goblin, position)
    }

    fn new_elf(position: Position) -> Self {
        Self::new(UnitType::Elf, position)
    }
}

struct Grid {
    spaces: HashSet<Position>,
    goblins: Vec<Unit>,
    elves: Vec<Unit>,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spaces = HashSet::new();
        let mut goblins = Vec::new();
        let mut elves = Vec::new();

        for (position, c) in griderator(s) {
            let position = position.into();

            match c {
                '.' => {
                    spaces.insert(position);
                }
                '#' => {
                    continue;
                }
                'G' => {
                    let goblin = Unit::new_goblin(position);
                    goblins.push(goblin);
                }
                'E' => {
                    let elf = Unit::new_elf(position);
                    elves.push(elf);
                }
                _ => {
                    bail!("Unrecognized character: '{c}'");
                }
            }
        }

        Ok(Grid {
            spaces,
            goblins,
            elves,
        })
    }
}
