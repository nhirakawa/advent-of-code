use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
};
use std::time::SystemTime;
use crate::common::answer::*;
use log::debug;

use crate::year_2019::computer::Computer;

pub fn run() -> AdventOfCodeResult {
    let program = include_str!("input/day-15.txt");

    let part_one = part_one(program);
    let part_two = part_two(program);

    Ok((part_one, part_two))
}

fn part_one(program: &str) -> PartAnswer {
    let start = SystemTime::now();

    let area_map = build_area_map(program);
    let oxygen_coordinates = find_oxygen_system_coordinate(&area_map);

    let search_costs = breadth_first_search(&area_map, (0, 0));

    let oxygen_coordinate_steps = search_costs.get(&oxygen_coordinates).copied().unwrap();

    PartAnswer::new(oxygen_coordinate_steps, start.elapsed().unwrap())
}

fn part_two(program: &str) -> PartAnswer {
    let start = SystemTime::now();

    let area_map = build_area_map(program);
    let oxygen_coordinates = find_oxygen_system_coordinate(&area_map);

    let search_costs = breadth_first_search(&area_map, oxygen_coordinates);

    let number_of_minutes_to_fill = search_costs.values().max().unwrap();

    PartAnswer::new(number_of_minutes_to_fill, start.elapsed().unwrap())
}

fn find_oxygen_system_coordinate(map: &HashMap<(isize, isize), Status>) -> (isize, isize) {
    map.iter()
        .filter(|(_coordinate, status)| **status == Status::OxygenSystem)
        .map(|(coordinate, _)| *coordinate)
        .next()
        .unwrap()
}

fn build_area_map(program: &str) -> HashMap<(isize, isize), Status> {
    let mut robot = Robot::from_program(program);
    loop {
        if !robot.step() {
            break;
        }
    }

    robot.area_map
}

fn breadth_first_search(
    map: &HashMap<(isize, isize), Status>,
    start: (isize, isize),
) -> HashMap<(isize, isize), usize> {
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));

    let mut visited = HashSet::new();

    let mut distances = HashMap::new();

    while let Some((coordinate, cost)) = queue.pop_front() {
        if !visited.insert(coordinate) {
            continue;
        }

        distances.insert(coordinate, cost);
        debug!("checking {:?}", coordinate);

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

    distances
}

#[allow(dead_code)]
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

    #[allow(dead_code)]
    fn set_current_position(&mut self, position: (isize, isize)) {
        if let Navigator::Debug(debug) = self {
            debug.current = position;
        }
    }
}

#[derive(Debug)]
struct Robot {
    area_map: HashMap<(isize, isize), Status>,
    current_position: (isize, isize),
    moves: Vec<((isize, isize), Direction)>,
    navigator: Navigator,
    oxygen_system_coordinate: Option<(isize, isize)>,
}

impl Robot {
    #[allow(dead_code)]
    fn new(navigator: Navigator) -> Robot {
        let area_map = HashMap::new();
        let current_position = (0, 0);

        let moves = Vec::new();
        Robot {
            area_map,
            current_position,
            moves,
            navigator,
            oxygen_system_coordinate: None,
        }
    }

    fn from_program(program: &str) -> Robot {
        let area_map = HashMap::new();
        let current_position = (0, 0);

        let moves = Vec::new();
        let computer = Computer::from_program(program);
        let navigator = Navigator::Computer(computer);

        Robot {
            area_map,
            current_position,
            moves,
            navigator,
            oxygen_system_coordinate: None,
        }
    }

    // returns true if the robot has a next move
    fn step(&mut self) -> bool {
        for direction in [
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
                    debug!("already found oxygen system");
                }

                self.oxygen_system_coordinate = Some(next_position);
            }

            if status == Status::Open || status == Status::OxygenSystem {
                debug!("moving {:?} to {:?}", direction, next_position);

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
            debug!(
                "robot is stuck - moving {:?} back to {:?}",
                reverse_direction, last_position
            );
            self.navigator.advance(reverse_direction);
            self.current_position = last_position;

            return true;
        }

        false
    }

    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

#[allow(clippy::from_over_into)]
impl Into<crate::year_2019::computer::Data> for Direction {
    fn into(self) -> crate::year_2019::computer::Data {
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

#[allow(clippy::from_over_into)]
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
                        .unwrap_or_else(|| panic!("{:?} not found", (x, y))),
                    expected_status
                );
            }
        }

        robot.print_area_map();
    }
}
