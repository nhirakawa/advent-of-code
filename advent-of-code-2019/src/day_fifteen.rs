use core::panic;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
};

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

    robot.print_area_map();

    let cost = shortest_path_to_oxygen(&robot.area_map);

    PartAnswer::new(cost, start.elapsed().unwrap())
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn shortest_path_to_oxygen(map: &HashMap<(isize, isize), Status>) -> usize {
    let mut queue = VecDeque::new();

    queue.push_back(((0, 0), 0));

    let mut visited = HashSet::new();

    while let Some((coordinate, cost)) = queue.pop_front() {
        if !visited.insert(coordinate) {
            continue;
        }
        println!("checking {:?}", coordinate);
        let status = map.get(&coordinate).copied().unwrap_or(Status::Wall);
        if status == Status::OxygenSystem {
            return cost;
        }

        vec![
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .iter()
        .map(|d| d.apply(coordinate))
        .filter(|c| !visited.contains(c))
        .filter(|c| map.get(c).copied().unwrap_or(Status::Wall) != Status::Wall)
        .for_each(|neighbor| queue.push_back((neighbor, cost + 1)));
    }

    unreachable!()
}

#[derive(Debug)]
enum Navigator {
    Computer(Computer),
    Debug(DebugNavigator),
}

impl Navigator {
    fn advance(&mut self, direction: Direction) -> Status {
        match self {
            Navigator::Computer(computer) => {
                computer.push_input(direction.into());
                computer.step_until_output();
                computer.get_output().unwrap().try_into().unwrap()
            }
            Navigator::Debug(debug_navigator) => debug_navigator.advance(direction),
        }
    }

    fn set_current_position(&mut self, position: (isize, isize)) {
        if let Navigator::Debug(debug) = self {
            debug.current = position;
        }
    }

    fn current_position(&self) -> (isize, isize) {
        match self {
            Navigator::Computer(_) => (0, 0),
            Navigator::Debug(debug) => debug.current_position(),
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
    oxygen_system_coordinate: Option<(isize, isize)>,
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
            oxygen_system_coordinate: None,
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
            oxygen_system_coordinate: None,
        }
    }

    // returns true if the robot has a next move
    fn step(&mut self) -> bool {
        for direction in vec![
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            let next_position = direction.apply(self.current_position);
            if self.area_map.contains_key(&next_position) {
                continue;
            }

            let status = self.navigator.advance(direction);
            self.area_map.insert(next_position, status);

            if status == Status::OxygenSystem {
                if self.oxygen_system_coordinate.is_some() {
                    panic!("already found oxygen system");
                }

                self.oxygen_system_coordinate = Some(next_position);
            }

            if status == Status::Open || status == Status::OxygenSystem {
                println!("moving {:?} to {:?}", direction, next_position);

                self.moves.push((self.current_position, direction));
                self.current_position = next_position;
                return true;
            }
        }

        if let Some((last_position, last_direction)) = self.moves.pop() {
            let reverse_direction = match last_direction {
                Direction::North => Direction::South,
                Direction::West => Direction::East,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
            };
            println!(
                "robot is stuck - moving {:?} back to {:?}",
                reverse_direction, last_position
            );
            self.navigator.advance(reverse_direction);
            self.current_position = last_position;

            return true;
        }

        return false;
    }

    fn print_area_map(&self) {
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;

        let mut max_x = isize::MIN;
        let mut max_y = isize::MIN;

        for (x, y) in self.area_map.keys().copied() {
            min_x = min_x.min(x);
            max_x = max_x.max(x);

            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let out = if x == 0 && y == 0 {
                    "S"
                } else {
                    match self.area_map.get(&(x, y)) {
                        Some(Status::Open) => " ",
                        Some(Status::Wall) => "\u{2588}",
                        Some(Status::OxygenSystem) => "O",
                        None => "",
                    }
                };

                print!("{}", out);
            }
            println!();
        }
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
                if self.current.1 == -2 {
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

    fn current_position(&self) -> (isize, isize) {
        self.current.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn apply(&self, c: (isize, isize)) -> (isize, isize) {
        let (x, y) = c;

        match self {
            Direction::North => (x, y + 1),
            Direction::South => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
    }
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

#[derive(Debug, PartialEq, Copy, Clone)]
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

impl From<i128> for Status {
    fn from(raw: i128) -> Status {
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
    fn test_direction() {
        assert_eq!(Direction::North.apply((0, 0)), (0, 1));
        assert_eq!(Direction::West.apply((1, 24)), (0, 24));
        assert_eq!(Direction::East.apply((6, -19)), (7, -19));
        assert_eq!(Direction::South.apply((9, 9)), (9, 8));
    }

    #[test]
    fn test_debug_navigator_small_walk() {
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

    #[test]
    fn test_debug_navigator_advance_south() {
        let mut debug_navigator = DebugNavigator::new();

        assert_eq!(debug_navigator.advance(Direction::South), Status::Open);
        assert_eq!(debug_navigator.current, (0, -1));

        assert_eq!(debug_navigator.advance(Direction::South), Status::Open);
        assert_eq!(debug_navigator.current, (0, -2));

        assert_eq!(debug_navigator.advance(Direction::South), Status::Wall);
        assert_eq!(debug_navigator.current, (0, -2));
    }

    #[test]
    fn test_debug_navigator() {
        let navigator = DebugNavigator::new();
        let navigator = Navigator::Debug(navigator);

        let mut robot = Robot::new(navigator);

        let mut counter = 0;
        loop {
            assert!(counter < 200);
            assert!(robot.current_position.0.abs() <= 2);
            assert!(robot.current_position.1.abs() <= 2);

            counter += 1;

            let was_successful = robot.step();
            if !was_successful {
                break;
            }
        }

        for x in -2isize..=2isize {
            for y in -2isize..=2isize {
                if x.abs() == 3 && y.abs() == 3 {
                    continue;
                }

                let expected_status = if x.abs() == 3 || y.abs() == 3 {
                    Status::Wall
                } else if x == 1 && y == 1 {
                    Status::OxygenSystem
                } else {
                    Status::Open
                };

                assert_eq!(
                    robot
                        .area_map
                        .get(&(x, y))
                        .copied()
                        .expect(format!("{:?} not found", (x, y)).as_str()),
                    expected_status
                );
            }
        }

        robot.print_area_map();
    }

    #[test]
    fn test_shortest_path() {
        let navigator = DebugNavigator::new();
        let navigator = Navigator::Debug(navigator);

        let mut robot = Robot::new(navigator);

        let mut counter = 0;
        loop {
            assert!(counter < 200);
            assert!(robot.current_position.0.abs() <= 2);
            assert!(robot.current_position.1.abs() <= 2);

            counter += 1;

            let was_successful = robot.step();
            if !was_successful {
                break;
            }
        }

        println!("{}", shortest_path_to_oxygen(&robot.area_map));
    }
}
