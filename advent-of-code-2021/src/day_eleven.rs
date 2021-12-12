use std::collections::{HashMap, HashSet, VecDeque};

use common::{parse::unsigned_number, prelude::*};
use nom::{
    bytes::complete::{tag, take},
    combinator::{all_consuming, into, map_parser},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-11.txt");
    let grid = parse_grid(input);

    let part_one = part_one(grid.clone());
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(mut grid: Grid) -> PartAnswer {
    let start = SystemTime::now();

    let mut number_of_flashes = 0;

    for _ in 0..100 {
        number_of_flashes += grid.step();
    }

    PartAnswer::new(number_of_flashes, start.elapsed().unwrap())
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

#[derive(Debug, PartialEq, Clone)]
struct Grid {
    grid: HashMap<(usize, usize), u8>,
}

impl Grid {
    fn step(&mut self) -> usize {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        let mut after = HashMap::new();

        // increment all energy levels by 1
        for ((x, y), energy_level) in self.grid.iter() {
            let next_energy_level = *energy_level + 1;

            if next_energy_level >= 9 {
                if seen.insert((*x, *y)) {
                    queue.push_back((*x, *y));
                }
            }

            after.insert((*x, *y), next_energy_level);
        }

        // breadth-first search starting from all sites that initially flashed
        while !queue.is_empty() {
            let (next_x, next_y) = queue.pop_front().unwrap();

            let potential_neighbor_coordinates = vec![
                (next_x + 1, next_y),
                (next_x - 1, next_y),
                (next_x, next_y + 1),
                (next_x, next_y - 1),
                (next_x + 1, next_y + 1),
                (next_x - 1, next_y - 1),
                (next_x + 1, next_y - 1),
                (next_x - 1, next_y + 1),
            ];

            for coordinates in potential_neighbor_coordinates {
                if let Some(current_energy_level) = after.get_mut(&coordinates) {
                    *current_energy_level += 1;

                    if *current_energy_level >= 9 && seen.insert(coordinates) {
                        queue.push_back(coordinates);
                    }
                }
            }
        }

        for (_coordinates, energy_level) in after.iter_mut() {
            if *energy_level >= 9 {
                *energy_level = 0;
            }
        }

        self.grid = after;

        seen.len()
    }
}

impl From<Vec<Vec<u8>>> for Grid {
    fn from(matrix: Vec<Vec<u8>>) -> Grid {
        let mut grid = HashMap::new();

        for (y, row) in matrix.iter().enumerate() {
            for (x, octopus) in row.iter().enumerate() {
                grid.insert((x, y), *octopus);
            }
        }

        Grid { grid }
    }
}

fn parse_grid(i: &str) -> Grid {
    all_consuming(terminated(grid, tag("\n")))(i).unwrap().1
}

fn grid(i: &str) -> IResult<&str, Grid> {
    into(separated_list1(tag("\n"), row))(i)
}

fn row(i: &str) -> IResult<&str, Vec<u8>> {
    many1(octopus)(i)
}

fn octopus(i: &str) -> IResult<&str, u8> {
    map_parser(take(1_usize), unsigned_number)(i)
}
