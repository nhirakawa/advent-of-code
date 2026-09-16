use crate::year_2019::computer::{Computer, Data};
use anyhow::{anyhow, bail};
use std::collections::{HashMap, VecDeque};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut network = Network::new(input, NatBehavior::First);
    for _ in 0..10_000 {
        network.step()?;

        if let Some(Packet([_, y])) = network.nat {
            return Ok(y);
        }
    }

    bail!("No solution found")
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Packet([Data; 2]);

enum NatBehavior {
    /// Network.nat will hold the first packet addressed to the NAT
    First,
    /// Network.nat will hold the last packet addressed to the NAT
    Last,
}

struct Network {
    nat_behavior: NatBehavior,
    /// The computers in the network
    computers: [Computer; 50],
    /// The inbound packet queues for each computer
    packet_queues: [VecDeque<Packet>; 50],
    /// The last packet sent to the NAT
    nat: Option<Packet>,
    /// A history of the packets delivered to computer#0
    delivered_to_first_computer: Vec<Packet>,
}

impl Network {
    fn new(program: &str, nat_behavior: NatBehavior) -> Self {
        let computers = std::array::from_fn(|index| {
            Computer::from_program_and_input(program, vec![index as Data])
        });
        let packet_queues = std::array::from_fn(|_| VecDeque::new());
        Self {
            nat_behavior,
            computers,
            packet_queues,
            nat: None,
            delivered_to_first_computer: Vec::new(),
        }
    }

    fn step(&mut self) -> anyhow::Result<()> {
        // Step each computer, and collect packets into destination buffers
        for (index, computer) in self.computers.iter_mut().enumerate() {
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
                    match self.nat_behavior {
                        NatBehavior::First => {
                            if self.nat.is_none() {
                                self.nat = Some(Packet([x, y]));
                            }
                        }
                        NatBehavior::Last => {
                            self.nat = Some(Packet([x, y]));
                        }
                    }
                } else {
                    let mut queue = &mut self.packet_queues[destination as usize];
                    queue.push_back(Packet([x, y]));
                }
            }
        }

        // Drain the buffers, and unblock any computers that (will) need input
        for (index, computer) in self.computers.iter_mut().enumerate() {
            let mut queue = &mut self.packet_queues[index];

            if !queue.is_empty() {
                while let Some(Packet([x, y])) = queue.pop_front() {
                    computer.push_input(x);
                    computer.push_input(y);
                }
            } else if computer.is_blocked_on_input() {
                // No packets available but computer needs to be unblocked
                computer.push_input(-1);
            }
        }

        Ok(())
    }
}
