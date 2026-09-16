use crate::year_2019::computer::{Computer, Data};
use anyhow::{anyhow, bail};
use itertools::Itertools;
use std::collections::VecDeque;

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

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let mut network = Network::new(input, NatBehavior::Last);
    for _ in 0..10_000 {
        network.step()?;

        if network.delivered_to_first_computer.len() >= 2 {
            let first_consecutive_y = network
                .delivered_to_first_computer
                .iter()
                .tuple_windows()
                .find(|(first, second)| first.0[1] == second.0[1]);

            if let Some((Packet([_, y]), _)) = first_consecutive_y {
                return Ok(*y);
            }
        }
    }

    bail!("No solution found")
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Packet([Data; 2]);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    all_computers_idle_counter: usize,
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
            all_computers_idle_counter: 0,
        }
    }

    fn step(&mut self) -> anyhow::Result<()> {
        // Run each computer until it needs more input, and collect packets into destination buffers
        for (index, computer) in self.computers.iter_mut().enumerate() {
            computer.step_until_blocked();

            while computer.get_number_of_outputs() >= 3 {
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
                    let queue = &mut self.packet_queues[destination as usize];
                    queue.push_back(Packet([x, y]));
                }
            }
        }

        let mut number_of_idle_computers = 0;

        // Drain the buffers, and unblock any computers that (will) need input
        for (index, computer) in self.computers.iter_mut().enumerate() {
            let queue = &mut self.packet_queues[index];

            if !queue.is_empty() {
                while let Some(Packet([x, y])) = queue.pop_front() {
                    computer.push_input(x);
                    computer.push_input(y);
                }
            } else if self.nat_behavior == NatBehavior::Last
                && self.all_computers_idle_counter >= 10
                && index == 0
            {
                // Send the last NAT packet to computer#0
                if let Some(Packet([x, y])) = self.nat {
                    computer.push_input(x);
                    computer.push_input(y);
                    self.delivered_to_first_computer.push(Packet([x, y]));
                } else {
                    bail!("No NAT packet to send, but all computers are idle");
                }
            } else if computer.is_blocked_on_input() {
                // No packets available but computer needs to be unblocked
                computer.push_input(-1);
                number_of_idle_computers += 1;
            }
        }

        if number_of_idle_computers == 50 {
            // All computers on the network are idle - increment by one
            self.all_computers_idle_counter += 1;
        } else {
            // At least one computer is either
            // (1) not waiting on input
            // (2) received an input from its packet queue
            self.all_computers_idle_counter = 0;
        }

        Ok(())
    }
}
