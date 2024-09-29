use std::time::SystemTime;
use crate::common::answer::*;

use crate::year_2019::computer::Computer;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-13.txt");
    let part_one = part_one(input);
    let part_two = part_two(input);
    Ok((part_one, part_two))
}

fn part_one(program: &str) -> PartAnswer {
    let start = SystemTime::now();
    let mut arcade_cabinet = ArcadeCabinet::new(program);

    arcade_cabinet.play();

    let num_blocks = arcade_cabinet.count_number_of_blocks();

    PartAnswer::new(num_blocks, start.elapsed().unwrap())
}

fn part_two(program: &str) -> PartAnswer {
    let start = SystemTime::now();

    let mut arcade_cabinet = ArcadeCabinet::new(program);

    arcade_cabinet.insert_quarters();
    arcade_cabinet.play();

    PartAnswer::new(arcade_cabinet.last_score, start.elapsed().unwrap())
}

struct ArcadeCabinet {
    computer: Computer,
    last_score: i128,
    last_ball_position: (i128, i128),
    last_paddle_position: (i128, i128),
}

impl ArcadeCabinet {
    fn new(program: &str) -> ArcadeCabinet {
        let computer = Computer::from_program(program);

        ArcadeCabinet {
            computer,
            last_score: 0,
            last_ball_position: (0, 0),
            last_paddle_position: (0, 0),
        }
    }

    fn insert_quarters(&mut self) {
        self.computer.set(0, 2);
    }

    fn play(&mut self) {
        while !self.computer.is_halted() {
            self.computer.step();

            if self.computer.get_number_of_outputs() >= 3 {
                let x = self.computer.get_output().unwrap();
                let y = self.computer.get_output().unwrap();
                let tile_id = self.computer.get_output().unwrap();

                if (x, y) == (-1, 0) {
                    self.last_score = tile_id;
                } else {
                    let tile_type = tile_id.into();

                    match tile_type {
                        TileType::Ball => {
                            self.last_ball_position = (x, y);
                        }
                        TileType::HorizontalPaddle => {
                            self.last_paddle_position = (x, y);
                        }
                        _ => {}
                    };
                }
            }

            if self.computer.is_blocked_on_input() {
                let last_ball_position = self.last_ball_position;
                let last_paddle_position = self.last_paddle_position;

                let input = get_arcade_input(&last_ball_position, &last_paddle_position);
                self.computer.push_input(input);
            }
        }
    }

    fn count_number_of_blocks(&mut self) -> usize {
        let mut num_blocks = 0;

        for chunk in self.computer.get_outputs().chunks(3) {
            let tile_id = chunk[2];

            if TileType::Block == tile_id.into() {
                num_blocks += 1;
            }
        }

        num_blocks
    }
}

fn get_arcade_input(ball_position: &(i128, i128), paddle_position: &(i128, i128)) -> i128 {
    (ball_position.0 - paddle_position.0).signum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileType {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl From<i128> for TileType {
    fn from(i: i128) -> TileType {
        match i {
            0 => TileType::Empty,
            1 => TileType::Wall,
            2 => TileType::Block,
            3 => TileType::HorizontalPaddle,
            4 => TileType::Ball,
            _ => panic!("cannot convert {} to TileType", i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_arcade_input() {
        let ball_position = (4, 4);
        let paddle_position = (0, 10);

        let input = get_arcade_input(&ball_position, &paddle_position);
        assert_eq!(input, 1);

        let paddle_position = (10, 10);

        let input = get_arcade_input(&ball_position, &paddle_position);
        assert_eq!(input, -1);

        let paddle_position = (4, 10);

        let input = get_arcade_input(&ball_position, &paddle_position);
        assert_eq!(input, 0);
    }
}
