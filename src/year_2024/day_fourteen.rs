use std::{collections::HashSet, io::Cursor};

use anyhow::anyhow;
use image::{ImageFormat, Rgba, RgbaImage};

use nom::{
    bytes::complete::tag,
    combinator::into,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

use crate::common::{
    constants,
    parse::{finish, number, unsigned_number},
};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let robots = parse_robots(input)?;

    let mut robots = Robots::new(robots, 101, 103);

    for _ in 0..100 {
        robots.step();
    }

    Ok(robots.safety_factor().to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let robots = parse_robots(input)?;

    let mut robots = Robots::new(robots, 101, 103);

    let mut image_sizes = Vec::new();

    for index in 0..10000 {
        let image = robots.render_as_image();
        let mut writer = Cursor::new(Vec::new());
        image.write_to(&mut writer, ImageFormat::Png).unwrap();

        let image_bytes = writer.into_inner().len();
        image_sizes.push((index, image_bytes));

        robots.step();
    }

    let (index, _) = image_sizes
        .iter()
        .min_by(|(index, size), (other_index, other_size)| {
            size.cmp(other_size).then(index.cmp(other_index))
        })
        .copied()
        .ok_or(anyhow!("Could not find minimum image size"))?;

    Ok(index.to_string())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: u16,
    y: u16,
}

impl From<(u16, u16)> for Position {
    fn from((x, y): (u16, u16)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Velocity {
    x: i16,
    y: i16,
}

impl From<(i16, i16)> for Velocity {
    fn from((x, y): (i16, i16)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Robots {
    robots: Vec<Robot>,
    width: u16,
    height: u16,
}

impl Robots {
    pub fn new(robots: Vec<Robot>, width: u16, height: u16) -> Self {
        Self {
            robots,
            width,
            height,
        }
    }

    pub fn step(&mut self) {
        for robot in &mut self.robots {
            let mut new_x = robot.position.x as i16 + robot.velocity.x;
            let mut new_y = robot.position.y as i16 + robot.velocity.y;

            if new_x < 0 {
                new_x += self.width as i16;
            }

            if new_x >= self.width as i16 {
                new_x -= self.width as i16;
            }

            if new_y < 0 {
                new_y += self.height as i16;
            }

            if new_y >= self.height as i16 {
                new_y -= self.height as i16;
            }

            robot.position = (new_x as u16, new_y as u16).into();
        }
    }

    pub fn safety_factor(&self) -> usize {
        let width_midpoint = self.width / 2;
        let height_midpoint = self.height / 2;

        let upper_left_quadrant = (0..width_midpoint, 0..height_midpoint);
        let upper_right_quadrant = (width_midpoint + 1..self.width, 0..height_midpoint);
        let lower_left_qudrant = (0..width_midpoint, height_midpoint + 1..self.height);
        let lower_right_quadrant = (
            width_midpoint + 1..self.width,
            height_midpoint + 1..self.height,
        );

        let mut upper_left_quadrant_count = 0;
        let mut upper_right_quadrant_count = 0;
        let mut lower_left_quadrant_count = 0;
        let mut lower_right_quadrant_count = 0;

        for robot in &self.robots {
            if upper_left_quadrant.0.contains(&robot.position.x)
                && upper_left_quadrant.1.contains(&robot.position.y)
            {
                upper_left_quadrant_count += 1;
            }

            if upper_right_quadrant.0.contains(&robot.position.x)
                && upper_right_quadrant.1.contains(&robot.position.y)
            {
                upper_right_quadrant_count += 1;
            }

            if lower_left_qudrant.0.contains(&robot.position.x)
                && lower_left_qudrant.1.contains(&robot.position.y)
            {
                lower_left_quadrant_count += 1;
            }

            if lower_right_quadrant.0.contains(&robot.position.x)
                && lower_right_quadrant.1.contains(&robot.position.y)
            {
                lower_right_quadrant_count += 1;
            }
        }

        upper_left_quadrant_count
            * upper_right_quadrant_count
            * lower_left_quadrant_count
            * lower_right_quadrant_count
    }

    #[allow(dead_code)]
    pub fn render(&self) -> String {
        let mut grid =
            vec![vec![constants::empty_square(); self.width as usize]; self.height as usize];

        for robot in &self.robots {
            grid[robot.position.y as usize][robot.position.x as usize] = constants::solid_square();
        }

        grid.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn render_as_image(&self) -> RgbaImage {
        let robot_size = 4;

        let image_width = self.width as u32 * robot_size;
        let image_height = self.height as u32 * robot_size;

        let mut image = RgbaImage::new(image_width, image_height);

        let mut seen = HashSet::new();

        for robot in &self.robots {
            let x = robot.position.x as u32;
            let y = robot.position.y as u32;

            seen.insert((x, y));

            let x = x * robot_size;
            let y = y * robot_size;

            for x in x..x + robot_size {
                for y in y..y + robot_size {
                    image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
                }
            }
        }

        for x in 0..self.width {
            for y in 0..self.height {
                let x = x as u32;
                let y = y as u32;

                if seen.contains(&(x, y)) {
                    continue;
                }

                let x = x * robot_size;
                let y = y * robot_size;

                for x in x..x + robot_size {
                    for y in y..y + robot_size {
                        image.put_pixel(x, y, Rgba([0, 0, 0, 255]));
                    }
                }
            }
        }

        image
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    pub fn new(position: Position, velocity: Velocity) -> Self {
        Self { position, velocity }
    }
}

impl From<(Position, Velocity)> for Robot {
    fn from((position, velocity): (Position, Velocity)) -> Self {
        Self::new(position, velocity)
    }
}

fn parse_robots(i: &str) -> anyhow::Result<Vec<Robot>> {
    finish(robots, i)
}

fn robots(i: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(tag("\n"), robot).parse(i)
}

fn robot(i: &str) -> IResult<&str, Robot> {
    into(separated_pair(position, tag(" "), velocity)).parse(i)
}

fn position(i: &str) -> IResult<&str, Position> {
    preceded(
        tag("p="),
        into(separated_pair(unsigned_number, tag(","), unsigned_number)),
    )
    .parse(i)
}

fn velocity(i: &str) -> IResult<&str, Velocity> {
    preceded(tag("v="), into(separated_pair(number, tag(","), number))).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let position = Position::from((2, 4));
        let velocity = Velocity::from((2, -3));

        let robot = Robot::from((position, velocity));

        let mut robots = Robots::new(vec![robot], 11, 7);

        robots.step();

        assert_eq!(robots.robots[0].position, Position::from((4, 1)));

        robots.step();

        assert_eq!(robots.robots[0].position, Position::from((6, 5)));

        robots.step();

        assert_eq!(robots.robots[0].position, Position::from((8, 2)));

        robots.step();

        assert_eq!(robots.robots[0].position, Position::from((10, 6)));

        robots.step();

        assert_eq!(robots.robots[0].position, Position::from((1, 3)));
    }
}
