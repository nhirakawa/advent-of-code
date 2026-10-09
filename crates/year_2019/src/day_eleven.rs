use anyhow::anyhow;
use log::debug;
use std::collections::HashMap;

use super::computer::{self, Computer};

pub fn part_one(program: &str) -> anyhow::Result<impl ToString> {
    let solution = run_robot(program, Color::Black)?;
    Ok(solution.grid.len())
}

pub fn part_two(program: &str) -> anyhow::Result<impl ToString> {
    let solution = run_robot(program, Color::White)?;

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

    Ok(panels.join(""))
}

fn run_robot(program: &str, default_color: Color) -> anyhow::Result<RobotPainter> {
    let mut computer = Computer::from_program(program);

    let mut robot_painter = RobotPainter::new();

    while !computer.is_halted() {
        if computer.is_blocked_on_input() {
            let color = robot_painter.get_current_color().unwrap_or(default_color);
            debug!("Pushing color {color:?} as input");
            computer.push_input(color.into());
        }

        if computer.has_output() {
            let next_color = computer
                .get_output()
                .ok_or(anyhow!("No output (next_color)"))?
                .into();

            // step again so we have 2 outputs
            computer.step_until_output();

            let next_turn = computer
                .get_output()
                .ok_or(anyhow!("No output (next_turn)"))?
                .into();

            robot_painter.set_current_color(next_color);
            robot_painter.turn(next_turn);
        }

        computer.step();
    }

    Ok(robot_painter)
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
