use anyhow::bail;
use std::collections::HashMap;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut stones = parse_stones(input)?;

    for _ in 0..25 {
        stones = stones.blink();
    }

    Ok(stones.len())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let mut stones = parse_stones(input)?;

    for _ in 0..75 {
        stones = stones.blink();
    }

    Ok(stones.len())
}

type EngravedInteger = u64;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Stones(HashMap<Stone, usize>);

impl Stones {
    pub fn new<I: IntoIterator<Item = Stone>>(stones: I) -> Stones {
        let mut counts = HashMap::new();

        for stone in stones {
            *counts.entry(stone).or_insert(0) += 1;
        }

        Stones(counts)
    }

    pub fn blink(&self) -> Stones {
        let mut counts = HashMap::new();

        for (stone, count) in &self.0 {
            for new_stone in stone.blink() {
                *counts.entry(new_stone).or_insert(0) += count;
            }
        }

        Stones(counts)
    }

    pub fn len(&self) -> usize {
        self.0.values().sum()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Stone(String);

impl Stone {
    pub fn new<S: ToString>(s: S) -> Stone {
        Stone(s.to_string())
    }

    fn blink(&self) -> Vec<Stone> {
        if self.0 == "0" {
            vec![Stone("1".to_string())]
        } else if self.0.len().is_multiple_of(2) {
            let (left, right) = self.0.split_at(self.0.len() / 2);
            let left = left.parse::<EngravedInteger>().unwrap();
            let right = right.parse::<EngravedInteger>().unwrap();
            vec![Stone::new(left), Stone::new(right)]
        } else {
            let value = self.0.parse::<EngravedInteger>().unwrap();
            vec![Stone::new(value * 2024)]
        }
    }
}

fn parse_stones(input: &str) -> anyhow::Result<Stones> {
    for num in input.split_whitespace() {
        if num.parse::<EngravedInteger>().is_err() {
            bail!("Invalid stone: {}", num);
        }
    }

    Ok(Stones::new(input.split_whitespace().map(Stone::new)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_stone_blink() {
        assert_eq!(Stone::new("0").blink(), vec![Stone::new("1")]);

        assert_eq!(Stone::new("1").blink(), vec![Stone::new("2024")]);

        assert_eq!(
            Stone::new("10").blink(),
            vec![Stone::new("1"), Stone::new("0")]
        );

        assert_eq!(
            Stone::new("99").blink(),
            vec![Stone::new("9"), Stone::new("9")]
        );

        assert_eq!(Stone::new("999").blink(), vec![Stone::new("2021976")]);

        assert_eq!(
            Stone::new("1000").blink(),
            vec![Stone::new("10"), Stone::new("0")]
        );

        assert_eq!(
            Stone::new("2024").blink(),
            vec![Stone::new("20"), Stone::new("24")]
        );
    }

    #[test]
    fn test_repeated_multiple_stones_blink() {
        assert_eq!(
            parse_stones("125 17").unwrap().blink(),
            parse_stones("253000 1 7").unwrap()
        );

        assert_eq!(
            parse_stones("253000 1 7").unwrap().blink(),
            parse_stones("253 0 2024 14168").unwrap()
        );

        assert_eq!(
            parse_stones("253 0 2024 14168").unwrap().blink(),
            parse_stones("512072 1 20 24 28676032").unwrap()
        );

        assert_eq!(
            parse_stones("512072 1 20 24 28676032").unwrap().blink(),
            parse_stones("512 72 2024 2 0 2 4 2867 6032").unwrap()
        );

        assert_eq!(
            parse_stones("512 72 2024 2 0 2 4 2867 6032")
                .unwrap()
                .blink(),
            parse_stones("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32").unwrap()
        );

        assert_eq!(
            parse_stones("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32")
                .unwrap()
                .blink(),
            parse_stones("2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2")
                .unwrap()
        );
    }

    #[test]
    fn test_iterated_multiple_stones_blink() {
        let mut stones = parse_stones("125 17").unwrap();

        for _ in 0..25 {
            stones = stones.blink();
        }

        assert_eq!(stones.len(), 55312);
    }
}
