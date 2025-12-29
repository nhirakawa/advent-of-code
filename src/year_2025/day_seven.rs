use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut manifold = parser::parse(input)?;

    while manifold.can_advance() {
        manifold.advance_beams();
    }

    Ok(manifold.num_triggered_splitters())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let mut manifold = parser::parse(input)?;

    while manifold.can_advance() {
        manifold.advance_beams();
    }

    Ok(manifold.num_beams())
}

#[derive(Debug, PartialEq, Eq)]
struct TachyonManifold {
    start: Position,
    splitters: HashSet<Position>,
    beams: HashMap<Position, usize>,
    triggered_splitters: HashSet<Position>,
}

impl TachyonManifold {
    fn new(start: Position, splitters: HashSet<Position>) -> TachyonManifold {
        let mut beams = HashMap::new();
        beams.insert(start, 1);

        let triggered_splitters = HashSet::new();

        TachyonManifold {
            start,
            splitters,
            beams,
            triggered_splitters,
        }
    }

    fn can_advance(&self) -> bool {
        let max_splitter_y = self
            .splitters
            .iter()
            .map(|position| position.0.1)
            .max()
            .expect("Must have at least 1 splitter");

        if self.beams.keys().all(|beam| beam.0.1 > max_splitter_y) {
            return false;
        }

        true
    }

    fn advance_beams(&mut self) {
        let mut new_beams = HashMap::new();

        for (beam, count) in &self.beams {
            if self.splitters.contains(beam) {
                self.triggered_splitters.insert(*beam);

                let (left, right) = beam.split();
                *new_beams.entry(left).or_default() += count;
                *new_beams.entry(right).or_default() += count;
            } else {
                let down = beam.down();
                *new_beams.entry(down).or_default() += count;
            }
        }

        self.beams = new_beams;
    }

    fn num_triggered_splitters(&self) -> usize {
        self.triggered_splitters.len()
    }

    fn num_beams(&self) -> usize {
        self.beams.values().sum()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Position((isize, isize));

impl Position {
    fn down(&self) -> Position {
        let (x, y) = self.0;
        Position((x, y + 1))
    }

    fn split(&self) -> (Position, Position) {
        let (x, y) = self.0;
        (Position((x - 1, y + 1)), Position((x + 1, y + 1)))
    }
}

mod parser {
    use std::collections::HashSet;

    use anyhow::bail;

    use crate::{
        common::parse::griderator,
        year_2025::day_seven::{Position, TachyonManifold},
    };

    pub fn parse(input: &str) -> anyhow::Result<TachyonManifold> {
        let mut start = None;
        let mut splitters = HashSet::new();

        for (position, c) in griderator(input) {
            if c == 'S' {
                if start.is_none() {
                    start = Some(position);
                } else {
                    bail!("Multiple start positions found");
                }
            } else if c == '^' {
                splitters.insert(Position(position));
            } else if c == '.' {
                continue;
            } else {
                bail!("Invalid character - {c}");
            }
        }

        match start {
            Some(start) => Ok(TachyonManifold::new(Position(start), splitters)),
            None => Err(anyhow::anyhow!("No start found")),
        }
    }
}
