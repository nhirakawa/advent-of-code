use crate::year_2019::computer::{Computer, Data};
use anyhow::anyhow;
use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let computer = Computer::from_program(input);
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

    let items = [
        "candy cane",
        "wreath",
        "hypercube",
        "food ration",
        "weather machine",
        "space law space brochure",
        "prime number",
        "astrolabe",
    ];

    for item in &items {
        computer.push_inputs(&format!("drop {item}"));
    }

    for item_set in items.into_iter().powerset() {
        for item in &item_set {
            computer.push_inputs(&format!("take {item}"));
        }
        computer.push_inputs("west");

        computer.step_until_input();

        for item in &item_set {
            computer.push_inputs(&format!("drop {item}"));
        }
    }

    computer.step_until_input();

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

    fn step_until_input(&mut self) {
        while !self.inner.is_blocked_on_input() {
            self.step();
        }
    }

    fn is_blocked_on_input(&self) -> bool {
        self.inner.is_blocked_on_input()
    }

    fn flush(&mut self) {
        println!("{}", self.buffer);
        self.buffer.clear();
    }
}

fn to_data(command: &str) -> Vec<Data> {
    let mut literal = command.chars().map(|c| c as u8 as Data).collect_vec();
    if literal[literal.len() - 1] != '\n' as u8 as Data {
        literal.push('\n' as u8 as Data);
    }
    literal
}
