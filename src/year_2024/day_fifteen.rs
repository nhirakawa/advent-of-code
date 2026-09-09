use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    iter::FromIterator,
    ops::RangeInclusive,
};

use anyhow::bail;
use itertools::Itertools;
use log::debug;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (mut warehouse_map, directions) = parse_warehouse_map_and_directions(input, Scale::Single)?;

    for direction in directions {
        warehouse_map = move_robot(warehouse_map, &direction)?;
    }

    Ok(warehouse_map.gps_coordinate_sum())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (mut warehouse_map, directions) = parse_warehouse_map_and_directions(input, Scale::Double)?;

    for direction in directions {
        warehouse_map = move_robot(warehouse_map, &direction)?;
    }

    Ok(warehouse_map.gps_coordinate_sum())
}

fn next_coordinate(current: &Coordinate, direction: &Direction) -> Coordinate {
    match direction {
        Direction::Up => (current.0, current.1 - 1),
        Direction::Right => (current.0 + 1, current.1),
        Direction::Down => (current.0, current.1 + 1),
        Direction::Left => (current.0 - 1, current.1),
    }
}

fn move_robot(warehouse_map: WarehouseMap, direction: &Direction) -> anyhow::Result<WarehouseMap> {
    let number_of_obstacles_before = warehouse_map.obstacles.len();

    let (x, y) = warehouse_map.robot;

    if !warehouse_map.walls.x_range.contains(&x) || !warehouse_map.walls.y_range.contains(&y) {
        panic!("Robot out of bounds at {},{}", x, y);
    }

    let new_robot = next_coordinate(&(x, y), direction);

    debug!("Attempting to move robot to {new_robot:?}");

    if warehouse_map.walls.contains(&new_robot) {
        debug!("Robot cannot move to {new_robot:?} because there is a wall");
        return Ok(warehouse_map);
    }

    debug!("Robot can move to {new_robot:?}");

    if !warehouse_map.obstacles.contains(&new_robot) {
        debug!("Moving robot to  {new_robot:?} because there are no obstacles");
        return Ok(WarehouseMap::new(
            new_robot,
            warehouse_map.obstacles,
            warehouse_map.walls,
        ));
    }

    debug!("Gathering obstacles");

    // Gather the obstacles to move
    // None means that there are obstacles that cannot be moved because of a wall, therefore the robot cannot move
    // Some means there may be obstacles to move
    // Some(empty set) means there are no obstacles to move, and no walls were encountered
    // Some(set) means there are obstacles to move
    if let Some(obstacles_to_move) = gather_obstacles(new_robot, direction, &warehouse_map) {
        if obstacles_to_move.is_empty() {
            Ok(WarehouseMap::new(
                new_robot,
                warehouse_map.obstacles,
                warehouse_map.walls,
            ))
        } else {
            debug!(
                "Gathered {} obstacles to move: {obstacles_to_move:?}",
                obstacles_to_move.len()
            );

            let mut obstacles = HashSet::new();

            for obstacle in warehouse_map.obstacles.0 {
                if obstacles_to_move.contains(&obstacle) {
                    obstacles.insert(obstacle.push(direction));
                } else {
                    obstacles.insert(obstacle);
                }
            }

            if obstacles.len() != number_of_obstacles_before {
                bail!(
                    "Expected {} obstacles, found {}",
                    number_of_obstacles_before,
                    obstacles.len()
                );
            }

            Ok(WarehouseMap::new(
                new_robot,
                obstacles.into(),
                warehouse_map.walls,
            ))
        }
    } else {
        Ok(warehouse_map)
    }
}

fn gather_obstacles(
    start: Coordinate,
    direction: &Direction,
    warehouse_map: &WarehouseMap,
) -> Option<HashSet<Obstacle>> {
    let mut obstacles = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        debug!("Visiting {current:?}");
        if let Some(current_obstacle) = warehouse_map.obstacles.get(&current) {
            if !obstacles.insert(current_obstacle) {
                continue;
            }

            let next_coordinates = match (current_obstacle, direction) {
                (Obstacle::Single(only), _) => vec![next_coordinate(&only, direction)],
                (Obstacle::Double(left, _), Direction::Left) => {
                    vec![next_coordinate(&left, direction)]
                }
                (Obstacle::Double(_, right), Direction::Right) => {
                    vec![next_coordinate(&right, direction)]
                }
                (Obstacle::Double(left, right), Direction::Up | Direction::Down) => vec![
                    next_coordinate(&left, direction),
                    next_coordinate(&right, direction),
                ],
            };

            debug!("Checking for obstacles at {next_coordinates:?}");

            for next_coordinate in &next_coordinates {
                if warehouse_map.walls.contains(next_coordinate) {
                    debug!("Wall at {next_coordinate:?}");
                    return None;
                }
            }

            for next_coordinate in next_coordinates {
                debug!("Queueing {next_coordinate:?}");
                queue.push_back(next_coordinate);
            }
        } else {
            debug!("No obstacle at {current:?}");
        }
    }

    Some(obstacles)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Scale {
    Single,
    Double,
}

type Coordinate = (isize, isize);

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Obstacle {
    Single(Coordinate),
    Double(Coordinate, Coordinate),
}

impl Obstacle {
    pub fn contains(&self, coordinate: &Coordinate) -> bool {
        match self {
            Obstacle::Single(c) => c == coordinate,
            Obstacle::Double(left, right) => left == coordinate || right == coordinate,
        }
    }

    pub fn push(&self, direction: &Direction) -> Obstacle {
        match self {
            Obstacle::Single(only) => Obstacle::Single(next_coordinate(only, direction)),
            Obstacle::Double(left, right) => Obstacle::Double(
                next_coordinate(left, direction),
                next_coordinate(right, direction),
            ),
        }
    }

    pub fn gps_coordinate(&self) -> usize {
        match self {
            Obstacle::Single((x, y)) => (x.abs() + (y.abs() * 100)) as usize,
            Obstacle::Double((x, y), _) => (x.abs() + (y.abs() * 100)) as usize,
        }
    }
}

impl From<Coordinate> for Obstacle {
    fn from(coordinate: Coordinate) -> Self {
        Obstacle::Single(coordinate)
    }
}

impl From<(Coordinate, Coordinate)> for Obstacle {
    fn from((left, right): (Coordinate, Coordinate)) -> Self {
        Obstacle::Double(left, right)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Obstacles(HashSet<Obstacle>);

impl Obstacles {
    pub fn new(obstacles: HashSet<Obstacle>) -> Self {
        Self(obstacles)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn contains(&self, coordinate: &Coordinate) -> bool {
        self.0.iter().any(|obstacle| obstacle.contains(coordinate))
    }

    pub fn gps_coordinate_sum(&self) -> usize {
        self.0.iter().map(Obstacle::gps_coordinate).sum()
    }

    pub fn get(&self, coordinate: &Coordinate) -> Option<Obstacle> {
        self.0
            .iter()
            .find(|obstacle| obstacle.contains(coordinate))
            .copied()
    }
}

impl FromIterator<Obstacle> for Obstacles {
    fn from_iter<T: IntoIterator<Item = Obstacle>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl From<HashSet<Obstacle>> for Obstacles {
    fn from(obstacles: HashSet<Obstacle>) -> Self {
        Self::new(obstacles)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Wall {
    Single(Coordinate),
    Double(Coordinate, Coordinate),
}

impl Wall {
    pub fn get_x_components(&self) -> Vec<isize> {
        match self {
            Wall::Single((x, _)) => vec![*x],
            Wall::Double((x1, _), (x2, _)) => vec![*x1, *x2],
        }
    }

    pub fn get_y_components(&self) -> Vec<isize> {
        match self {
            Wall::Single((_, y)) => vec![*y],
            Wall::Double((_, y1), (_, y2)) => vec![*y1, *y2],
        }
    }

    pub fn contains(&self, coordinate: &Coordinate) -> bool {
        match self {
            Wall::Single(c) => c == coordinate,
            Wall::Double(left, right) => left == coordinate || right == coordinate,
        }
    }
}

impl From<Coordinate> for Wall {
    fn from(coordinate: Coordinate) -> Self {
        Wall::Single(coordinate)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Walls {
    walls: HashSet<Wall>,
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
}

impl Walls {
    pub fn new(walls: HashSet<Wall>) -> Self {
        if walls.is_empty() {
            panic!("Cannot construct Walls from empty set");
        }

        let max_x = walls.iter().flat_map(Wall::get_x_components).max().unwrap();
        let x_range = 0..=max_x;

        let max_y = walls.iter().flat_map(Wall::get_y_components).max().unwrap();
        let y_range = 0..=max_y;

        Self {
            walls,
            x_range,
            y_range,
        }
    }

    pub fn contains(&self, coordinate: &Coordinate) -> bool {
        self.walls.iter().any(|wall| wall.contains(coordinate))
    }
}

impl From<HashSet<Wall>> for Walls {
    fn from(walls: HashSet<Wall>) -> Self {
        Self::new(walls)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct WarehouseMap {
    robot: Coordinate,
    obstacles: Obstacles,
    walls: Walls,
}

impl WarehouseMap {
    pub fn new(robot: Coordinate, obstacles: Obstacles, walls: Walls) -> Self {
        Self {
            robot,
            obstacles,
            walls,
        }
    }

    pub fn gps_coordinate_sum(&self) -> usize {
        self.obstacles.gps_coordinate_sum()
    }

    #[allow(dead_code)]
    pub fn render_to_string(&self) -> String {
        let mut result = String::new();

        for y in self.walls.y_range.clone() {
            for x in self.walls.x_range.clone() {
                let coordinate = (x, y);

                if self.robot == coordinate {
                    result.push('@');
                } else if self.obstacles.contains(&coordinate) {
                    result.push('O');
                } else if self.walls.contains(&coordinate) {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }

            result.push('\n');
        }

        result
    }
}

type Directions = Vec<Direction>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse_warehouse_map_and_directions(
    input: &str,
    scale: Scale,
) -> anyhow::Result<(WarehouseMap, Directions)> {
    let (warehouse_map_str, directions_str) = input
        .split("\n\n")
        .collect_tuple()
        .ok_or(anyhow::anyhow!("Invalid input"))?;

    let warehouse_map = parse_warehouse_map(warehouse_map_str, scale)?;
    let directions = parse_directions(directions_str)?;

    Ok((warehouse_map, directions))
}

fn parse_warehouse_map(input: &str, scale: Scale) -> anyhow::Result<WarehouseMap> {
    let mut robot = None;
    let mut obstacles = HashSet::new();
    let mut walls = HashSet::new();

    let mut x = 0;

    let x_step = match scale {
        Scale::Single => 1,
        Scale::Double => 2,
    };

    for (y, line) in input.lines().enumerate() {
        for c in line.chars() {
            let coordinate = (x as isize, y as isize);
            match c {
                '@' => {
                    if robot.is_some() {
                        bail!("Multiple robots found");
                    }
                    robot = Some(coordinate);
                }
                'O' => {
                    let obstacle = match scale {
                        Scale::Single => Obstacle::Single(coordinate),
                        Scale::Double => {
                            let next_coordinate = (x as isize + 1, y as isize);
                            Obstacle::Double(coordinate, next_coordinate)
                        }
                    };
                    obstacles.insert(obstacle);
                }
                '#' => {
                    let wall = match scale {
                        Scale::Single => Wall::Single(coordinate),
                        Scale::Double => {
                            let next_coordinate = (x as isize + 1, y as isize);
                            Wall::Double(coordinate, next_coordinate)
                        }
                    };
                    walls.insert(wall);
                }
                '.' => {}
                _ => bail!("Invalid character '{c}'"),
            }

            x += x_step;
        }

        x = 0;
    }

    let robot = robot.ok_or(anyhow::anyhow!("No robot found"))?;
    let obstacles = Obstacles::new(obstacles);
    let walls = Walls::new(walls);

    Ok(WarehouseMap::new(robot, obstacles, walls))
}

fn parse_directions(input: &str) -> anyhow::Result<Directions> {
    input
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => Ok(Direction::Up),
                '>' => Ok(Direction::Right),
                'v' => Ok(Direction::Down),
                '<' => Ok(Direction::Left),
                _ => Err(anyhow::anyhow!("Invalid direction")),
            })
        })
        .collect::<Result<_, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const WAREHOUSE_LARGE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const SMALL_EXAMPLE_DOUBLE: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn test_parse_warehouse_map_double() {
        let input = WAREHOUSE_LARGE;
        let (warehouse_map, _) = parse_warehouse_map_and_directions(input, Scale::Double).unwrap();

        assert_eq!(warehouse_map.robot, (8, 4));

        let expected_obstacles = [
            ((6, 1), (7, 1)),
            ((12, 1), (13, 1)),
            ((16, 1), (17, 1)),
            ((14, 2), (15, 2)),
            ((4, 3), (5, 3)),
            ((6, 3), (7, 3)),
            ((12, 3), (13, 3)),
            ((16, 3), (17, 3)),
            ((6, 4), (7, 4)),
            ((14, 4), (15, 4)),
            ((2, 5), (3, 5)),
            ((10, 5), (11, 5)),
            ((2, 6), (3, 6)),
            ((8, 6), (9, 6)),
            ((14, 6), (15, 6)),
            ((4, 7), (5, 7)),
            ((6, 7), (7, 7)),
            ((10, 7), (11, 7)),
            ((14, 7), (15, 7)),
            ((16, 7), (17, 7)),
            ((10, 8), (11, 8)),
        ]
        .iter()
        .copied()
        .map(Into::into)
        .collect::<HashSet<_>>();

        for expected_obstacle in &expected_obstacles {
            println!("Checking if obstacle {expected_obstacle:?} is in warehouse map");
            assert!(warehouse_map.obstacles.0.contains(expected_obstacle));
        }

        assert_eq!(warehouse_map.obstacles, expected_obstacles.into());

        let mut expected_walls = HashSet::new();

        for x in (0..20).step_by(2) {
            let wall = Wall::Double((x, 0), (x + 1, 0));
            expected_walls.insert(wall);

            let wall = Wall::Double((x, 9), (x + 1, 9));
            expected_walls.insert(wall);
        }

        for expected_wall in expected_walls {
            println!("Checking if wall {expected_wall:?} is in warehouse map");
            assert!(warehouse_map.walls.walls.contains(&expected_wall));
        }
    }

    #[test]
    fn test_push_single_obstacle() {
        let obstacle = Obstacle::Single((1, 1));

        assert_eq!(obstacle.push(&Direction::Up), Obstacle::Single((1, 0)));
        assert_eq!(obstacle.push(&Direction::Right), Obstacle::Single((2, 1)));
        assert_eq!(obstacle.push(&Direction::Down), Obstacle::Single((1, 2)));
        assert_eq!(obstacle.push(&Direction::Left), Obstacle::Single((0, 1)));
    }

    #[test]
    fn test_small_example_single_scale() {
        let mut walls = HashSet::new();

        for x in 0..=7 {
            walls.insert((x, 0).into());
            walls.insert((x, 7).into());
        }

        for y in 0..=7 {
            walls.insert((0, y).into());
            walls.insert((7, y).into());
        }

        walls.insert((1, 2).into());
        walls.insert((2, 4).into());

        let obstacles = [(3, 1), (5, 1), (4, 2), (4, 3), (4, 4), (4, 5)]
            .iter()
            .copied()
            .map(Into::into)
            .collect::<HashSet<_>>()
            .into();

        let robot = (2, 2);
        let walls = walls.into();

        let mut warehouse_map = WarehouseMap::new(robot, obstacles, walls);

        warehouse_map = move_robot(warehouse_map, &Direction::Left).unwrap();

        assert_eq!(warehouse_map.robot, (2, 2));
        assert_eq!(
            warehouse_map.obstacles,
            [(3, 1), (5, 1), (4, 2), (4, 3), (4, 4), (4, 5)]
                .iter()
                .copied()
                .map(Into::into)
                .collect::<HashSet<_>>()
                .into()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Up).unwrap();

        assert_eq!(warehouse_map.robot, (2, 1));
        assert_eq!(
            warehouse_map.obstacles,
            [(3, 1), (5, 1), (4, 2), (4, 3), (4, 4), (4, 5)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Up).unwrap();

        assert_eq!(warehouse_map.robot, (2, 1));
        assert_eq!(
            warehouse_map.obstacles,
            [(3, 1), (5, 1), (4, 2), (4, 3), (4, 4), (4, 5)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Right).unwrap();

        assert_eq!(warehouse_map.robot, (3, 1));
        assert_eq!(
            warehouse_map.obstacles,
            [(5, 1), (4, 1), (4, 2), (4, 3), (4, 4), (4, 5)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Right).unwrap();

        assert_eq!(warehouse_map.robot, (4, 1));
        assert_eq!(
            warehouse_map.obstacles,
            [(5, 1), (6, 1), (4, 2), (4, 3), (4, 4), (4, 5)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Right).unwrap();

        assert_eq!(warehouse_map.robot, (4, 1));
        assert_eq!(
            warehouse_map.obstacles,
            [(5, 1), (6, 1), (4, 2), (4, 3), (4, 4), (4, 5)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Down).unwrap();

        assert_eq!(warehouse_map.robot, (4, 2));
        assert_eq!(
            warehouse_map.obstacles,
            [(5, 1), (6, 1), (4, 3), (4, 4), (4, 5), (4, 6)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Down).unwrap();

        assert_eq!(warehouse_map.robot, (4, 2));
        assert_eq!(
            warehouse_map.obstacles,
            [(5, 1), (6, 1), (4, 3), (4, 4), (4, 5), (4, 6)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Left).unwrap();

        assert_eq!(warehouse_map.robot, (3, 2));
        assert_eq!(
            warehouse_map.obstacles,
            [(5, 1), (6, 1), (4, 3), (4, 4), (4, 5), (4, 6)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Down).unwrap();

        assert_eq!(warehouse_map.robot, (3, 3));
        assert_eq!(
            warehouse_map.obstacles,
            [(5, 1), (6, 1), (4, 3), (4, 4), (4, 5), (4, 6)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Right).unwrap();

        assert_eq!(warehouse_map.robot, (4, 3));
        assert_eq!(
            warehouse_map.obstacles,
            [(5, 1), (6, 1), (5, 3), (4, 4), (4, 5), (4, 6)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Right).unwrap();

        assert_eq!(warehouse_map.robot, (5, 3));
        assert_eq!(
            warehouse_map.obstacles,
            [(6, 1), (6, 3), (5, 1), (4, 4), (4, 5), (4, 6)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Down).unwrap();

        assert_eq!(warehouse_map.robot, (5, 4));
        assert_eq!(
            warehouse_map.obstacles,
            [(6, 1), (6, 3), (5, 1), (4, 4), (4, 5), (4, 6)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Left).unwrap();

        assert_eq!(warehouse_map.robot, (4, 4));
        assert_eq!(
            warehouse_map.obstacles,
            [(6, 1), (6, 3), (5, 1), (4, 5), (4, 6), (3, 4)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        warehouse_map = move_robot(warehouse_map, &Direction::Left).unwrap();

        assert_eq!(warehouse_map.robot, (4, 4));
        assert_eq!(
            warehouse_map.obstacles,
            [(6, 1), (6, 3), (5, 1), (4, 5), (4, 6), (3, 4)]
                .iter()
                .copied()
                .map(Into::into)
                .collect()
        );

        assert_eq!(warehouse_map.gps_coordinate_sum(), 2028);
    }

    #[test]
    fn test_small_example_double_scale() {
        let (mut warehouse_map, _) =
            parse_warehouse_map_and_directions(SMALL_EXAMPLE_DOUBLE, Scale::Double).unwrap();

        assert_eq!(warehouse_map.robot, (10, 3));

        println!("Moving robot left");

        warehouse_map = move_robot(warehouse_map, &Direction::Left).unwrap();

        assert_eq!(warehouse_map.robot, (9, 3));

        let expected_obstacles = [((5, 3), (6, 3)), ((7, 3), (8, 3)), ((6, 4), (7, 4))]
            .iter()
            .copied()
            .map(Into::into)
            .collect();

        assert_eq!(warehouse_map.obstacles, expected_obstacles);
    }

    #[test]
    fn test_large_example() {
        let (mut warehouse_map, directions) =
            parse_warehouse_map_and_directions(WAREHOUSE_LARGE, Scale::Single).unwrap();

        assert_eq!(warehouse_map.robot, (4, 4));

        for direction in directions {
            warehouse_map = move_robot(warehouse_map, &direction).unwrap();
        }

        assert_eq!(warehouse_map.robot, (3, 4));

        let expected_obstacles = [
            (2, 1),
            (4, 1),
            (6, 1),
            (7, 1),
            (8, 1),
            (1, 3),
            (2, 3),
            (1, 4),
            (2, 4),
            (1, 5),
            (8, 5),
            (1, 6),
            (7, 6),
            (8, 6),
            (1, 7),
            (7, 7),
            (8, 7),
            (1, 8),
            (2, 8),
            (7, 8),
            (8, 8),
        ]
        .iter()
        .copied()
        .map(Into::into)
        .collect::<HashSet<_>>()
        .into();

        assert_eq!(warehouse_map.obstacles, expected_obstacles);

        assert_eq!(warehouse_map.gps_coordinate_sum(), 10092);
    }
}
