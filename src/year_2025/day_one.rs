use anyhow::bail;
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let rotations = parse_rotations(input)?;

    let zero_count = turn_dial_iterated(rotations)
        .filter(|&pos| pos == 0)
        .count();

    Ok(zero_count)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let rotations = parse_rotations(input)?;
    let rotations = rotations
        .iter()
        .flat_map(|(direction, amount)| std::iter::repeat_n((*direction, 1), *amount as usize))
        .collect_vec();

    let zero_count = turn_dial_iterated(rotations)
        .filter(|&pos| pos == 0)
        .count();

    Ok(zero_count)
}

fn parse_rotations(input: &str) -> anyhow::Result<Vec<(LeftOrRight, u32)>> {
    let mut output = Vec::new();

    for line in input.lines() {
        let first_char = line
            .chars()
            .next()
            .ok_or(anyhow::anyhow!("No first char found"))?;

        let left_or_right = first_char.try_into()?;

        let amount = line[1..].parse::<u32>()?;

        output.push((left_or_right, amount));
    }

    Ok(output)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum LeftOrRight {
    Left,
    Right,
}

impl TryFrom<char> for LeftOrRight {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(LeftOrRight::Left),
            'R' => Ok(LeftOrRight::Right),
            _ => bail!(""),
        }
    }
}

fn turn_dial(current: u32, direction: LeftOrRight, amount: u32) -> u32 {
    let mut current = current;
    for _ in 0..amount {
        current = match direction {
            LeftOrRight::Left => {
                if current == 0 {
                    99
                } else {
                    current - 1
                }
            }
            LeftOrRight::Right => {
                if current == 99 {
                    0
                } else {
                    current + 1
                }
            }
        };
    }

    current
}

fn turn_dial_iterated(rotations: Vec<(LeftOrRight, u32)>) -> impl Iterator<Item = u32> {
    rotations
        .into_iter()
        .scan(50, |state, (direction, amount)| {
            *state = turn_dial(*state, direction, amount);
            Some(*state)
        })
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn it_moves_dial() {
        assert_eq!(turn_dial(11, LeftOrRight::Right, 8), 19);
        assert_eq!(turn_dial(19, LeftOrRight::Left, 19), 0);

        assert_eq!(turn_dial(0, LeftOrRight::Left, 1), 99);
        assert_eq!(turn_dial(99, LeftOrRight::Right, 1), 0);

        assert_eq!(turn_dial(5, LeftOrRight::Left, 10), 95);
        assert_eq!(turn_dial(95, LeftOrRight::Right, 5), 0);
    }

    #[test]
    fn it_moves_dial_iterated() {
        let rotations = vec![
            (LeftOrRight::Left, 68),
            (LeftOrRight::Left, 30),
            (LeftOrRight::Right, 48),
            (LeftOrRight::Left, 5),
            (LeftOrRight::Right, 60),
            (LeftOrRight::Left, 55),
            (LeftOrRight::Left, 1),
            (LeftOrRight::Left, 99),
            (LeftOrRight::Right, 14),
            (LeftOrRight::Left, 82),
        ];

        let pointers = turn_dial_iterated(rotations).collect_vec();

        assert_eq!(pointers, vec![82, 52, 0, 95, 55, 0, 99, 0, 14, 32]);
    }
}
