use std::{
    fmt::Debug,
    ops::{Add, AddAssign},
};

use common::math::lcm;
use common::parse::number;
use common::prelude::*;
use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map, value},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-12.txt");

    let part_one = part_one(input);
    let part_two = part_two(input);

    Ok((part_one, part_two))
}

fn part_one(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let moons = parse_moons(input);

    let final_state = simulate_gravity_iterated(moons, 1000);

    let solution = total_energy(&final_state);

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two(input: &str) -> PartAnswer {
    let start = SystemTime::now();
    let moons = parse_moons(input);

    let solution = simulate_until_repeated_state(moons);

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn simulate_gravity_iterated(moons: Vec<Moon>, iterations: usize) -> Vec<Moon> {
    let mut moons = moons;

    for _ in 0..iterations {
        moons = simulate_gravity(&moons);
    }

    moons
}

fn simulate_gravity(moons: &[Moon]) -> Vec<Moon> {
    let mut new_moons = Vec::with_capacity(moons.len());

    for i in 0..moons.len() {
        let first = moons[i];

        let mut delta_velocity = Velocity::default();

        for j in 0..moons.len() {
            if i == j {
                continue;
            }

            let second = moons[j];

            delta_velocity += first.get_velocity_delta(&second);
        }

        let new_velocity = first.velocity + delta_velocity;

        let new_position = first.position + new_velocity;

        let new_moon = Moon {
            position: new_position,
            velocity: new_velocity,
        };

        new_moons.push(new_moon);
    }

    new_moons
}

fn simulate_until_repeated_state(moons: Vec<Moon>) -> i128 {
    let mut iterations_x = 1;
    let mut current_state = simulate_gravity(&moons);

    loop {
        let all_zero = current_state.iter().all(|m| m.velocity.x == 0);

        if all_zero {
            break;
        } else {
            current_state = simulate_gravity(&current_state);
            iterations_x += 1;
        }
    }

    let mut iterations_y = 1;
    let mut current_state = simulate_gravity(&moons);

    loop {
        let all_zero = current_state.iter().all(|m| m.velocity.y == 0);

        if all_zero {
            break;
        } else {
            current_state = simulate_gravity(&current_state);
            iterations_y += 1;
        }
    }

    let mut iterations_z = 1;
    let mut current_state = simulate_gravity(&moons);

    loop {
        let all_zero = current_state.iter().all(|m| m.velocity.z == 0);

        if all_zero {
            break;
        } else {
            current_state = simulate_gravity(&current_state);
            iterations_z += 1;
        }
    }

    // todo - figure out why we need a multiplier
    lcm(lcm(iterations_x, iterations_y), iterations_z) * 2
}

fn total_energy(moons: &[Moon]) -> u32 {
    moons.iter().map(Moon::total_energy).sum()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Moon {
    position: Position,
    velocity: Velocity,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            position: Position::new(x, y, z),
            velocity: Velocity::default(),
        }
    }

    fn potential_energy(&self) -> u32 {
        self.position.x.abs() as u32 + self.position.y.abs() as u32 + self.position.z.abs() as u32
    }

    fn kinetic_energy(&self) -> u32 {
        self.velocity.x.abs() as u32 + self.velocity.y.abs() as u32 + self.velocity.z.abs() as u32
    }

    fn total_energy(&self) -> u32 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn get_velocity_delta(&self, other: &Moon) -> Velocity {
        let delta_x = get_delta(self.position.x, other.position.x);
        let delta_y = get_delta(self.position.y, other.position.y);
        let delta_z = get_delta(self.position.z, other.position.z);

        Velocity::new(delta_x, delta_y, delta_z)
    }
}

impl Debug for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "p:{:?} v:{:?}", self.position, self.velocity)
    }
}

fn get_delta(this: i32, that: i32) -> i32 {
    if this > that {
        -1
    } else if that > this {
        1
    } else {
        0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn new(x: i32, y: i32, z: i32) -> Position {
        Position { x, y, z }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add<Velocity> for Position {
    type Output = Position;
    fn add(self, rhs: Velocity) -> Position {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;

        Position { x, y, z }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

impl Velocity {
    fn new(x: i32, y: i32, z: i32) -> Velocity {
        Velocity { x, y, z }
    }
}

impl Debug for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Velocity {
    type Output = Velocity;

    fn add(self, rhs: Velocity) -> Velocity {
        Velocity::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Velocity {
    fn add_assign(&mut self, other: Velocity) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

fn parse_moons(i: &str) -> Vec<Moon> {
    let (_, moons) = all_consuming(terminated(moons, tag("\n")))(i).unwrap();

    moons
}

fn moons(i: &str) -> IResult<&str, Vec<Moon>> {
    separated_list1(tag("\n"), moon)(i)
}

fn moon(i: &str) -> IResult<&str, Moon> {
    let coordinates = tuple((
        open_bracket,
        x_coordinate,
        separator,
        y_coordinate,
        separator,
        z_coordinate,
        close_bracket,
    ));

    map(coordinates, |(_, x, _, y, _, z, _)| Moon::new(x, y, z))(i)
}

fn x_coordinate(i: &str) -> IResult<&str, i32> {
    preceded(tag("x="), number)(i)
}

fn y_coordinate(i: &str) -> IResult<&str, i32> {
    preceded(tag("y="), number)(i)
}

fn z_coordinate(i: &str) -> IResult<&str, i32> {
    preceded(tag("z="), number)(i)
}

fn open_bracket(i: &str) -> IResult<&str, ()> {
    value((), tag("<"))(i)
}

fn close_bracket(i: &str) -> IResult<&str, ()> {
    value((), tag(">"))(i)
}

fn separator(i: &str) -> IResult<&str, ()> {
    value((), tag(", "))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moon() {
        assert_eq!(moon("<x=1, y=2, z=6>"), Ok(("", Moon::new(1, 2, 6))));
    }

    #[test]
    fn test_simulate_gravity() {
        let mut moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];

        let new_moons = simulate_gravity(&mut moons);

        assert_eq!(
            new_moons,
            vec![
                Moon {
                    position: Position { x: 2, y: -1, z: 1 },
                    velocity: Velocity { x: 3, y: -1, z: -1 }
                },
                Moon {
                    position: Position { x: 3, y: -7, z: -4 },
                    velocity: Velocity { x: 1, y: 3, z: 3 }
                },
                Moon {
                    position: Position { x: 1, y: -7, z: 5 },
                    velocity: Velocity { x: -3, y: 1, z: -3 }
                },
                Moon {
                    position: Position { x: 2, y: 2, z: 0 },
                    velocity: Velocity { x: -1, y: -3, z: 1 }
                }
            ]
        );

        moons = new_moons;
        let new_moons = simulate_gravity(&mut moons);

        assert_eq!(
            new_moons,
            vec![
                Moon {
                    position: Position { x: 5, y: -3, z: -1 },
                    velocity: Velocity { x: 3, y: -2, z: -2 }
                },
                Moon {
                    position: Position { x: 1, y: -2, z: 2 },
                    velocity: Velocity { x: -2, y: 5, z: 6 }
                },
                Moon {
                    position: Position { x: 1, y: -4, z: -1 },
                    velocity: Velocity { x: 0, y: 3, z: -6 }
                },
                Moon {
                    position: Position { x: 1, y: -4, z: 2 },
                    velocity: Velocity { x: -1, y: -6, z: 2 }
                }
            ]
        );
    }

    #[test]
    fn test_simulate_gravity_ten_steps() {
        let moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];

        let final_state = simulate_gravity_iterated(moons, 10);

        assert_eq!(
            final_state,
            vec![
                Moon {
                    position: Position::new(2, 1, -3),
                    velocity: Velocity::new(-3, -2, 1)
                },
                Moon {
                    position: Position::new(1, -8, 0),
                    velocity: Velocity::new(-1, 1, 3)
                },
                Moon {
                    position: Position::new(3, -6, 1),
                    velocity: Velocity::new(3, 2, -3)
                },
                Moon {
                    position: Position::new(2, 0, 4),
                    velocity: Velocity::new(1, -1, -1)
                }
            ]
        );
    }

    #[test]
    fn test_total_energy() {
        let moon = Moon {
            position: Position::new(2, 1, -3),
            velocity: Velocity::new(-3, -2, 1),
        };

        assert_eq!(moon.total_energy(), 36);
    }

    #[test]
    fn test_repeats() {
        let moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];

        let count = simulate_until_repeated_state(moons);

        assert_eq!(count, 2772);

        let moons = vec![
            Moon::new(-8, -10, 0),
            Moon::new(5, 5, 10),
            Moon::new(2, -7, 3),
            Moon::new(9, -8, -3),
        ];

        let count = simulate_until_repeated_state(moons);
        assert_eq!(count, 4686774924);
    }
}
