use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-10.txt");

    let operations = parse(input);

    let part_one = part_one(&operations);
    let part_two = part_two(&operations);

    Ok((part_one, part_two))
}

fn part_one(operations: &[Operation]) -> PartAnswer {
    let start = SystemTime::now();

    let mut cpu = Cpu::new();

    for operation in operations {
        cpu.apply(operation);
    }

    let answer: isize = cpu.signal_strength_values.iter().sum();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn part_two(operations: &[Operation]) -> PartAnswer {
    let start = SystemTime::now();

    let mut cpu = Cpu::new();

    for operation in operations {
        cpu.apply(operation);
    }

    let image = cpu
        .pixels
        .chunks_exact(40)
        .map(|v| {
            v.iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("\n");

    let image = "\n".to_string() + &image + "\n";

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(image, elapsed)
}

#[derive(Debug)]
struct Cpu {
    current_cycle: usize,
    current_value: isize,
    signal_strength_values: Vec<isize>,
    pixels: Vec<char>,
    crt_position: usize,
}

impl Cpu {
    fn new() -> Cpu {
        let current_cycle = 1;
        let current_value = 1;
        let signal_strength_values = vec![];
        let pixels = vec![];
        let crt_position = 0;

        Cpu {
            current_cycle,
            current_value,
            signal_strength_values,
            pixels,
            crt_position,
        }
    }

    fn apply(&mut self, operation: &Operation) {
        self.advance_cycle(operation);

        match operation {
            Operation::Noop => {}
            Operation::Add(value) => {
                self.current_value += value;
            }
        }
    }

    fn advance_cycle(&mut self, operation: &Operation) {
        let number_of_cycles = match operation {
            Operation::Noop => 1,
            Operation::Add(_) => 2,
        };

        for _ in 0..number_of_cycles {
            if self.current_cycle == 20
                || self.current_cycle == 60
                || self.current_cycle == 100
                || self.current_cycle == 140
                || self.current_cycle == 180
                || self.current_cycle == 220
            {
                // println!("cycle {}, value {}", self.current_cycle, self.current_value);

                self.signal_strength_values
                    .push(self.current_cycle as isize * self.current_value);
            }

            let pixel_value = if ((self.crt_position as isize % 40) - self.current_value).abs() <= 1
            {
                // one of the three pixel sprites is under the CRT beam
                solid_square()
            } else {
                empty_square()
            };

            // if self.current_cycle <= 21 {
            //     println!(
            //         "[current_cycle={:02}|crt_position={:02}|current_value={:02}] {pixel_value}",
            //         self.current_cycle, self.crt_position, self.current_value
            //     );
            // }

            self.pixels.push(pixel_value);

            self.crt_position += 1;

            self.current_cycle += 1;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Operation {
    Add(isize),
    Noop,
}

fn parse(i: &str) -> Vec<Operation> {
    finish(operations)(i).unwrap().1
}

fn operations(i: &str) -> IResult<&str, Vec<Operation>> {
    separated_list1(tag("\n"), operation)(i)
}

fn operation(i: &str) -> IResult<&str, Operation> {
    alt((add, noop))(i)
}

fn add(i: &str) -> IResult<&str, Operation> {
    map(preceded(tag("addx "), number), Operation::Add)(i)
}

fn noop(i: &str) -> IResult<&str, Operation> {
    value(Operation::Noop, tag("noop"))(i)
}
