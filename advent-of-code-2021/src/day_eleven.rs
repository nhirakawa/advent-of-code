use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
};

use common::{parse::unsigned_number, prelude::*};
use log::debug;
use nom::{
    bytes::complete::{tag, take},
    character::complete::multispace0,
    combinator::{all_consuming, into, map_parser},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-11.txt");
    let grid = parse_grid(input);

    let part_one = part_one(grid.clone());
    let part_two = part_two(grid);

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

fn part_two(mut grid: Grid) -> PartAnswer {
    let start = SystemTime::now();

    let mut number_of_steps = 0;

    while !grid.all_flashing() {
        grid.step();
        number_of_steps += 1;
    }

    PartAnswer::new(number_of_steps, start.elapsed().unwrap())
}

#[derive(PartialEq, Clone)]
struct Grid {
    grid: HashMap<(isize, isize), u8>,
    max_x: isize,
    max_y: isize,
}

impl Grid {
    fn all_flashing(&self) -> bool {
        self.grid.values().all(|energy| *energy == 0)
    }

    fn step(&mut self) -> usize {
        debug!("before {}", self);

        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        let mut after = HashMap::new();

        // increment all energy levels by 1
        for ((x, y), energy_level) in self.grid.iter() {
            let next_energy_level = *energy_level + 1;

            debug!("{:?} is now {}", (x, y), next_energy_level);

            if next_energy_level > 9 && seen.insert((*x, *y)) {
                debug!("enqueueing {:?} because it flashed", (x, y));
                queue.push_back((*x, *y));
            }

            after.insert((*x, *y), next_energy_level);
        }

        // breadth-first search starting from all sites that initially flashed
        while !queue.is_empty() {
            let (next_x, next_y) = queue.pop_front().unwrap();
            debug!("visiting neighbors of {:?}", (next_x, next_y));

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
                    debug!(
                        "incrementing energy of {:?} because its neighbor flashed",
                        coordinates
                    );

                    *current_energy_level += 1;

                    if *current_energy_level > 9 && seen.insert(coordinates) {
                        debug!("enqueueing {:?} because it flashed", coordinates);
                        queue.push_back(coordinates);
                    }
                }
            }
        }

        for (_coordinates, energy_level) in after.iter_mut() {
            if *energy_level > 9 {
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

        let mut max_x = 0;
        let mut max_y = 0;

        for (x, row) in matrix.iter().enumerate() {
            for (y, octopus) in row.iter().enumerate() {
                let x = x as isize;
                let y = y as isize;

                grid.insert((x as isize, y as isize), *octopus);

                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }

        Grid { grid, max_x, max_y }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        for x in 0..=self.max_x {
            for y in 0..=self.max_y {
                let energy = self.grid[&(x, y)];
                write!(f, "{}", energy)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse_grid(i: &str) -> Grid {
    all_consuming(terminated(grid, multispace0))(i).unwrap().1
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_small() {
        let mut grid = parse_grid("11111\n19991\n19191\n19991\n11111\n");

        grid.step();
        assert_eq!(grid, parse_grid("34543\n40004\n50005\n40004\n34543\n"));

        grid.step();
        assert_eq!(grid, parse_grid("45654\n51115\n61116\n51115\n45654\n"));
    }

    #[test]
    fn test_example() {
        let mut grid = parse_grid("5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n");

        grid.step();
        assert_eq!(grid, parse_grid("6594254334\n3856965822\n6375667284\n7252447257\n7468496589\n5278635756\n3287952832\n7993992245\n5957959665\n6394862637\n"));

        grid.step();
        assert_eq!(grid, parse_grid("8807476555\n5089087054\n8597889608\n8485769600\n8700908800\n6600088989\n6800005943\n0000007456\n9000000876\n8700006848\n"));
    }

    #[test]
    fn test_iterated() {
        let mut grid = parse_grid("5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n");
        let mut flashes = 0;

        for _ in 0..10 {
            flashes += grid.step();
        }
        assert_eq!(grid, parse_grid("0481112976\n0031112009\n0041112504\n0081111406\n0099111306\n0093511233\n0442361130\n5532252350\n0532250600\n0032240000\n"));
        assert_eq!(flashes, 204);

        for _ in 0..10 {
            flashes += grid.step();
        }
        assert_eq!(grid, parse_grid("3936556452\n5686556806\n4496555690\n4448655580\n4456865570\n5680086577\n7000009896\n0000000344\n6000000364\n4600009543\n"));

        for _ in 0..10 {
            flashes += grid.step();
        }
        assert_eq!(grid, parse_grid("0643334118\n4253334611\n3374333458\n2225333337\n2229333338\n2276733333\n2754574565\n5544458511\n9444447111\n7944446119\n"));

        for _ in 0..10 {
            flashes += grid.step();
        }
        assert_eq!(grid, parse_grid("6211111981\n0421111119\n0042111115\n0003111115\n0003111116\n0065611111\n0532351111\n3322234597\n2222222976\n2222222762\n"));

        for _ in 0..10 {
            flashes += grid.step();
        }
        assert_eq!(grid, parse_grid("9655556447\n4865556805\n4486555690\n4458655580\n4574865570\n5700086566\n6000009887\n8000000533\n6800000633\n5680000538\n"));

        for _ in 0..10 {
            flashes += grid.step();
        }
        assert_eq!(grid, parse_grid("2533334200\n2743334640\n2264333458\n2225333337\n2225333338\n2287833333\n3854573455\n1854458611\n1175447111\n1115446111\n"));

        for _ in 0..10 {
            flashes += grid.step();
        }
        assert_eq!(grid, parse_grid("8211111164\n0421111166\n0042111114\n0004211115\n0000211116\n0065611111\n0532351111\n7322235117\n5722223475\n4572222754\n"));

        for _ in 0..10 {
            flashes += grid.step();
        }
        assert_eq!(grid, parse_grid("1755555697\n5965555609\n4486555680\n4458655580\n4570865570\n5700086566\n7000008666\n0000000990\n0000000800\n0000000000\n"));

        for _ in 0..10 {
            flashes += grid.step();
        }
        assert_eq!(grid, parse_grid("7433333522\n2643333522\n2264333458\n2226433337\n2222433338\n2287833333\n2854573333\n4854458333\n3387779333\n3333333333\n"));

        for _ in 0..10 {
            flashes += grid.step();
        }
        assert_eq!(grid, parse_grid("0397666866\n0749766918\n0053976933\n0004297822\n0004229892\n0053222877\n0532222966\n9322228966\n7922286866\n6789998766"));
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn test_step_until_synchronized() {
        let mut grid = parse_grid("5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n");

        let mut number_of_steps = 0;

        while !grid.all_flashing() {
            grid.step();
            number_of_steps += 1;
        }

        assert_eq!(number_of_steps, 195);
    }
}
