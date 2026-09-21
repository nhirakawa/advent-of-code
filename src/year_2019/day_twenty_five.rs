use crate::year_2019::computer::{Computer, Data};
use anyhow::anyhow;
use itertools::Itertools;
use std::io;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut computer = Computer::from_program(input);

    let mut computer = AsciiComputer::new(computer);

    // to hot chocolate fountain
    computer.push_inputs("north");
    computer.push_inputs("take wreath");

    // to sick bay
    computer.push_inputs("east");

    // to gift wrapping center
    computer.push_inputs("east");

    // to navigation
    computer.push_inputs("east");
    computer.push_inputs("take weather machine");

    // to gift wrapping center
    computer.push_inputs("west");

    // to sick bay
    computer.push_inputs("west");

    // to hot chocolate fountain
    computer.push_inputs("west");

    // to hull breach
    computer.push_inputs("south");

    // to hallway
    computer.push_inputs("south");

    // to holodeck
    computer.push_inputs("west");
    computer.push_inputs("take prime number");

    // to stables
    computer.push_inputs("west");
    computer.push_inputs("take astrolabe");

    // to holodeck
    computer.push_inputs("east");

    // to hallway
    computer.push_inputs("east");

    // to passages
    computer.push_inputs("south");
    computer.push_inputs("take candy cane");

    // to hallway
    computer.push_inputs("north");

    // to hull breach
    computer.push_inputs("north");

    // to observatory
    computer.push_inputs("east");
    computer.push_inputs("take food ration");

    // to science lab
    computer.push_inputs("south");

    // to corridor
    computer.push_inputs("east");

    // to engineering
    computer.push_inputs("south");
    computer.push_inputs("take hypercube");

    // to crew quarters
    computer.push_inputs("east");
    computer.push_inputs("take space law space brochure");

    // to security checkpoint
    computer.push_inputs("north");

    loop {
        computer.step();

        if computer.is_blocked_on_input() {
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input)?;
            computer.push_inputs(input.trim());
        }
    }

    Err::<usize, _>(anyhow!("Not implemented"))
}

struct AsciiComputer {
    inner: Computer,
    output_lines: Vec<String>,
    buffer: String,
}

impl AsciiComputer {
    fn new(inner: Computer) -> Self {
        Self {
            inner,
            output_lines: Vec::new(),
            buffer: String::new(),
        }
    }

    fn push_inputs(&mut self, command: &str) {
        self.inner.push_inputs(to_data(command));
    }

    fn step(&mut self) {
        self.inner.step();

        if let Some(output) = self.inner.get_output() {
            let c = output as u8 as char;
            if c == '\n' {
                println!("{}", self.buffer);
                self.output_lines.push(self.buffer.clone());
                self.buffer.clear();
            } else {
                self.buffer.push(c);
            }
        }
    }

    fn is_blocked_on_input(&self) -> bool {
        self.inner.is_blocked_on_input()
    }
}

fn to_data(command: &str) -> Vec<Data> {
    let mut literal = command.chars().map(|c| c as u8 as Data).collect_vec();
    if literal[literal.len() - 1] != '\n' as u8 as Data {
        literal.push('\n' as u8 as Data);
    }
    literal
}
