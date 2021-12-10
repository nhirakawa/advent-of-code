use std::collections::HashSet;
use std::iter::IntoIterator;

use common::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-17.txt");
    let cubes = parse_input(input);

    let part_one = part_one(&cubes);
    let part_two = part_two(&cubes);

    Ok((part_one, part_two))
}

fn part_one(cubes: &Cubes) -> PartAnswer {
    let start = SystemTime::now();

    let mut cubes = cubes.clone();

    for _ in 1..=6 {
        cubes.next_iteration();
    }

    let solution = count_active_cubes(&cubes.cubes);

    let elapsed = start.elapsed().unwrap();

    (solution, elapsed).into()
}

fn get_next_state(current_state: &ActiveState, number_of_active_neighbors: usize) -> ActiveState {
    match current_state {
        ActiveState::Active => match number_of_active_neighbors {
            2 | 3 => ActiveState::Active,
            _ => ActiveState::Inactive,
        },
        ActiveState::Inactive => {
            if number_of_active_neighbors == 3 {
                ActiveState::Active
            } else {
                ActiveState::Inactive
            }
        }
    }
}

fn count_active_cubes(cubes: &HashSet<Coordinates>) -> u64 {
    cubes.len() as u64
}

fn get_number_of_active_cubes(coordinates: &Coordinates, cubes: &Cubes) -> usize {
    let neighbors = get_neighboring_cubes(coordinates);

    let mut active_counter = 0;

    for neighbor in neighbors {
        let active_state = cubes
            .cubes
            .get(&neighbor)
            .map(|_| ActiveState::Active)
            .unwrap_or(ActiveState::Inactive);

        if active_state == ActiveState::Active {
            active_counter += 1;
        }
    }

    active_counter
}

fn get_neighboring_cubes(coordinates: &Coordinates) -> HashSet<Coordinates> {
    match coordinates {
        Coordinates::ThreeDimensional(c) => get_three_dimensional_neighbors(c),
        Coordinates::FourDimensional(c) => get_four_dimensional_neighbors(c),
    }
}

fn get_three_dimensional_neighbors(coordinates: &(i64, i64, i64)) -> HashSet<Coordinates> {
    let mut output = HashSet::new();

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }

                let neighbor = (coordinates.0 + x, coordinates.1 + y, coordinates.2 + z);
                let neighbor = Coordinates::ThreeDimensional(neighbor);

                output.insert(neighbor);
            }
        }
    }

    output
}

fn get_four_dimensional_neighbors(coordinates: &(i64, i64, i64, i64)) -> HashSet<Coordinates> {
    let mut output = HashSet::new();

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }

                    let neighbor = (
                        coordinates.0 + x,
                        coordinates.1 + y,
                        coordinates.2 + z,
                        coordinates.3 + w,
                    );
                    let neighbor = Coordinates::FourDimensional(neighbor);

                    output.insert(neighbor);
                }
            }
        }
    }

    output
}

fn part_two(cubes: &Cubes) -> PartAnswer {
    let start = SystemTime::now();

    let cubes: HashSet<Coordinates> = cubes
        .clone()
        .cubes
        .into_iter()
        .map(|coordinates| match coordinates {
            Coordinates::ThreeDimensional((x, y, z)) => Coordinates::FourDimensional((x, y, z, 0)),
            Coordinates::FourDimensional(c) => Coordinates::FourDimensional(c),
        })
        .collect();

    let mut cubes: Cubes = cubes.into();

    for _ in 1..=6 {
        cubes.next_iteration();
    }

    let solution = count_active_cubes(&cubes.cubes);

    let elapsed = start.elapsed().unwrap();

    (solution, elapsed).into()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Coordinates {
    ThreeDimensional((i64, i64, i64)),
    FourDimensional((i64, i64, i64, i64)),
}

#[derive(Debug, PartialEq, Clone)]
struct Cubes {
    cubes: HashSet<Coordinates>,
    iteration: usize,
}

impl Cubes {
    pub fn next_iteration(&mut self) {
        let mut after = HashSet::new();

        let mut cubes_to_consider = HashSet::new();

        for coordinate in &self.cubes {
            cubes_to_consider.insert(*coordinate);

            let neighbors = get_neighboring_cubes(coordinate);
            cubes_to_consider.extend(neighbors);
        }

        for coordinates in cubes_to_consider {
            let state = self
                .cubes
                .get(&coordinates)
                .map(|_| ActiveState::Active)
                .unwrap_or(ActiveState::Inactive);

            let number_of_active_neighbors = get_number_of_active_cubes(&coordinates, self);

            let next_state = get_next_state(&state, number_of_active_neighbors);

            if next_state == ActiveState::Active {
                after.insert(coordinates);
            }
        }

        self.cubes = after;
        self.iteration += 1;
    }
}

impl From<HashSet<Coordinates>> for Cubes {
    fn from(cubes: HashSet<Coordinates>) -> Self {
        Self {
            cubes,
            iteration: 0,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ActiveState {
    Active,
    Inactive,
}

fn parse_input(i: &str) -> Cubes {
    let mut output = HashSet::new();

    for (y, row) in i.split('\n').enumerate() {
        for (x, column) in row.chars().enumerate() {
            let coordinates = (x as i64, y as i64, 0);
            let coordinates = Coordinates::ThreeDimensional(coordinates);

            if column == '#' {
                output.insert(coordinates);
            }
        }
    }

    output.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_state() {
        assert_eq!(
            get_next_state(&ActiveState::Active, 1),
            ActiveState::Inactive
        );
        assert_eq!(get_next_state(&ActiveState::Active, 2), ActiveState::Active);
        assert_eq!(get_next_state(&ActiveState::Active, 3), ActiveState::Active);
        assert_eq!(
            get_next_state(&ActiveState::Active, 4),
            ActiveState::Inactive
        );

        assert_eq!(
            get_next_state(&ActiveState::Inactive, 2),
            ActiveState::Inactive
        );
        assert_eq!(
            get_next_state(&ActiveState::Inactive, 3),
            ActiveState::Active
        );
        assert_eq!(
            get_next_state(&ActiveState::Inactive, 4),
            ActiveState::Inactive
        );
    }

    #[test]
    fn test_count_active_cubes() {
        let cubes = vec![
            Coordinates::ThreeDimensional((0, 0, 0)),
            Coordinates::ThreeDimensional((2, 3, 4)),
        ]
        .into_iter()
        .collect();

        assert_eq!(count_active_cubes(&cubes), 2);
    }

    #[test]
    fn test_get_three_dimensional_neighbors() {
        let actual = get_three_dimensional_neighbors(&(2, -1, 1));
        let expected = vec![
            Coordinates::ThreeDimensional((1, -1, 1)),
            Coordinates::ThreeDimensional((1, 0, 1)),
            Coordinates::ThreeDimensional((1, -2, 1)),
            Coordinates::ThreeDimensional((1, -1, 0)),
            Coordinates::ThreeDimensional((1, -1, 2)),
            Coordinates::ThreeDimensional((1, -1, 0)),
            Coordinates::ThreeDimensional((1, 0, 2)),
            Coordinates::ThreeDimensional((1, 0, 0)),
            Coordinates::ThreeDimensional((1, -2, 0)),
            Coordinates::ThreeDimensional((1, -2, 2)),
            Coordinates::ThreeDimensional((3, -2, 0)),
            Coordinates::ThreeDimensional((3, -2, 2)),
            Coordinates::ThreeDimensional((3, -1, 1)),
            Coordinates::ThreeDimensional((3, -1, 2)),
            Coordinates::ThreeDimensional((3, -1, 0)),
            Coordinates::ThreeDimensional((3, -2, 1)),
            Coordinates::ThreeDimensional((3, 0, 1)),
            Coordinates::ThreeDimensional((3, 0, 0)),
            Coordinates::ThreeDimensional((3, 0, 2)),
            Coordinates::ThreeDimensional((2, 0, 1)),
            Coordinates::ThreeDimensional((2, -2, 1)),
            Coordinates::ThreeDimensional((2, -1, 0)),
            Coordinates::ThreeDimensional((2, -1, 2)),
            Coordinates::ThreeDimensional((2, 0, 2)),
            Coordinates::ThreeDimensional((2, 0, 0)),
            Coordinates::ThreeDimensional((2, -2, 0)),
            Coordinates::ThreeDimensional((2, -2, 2)),
        ]
        .into_iter()
        .collect();

        assert_eq!(actual, expected);
    }
}
