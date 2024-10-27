use crate::year_2019::computer::{self, Computer};

pub fn part_one(i: &str) -> anyhow::Result<String> {
    Ok(run_computer(i, 1).to_string())
}

pub fn part_two(i: &str) -> anyhow::Result<String> {
    Ok(run_computer(i, 2).to_string())
}

fn run_computer(i: &str, input: computer::Data) -> computer::Data {
    let mut computer = Computer::from_program(i);
    computer.push_input(input);
    computer.step_until_halt();
    let outputs = computer.get_outputs();
    for i in 0..outputs.len() - 1 {
        if outputs[i] != 0 {
            panic!("found abnormal output in {:?}", outputs);
        }
    }

    outputs.last().cloned().unwrap()
}
