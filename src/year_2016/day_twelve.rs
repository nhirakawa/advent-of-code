use anyhow::{Context, anyhow, bail};
use itertools::Itertools;
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut interpreter = AssembunnyInterpreter::from_str(input)?;
    interpreter.run()?;
    Ok(interpreter.registers[0])
}
pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let mut interpreter = AssembunnyInterpreter::from_str(input)?;
    interpreter.registers[2] = 1;
    interpreter.run()?;
    Ok(interpreter.registers[0])
}

#[derive(Debug, Default)]
struct AssembunnyInterpreter {
    registers: [i64; 4],
    instructions: Vec<Instruction>,
    program_counter: usize,
}

impl AssembunnyInterpreter {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            ..Default::default()
        }
    }

    fn run(&mut self) -> anyhow::Result<()> {
        while let Some(instruction) = self.instructions.get(self.program_counter).copied() {
            match instruction {
                Instruction::Copy(Argument::Register(from), to) => {
                    let value = self.read_from_register(from);
                    self.write_to_register(value, to);
                }
                Instruction::Copy(Argument::Literal(value), to) => {
                    self.write_to_register(value, to);
                }
                Instruction::Increment(register) => {
                    let value = self.read_from_register(register);
                    self.write_to_register(value + 1, register);
                }
                Instruction::Decrement(register) => {
                    let value = self.read_from_register(register);
                    self.write_to_register(value - 1, register);
                }
                Instruction::JumpNotZero(argument, offset) => {
                    let value = match argument {
                        Argument::Literal(value) => value,
                        Argument::Register(register) => self.read_from_register(register),
                    };

                    if value != 0 {
                        let offset = offset.try_into()?;
                        let modified_program_counter =
                            self.program_counter.checked_add_signed(offset);

                        match modified_program_counter {
                            None => {
                                // overflow or underflow - either is out of bounds
                                return Ok(());
                            }
                            Some(program_counter) => {
                                self.program_counter = program_counter;
                                continue;
                            }
                        }
                    }
                }
            };
            self.program_counter += 1;
        }

        Ok(())
    }

    fn read_from_register(&self, source: Register) -> i64 {
        match source {
            Register::A => self.registers[0],
            Register::B => self.registers[1],
            Register::C => self.registers[2],
            Register::D => self.registers[3],
        }
    }

    fn write_to_register(&mut self, value: i64, destination: Register) {
        let index = match destination {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
            Register::D => 3,
        };
        self.registers[index] = value;
    }
}

impl FromStr for AssembunnyInterpreter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = Vec::new();
        for (index, line) in s.lines().enumerate() {
            let instruction = Instruction::from_str(line)
                .with_context(|| format!("Could not parse line {index}"))?;
            instructions.push(instruction);
        }
        Ok(Self::new(instructions))
    }
}

#[derive(Debug, Copy, Clone)]
enum Register {
    A,
    B,
    C,
    D,
}

impl FromStr for Register {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => Err(anyhow!("Invalid register {s}")),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Argument {
    Literal(i64),
    Register(Register),
}

impl FromStr for Argument {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(integer) = i64::from_str_radix(s, 10) {
            Ok(Argument::Literal(integer))
        } else if let Ok(register) = Register::from_str(s) {
            Ok(Argument::Register(register))
        } else {
            bail!("Could not parse {s} as integer literal or register")
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Copy(Argument, Register),
    Increment(Register),
    Decrement(Register),
    JumpNotZero(Argument, i64),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            bail!("Cannot parse instruction from empty string");
        }

        let tokens = s.split(" ").collect_vec();

        if tokens.len() < 2 {
            bail!("Expected at least 2 tokens, found {}", tokens.len());
        }

        if tokens[0] == "cpy" {
            let value = tokens
                .get(1)
                .ok_or(anyhow!("Could not get argument for cpy instruction"))?;
            let register = tokens
                .get(2)
                .ok_or(anyhow!("Could not get register for cpy instruction"))?;

            let value = Argument::from_str(value)?;
            let register = Register::from_str(register)?;

            Ok(Instruction::Copy(value, register))
        } else if tokens[0] == "inc" {
            let register = tokens
                .get(1)
                .ok_or(anyhow!("Could not get register for inc instruction"))?;
            let register = Register::from_str(register)?;
            Ok(Instruction::Increment(register))
        } else if tokens[0] == "dec" {
            let register = tokens
                .get(1)
                .ok_or(anyhow!("Could not get register for dec instruction"))?;
            let register = Register::from_str(register)?;
            Ok(Instruction::Decrement(register))
        } else if tokens[0] == "jnz" {
            let value = tokens
                .get(1)
                .ok_or(anyhow!("Could not get argument for jnz instruction"))?;
            let offset = tokens
                .get(2)
                .ok_or(anyhow!("Could not get offset for jnz instruction"))?;

            let value = Argument::from_str(value)?;
            let offset = i64::from_str(offset)?;

            Ok(Instruction::JumpNotZero(value, offset))
        } else {
            Err(anyhow!("Invalid instruction {}", tokens[0]))
        }
    }
}
