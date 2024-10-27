use crate::year_2019::computer::{self, Computer};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let inputs = vec![1];
    let mut computer = Computer::from_program_and_input(input, inputs);
    Ok(run_computer(&mut computer).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let inputs = vec![5];
    let mut computer = Computer::from_program_and_input(input, inputs);
    Ok(run_computer(&mut computer).to_string())
}

fn run_computer(computer: &mut Computer) -> computer::Data {
    computer.step_until_halt();

    let outputs = computer.get_outputs();

    if outputs.len() == 1 {
        return outputs[0];
    }

    for i in 0..outputs.len() - 2 {
        let output = outputs[i];
        if output != 0 {
            panic!("found non-zero output {} ({:?})", output, outputs);
        }
    }

    outputs[outputs.len() - 1]
}
