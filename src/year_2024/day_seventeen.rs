use std::{collections::HashSet, convert::TryFrom};

use anyhow::{anyhow, bail};
use itertools::Itertools;
use log::trace;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (register_a, register_b, register_c, instructions) = parse(input)?;

    let mut computer = Computer::new(register_a, register_b, register_c, &instructions);

    computer.step_until_halt(false)?;

    Ok(computer.outputs.into_iter().join(","))
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (_register_a, register_b, register_c, instructions) = parse(input)?;

    let register_a = dfs_quine(register_b, register_c, &instructions)?;

    Ok(register_a)
}

fn dfs_quine(register_b: u128, register_c: u128, program: &[u8]) -> anyhow::Result<u128> {
    let mut visited = HashSet::new();

    let mut stack = vec![];

    for i in 0..8 {
        stack.push(i);
    }

    while let Some(current) = stack.pop() {
        if !visited.insert(current) {
            continue;
        }

        let mut computer = Computer::new(current, register_b, register_c, program);

        computer.step_until_halt(false)?;

        let output = computer.outputs.into_iter().map(|u| u as u8).collect_vec();

        if output == program {
            // Base case
            // If the output of the program is the same as the program itself, then we have found a quine
            return Ok(current);
        }

        if program.ends_with(&output) {
            // The same output is produced for small variations of register A
            // We need to explore each of these variations because they have downstream effects
            for n in (0..8).rev() {
                let register_a = (8 * current) + n;
                stack.push(register_a);
            }
        }
    }

    bail!("No quine found")
}

#[derive(Debug)]
struct Computer<'a> {
    register_a: u128,
    register_b: u128,
    register_c: u128,
    instruction_pointer: usize,
    instructions: &'a [u8],
    outputs: Vec<u128>,
}

impl<'a> Computer<'a> {
    pub fn new(
        register_a: u128,
        register_b: u128,
        register_c: u128,
        instructions: &'a [u8],
    ) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
            instructions,
            outputs: Vec::new(),
        }
    }

    pub fn step_until_halt(&mut self, verbose: bool) -> anyhow::Result<()> {
        while self.instructions.get(self.instruction_pointer).is_some() {
            self.step(verbose)?;
        }

        Ok(())
    }

    pub fn step(&mut self, verbose: bool) -> anyhow::Result<()> {
        let instruction = self
            .instructions
            .get(self.instruction_pointer)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Instruction pointer out of bounds: {}",
                    self.instruction_pointer
                )
            })?;

        let operand = self
            .instructions
            .get(self.instruction_pointer + 1)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Operand pointer out of bounds: {}",
                    self.instruction_pointer + 1
                )
            })?;

        if verbose {
            println!(
                "{} => Executing instruction: {} with operand: {}",
                self.instruction_pointer, instruction, operand
            );
        }

        trace!(
            "{}: Executing instruction: {} with operand: {}",
            self.instruction_pointer,
            instruction,
            operand
        );

        let instruction = Instruction::try_from((*instruction, *operand))?;

        match instruction {
            Instruction::Adv { operand } => {
                let value = match operand {
                    ComboOperand::Literal0 => 0,
                    ComboOperand::Literal1 => 1,
                    ComboOperand::Literal2 => 2,
                    ComboOperand::Literal3 => 3,
                    ComboOperand::RegisterA => self.register_a,
                    ComboOperand::RegisterB => self.register_b,
                    ComboOperand::RegisterC => self.register_c,
                    ComboOperand::Reserved => unreachable!(),
                };

                let numerator = self.register_a;
                let denominator = 2_u128.pow(value as u32);

                self.register_a = numerator / denominator;
                self.instruction_pointer += 2;
            }
            Instruction::Bxl { operand } => {
                let lhs = self.register_b;
                let rhs: u128 = operand.into();

                self.register_b = lhs ^ rhs;
                self.instruction_pointer += 2;
            }
            Instruction::Bst { operand } => {
                let lhs = match operand {
                    ComboOperand::Literal0 => 0,
                    ComboOperand::Literal1 => 1,
                    ComboOperand::Literal2 => 2,
                    ComboOperand::Literal3 => 3,
                    ComboOperand::RegisterA => self.register_a,
                    ComboOperand::RegisterB => self.register_b,
                    ComboOperand::RegisterC => self.register_c,
                    ComboOperand::Reserved => unreachable!(),
                };

                self.register_b = lhs % 8;
                self.instruction_pointer += 2;
            }
            Instruction::Jnz { operand } => {
                if self.register_a == 0 {
                    self.instruction_pointer += 2;
                } else {
                    self.instruction_pointer = operand.into();
                }
            }
            Instruction::Bxc => {
                let lhs = self.register_b;
                let rhs = self.register_c;

                self.register_b = lhs ^ rhs;
                self.instruction_pointer += 2;
            }
            Instruction::Out { operand } => {
                let value = match operand {
                    ComboOperand::Literal0 => 0,
                    ComboOperand::Literal1 => 1,
                    ComboOperand::Literal2 => 2,
                    ComboOperand::Literal3 => 3,
                    ComboOperand::RegisterA => self.register_a,
                    ComboOperand::RegisterB => self.register_b,
                    ComboOperand::RegisterC => self.register_c,
                    ComboOperand::Reserved => unreachable!(),
                };

                self.outputs.push(value % 8);
                self.instruction_pointer += 2;
            }
            Instruction::Bdv { operand } => {
                let value = match operand {
                    ComboOperand::Literal0 => 0,
                    ComboOperand::Literal1 => 1,
                    ComboOperand::Literal2 => 2,
                    ComboOperand::Literal3 => 3,
                    ComboOperand::RegisterA => self.register_a,
                    ComboOperand::RegisterB => self.register_b,
                    ComboOperand::RegisterC => self.register_c,
                    ComboOperand::Reserved => unreachable!(),
                };

                let numerator = self.register_a;
                let denominator = 2_u128.pow(value as u32);

                self.register_b = numerator / denominator;
                self.instruction_pointer += 2;
            }
            Instruction::Cdv { operand } => {
                let value = match operand {
                    ComboOperand::Literal0 => 0,
                    ComboOperand::Literal1 => 1,
                    ComboOperand::Literal2 => 2,
                    ComboOperand::Literal3 => 3,
                    ComboOperand::RegisterA => self.register_a,
                    ComboOperand::RegisterB => self.register_b,
                    ComboOperand::RegisterC => self.register_c,
                    ComboOperand::Reserved => unreachable!(),
                };

                let numerator = self.register_a;
                let denominator = 2_u128.pow(value as u32);

                self.register_c = numerator / denominator;
                self.instruction_pointer += 2;
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Adv { operand: ComboOperand },
    Bxl { operand: LiteralOperand },
    Bst { operand: ComboOperand },
    Jnz { operand: LiteralOperand },
    Bxc,
    Out { operand: ComboOperand },
    Bdv { operand: ComboOperand },
    Cdv { operand: ComboOperand },
}

impl TryFrom<(u8, u8)> for Instruction {
    type Error = anyhow::Error;

    fn try_from((instruction, operand): (u8, u8)) -> Result<Self, Self::Error> {
        match instruction {
            0 => Ok(Self::Adv {
                operand: ComboOperand::try_from(operand)?,
            }),
            1 => Ok(Self::Bxl {
                operand: LiteralOperand::try_from(operand)?,
            }),
            2 => Ok(Self::Bst {
                operand: ComboOperand::try_from(operand)?,
            }),
            3 => Ok(Self::Jnz {
                operand: LiteralOperand::try_from(operand)?,
            }),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out {
                operand: ComboOperand::try_from(operand)?,
            }),
            6 => Ok(Self::Bdv {
                operand: ComboOperand::try_from(operand)?,
            }),
            7 => Ok(Self::Cdv {
                operand: ComboOperand::try_from(operand)?,
            }),
            _ => bail!("Invalid instruction: {}", instruction),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum LiteralOperand {
    Literal0,
    Literal1,
    Literal2,
    Literal3,
    Literal4,
    Literal5,
    Literal6,
    Literal7,
}

#[allow(clippy::from_over_into)]
impl Into<u8> for LiteralOperand {
    fn into(self) -> u8 {
        match self {
            LiteralOperand::Literal0 => 0,
            LiteralOperand::Literal1 => 1,
            LiteralOperand::Literal2 => 2,
            LiteralOperand::Literal3 => 3,
            LiteralOperand::Literal4 => 4,
            LiteralOperand::Literal5 => 5,
            LiteralOperand::Literal6 => 6,
            LiteralOperand::Literal7 => 7,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<usize> for LiteralOperand {
    fn into(self) -> usize {
        match self {
            LiteralOperand::Literal0 => 0,
            LiteralOperand::Literal1 => 1,
            LiteralOperand::Literal2 => 2,
            LiteralOperand::Literal3 => 3,
            LiteralOperand::Literal4 => 4,
            LiteralOperand::Literal5 => 5,
            LiteralOperand::Literal6 => 6,
            LiteralOperand::Literal7 => 7,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<u128> for LiteralOperand {
    fn into(self) -> u128 {
        match self {
            LiteralOperand::Literal0 => 0,
            LiteralOperand::Literal1 => 1,
            LiteralOperand::Literal2 => 2,
            LiteralOperand::Literal3 => 3,
            LiteralOperand::Literal4 => 4,
            LiteralOperand::Literal5 => 5,
            LiteralOperand::Literal6 => 6,
            LiteralOperand::Literal7 => 7,
        }
    }
}

impl TryFrom<u8> for LiteralOperand {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Literal0),
            1 => Ok(Self::Literal1),
            2 => Ok(Self::Literal2),
            3 => Ok(Self::Literal3),
            4 => Ok(Self::Literal4),
            5 => Ok(Self::Literal5),
            6 => Ok(Self::Literal6),
            7 => Ok(Self::Literal7),
            _ => bail!("Invalid value for LiteralOperand: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ComboOperand {
    Literal0,
    Literal1,
    Literal2,
    Literal3,
    RegisterA,
    RegisterB,
    RegisterC,
    Reserved,
}

impl TryFrom<u8> for ComboOperand {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Literal0),
            1 => Ok(Self::Literal1),
            2 => Ok(Self::Literal2),
            3 => Ok(Self::Literal3),
            4 => Ok(Self::RegisterA),
            5 => Ok(Self::RegisterB),
            6 => Ok(Self::RegisterC),
            7 => Ok(Self::Reserved),
            _ => bail!("Invalid value for ComboOperand: {}", value),
        }
    }
}

fn parse(i: &str) -> anyhow::Result<(u128, u128, u128, Vec<u8>)> {
    let (register_a, register_b, register_c, _, program) = i
        .lines()
        .collect_tuple()
        .ok_or_else(|| anyhow!("Invalid input"))?;

    let register_a = register_a[12..].parse()?;
    let register_b = register_b[12..].parse()?;
    let register_c = register_c[12..].parse()?;

    let instructions = program[9..]
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    Ok((register_a, register_b, register_c, instructions))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_one() {
        let mut computer = Computer::new(0, 0, 9, &[2, 6]);
        computer.step(false).unwrap();

        assert_eq!(computer.register_a, 0);
        assert_eq!(computer.register_b, 1);
        assert_eq!(computer.register_c, 9);
        assert_eq!(computer.instruction_pointer, 2);
    }

    #[test]
    fn test_example_two() {
        let mut computer = Computer::new(10, 0, 0, &[5, 0, 5, 1, 5, 4]);
        computer.step(false).unwrap();

        assert_eq!(computer.outputs, vec![0]);

        computer.step(false).unwrap();

        assert_eq!(computer.outputs, vec![0, 1]);

        computer.step(false).unwrap();

        assert_eq!(computer.outputs, vec![0, 1, 2]);
    }

    #[test]
    fn test_example_three() {
        let mut computer = Computer::new(2024, 0, 0, &[0, 1, 5, 4, 3, 0]);

        computer.step_until_halt(false).unwrap();

        assert_eq!(computer.outputs, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);

        assert_eq!(computer.register_a, 0);
    }

    #[test]
    fn test_example_four() {
        // If register B contains 29, the program 1,7 would set register B to 26
        let mut computer = Computer::new(0, 29, 0, &[1, 7]);
        computer.step(false).unwrap();

        assert_eq!(computer.register_a, 0);
        assert_eq!(computer.register_b, 26);
        assert_eq!(computer.register_c, 0);
    }

    #[test]
    fn test_example_five() {
        // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354
        let mut computer = Computer::new(0, 2024, 43690, &[4, 0]);
        computer.step(false).unwrap();

        assert_eq!(computer.register_a, 0);
        assert_eq!(computer.register_b, 44354);
        assert_eq!(computer.register_c, 43690);
    }
}
