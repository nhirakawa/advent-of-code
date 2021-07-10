use log::{debug, trace};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{all_consuming, map, map_opt},
    multi::{many0, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    ops::Index,
};
use uuid::Uuid;

pub type Data = i128;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Instruction {
    Add {
        first: Parameter,
        second: Parameter,
        third: Parameter,
    },
    Multiply {
        first: Parameter,
        second: Parameter,
        third: Parameter,
    },
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue {
        first: Parameter,
        second: Parameter,
    },
    JumpIfFalse {
        first: Parameter,
        second: Parameter,
    },
    LessThan {
        first: Parameter,
        second: Parameter,
        third: Parameter,
    },
    Equal {
        first: Parameter,
        second: Parameter,
        third: Parameter,
    },
    AdjustRelativeBase {
        only: Parameter,
    },
    Halt,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Instruction::Add {
                first,
                second,
                third,
            } => {
                write!(
                    f,
                    "({} + {}  = {}) => [{}]",
                    first.get_value(),
                    second.get_value(),
                    first.get_value() + second.get_value(),
                    third.get_address()
                )
            }
            Instruction::Multiply {
                first,
                second,
                third,
            } => {
                write!(
                    f,
                    "({} * {} = {}) => [{}]",
                    first.get_value(),
                    second.get_value(),
                    first.get_value() * second.get_value(),
                    third.get_address()
                )
            }
            _s => {
                write!(f, "{:?}", _s)
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Parameter {
    Immediate(ImmediateParameter),
    Position(PositionParameter),
    Relative(RelativeParameter),
}

impl Parameter {
    fn get_value(&self) -> Data {
        match &self {
            Parameter::Immediate(immediate) => immediate.value,
            Parameter::Position(position) => position.value,
            Parameter::Relative(relative) => relative.value,
        }
    }

    fn get_address(&self) -> usize {
        match &self {
            Parameter::Immediate(immediate) => immediate.program_counter,
            Parameter::Position(position) => position.pointer,
            Parameter::Relative(relative) => relative.address,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct ImmediateParameter {
    program_counter: usize,
    value: Data,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct PositionParameter {
    program_counter: usize,
    pointer: usize,
    value: Data,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct RelativeParameter {
    base: Data,
    offset: Data,
    address: usize,
    value: Data,
}

#[derive(Debug)]
pub struct Computer {
    id: Uuid,
    program_counter: usize,
    relative_base: Data,
    memory: HashMap<usize, Data>,
    inputs: Box<Vec<Data>>,
    input_index: usize,
    outputs: Box<Vec<Data>>,
    output_index: usize,
    is_halted: bool,
}

impl Computer {
    fn new(memory: Vec<Data>, inputs: Box<Vec<Data>>) -> Computer {
        let memory = memory.into_iter().enumerate().collect();

        Computer {
            id: Uuid::new_v4(),
            program_counter: 0,
            relative_base: 0,
            memory,
            is_halted: false,
            inputs,
            input_index: 0,
            outputs: Box::new(Vec::new()),
            output_index: 0,
        }
    }

    pub fn from_program(program: &str) -> Computer {
        let input = Box::new(Vec::new());
        Computer::from_program_and_input(program, input)
    }

    pub fn from_program_and_input(i: &str, inputs: Box<Vec<Data>>) -> Computer {
        let memory = parse_program(i);
        Computer::new(memory, inputs)
    }

    pub fn push_input(&mut self, input: Data) {
        self.inputs.push(input);
    }

    pub fn get_outputs(&self) -> Box<Vec<Data>> {
        self.outputs.clone()
    }

    pub fn has_output(&self) -> bool {
        self.outputs.get(self.output_index).is_some()
    }

    pub fn get_number_of_outputs(&self) -> usize {
        self.outputs.len() - self.output_index
    }

    pub fn get_output(&mut self) -> Option<Data> {
        if let Some(output) = self.outputs.get(self.output_index) {
            self.output_index += 1;
            Some(*output)
        } else {
            None
        }
    }

    pub fn is_halted(&self) -> bool {
        self.is_halted
    }

    pub fn is_blocked_on_input(&self) -> bool {
        let next_instruction = self.fetch_instruction();

        if let Instruction::Input(_) = next_instruction {
            let next_input = self.inputs.get(self.input_index);
            if next_input.is_none() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn fetch_instruction(&self) -> Instruction {
        let raw = format!(
            "{:05}",
            self.memory.get(&self.program_counter).cloned().unwrap_or(0)
        );

        let _third_parameter_mode = raw.get(0..1).unwrap();
        let second_parameter_mode = raw.get(1..2).unwrap();
        let first_parameter_mode = raw.get(2..3).unwrap();

        let op_code = raw.get(3..5).unwrap();

        let instruction = match op_code {
            "01" => {
                let first = self.fetch_parameter(self.program_counter + 1, first_parameter_mode);
                let second = self.fetch_parameter(self.program_counter + 2, second_parameter_mode);
                let third = self.fetch_parameter(self.program_counter + 3, _third_parameter_mode);

                Instruction::Add {
                    first,
                    second,
                    third,
                }
            }
            "02" => {
                let first = self.fetch_parameter(self.program_counter + 1, first_parameter_mode);
                let second = self.fetch_parameter(self.program_counter + 2, second_parameter_mode);
                let third = self.fetch_parameter(self.program_counter + 3, _third_parameter_mode);

                Instruction::Multiply {
                    first,
                    second,
                    third,
                }
            }
            "03" => {
                let first = self.fetch_parameter(self.program_counter + 1, first_parameter_mode);
                Instruction::Input(first)
            }
            "04" => {
                let first = self.fetch_parameter(self.program_counter + 1, first_parameter_mode);
                Instruction::Output(first)
            }
            "05" => {
                let first = self.fetch_parameter(self.program_counter + 1, first_parameter_mode);
                let second = self.fetch_parameter(self.program_counter + 2, second_parameter_mode);

                Instruction::JumpIfTrue { first, second }
            }
            "06" => {
                let first = self.fetch_parameter(self.program_counter + 1, first_parameter_mode);
                let second = self.fetch_parameter(self.program_counter + 2, second_parameter_mode);

                Instruction::JumpIfFalse { first, second }
            }
            "07" => {
                let first = self.fetch_parameter(self.program_counter + 1, first_parameter_mode);
                let second = self.fetch_parameter(self.program_counter + 2, second_parameter_mode);
                let third = self.fetch_parameter(self.program_counter + 3, _third_parameter_mode);

                Instruction::LessThan {
                    first,
                    second,
                    third,
                }
            }
            "08" => {
                let first = self.fetch_parameter(self.program_counter + 1, first_parameter_mode);
                let second = self.fetch_parameter(self.program_counter + 2, second_parameter_mode);
                let third = self.fetch_parameter(self.program_counter + 3, _third_parameter_mode);

                Instruction::Equal {
                    first,
                    second,
                    third,
                }
            }
            "09" => {
                let only = self.fetch_parameter(self.program_counter + 1, first_parameter_mode);

                Instruction::AdjustRelativeBase { only }
            }
            "99" => Instruction::Halt,
            _ => panic!("Could not interpret {:?} as an instruction", op_code),
        };

        instruction
    }

    fn fetch_parameter(&self, program_counter: usize, mode: &str) -> Parameter {
        match mode {
            "0" => {
                let parameter = self.fetch_position_parameter(program_counter);

                Parameter::Position(parameter)
            }
            "1" => {
                let value = self.memory.get(&program_counter).cloned().unwrap_or(0);

                let parameter = ImmediateParameter {
                    program_counter,
                    value,
                };

                Parameter::Immediate(parameter)
            }
            "2" => {
                let base = self.relative_base;
                let offset = self.memory.get(&program_counter).cloned().unwrap_or(0);

                let address = (base + offset) as usize;

                let value = self.memory.get(&address).cloned().unwrap_or(0);

                let parameter = RelativeParameter {
                    base,
                    offset,
                    address,
                    value,
                };

                Parameter::Relative(parameter)
            }
            _ => panic!("{} is not a valid parameter mode", mode),
        }
    }

    fn fetch_position_parameter(&self, program_counter: usize) -> PositionParameter {
        let pointer = self.memory.get(&program_counter).cloned().unwrap_or(0) as usize;
        let value = self.memory.get(&pointer).cloned().unwrap_or(0);

        let parameter = PositionParameter {
            program_counter,
            pointer,
            value,
        };

        parameter
    }

    pub fn step(&mut self) {
        let _old_program_counter = self.program_counter.clone();

        let op_code = self.fetch_instruction();

        debug!("[{}] Executing {:?}", self.program_counter, op_code);

        match op_code {
            Instruction::Add {
                first,
                second,
                third,
            } => {
                self.memory
                    .insert(third.get_address(), first.get_value() + second.get_value());

                self.program_counter += 4;
            }
            Instruction::Multiply {
                first,
                second,
                third,
            } => {
                self.memory
                    .insert(third.get_address(), first.get_value() * second.get_value());

                self.program_counter += 4;
            }
            Instruction::Input(parameter) => {
                trace!("{} - {:?}", self.input_index, self.inputs);
                let input = self.inputs.get(self.input_index);

                if input.is_none() {
                    return;
                }

                self.memory
                    .insert(parameter.get_address(), input.unwrap().clone());

                self.input_index += 1;
                self.program_counter += 2;
            }
            Instruction::Output(position) => {
                self.program_counter += 2;

                self.outputs.push(position.get_value());
            }
            Instruction::JumpIfTrue { first, second } => {
                if first.get_value() != 0 {
                    self.program_counter = second.get_value() as usize;
                } else {
                    self.program_counter += 3;
                }
            }
            Instruction::JumpIfFalse { first, second } => {
                if first.get_value() == 0 {
                    self.program_counter = second.get_value() as usize;
                } else {
                    self.program_counter += 3;
                }
            }
            Instruction::LessThan {
                first,
                second,
                third,
            } => {
                let result = if first.get_value() < second.get_value() {
                    1
                } else {
                    0
                };

                self.memory.insert(third.get_address(), result);

                self.program_counter += 4;
            }
            Instruction::Equal {
                first,
                second,
                third,
            } => {
                let result = if first.get_value() == second.get_value() {
                    1
                } else {
                    0
                };

                self.memory.insert(third.get_address(), result);

                self.program_counter += 4;
            }
            Instruction::AdjustRelativeBase { only } => {
                self.relative_base += only.get_value();

                self.program_counter += 2;
            }
            Instruction::Halt => {
                self.is_halted = true;
            }
        };
    }

    pub fn step_until_halt(&mut self) {
        loop {
            let op_code = self.fetch_instruction();

            if op_code == Instruction::Halt {
                self.is_halted = true;
                return;
            }

            self.step();
        }
    }

    pub fn step_until_output(&mut self) {
        loop {
            let op_code = self.fetch_instruction();

            self.step();

            if let Instruction::Output(_) = op_code {
                return;
            }
        }
    }

    pub fn set(&mut self, index: usize, value: Data) {
        self.memory.insert(index, value);
    }
}

impl Index<usize> for Computer {
    type Output = Data;

    fn index(&self, index: usize) -> &Self::Output {
        self.memory.index(&index)
    }
}

fn parse_program(i: &str) -> Vec<Data> {
    all_consuming(terminated(
        separated_list1(tag(","), number),
        many0(line_ending),
    ))(i)
    .unwrap()
    .1
}

fn number(i: &str) -> IResult<&str, Data> {
    alt((negative_number, unsigned_number))(i)
}

fn unsigned_number(i: &str) -> IResult<&str, Data> {
    map_opt(digit1, |s: &str| s.parse::<Data>().ok())(i)
}

fn negative_number(i: &str) -> IResult<&str, Data> {
    map(preceded(tag("-"), unsigned_number), |d| -d)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut computer = Computer::from_program("1101,10,2,3,8");
        computer.step();

        assert_eq!(computer.program_counter, 4);
        assert_eq!(computer[3], 12);
    }

    #[test]
    fn test_multiply() {
        let mut computer = Computer::from_program("1102,10,2,3,8");
        computer.step();

        assert_eq!(computer.program_counter, 4);
        assert_eq!(computer[3], 20);
    }

    #[test]
    fn test_input() {
        let input = Box::new(vec![4]);
        let mut computer = Computer::new(vec![3, 2, 999], input);
        computer.step();

        assert_eq!(computer.program_counter, 2);
        assert_eq!(computer[2], 4);
    }

    #[test]
    fn test_output() {
        let input = Box::new(vec![10]);
        let mut computer = Computer::new(vec![3, 0, 4, 0, 99], input);
        computer.step();

        assert!(computer.get_outputs().is_empty());

        computer.step();

        assert_eq!(*computer.get_outputs(), vec![10]);
    }

    #[test]
    fn test_fetch_instruction() {
        let computer = Computer::from_program("1002,3,3,4,33");
        let instruction = computer.fetch_instruction();

        assert_eq!(
            instruction,
            Instruction::Multiply {
                first: Parameter::Position(PositionParameter {
                    program_counter: 1,
                    pointer: 3,
                    value: 4
                }),
                second: Parameter::Immediate(ImmediateParameter {
                    program_counter: 2,
                    value: 3
                }),
                third: Parameter::Position(PositionParameter {
                    program_counter: 3,
                    pointer: 4,
                    value: 33
                })
            }
        );
    }

    #[test]
    fn test_jump_position_mode() {
        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let mut computer = Computer::from_program_and_input(input, Box::new(vec![0]));

        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![0]);

        let mut computer = Computer::from_program_and_input(input, Box::new(vec![42]));
        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![1]);
    }

    #[test]
    fn test_jump_immediate_mode() {
        let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let mut computer = Computer::from_program_and_input(input, Box::new(vec![0]));

        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![0]);

        let mut computer = Computer::from_program_and_input(input, Box::new(vec![42]));
        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![1]);
    }

    #[test]
    fn test_equal_position_mode() {
        let program = "3,9,8,9,10,9,4,9,99,-1,8";
        let input = Box::new(vec![8]);
        let mut computer = Computer::from_program_and_input(program, input);

        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![1]);

        let input = Box::new(vec![42]);
        let mut computer = Computer::from_program_and_input(program, input);

        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![0]);
    }

    #[test]
    fn test_equal_immediate_mode() {
        let program = "3,3,1108,-1,8,3,4,3,99";
        let input = Box::new(vec![8]);
        let mut computer = Computer::from_program_and_input(program, input);

        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![1]);

        let input = Box::new(vec![9]);
        let mut computer = Computer::from_program_and_input(program, input);

        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![0]);
    }

    #[test]
    fn test_less_than_position_mode() {
        let program = "3,9,7,9,10,9,4,9,99,-1,8";
        let input = Box::new(vec![4]);
        let mut computer = Computer::from_program_and_input(program, input);

        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![1]);

        let input = Box::new(vec![11]);
        let mut computer = Computer::from_program_and_input(program, input);

        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![0]);

        let input = Box::new(vec![8]);
        let mut computer = Computer::from_program_and_input(program, input);

        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![0]);
    }

    #[test]
    fn test_less_than_immediate_mode() {
        let program = "3,3,1107,-1,8,3,4,3,99";
        let input = Box::new(vec![4]);
        let mut computer = Computer::from_program_and_input(program, input);

        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![1]);

        let input = Box::new(vec![11]);
        let mut computer = Computer::from_program_and_input(program, input);

        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![0]);
    }

    #[test]
    fn test_day_five() {
        let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let input = Box::new(vec![900]);

        let mut computer = Computer::from_program_and_input(program, input);

        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![1001]);

        let input = Box::new(vec![8]);
        let mut computer = Computer::from_program_and_input(program, input);

        computer.step_until_halt();

        assert_eq!(*computer.get_outputs(), vec![1000]);
    }

    #[test]
    fn test_jump_if_true() {
        let mut computer = Computer::from_program("1105,6,8");
        computer.step();

        assert_eq!(computer.program_counter, 8);

        let mut computer = Computer::from_program("1105,0,8");
        computer.step();

        assert_eq!(computer.program_counter, 3);

        let mut computer = Computer::from_program("0,5,0,1");
        computer.program_counter = 1;
        computer.step();

        assert_eq!(computer.program_counter, 4);

        let mut computer = Computer::from_program("1,5,0,1");
        computer.program_counter = 1;
        computer.step();

        assert_eq!(computer.program_counter, 5);
    }

    #[test]
    fn test_jump_if_false() {
        let mut computer = Computer::from_program("1106,6,8");
        computer.step();

        assert_eq!(computer.program_counter, 3);

        let mut computer = Computer::from_program("1106,0,8");
        computer.step();

        assert_eq!(computer.program_counter, 8);

        let mut computer = Computer::from_program("0,7,6,0,1");
        computer.program_counter = 2;
        computer.step();

        assert_eq!(computer.program_counter, 7);

        let mut computer = Computer::from_program("1,7,6,0,1");
        computer.program_counter = 2;
        computer.step();

        assert_eq!(computer.program_counter, 5);
    }

    #[test]
    fn test_less_than() {
        let mut computer = Computer::from_program("1107,1,2,3,999");
        computer.step();

        assert_eq!(computer[3], 1);
        assert_eq!(computer.program_counter, 4);

        let mut computer = Computer::from_program("1107,2,1,3,999");
        computer.step();

        assert_eq!(computer[3], 0);
        assert_eq!(computer.program_counter, 4);
    }

    #[test]
    fn test_equals() {
        let mut computer = Computer::from_program("1108,2,2,3,999");
        computer.step();

        assert_eq!(computer[3], 1);
        assert_eq!(computer.program_counter, 4);

        let mut computer = Computer::from_program("1108,1,2,3,999");
        computer.step();

        assert_eq!(computer[3], 0);
        assert_eq!(computer.program_counter, 4);
    }

    #[test]
    fn test_get_output() {
        let mut computer = Computer::from_program("4,1");

        assert_eq!(computer.output_index, 0);
        assert_eq!(computer.get_output(), None);
        assert_eq!(computer.output_index, 0);
        assert_eq!(computer.get_number_of_outputs(), 0);

        computer.step();

        assert_eq!(computer.get_number_of_outputs(), 1);
        assert_eq!(computer.get_output(), Some(1));
        assert_eq!(computer.output_index, 1);
        assert_eq!(computer.get_number_of_outputs(), 0);

        assert_eq!(computer.get_output(), None);
    }

    #[test]
    fn test_quine() {
        let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";

        let mut computer = Computer::from_program(program);

        computer.step_until_halt();

        let mut outputs = Vec::new();

        while let Some(output) = computer.get_output() {
            outputs.push(output);
        }

        assert_eq!(
            outputs,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn test_adjust_relative_base() {
        let program = "109,19,204,-34";
        let mut computer = Computer::from_program(program);
        computer.set(1985, 100);

        computer.relative_base = 2000;

        computer.step();

        assert_eq!(computer.relative_base, 2019);
        assert_eq!(computer.get_output(), None);

        computer.step();

        assert_eq!(computer.relative_base, 2019);
        assert_eq!(computer.get_output(), Some(100));
    }

    #[test]
    fn test_output_large_number() {
        let program = "1102,34915192,34915192,7,4,7,99,0";
        let mut computer = Computer::from_program(program);

        computer.step_until_halt();

        let output = computer.get_output().unwrap();

        assert!(output > 1_000_000_000_000_000);

        let program = "104,1125899906842624,99";
        let mut computer = Computer::from_program(program);

        computer.step_until_halt();

        let output = computer.get_output().unwrap();
        assert_eq!(output, 1125899906842624);
    }
}
