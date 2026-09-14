use crate::year_2019::computer::{Computer, Data};
use anyhow::{anyhow, bail};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut affected_points = 0;
    for x in 0..50 {
        for y in 0..50 {
            let is_in_tractor_beam = is_in_tractor_beam(input, x, y)?;
            if is_in_tractor_beam {
                affected_points += 1;
            }
        }
    }
    Ok(affected_points)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let mut start: Data = 0;
    for row in 100..5_000 {
        for column in start.saturating_sub(10)..(start + 500) {
            let bottom_left_corner = is_in_tractor_beam(input, column, row)?;
            let top_right_corner = is_in_tractor_beam(input, column + 99, row - 99)?;

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

fn is_in_tractor_beam(program: &str, x: Data, y: Data) -> anyhow::Result<bool> {
    let mut computer = Computer::from_program_and_input(program, vec![x, y]);
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
