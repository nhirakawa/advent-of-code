use std::{fmt::Display, str::FromStr};

use anyhow::{anyhow, bail};
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let operations = operations(input)?;
    let mut password = Password::from_str("abcdefgh")?;

    for operation in operations {
        password.apply(operation)?;
    }

    Ok(password)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let operations = operations(input)?;

    let scrambled = Password::from_str("fbgdceah")?;

    for permutation in scrambled.0.iter().copied().permutations(scrambled.0.len()) {
        let mut password = Password(permutation.clone());
        apply_operations(&mut password, &operations)?;
        if password == scrambled {
            let unscrambled = Password(permutation);
            return Ok(unscrambled);
        }
    }

    bail!("No solution found")
}

fn apply_operations(password: &mut Password, operations: &[Operation]) -> anyhow::Result<()> {
    for operation in operations {
        password.apply(*operation)?;
    }
    Ok(())
}

fn operations(s: &str) -> anyhow::Result<Vec<Operation>> {
    let mut operations = Vec::new();

    for line in s.lines() {
        let operation = Operation::from_str(line)?;
        operations.push(operation);
    }

    Ok(operations)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Password(Vec<u8>);

impl Password {
    fn apply(&mut self, operation: Operation) -> anyhow::Result<()> {
        match operation {
            Operation::SwapIndex(x, y) => {
                if x >= self.0.len() {
                    bail!("{x} is greater than max index {}", self.0.len() - 1);
                }
                if y >= self.0.len() {
                    bail!("{y} is greater than max index {}", self.0.len() - 1);
                }
                self.0.swap(x, y);
                Ok(())
            }
            Operation::SwapLetter(x, y) => {
                let index_of_x = self.index_of(x)?;
                let index_of_y = self.index_of(y)?;
                self.0.swap(index_of_x, index_of_y);
                Ok(())
            }
            Operation::RotateLeft(offset) => {
                if offset == 0 {
                    return Ok(());
                }

                self.0.rotate_left(offset);

                Ok(())
            }
            Operation::RotateRight(offset) => {
                if offset == 0 {
                    return Ok(());
                }

                self.0.rotate_right(offset);

                Ok(())
            }
            Operation::RotateBase(x) => {
                let index_of_x = self.index_of(x)?;

                let offset = if index_of_x >= 4 {
                    2 + index_of_x
                } else {
                    1 + index_of_x
                };

                let offset = offset % self.0.len();

                self.0.rotate_right(offset);

                Ok(())
            }
            Operation::Reverse(x, y) => {
                if x == y {
                    return Ok(());
                }
                if x > y {
                    bail!("{x} must be less than {y}");
                }

                self.0[x..=y].reverse();
                Ok(())
            }
            Operation::Move(x, y) => {
                if x >= self.0.len() {
                    bail!("Invalid index {x}");
                }
                if y >= self.0.len() {
                    bail!("Invalid index {y}");
                }
                let element = self.0.remove(x);
                self.0.insert(y, element);

                Ok(())
            }
        }
    }

    fn index_of(&self, c: char) -> anyhow::Result<usize> {
        self.0
            .iter()
            .position(|&b| b == c as u8)
            .ok_or(anyhow!("Could not find {c}"))
    }
}

impl FromStr for Password {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Password(s.chars().map(|c| c as u8).collect()))
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.0 {
            write!(f, "{}", *b as char)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operation {
    SwapIndex(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBase(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_ascii_whitespace().collect_vec();

        let operation = match &tokens[0..2] {
            ["swap", "position"] => {
                let x = tokens
                    .get(2)
                    .ok_or(anyhow!("Could not get x for swap index"))?;
                let x = x.parse()?;

                let y = tokens
                    .last()
                    .ok_or(anyhow!("Could not get y for swap index"))?;
                let y = y.parse()?;

                Operation::SwapIndex(x, y)
            }
            ["swap", "letter"] => {
                let x = tokens
                    .get(2)
                    .ok_or(anyhow!("Could not get x for swap letter"))?;
                let y = tokens
                    .last()
                    .ok_or(anyhow!("Could not get y for swap letter"))?;
                if x.len() != 1 {
                    bail!("Expected x to have length 1:'{x}'");
                }
                if y.len() != 1 {
                    bail!("Expected y to have length 1:'{y}'");
                }
                let x = x.chars().next().unwrap();
                let y = y.chars().next().unwrap();

                Operation::SwapLetter(x, y)
            }
            ["rotate", "left"] => {
                let steps = tokens
                    .get(2)
                    .ok_or(anyhow!("Could not get steps for rotate left"))?;
                let steps = steps.parse()?;
                Operation::RotateLeft(steps)
            }
            ["rotate", "right"] => {
                let steps = tokens
                    .get(2)
                    .ok_or(anyhow!("Could not get steps for rotate right"))?;
                let steps = steps.parse()?;
                Operation::RotateRight(steps)
            }
            ["rotate", "based"] => {
                let x = tokens
                    .last()
                    .ok_or(anyhow!("Could not get x for rotate base"))?;
                if x.len() != 1 {
                    bail!("Expected x to have length 1:'{x}'");
                }
                let x = x.chars().next().unwrap();
                Operation::RotateBase(x)
            }
            ["reverse", "positions"] => {
                let x = tokens
                    .get(2)
                    .ok_or(anyhow!("Could not get x for reverse"))?;
                let y = tokens
                    .last()
                    .ok_or(anyhow!("Could not get y for reverse"))?;

                let x = x.parse()?;
                let y = y.parse()?;
                Operation::Reverse(x, y)
            }
            ["move", "position"] => {
                let x = tokens.get(2).ok_or(anyhow!("Could not get x for move"))?;
                let y = tokens.last().ok_or(anyhow!("Could not get y for move"))?;
                let x = x.parse()?;
                let y = y.parse()?;
                Operation::Move(x, y)
            }
            _ => {
                bail!("Invalid operation: {tokens:?}")
            }
        };

        Ok(operation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_index() {
        let mut password = Password(vec![b'a', b'b', b'c', b'd', b'e']);
        password.apply(Operation::SwapIndex(4, 0)).unwrap();
        assert_eq!(password.0, vec![b'e', b'b', b'c', b'd', b'a']);
    }

    #[test]
    fn test_swap_letter() {
        let mut password = Password::from_str("ebcda").unwrap();
        password.apply(Operation::SwapLetter('d', 'b')).unwrap();
        assert_eq!(password, Password::from_str("edcba").unwrap());
    }

    #[test]
    fn test_reverse() {
        let mut password = Password::from_str("edcba").unwrap();
        password.apply(Operation::Reverse(0, 4)).unwrap();
        assert_eq!(password, Password::from_str("abcde").unwrap());
    }

    #[test]
    fn test_rotate_left() {
        let mut password = Password::from_str("abcde").unwrap();
        password.apply(Operation::RotateLeft(1)).unwrap();
        assert_eq!(password, Password::from_str("bcdea").unwrap());
    }

    #[test]
    fn test_rotate_right() {
        let mut password = Password::from_str("bcdea").unwrap();
        password.apply(Operation::RotateRight(1)).unwrap();
        assert_eq!(password, Password::from_str("abcde").unwrap());
    }

    #[test]
    fn test_move() {
        let mut password = Password::from_str("bcdea").unwrap();
        password.apply(Operation::Move(1, 4)).unwrap();
        assert_eq!(password, Password::from_str("bdeac").unwrap());

        password.apply(Operation::Move(3, 0)).unwrap();
        assert_eq!(password, Password::from_str("abdec").unwrap());
    }

    #[test]
    fn test_rotate_base() {
        let mut password = Password::from_str("abdec").unwrap();
        password.apply(Operation::RotateBase('b')).unwrap();
        assert_eq!(password, Password::from_str("ecabd").unwrap());

        password.apply(Operation::RotateBase('d')).unwrap();
        assert_eq!(password, Password::from_str("decab").unwrap());
    }
}
