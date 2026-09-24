use crate::common::math;
use crate::common::math::Congruence;
use anyhow::{Context, anyhow, bail};
use itertools::Itertools;
use log::info;
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let discs = Discs::from_str(input)?;
    math::chinese_remainder::<Integer, Vec<Congruence<Integer>>>(discs.into())
        .ok_or(anyhow!("No solution found"))
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

type Integer = i64;

struct Discs(Vec<Disc>);

impl FromStr for Discs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut discs = Vec::new();
        for line in s.lines() {
            let disc = Disc::from_str(line)?;
            discs.push(disc);
        }
        Ok(Self(discs))
    }
}

impl Into<Vec<Congruence<Integer>>> for Discs {
    fn into(self) -> Vec<Congruence<Integer>> {
        self.0.iter().copied().map(Disc::into).collect()
    }
}

#[derive(Debug, Copy, Clone)]
struct Disc {
    index: Integer,
    number_of_positions: Integer,
    starting_position: Integer,
}

impl FromStr for Disc {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().replace(['#', ';', '=', ',', '.'], " ");

        let tokens = s.split_ascii_whitespace().collect_vec();

        if tokens.len() != 13 {
            bail!("Expected 13 tokens but found {} - {tokens:?}", tokens.len());
        }

        info!("Tokens: {tokens:?}");

        let index = tokens[1]
            .parse()
            .with_context(|| format!("Could not parse {}", tokens[1]))?;
        let number_of_positions = tokens[3]
            .parse()
            .with_context(|| format!("Could not parse number of positions from {}", tokens[2]))?;
        let starting_position = tokens[12]
            .parse()
            .with_context(|| format!("Could not parse starting position from {}", tokens[12]))?;

        Ok(Disc {
            index,
            number_of_positions,
            starting_position,
        })
    }
}

impl Into<Congruence<Integer>> for Disc {
    fn into(self) -> Congruence<Integer> {
        let residue = -(self.starting_position + self.index);
        let modulus = self.number_of_positions;
        Congruence { residue, modulus }
    }
}
