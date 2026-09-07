use anyhow::{Context, anyhow, bail};
use num_traits::One;
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut computer = Computer::from_str(input)?;
    computer.execute_all(1_000_000)?;
    Ok(computer.register_b)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let mut computer = Computer::from_str(input)?;
    computer.register_a = 1;
    computer.execute_all(1_000_000)?;
    Ok(computer.register_b)
}

#[derive(Default)]
struct Computer {
    register_a: usize,
    register_b: usize,
    instructions: Vec<Instruction>,
    program_counter: usize,
}

impl Computer {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            ..Default::default()
        }
    }

    fn execute_all(&mut self, maximum_instructions: usize) -> anyhow::Result<()> {
        for _i in 0..maximum_instructions {
            let should_halt = self.execute();
            if should_halt {
                return Ok(());
            }
        }

        bail!("Too many instructions to execute")
    }

    // TODO convert to explicit enum
    /// Returns true if execution should halt, false if execution should continue
    fn execute(&mut self) -> bool {
        if self.program_counter >= self.instructions.len() {
            return true;
        }

        let instruction = &self.instructions[self.program_counter];
        match instruction {
            Instruction::Half(register) => {
                match register {
                    Register::A => self.register_a /= 2,
                    Register::B => self.register_b /= 2,
                };
                self.program_counter += 1;
            }
            Instruction::Triple(register) => {
                match register {
                    Register::A => self.register_a *= 3,
                    Register::B => self.register_b *= 3,
                };
                self.program_counter += 1;
            }
            Instruction::Increment(register) => {
                match register {
                    Register::A => self.register_a += 1,
                    Register::B => self.register_b += 1,
                };
                self.program_counter += 1;
            }
            Instruction::JumpOffset(offset) => {
                if *offset > 0 {
                    self.program_counter += offset.abs() as usize;
                } else {
                    self.program_counter -= offset.abs() as usize;
                }
            }
            Instruction::JumpIfEven(register, offset) => {
                let predicate_result = match register {
                    Register::A => self.register_a % 2 == 0,
                    Register::B => self.register_b % 2 == 0,
                };
                if predicate_result {
                    if *offset > 0 {
                        self.program_counter += offset.abs() as usize;
                    } else {
                        self.program_counter -= offset.abs() as usize;
                    }
                } else {
                    self.program_counter += 1;
                }
            }
            Instruction::JumpIfOne(register, offset) => {
                let predicate_result = match register {
                    Register::A => self.register_a.is_one(),
                    Register::B => self.register_b.is_one(),
                };
                if predicate_result {
                    if *offset > 0 {
                        self.program_counter += offset.abs() as usize;
                    } else {
                        self.program_counter -= offset.abs() as usize;
                    }
                } else {
                    self.program_counter += 1;
                }
            }
        };

        false
    }
}

impl FromStr for Computer {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();
        for line in s.lines() {
            let instruction =
                Instruction::from_str(line).with_context(|| format!("Invalid line: {}", line))?;
            instructions.push(instruction);
        }
        Ok(Self::new(instructions))
    }
}

enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let "a" = s {
            Ok(Register::A)
        } else if let "b" = s {
            Ok(Register::B)
        } else {
            bail!("Invalid register: {}", s)
        }
    }
}

enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    JumpOffset(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, operands) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Invalid instruction: {}", s))?;

        if let "hlf" = instruction {
            let register = operands.trim();
            let register = register
                .parse()
                .with_context(|| format!("Invalid hlf register: {register}"))?;
            Ok(Instruction::Half(register))
        } else if let "tpl" = instruction {
            let register = operands.trim();
            let register = register
                .parse()
                .with_context(|| format!("Invalid tpl operand: {register}"))?;
            Ok(Instruction::Triple(register))
        } else if let "inc" = instruction {
            let register = operands.trim();
            let register = register
                .parse()
                .with_context(|| format!("Invalid inc register: {register}"))?;
            Ok(Instruction::Increment(register))
        } else if let "jmp" = instruction {
            let offset = operands.trim();
            let offset = offset
                .parse()
                .with_context(|| format!("Invalid jmp offset: {offset}"))?;
            Ok(Instruction::JumpOffset(offset))
        } else if let "jie" = instruction {
            let (register, offset) = operands
                .split_once(", ")
                .ok_or_else(|| anyhow!("Invalid operands: {operands}"))?;

            let register = register.trim();
            let register = register
                .parse()
                .with_context(|| format!("Invalid jie register: {register}"))?;

            let offset = offset.trim();
            let offset = offset
                .parse()
                .with_context(|| format!("Invalid jie offset: {offset}"))?;

            Ok(Instruction::JumpIfEven(register, offset))
        } else if let "jio" = instruction {
            let (register, offset) = operands
                .split_once(", ")
                .ok_or_else(|| anyhow!("Invalid instruction: {}", s))?;

            let register = register.trim();
            let register = register
                .parse()
                .with_context(|| format!("Invalid jio register: {register}"))?;

            let offset = offset.trim();
            let offset = offset
                .parse()
                .with_context(|| format!("Invalid jio offset: {offset}"))?;

            Ok(Instruction::JumpIfOne(register, offset))
        } else {
            bail!("Invalid instruction: {}", s)
        }
    }
}
