use anyhow::bail;
use itertools::Itertools;
use log::info;

use crate::assembunny::{self, ArgumentMode};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    for i in 0..1000 {
        let mut interpreter = assembunny::from_program(input, ArgumentMode::Lenient)?;
        interpreter.write_to_register(i, assembunny::Register::A);
        interpreter.run_limited(100_000)?;

        let outputs = interpreter.read_outputs();
        if outputs.is_empty() {
            info!("No outputs for a={i}");
            continue;
        }
        if outputs[0] != 0 {
            info!("outputs for a={i} does not start with 0");
            continue;
        }
        let is_repeating = outputs
            .iter()
            .tuple_windows()
            .all(|(first, second)| *first == 0 && *second == 1 || *first == 1 && *second == 0);

        if is_repeating {
            return Ok(i);
        }
    }

    bail!("No solution found")
}
