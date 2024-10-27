use crate::year_2019::computer;
use anyhow::bail;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    Ok(run_with_initial_memory(12, 2, input).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    for i in 0..100 {
        for j in 0..100 {
            let output = run_with_initial_memory(i, j, input);

            if output == 19690720 {
                let solution = (100 * i) + j;
                return Ok(solution.to_string());
            }
        }
    }

    bail!("No solution found");
}

fn run_with_initial_memory(
    arg1: computer::Data,
    arg2: computer::Data,
    input: &str,
) -> computer::Data {
    let mut computer = computer::Computer::from_program(input);
    computer.set(1, arg1);
    computer.set(2, arg2);
    computer.step_until_halt();

    computer[0]
}
