use anyhow::bail;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let data = Data::from_str(input)?;
    let data = data.expand(272);
    data.checksum().map(|bits| bits.to_string())
}
pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let data = Data::from_str(input)?;
    let data = data.expand(35651584);
    data.checksum().map(|bits| bits.to_string())
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Data(Vec<bool>);

impl Data {
    fn next(&self) -> Self {
        let copy = self.0.iter().rev().map(|b| !b).collect_vec();
        let updated = std::iter::chain(self.0.iter().copied(), std::iter::once(false));
        let updated = updated.chain(copy);
        let updated = updated.collect_vec();
        Self(updated)
    }

    fn expand(&self, len: usize) -> Self {
        let mut data = self.clone();

        while data.len() < len {
            data = data.next();
        }

        let truncated = data.0[0..len].iter().copied().collect_vec();
        Self(truncated)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn checksum(&self) -> anyhow::Result<Data> {
        let (chunks, []) = self.0.as_chunks::<2>() else {
            bail!(
                "Could not divide bit vector of length {} into twos",
                self.0.len()
            );
        };

        let bits = chunks.iter().map(|[a, b]| a == b).collect_vec();
        let data = Self(bits);
        if data.len().is_multiple_of(2) {
            data.checksum()
        } else {
            Ok(data)
        }
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for bit in &self.0 {
            let value = if *bit { 1 } else { 0 };
            write!(f, "{value}")?;
        }
        Ok(())
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

    #[test]
    fn test_data_checksum() {
        let data = Data::from_str("110010110100").unwrap();
        assert_eq!(data.checksum().unwrap(), Data::from_str("100").unwrap());
    }

    #[test]
    fn test_data_expand() {
        let data = Data::from_str("10000").unwrap();
        assert_eq!(
            data.expand(20),
            Data::from_str("10000011110010000111").unwrap()
        );
    }
}
