use crate::year_2019::computer::{Computer, Data};
use anyhow::bail;
use bitvec::macros::internal::funty::Fundamental;
use itertools::Itertools;
use log::debug;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let spring_script = vec![
        "NOT A T", "NOT B J", "OR T J", "NOT C T", "OR T J", "AND D J",
    ];
    let mut computer = Computer::from_program_and_input(
        input,
        to_spring_script(&spring_script, SensorMode::Normal),
    );
    computer.step_until_halt();

    let number_of_outputs = computer.get_number_of_outputs();
    let output = computer.get_outputs();

    let output_string = output
        .iter()
        .map(|d| {
            if *d > 0x7f {
                format!("{d}")
            } else {
                format!("{}", d.as_u8() as char)
            }
        })
        .join("");

    debug!("Received {number_of_outputs} outputs: {output_string}");

    let out_of_ascii_range = output.into_iter().filter(|d| *d > 0x7f).collect_vec();
    if out_of_ascii_range.len() != 1 {
        bail!("Expected 1 output out of ASCII range: {out_of_ascii_range:?}");
    }

    Ok(out_of_ascii_range[0])
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let spring_script = vec![
        "NOT A T", "NOT B J", "OR T J", "NOT C T", "OR T J", "AND D J", "NOT J T", "AND J T",
        "OR E T", "OR H T", "AND T J",
    ];

    let mut computer = Computer::from_program_and_input(
        input,
        to_spring_script(&spring_script, SensorMode::Extended),
    );
    computer.step_until_halt();

    let number_of_outputs = computer.get_number_of_outputs();
    let output = computer.get_outputs();

    let output_string = output
        .iter()
        .map(|d| {
            if *d > 0x7f {
                format!("{d}")
            } else {
                format!("{}", d.as_u8() as char)
            }
        })
        .join("");

    debug!("Received {number_of_outputs} outputs: {output_string}");

    let out_of_ascii_range = output.into_iter().filter(|d| *d > 0x7f).collect_vec();
    if out_of_ascii_range.len() != 1 {
        bail!("Expected 1 output out of ASCII range: {out_of_ascii_range:?}");
    }

    Ok(out_of_ascii_range[0])
}

enum SensorMode {
    Normal,
    Extended,
}

fn to_spring_script(program: &[&str], sensor_mode: SensorMode) -> Vec<Data> {
    let mut data = Vec::new();

    for instruction in program {
        data.append(&mut to_data(*instruction));
        data.push(b'\n' as Data);
    }
    let final_instruction = match sensor_mode {
        SensorMode::Normal => "WALK",
        SensorMode::Extended => "RUN",
    };
    data.append(&mut to_data(final_instruction));
    data.push(b'\n' as Data);

    data
}

fn to_data(instruction: &str) -> Vec<Data> {
    instruction.bytes().map(|b| b as Data).collect_vec()
}
