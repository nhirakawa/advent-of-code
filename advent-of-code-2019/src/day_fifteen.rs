use std::{collections::HashMap, convert::TryInto};

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

    let mut robot = Robot::from_program(program);

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
enum Navigator {
    Computer(Computer),
    Debug(DebugNavigator),
}

impl Navigator {
    fn advance(&mut self, direction: Direction) {
        match self {
            Navigator::Computer(computer) => computer.push_input(direction.into()),
            Navigator::Debug(debug_navigator) => {
                debug_navigator.advance(direction);
            }
        }
    }

    fn get_current_status(&mut self) -> u8 {
        match self {
            Navigator::Computer(computer) => {
                computer.step_until_output();
                computer.get_output().unwrap().try_into().unwrap()
            }
            Navigator::Debug(debug_navigator) => debug_navigator.get_current_status(),
        }
    }
}

#[derive(Debug)]
struct Robot {
    area_map: HashMap<(isize, isize), Status>,
    current_position: (isize, isize),
    next_direction: Direction,
    moves: Vec<((isize, isize), Direction)>,
    navigator: Navigator,
}

impl Robot {
    fn new(navigator: Navigator) -> Robot {
        let area_map = HashMap::new();
        let current_position = (0, 0);
        let next_direction = Direction::North;
        let moves = Vec::new();
        Robot {
            area_map,
            current_position,
            next_direction,
            moves,
            navigator,
        }
    }

    fn from_program(program: &str) -> Robot {
        let area_map = HashMap::new();
        let current_position = (0, 0);
        let next_direction = Direction::North;
        let moves = Vec::new();
        let computer = Computer::from_program(program);
        let navigator = Navigator::Computer(computer);

        Robot {
            area_map,
            current_position,
            next_direction,
            moves,
            navigator,
        }
    }

    // returns true if the robot has a next move
    fn step(&mut self) -> bool {
        // determine next position
        let next_position = match self.next_direction {
            Direction::North => (self.current_position.0, self.current_position.1 + 1),
            Direction::South => (self.current_position.0, self.current_position.1 - 1),
            Direction::West => (self.current_position.0 - 1, self.current_position.1),
            Direction::East => (self.current_position.0 + 1, self.current_position.1),
        };

        self.navigator.advance(self.next_direction);
        let last_output = self.navigator.get_current_status();

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
            self.next_direction = Direction::North;
            return true;
        }

        // check down
        let next_coordinate = (self.current_position.0, self.current_position.1 - 1);
        if !self.area_map.contains_key(&next_coordinate) {
            self.next_direction = Direction::South;
            return true;
        }

        // check left
        let next_coordinate = (self.current_position.0 - 1, self.current_position.1);
        if !self.area_map.contains_key(&next_coordinate) {
            self.next_direction = Direction::West;
            return true;
        }

        // check right
        let next_coordinate = (self.current_position.0 + 1, self.current_position.1);
        if !self.area_map.contains_key(&next_coordinate) {
            self.next_direction = Direction::East;
            return true;
        }

        // if no directions are available, go back one space
        if let Some((last_position, _last_direction)) = self.moves.pop() {
            self.current_position = last_position;
            return true;
        }

        false
    }
}

#[derive(Debug)]
struct DebugNavigator {
    current: (isize, isize),
}

impl DebugNavigator {
    fn new() -> Self {
        let current = (0, 0);
        Self { current }
    }

    fn advance(&mut self, direction: Direction) -> Status {
        match direction {
            Direction::North => {
                if self.current.1 >= 2 {
                    Status::Wall
                } else {
                    self.current.1 += 1;
                    if self.current == (1, 1) {
                        Status::OxygenSystem
                    } else {
                        Status::Open
                    }
                }
            }
            Direction::South => {
                if self.current.1 <= -2 {
                    Status::Wall
                } else {
                    self.current.1 -= 1;
                    if self.current == (1, 1) {
                        Status::OxygenSystem
                    } else {
                        Status::Open
                    }
                }
            }
            Direction::East => {
                if self.current.0 >= 2 {
                    Status::Wall
                } else {
                    self.current.0 += 1;
                    if self.current == (1, 1) {
                        Status::OxygenSystem
                    } else {
                        Status::Open
                    }
                }
            }
            Direction::West => {
                if self.current.0 <= -2 {
                    Status::Wall
                } else {
                    self.current.0 -= 1;
                    if self.current == (1, 1) {
                        Status::OxygenSystem
                    } else {
                        Status::Open
                    }
                }
            }
        }
    }

    fn get_current_status(&self) -> u8 {
        let (x, y) = self.current;

        if x.abs() >= 2 || y.abs() >= 2 {
            Status::Wall.into()
        } else if x == 1 && y == 1 {
            Status::OxygenSystem.into()
        } else {
            Status::Open.into()
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl From<u8> for Direction {
    fn from(raw: u8) -> Direction {
        match raw {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => unreachable!(),
        }
    }
}

impl Into<crate::computer::Data> for Direction {
    fn into(self) -> crate::computer::Data {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Status {
    Wall,
    Open,
    OxygenSystem,
}

impl From<u8> for Status {
    fn from(raw: u8) -> Status {
        match raw {
            0 => Status::Wall,
            1 => Status::Open,
            2 => Status::OxygenSystem,
            _ => unreachable!(),
        }
    }
}

impl Into<u8> for Status {
    fn into(self) -> u8 {
        match self {
            Status::Wall => 0,
            Status::Open => 1,
            Status::OxygenSystem => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_navigator() {
        let mut debug_navigator = DebugNavigator::new();
        assert_eq!(debug_navigator.current, (0, 0));

        let status = debug_navigator.advance(Direction::North);
        assert_eq!(status, Status::Open);
        assert_eq!(debug_navigator.current, (0, 1));

        let status = debug_navigator.advance(Direction::North);
        assert_eq!(status, Status::Open);
        assert_eq!(debug_navigator.current, (0, 2));

        let status = debug_navigator.advance(Direction::North);
        assert_eq!(status, Status::Wall);
        assert_eq!(debug_navigator.current, (0, 2));

        assert_eq!(debug_navigator.advance(Direction::South), Status::Open);
        assert_eq!(
            debug_navigator.advance(Direction::East),
            Status::OxygenSystem
        );
        assert_eq!(debug_navigator.current, (1, 1));
    }
}
