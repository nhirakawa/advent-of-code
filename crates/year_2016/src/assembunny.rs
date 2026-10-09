use anyhow::{Context, anyhow, bail};
use itertools::Itertools;
use log::{debug, trace};
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct AssembunnyInterpreter {
    argument_mode: ArgumentMode,
    registers: [i64; 4],
    instructions: Vec<Instruction>,
    program_counter: usize,
    outputs: Vec<i64>,
}

impl AssembunnyInterpreter {
    pub fn new(instructions: Vec<Instruction>, argument_mode: ArgumentMode) -> Self {
        Self {
            argument_mode,
            instructions,
            ..Default::default()
        }
    }

    pub fn read_outputs(&self) -> &[i64] {
        &self.outputs
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        self.run_limited(u64::MAX)
    }

    pub fn run_limited(&mut self, max_instructions: u64) -> anyhow::Result<()> {
        let mut steps: u64 = 0;
        while let Some(instruction) = self.instructions.get(self.program_counter).copied() {
            if steps >= max_instructions {
                return Ok(());
            }

            trace!(
                "step={steps} pc={} {instruction:?} registers={:?}",
                self.program_counter, self.registers
            );
            steps += 1;
            match instruction {
                Instruction::Copy(Argument::Register(from), Argument::Register(to)) => {
                    let value = self.read_from_register(from);
                    self.write_to_register(value, to);
                }
                Instruction::Copy(Argument::Literal(value), Argument::Register(to)) => {
                    self.write_to_register(value, to);
                }
                Instruction::Copy(_, Argument::Literal(_)) => {
                    if matches!(self.argument_mode, ArgumentMode::Strict) {
                        bail!("cpy must write to a register");
                    }
                }
                Instruction::Increment(Argument::Register(register)) => {
                    let value = self.read_from_register(register);
                    self.write_to_register(value + 1, register);
                }
                Instruction::Increment(Argument::Literal(_)) => {
                    if matches!(self.argument_mode, ArgumentMode::Strict) {
                        bail!("inc must write to a register");
                    }
                }
                Instruction::Decrement(Argument::Register(register)) => {
                    let value = self.read_from_register(register);
                    self.write_to_register(value - 1, register);
                }
                Instruction::Decrement(Argument::Literal(_)) => {
                    if matches!(self.argument_mode, ArgumentMode::Strict) {
                        bail!("dec must write to a register");
                    }
                }
                Instruction::JumpNotZero(argument, offset) => {
                    if self.resolve(argument) != 0 {
                        let offset = self.resolve(offset).try_into()?;
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
                Instruction::Toggle(Argument::Register(register)) => {
                    let offset = self.read_from_register(register);
                    self.toggle_at_offset(offset)?;
                }
                Instruction::Toggle(Argument::Literal(offset)) => {
                    self.toggle_at_offset(offset)?;
                }
                Instruction::Output(argument) => {
                    let output = self.resolve(argument);
                    self.outputs.push(output);
                }
            };
            self.program_counter += 1;
        }

        debug!(
            "halted after {steps} steps: pc={} registers={:?}",
            self.program_counter, self.registers
        );
        Ok(())
    }

    fn toggle_at_offset(&mut self, offset: i64) -> anyhow::Result<()> {
        let offset = offset.try_into()?;
        let program_counter = self.program_counter;
        if let Some(index) = program_counter.checked_add_signed(offset)
            && let Some(instruction) = self.instructions.get_mut(index)
        {
            let toggled = instruction.toggle();
            debug!("pc={program_counter} toggled index {index}: {instruction:?} -> {toggled:?}");
            *instruction = toggled;
            return Ok(());
        }
        debug!("pc={program_counter} toggle offset {offset} is out of bounds; ignoring");
        Ok(())
    }

    fn resolve(&self, argument: Argument) -> i64 {
        match argument {
            Argument::Literal(value) => value,
            Argument::Register(register) => self.read_from_register(register),
        }
    }

    pub fn read_from_register(&self, source: Register) -> i64 {
        match source {
            Register::A => self.registers[0],
            Register::B => self.registers[1],
            Register::C => self.registers[2],
            Register::D => self.registers[3],
        }
    }

    pub fn write_to_register(&mut self, value: i64, destination: Register) {
        let index = match destination {
            Register::A => 0,
            Register::B => 1,
            Register::C => 2,
            Register::D => 3,
        };
        self.registers[index] = value;
    }
}

pub fn from_program(
    program: &str,
    argument_mode: ArgumentMode,
) -> anyhow::Result<AssembunnyInterpreter> {
    let mut instructions = Vec::new();
    for (index, line) in program.lines().enumerate() {
        let instruction = Instruction::from_str(line)
            .with_context(|| format!("Could not parse line {index} ('{line}')"))?;
        instructions.push(instruction);
    }
    Ok(AssembunnyInterpreter::new(instructions, argument_mode))
}

#[derive(Debug, Default, Copy, Clone)]
pub enum ArgumentMode {
    #[default]
    Strict,
    Lenient,
}

#[derive(Debug, Copy, Clone)]
pub enum Register {
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
pub enum Argument {
    Literal(i64),
    Register(Register),
}

impl FromStr for Argument {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(integer) = s.parse() {
            Ok(Argument::Literal(integer))
        } else if let Ok(register) = Register::from_str(s) {
            Ok(Argument::Register(register))
        } else {
            bail!("Could not parse {s} as integer literal or register")
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Copy(Argument, Argument),
    Increment(Argument),
    Decrement(Argument),
    JumpNotZero(Argument, Argument),
    Toggle(Argument),
    Output(Argument),
}

impl Instruction {
    fn toggle(&self) -> Instruction {
        match self {
            Instruction::Copy(value, to) => Instruction::JumpNotZero(*value, *to),
            Instruction::Increment(register) => Instruction::Decrement(*register),
            Instruction::Decrement(register) => Instruction::Increment(*register),
            Instruction::JumpNotZero(argument, offset) => Instruction::Copy(*argument, *offset),
            Instruction::Toggle(offset) => Instruction::Increment(*offset),
            Instruction::Output(argument) => Instruction::Increment(*argument),
        }
    }
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

            Ok(Instruction::Copy(value, Argument::Register(register)))
        } else if tokens[0] == "inc" {
            let register = tokens
                .get(1)
                .ok_or(anyhow!("Could not get register for inc instruction"))?;
            let register = Register::from_str(register)?;
            Ok(Instruction::Increment(Argument::Register(register)))
        } else if tokens[0] == "dec" {
            let register = tokens
                .get(1)
                .ok_or(anyhow!("Could not get register for dec instruction"))?;
            let register = Register::from_str(register)?;
            Ok(Instruction::Decrement(Argument::Register(register)))
        } else if tokens[0] == "jnz" {
            let value = tokens
                .get(1)
                .ok_or(anyhow!("Could not get argument for jnz instruction"))?;
            let offset = tokens
                .get(2)
                .ok_or(anyhow!("Could not get offset for jnz instruction"))?;

            let value = Argument::from_str(value)?;
            let offset =
                Argument::from_str(offset).with_context(|| format!("Could not parse {offset}"))?;

            Ok(Instruction::JumpNotZero(value, offset))
        } else if tokens[0] == "tgl" {
            let argument = tokens
                .get(1)
                .ok_or(anyhow!("Could not get offset for tgl instruction"))?;

            let argument = Argument::from_str(argument)?;

            Ok(Instruction::Toggle(argument))
        } else if tokens[0] == "out" {
            let argument = tokens
                .get(1)
                .ok_or(anyhow!("Could not get argument for out instruction"))?;

            let argument = Argument::from_str(argument)?;

            Ok(Instruction::Output(argument))
        } else {
            Err(anyhow!("Invalid instruction {}", tokens[0]))
        }
    }
}
