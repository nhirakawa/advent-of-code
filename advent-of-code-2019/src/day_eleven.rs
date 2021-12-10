use crate::computer::{self, Computer};
use common::prelude::*;
use std::collections::HashMap;

pub fn run() -> AdventOfCodeResult {
    let program = include_str!("../input/day-11.txt");
    let part_one = part_one(program);
    let part_two = part_two(program);

    Ok((part_one, part_two))
}

fn part_one(program: &str) -> PartAnswer {
    let start = SystemTime::now();

    let solution = run_robot(program, Color::Black);

    PartAnswer::new(solution.grid.len(), start.elapsed().unwrap())
}

fn part_two(program: &str) -> PartAnswer {
    let start = SystemTime::now();

    let solution = run_robot(program, Color::White);

    let mut panels = vec!["\n"];

    for y in 0..=5 {
        for x in 0..=42 {
            let coordinate = (x, -y);

            let color = solution
                .grid
                .get(&coordinate)
                .cloned()
                .unwrap_or(Color::Black);

            let symbol = match color {
                Color::White => "\u{2588}",
                Color::Black => " ",
            };

            panels.push(symbol);
        }
        panels.push("\n");
    }

    let output = panels.join("");

    PartAnswer::new(output, start.elapsed().unwrap())
}

fn run_robot(program: &str, default_color: Color) -> RobotPainter {
    let mut has_used_default = false;
    let mut computer = Computer::from_program(program);

    let mut robot_painter = RobotPainter::new();

    while !computer.is_halted() {
        if computer.is_blocked_on_input() {
            let mut color = robot_painter.get_current_color();
            if color.is_none() {
                if !has_used_default {
                    has_used_default = true;
                    color = Some(default_color);
                } else {
                    color = Some(Color::Black);
                }
            }

            computer.push_input(color.unwrap().into());
        }

        if computer.has_output() {
            // step again so we have 2 outputs
            computer.step_until_output();

            let next_color = computer.get_output().unwrap().into();
            robot_painter.set_current_color(next_color);

            let next_turn = computer.get_output().unwrap().into();
            robot_painter.turn(next_turn);
        }

        computer.step();
    }

    robot_painter
}

#[derive(Debug)]
struct RobotPainter {
    position: Position,
    grid: HashMap<(i32, i32), Color>,
}

impl RobotPainter {
    fn new() -> RobotPainter {
        let position = Position::new();
        RobotPainter {
            position,
            grid: HashMap::new(),
        }
    }

    fn current_coordinates(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }

    fn get_current_color(&self) -> Option<Color> {
        let coordinates = self.current_coordinates();

        self.grid.get(&coordinates).cloned()
    }

    fn set_current_color(&mut self, color: Color) {
        let coordinates = self.current_coordinates();

        self.grid.insert(coordinates, color);
    }

    fn turn(&mut self, turn: Turn) {
        self.position.turn_and_move(turn);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color {
    White,
    Black,
}

impl From<computer::Data> for Color {
    fn from(value: computer::Data) -> Color {
        match value {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("cannot convert {} to color", value),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<computer::Data> for Color {
    fn into(self) -> computer::Data {
        match self {
            Color::White => 1,
            Color::Black => 0,
        }
    }
}

enum Turn {
    Left,
    Right,
}

impl From<computer::Data> for Turn {
    fn from(data: computer::Data) -> Turn {
        match data {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => panic!("Could not convert {} to Turn", data),
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Position {
    fn new() -> Position {
        Position {
            x: 0,
            y: 0,
            direction: Direction::Up,
        }
    }

    fn turn_and_move(&mut self, turn: Turn) {
        let new_direction = match turn {
            Turn::Left => self.direction.turn_left(),
            Turn::Right => self.direction.turn_right(),
        };

        self.direction = new_direction;

        match &self.direction {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
        };
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Self::Right,
            Direction::Right => Self::Down,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Self::Left,
            Direction::Left => Self::Down,
            Direction::Down => Self::Right,
            Direction::Right => Self::Up,
        }
    }
}
