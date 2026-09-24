use anyhow::{anyhow, bail};
use itertools::Itertools;
use std::str::FromStr;

pub fn part_one(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}
pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

#[derive(Debug, PartialEq, Eq)]
struct Data(Vec<bool>);

impl Data {
    fn next(&self) -> Self {
        let copy = self.0.iter().rev().map(|b| !b).collect_vec();
        let updated = std::iter::chain(self.0.iter().copied(), std::iter::once(false));
        let updated = updated.chain(copy);
        let updated = updated.collect_vec();
        Self(updated)
    }
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = Vec::with_capacity(s.len());

        for c in s.chars() {
            let value = match c {
                '0' => false,
                '1' => true,
                _ => bail!("Invalid bit {c}"),
            };
            bits.push(value);
        }

        Ok(Data(bits))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_next() {
        let data = Data::from_str("1").unwrap();
        assert_eq!(data.next().0, vec![true, false, false]);

        let data = Data::from_str("0").unwrap();
        assert_eq!(data.next().0, vec![false, false, true]);

        let data = Data::from_str("11111").unwrap();
        assert_eq!(data.next(), Data::from_str("11111000000").unwrap());

        let data = Data::from_str("111100001010").unwrap();
        assert_eq!(
            data.next(),
            Data::from_str("1111000010100101011110000").unwrap()
        );
    }
}
