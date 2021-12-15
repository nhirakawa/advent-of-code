use std::collections::HashMap;

use common::prelude::*;

use crate::computer::Computer;

pub fn run() -> AdventOfCodeResult {
    let program = include_str!("../input/day-15.txt");

    let part_one = part_one(program);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(program: &str) -> PartAnswer {
    let start = SystemTime::now();

    let mut robot = Robot::new(program);

    let mut number_of_steps = 0;

    loop {
        if robot.step() {
            number_of_steps += 1;
        } else {
            break;
        }
    }

    println!("{}", number_of_steps);

    if let Some(oxygen_system_coordinate) = robot
        .area_map
        .iter()
        .filter(|(_coordinate, status)| **status == Status::OxygenSystem)
        .map(|(coordinate, _)| *coordinate)
        .next()
    {
        println!("{:?}", oxygen_system_coordinate);
    }

    PartAnswer::default()
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

#[derive(Debug)]
struct Robot {
    area_map: HashMap<(isize, isize), Status>,
    current_position: (isize, isize),
    next_direction: u8,
    moves: Vec<((isize, isize), u8)>,
    computer: Computer,
}

impl Robot {
    fn new(program: &str) -> Robot {
        let area_map = HashMap::new();
        let current_position = (0, 0);
        let next_direction = 1;
        let moves = Vec::new();
        let computer = Computer::from_program(program);

        Robot {
            area_map,
            current_position,
            next_direction,
            moves,
            computer,
        }
    }

    // returns true if the robot has a next move
    fn step(&mut self) -> bool {
        // determine next position
        let next_position = match self.next_direction {
            1 => (self.current_position.0, self.current_position.1 + 1),
            2 => (self.current_position.0, self.current_position.1 - 1),
            3 => (self.current_position.0 - 1, self.current_position.1),
            4 => (self.current_position.0 + 1, self.current_position.1),
            _ => panic!(),
        };

        // move the robot and determine status
        self.computer.push_input(self.next_direction.into());
        self.computer.step_until_output();

        let last_output = self.computer.get_output().unwrap();

        // wall
        if last_output == 0 {
            self.area_map.insert(next_position, Status::Wall);
        } else {
            // the robot has moved - update current position
            self.moves.push((next_position, self.next_direction));
            self.current_position = next_position;

            if last_output == 1 {
                self.area_map.insert(next_position, Status::Open);
            } else if last_output == 2 {
                self.area_map.insert(next_position, Status::OxygenSystem);
            } else {
                panic!();
            }
        }

        // check up
        let next_coordinate = (self.current_position.0, self.current_position.1 + 1);
        if !self.area_map.contains_key(&next_coordinate) {
            self.next_direction = 1;
            return true;
        }

        // check down
        let next_coordinate = (self.current_position.0, self.current_position.1 - 1);
        if !self.area_map.contains_key(&next_coordinate) {
            self.next_direction = 2;
            return true;
        }

        // check left
        let next_coordinate = (self.current_position.0 - 1, self.current_position.1);
        if !self.area_map.contains_key(&next_coordinate) {
            self.next_direction = 3;
            return true;
        }

        // check right
        let next_coordinate = (self.current_position.0 + 1, self.current_position.1);
        if !self.area_map.contains_key(&next_coordinate) {
            self.next_direction = 4;
            return true;
        }

        // if no directions are available, go back one space
        if let Some((last_position, last_direction)) = self.moves.pop() {
            self.current_position = last_position;
            return true;
        }

        false
    }
}

#[derive(Debug, PartialEq)]
enum Status {
    Wall,
    Open,
    OxygenSystem,
}
