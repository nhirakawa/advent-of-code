use crate::year_2019::computer::{Computer, Data};
use anyhow::anyhow;
use itertools::Itertools;
use std::io;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut computer = Computer::from_program(input);

    // to hot chocolate fountain
    computer.push_inputs(to_data("north"));
    computer.push_inputs(to_data("take wreath"));

    // to sick bay
    computer.push_inputs(to_data("east"));

    // to gift wrapping center
    computer.push_inputs(to_data("east"));

    // to navigation
    computer.push_inputs(to_data("east"));
    computer.push_inputs(to_data("take weather machine"));

    // to gift wrapping center
    computer.push_inputs(to_data("west"));

    // to sick bay
    computer.push_inputs(to_data("west"));

    // to hot chocolate fountain
    computer.push_inputs(to_data("west"));

    // to hull breach
    computer.push_inputs(to_data("south"));

    // to hallway
    computer.push_inputs(to_data("south"));

    // to holodeck
    computer.push_inputs(to_data("west"));
    computer.push_inputs(to_data("take prime number"));

    // to stables
    computer.push_inputs(to_data("west"));
    computer.push_inputs(to_data("take astrolabe"));

    // to holodeck
    computer.push_inputs(to_data("east"));

    // to hallway
    computer.push_inputs(to_data("east"));

    // to passages
    computer.push_inputs(to_data("south"));
    computer.push_inputs(to_data("take candy cane"));

    // to hallway
    computer.push_inputs(to_data("north"));

    // to hull breach
    computer.push_inputs(to_data("north"));

    // to observatory
    computer.push_inputs(to_data("east"));
    computer.push_inputs(to_data("take food ration"));

    // to science lab
    computer.push_inputs(to_data("south"));

    // to corridor
    computer.push_inputs(to_data("east"));

    // to engineering
    computer.push_inputs(to_data("south"));
    computer.push_inputs(to_data("take hypercube"));

    // to crew quarters
    computer.push_inputs(to_data("east"));
    computer.push_inputs(to_data("take space lab space brochure"));

    // to security checkpoint
    computer.push_inputs(to_data("north"));

    loop {
        computer.step();

        if let Some(output) = computer.get_output() {
            print!("{}", output as u8 as char);
        }

        if computer.is_blocked_on_input() {
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input)?;
            for c in input.trim().chars() {
                computer.push_input(c as u8 as Data);
            }
            computer.push_input(b'\n' as Data);
        }
    }

    Err::<usize, _>(anyhow!("Not implemented"))
}

fn to_data(command: &str) -> Vec<Data> {
    let mut literal = command.chars().map(|c| c as u8 as Data).collect_vec();
    if literal[literal.len() - 1] != '\n' as u8 as Data {
        literal.push('\n' as u8 as Data);
    }
    literal
}
