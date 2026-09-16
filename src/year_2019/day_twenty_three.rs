use crate::year_2019::computer::{Computer, Data};
use anyhow::{anyhow, bail};
use std::collections::{HashMap, VecDeque};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut computers = Vec::with_capacity(50);
    let mut packet_queues = HashMap::with_capacity(50);

    for index in 0..50 {
        computers.push(Computer::from_program_and_input(input, vec![index]));
        packet_queues.insert(index, VecDeque::new());
    }

    for _ in 0..10_000 {
        // Step each computer, and collect packets into destination buffers
        for (index, computer) in computers.iter_mut().enumerate() {
            computer.step();

            if computer.has_output() {
                if computer.get_number_of_outputs() != 3 {
                    continue;
                }

                let destination = computer
                    .get_output()
                    .ok_or(anyhow!("Could not get destination from computer#{index}"))?;
                let x = computer
                    .get_output()
                    .ok_or(anyhow!("Could not get x from computer#{index}"))?;
                let y = computer
                    .get_output()
                    .ok_or(anyhow!("Could not get y from computer#{index}"))?;

                if destination == 255 {
                    return Ok(y);
                }

                let queue = packet_queues
                    .get_mut(&destination)
                    .ok_or(anyhow!("Could not get packet buffer for computer#{index}"))?;
                queue.push_back((x, y));
            }
        }

        // Drain the buffers, and unblock any computers that (will) need input
        for (index, computer) in computers.iter_mut().enumerate() {
            let queue = packet_queues
                .get_mut(&(index as Data))
                .ok_or(anyhow!("Could not get packet queue for computer#{index}"))?;

            if !queue.is_empty() {
                while let Some((x, y)) = queue.pop_front() {
                    computer.push_input(x);
                    computer.push_input(y);
                }
            } else if computer.is_blocked_on_input() {
                // No packets available but computer needs to be unblocked
                computer.push_input(-1);
            }
        }
    }

    bail!("No solution found")
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}
