use std::collections::HashSet;

use crate::common::constants;
use crate::common::parse::{finish, unsigned_number};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let instructions = parse(input)?;

    let mut screen = Screen::new(50, 6);

    for instruction in instructions {
        screen.apply(&instruction);
    }

    Ok(screen.pixels.len().to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let instructions = parse(input)?;

    let mut screen = Screen::new(50, 6);

    for instruction in instructions {
        screen.apply(&instruction);
    }

    let mut result = String::new();

    result.push('\n');

    for y in 0..6 {
        for x in 0..50 {
            if screen.pixels.contains(&(x, y)) {
                result.push(constants::solid_square());
            } else {
                result.push(constants::empty_square());
            }
        }
        result.push('\n');
    }

    Ok(result)
}

type Coordinate = (usize, usize);

struct Screen {
    pixels: HashSet<Coordinate>,
    width: usize,
    height: usize,
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        Screen {
            pixels: HashSet::new(),
            width,
            height,
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Rectangle { width, height } => {
                for x in 0..*width {
                    for y in 0..*height {
                        self.pixels.insert((x, y));
                    }
                }
            }
            Instruction::RotateRow { row, amount } => {
                let pixels_in_row = self
                    .pixels
                    .iter()
                    .filter(|(_, y)| y == row)
                    .cloned()
                    .collect::<HashSet<_>>();

                let new_pixels = pixels_in_row
                    .iter()
                    .map(|(x, y)| ((*x + amount) % self.width, *y))
                    .collect::<HashSet<_>>();

                self.pixels = self.pixels.difference(&pixels_in_row).cloned().collect();
                self.pixels = self.pixels.union(&new_pixels).cloned().collect();
            }
            Instruction::RotateColumn { column, amount } => {
                let pixels_in_column = self
                    .pixels
                    .iter()
                    .filter(|(x, _)| x == column)
                    .cloned()
                    .collect::<HashSet<_>>();

                let new_pixels = pixels_in_column
                    .iter()
                    .map(|(x, y)| (*x, (*y + amount) % self.height))
                    .collect::<HashSet<_>>();

                self.pixels = self.pixels.difference(&pixels_in_column).cloned().collect();
                self.pixels = self.pixels.union(&new_pixels).cloned().collect();
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Rectangle { width: usize, height: usize },
    RotateRow { row: usize, amount: usize },
    RotateColumn { column: usize, amount: usize },
}

type Instructions = Vec<Instruction>;

fn parse(i: &str) -> anyhow::Result<Instructions> {
    finish(instructions, i)
}

fn instructions(i: &str) -> IResult<&str, Instructions> {
    separated_list1(tag("\n"), instruction).parse(i)
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((
        rect_instruction,
        rotate_row_instruction,
        rotate_column_instruction,
    ))
    .parse(i)
}

fn rect_instruction(i: &str) -> IResult<&str, Instruction> {
    preceded(
        tag("rect "),
        map(
            separated_pair(unsigned_number, tag("x"), unsigned_number),
            |(width, height)| Instruction::Rectangle { width, height },
        ),
    )
    .parse(i)
}

fn rotate_row_instruction(i: &str) -> IResult<&str, Instruction> {
    preceded(
        tag("rotate row y="),
        map(
            separated_pair(unsigned_number, tag(" by "), unsigned_number),
            |(row, amount)| Instruction::RotateRow { row, amount },
        ),
    )
    .parse(i)
}

fn rotate_column_instruction(i: &str) -> IResult<&str, Instruction> {
    preceded(
        tag("rotate column x="),
        map(
            separated_pair(unsigned_number, tag(" by "), unsigned_number),
            |(column, amount)| Instruction::RotateColumn { column, amount },
        ),
    )
    .parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen() {
        let mut screen = Screen::new(8, 3);

        assert_eq!(screen.pixels.len(), 0);

        screen.apply(&Instruction::Rectangle {
            width: 3,
            height: 2,
        });

        assert_eq!(screen.pixels.len(), 6);
        assert_eq!(
            screen.pixels,
            [(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)]
                .iter()
                .cloned()
                .collect()
        );

        screen.apply(&Instruction::RotateColumn {
            column: 1,
            amount: 1,
        });

        assert_eq!(
            screen.pixels,
            [(0, 0), (2, 0), (0, 1), (1, 1), (2, 1), (1, 2)]
                .iter()
                .cloned()
                .collect()
        );

        screen.apply(&Instruction::RotateRow { row: 0, amount: 4 });

        assert_eq!(
            screen.pixels,
            [(4, 0), (6, 0), (0, 1), (1, 1), (2, 1), (1, 2)]
                .iter()
                .cloned()
                .collect()
        );

        screen.apply(&Instruction::RotateColumn {
            column: 1,
            amount: 1,
        });

        assert_eq!(
            screen.pixels,
            [(1, 0), (4, 0), (6, 0), (0, 1), (2, 1), (1, 2)]
                .iter()
                .cloned()
                .collect()
        )
    }
}
