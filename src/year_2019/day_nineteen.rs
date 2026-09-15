use crate::year_2019::computer::{self, Computer, Data};
use anyhow::{anyhow, bail};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let memory = computer::parse_program(input);
    let mut affected_points = 0;
    for x in 0..50 {
        for y in 0..50 {
            let is_in_tractor_beam = is_in_tractor_beam(&memory, x, y)?;
            if is_in_tractor_beam {
                affected_points += 1;
            }
        }
    }
    Ok(affected_points)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let memory = computer::parse_program(input);
    let mut start: Data = 0;
    for row in 100..5_000 {
        for column in start.saturating_sub(10)..(start + 500) {
            let bottom_left_corner = is_in_tractor_beam(&memory, column, row)?;
            let top_right_corner = is_in_tractor_beam(&memory, column + 99, row - 99)?;

            if bottom_left_corner && top_right_corner {
                return Ok((row - 99) + column * 10_000);
            }

            if bottom_left_corner {
                start = column;
                break;
            }
        }
    }

    bail!("No solution found")
}

fn is_in_tractor_beam(memory: &[Data], x: Data, y: Data) -> anyhow::Result<bool> {
    let mut computer = Computer::from_memory(memory.to_vec(), vec![x, y]);
    computer.step_until_output();
    let output = computer.get_output().ok_or(anyhow!("No output"))?;
    if output == 0 {
        Ok(false)
    } else if output == 1 {
        Ok(true)
    } else {
        bail!("Invalid output: '{output}'")
    }
}
