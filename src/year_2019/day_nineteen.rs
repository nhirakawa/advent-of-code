use crate::year_2019::computer::Computer;
use anyhow::{anyhow, bail};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut affected_points = 0;
    for x in 0..50 {
        for y in 0..50 {
            let mut computer = Computer::from_program_and_input(input, vec![x, y]);
            computer.step_until_output();
            let drone_status = computer.get_output().ok_or(anyhow!("No output found"))?;
            if drone_status == 0 || drone_status == 1 {
                affected_points += drone_status;
            } else {
                bail!("Invalid output: '{drone_status}'");
            }
        }
    }
    Ok(affected_points)
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}
