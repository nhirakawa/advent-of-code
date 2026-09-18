use crate::year_2019::computer::{Computer, Data};
use anyhow::anyhow;
use itertools::Itertools;
use std::io;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut computer = Computer::from_program(input);

    // let command_data = [b'C', b'o', b'm', b'm', b'a', b'n', b'd', b'?']
    //     .into_iter()
    //     .map(|b| b as Data)
    //     .collect_vec();

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
