use std::fmt::{Display, Formatter};

use common::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-12.txt").trim();
    let actions = parse_actions(input);

    let part_one = part_one(&actions);
    let part_two = part_two(&actions);

    Ok((part_one, part_two))
}

fn part_one(actions: &Actions) -> PartAnswer {
    let start = SystemTime::now();

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut direction = 90;

    for action in actions {
        match action.action_type {
            ActionType::North => y += action.value,
            ActionType::South => y -= action.value,
            ActionType::East => x += action.value,
            ActionType::West => x -= action.value,
            ActionType::Left => direction = (direction - action.value + 360) % 360,
            ActionType::Right => direction = (direction + action.value + 360) % 360,
            ActionType::Forward => match direction {
                0 => y += action.value,
                90 => x += action.value,
                180 => y -= action.value,
                270 => x -= action.value,
                _ => {}
            },
        }
    }

    let answer = (x.abs() + y.abs()) as u64;

    let elapsed = start.elapsed().unwrap();

    (answer, elapsed).into()
}

fn part_two(actions: &Actions) -> PartAnswer {
    let start = SystemTime::now();

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut waypoint = (10, 1);

    for action in actions {
        match action.action_type {
            ActionType::North => waypoint.1 += action.value,
            ActionType::South => waypoint.1 -= action.value,
            ActionType::East => waypoint.0 += action.value,
            ActionType::West => waypoint.0 -= action.value,
            ActionType::Left => waypoint = rotate_clockwise(waypoint, -action.value),
            ActionType::Right => waypoint = rotate_clockwise(waypoint, action.value),
            ActionType::Forward => {
                x += action.value * waypoint.0;
                y += action.value * waypoint.1;
            }
        }
    }

    let answer = (x.abs() + y.abs()) as u64;

    let elapsed = start.elapsed().unwrap();
    (answer, elapsed).into()
}

fn rotate_clockwise(vector: (i32, i32), theta: i32) -> (i32, i32) {
    let (mut x, mut y) = vector;
    let mut theta = theta;

    while theta != 0 {
        let temp = x;
        x = y;
        y = -temp;

        theta = (theta - 90) % 360;
    }

    (x, y)
}

type Actions = Vec<Action>;

#[derive(Debug, PartialEq)]
struct Action {
    action_type: ActionType,
    value: i32,
}

impl From<(ActionType, i32)> for Action {
    fn from(t: (ActionType, i32)) -> Action {
        let (action_type, value) = t;
        Action { action_type, value }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let action_type_string = match self.action_type {
            ActionType::North => "N",
            ActionType::East => "E",
            ActionType::South => "S",
            ActionType::West => "W",
            ActionType::Left => "L",
            ActionType::Right => "R",
            ActionType::Forward => "F",
        };

        write!(f, "{}{}", action_type_string, self.value)
    }
}

#[derive(Debug, PartialEq)]
enum ActionType {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

fn parse_actions(s: &str) -> Actions {
    let mut actions = Vec::new();

    for line in s.split("\n") {
        let (action_type, value) = line.split_at(1);
        let action_type = match action_type {
            "N" => Some(ActionType::North),
            "S" => Some(ActionType::South),
            "E" => Some(ActionType::East),
            "W" => Some(ActionType::West),
            "L" => Some(ActionType::Left),
            "R" => Some(ActionType::Right),
            "F" => Some(ActionType::Forward),
            _ => None,
        };

        let value = value.parse::<i32>().ok();

        let action = action_type.and_then(|a| value.map(|v| (a, v)));
        let action = action.map(Action::from);
        actions.push(action);
    }

    actions.into_iter().flatten().map(Action::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(rotate_clockwise((10, 4), 90), (4, -10));
        assert_eq!(rotate_clockwise((10, 4), 180), (-10, -4));
        assert_eq!(rotate_clockwise((10, 4), 270), (-4, 10));
        assert_eq!(rotate_clockwise((10, 4), 360), (10, 4));

        assert_eq!(rotate_clockwise((1, 0), 90), (0, -1));
        assert_eq!(rotate_clockwise((1, 0), 180), (-1, 0));
        assert_eq!(rotate_clockwise((1, 0), 270), (0, 1));

        assert_eq!(rotate_clockwise((1, 0), -90), (0, 1));
        assert_eq!(rotate_clockwise((1, 0), -180), (-1, 0));
        assert_eq!(rotate_clockwise((1, 0), -270), (0, -1));
    }
}
