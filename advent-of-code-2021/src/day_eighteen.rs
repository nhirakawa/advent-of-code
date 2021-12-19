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

#[derive(Debug, PartialEq, Clone)]
struct Pair {
    left: SnailfishNumber,
    right: SnailfishNumber,
}

#[derive(Debug, PartialEq, Clone)]
enum SnailfishNumber {
    Regular(u32),
    Pair(Box<Pair>),
}

impl Add for SnailfishNumber {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        Pair {
            left: self,
            right: rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_snailfish_number() {
        let left = SnailfishNumber::Regular(1);
        let right = SnailfishNumber::Regular(2);

        assert_eq!(left.clone() + right.clone(), Pair { left, right });

        let left = SnailfishNumber::Regular(3);
        let right = SnailfishNumber::Pair(Box::new(Pair {
            left: SnailfishNumber::Regular(4),
            right: SnailfishNumber::Regular(5),
        }));

        assert_eq!(left.clone() + right.clone(), Pair { left, right });
    }
}
