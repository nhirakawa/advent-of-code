use std::collections::{HashMap, HashSet};

use common::prelude::AdventOfCodeResult;

use common::prelude::*;
use itertools::Itertools;
use log::debug;

use crate::computer::Computer;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-17.txt");

    let part_one = part_one(input);
    let part_two = part_two(input);

    Ok((part_one, part_two))
}

fn part_one(program: &str) -> PartAnswer {
    let start = SystemTime::now();

    let mut computer = Computer::from_program(program);

    computer.step_until_halt();

    let mut map = HashMap::new();

    /*
     * Coordinate system
     * top-left is (0,0)
     * Moving right is positive x
     * Moving down is positive y
     */

    let mut x: usize = 0;
    let mut y: usize = 0;

    let mut max_x = 0;
    let mut max_y = 0;

    while let Some(output) = computer.get_output() {
        let status = output as u8 as char;

        if status == '\n' {
            y += 1;
            x = 0;
            continue;
        } else {
            map.insert((x, y), status);
            x += 1;
        }

        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    let mut intersections = HashSet::new();

    // intersections cannot be at borders
    for x in 1..max_x - 1 {
        for y in 1..max_y - 1 {
            let status = map.get(&(x, y)).unwrap();

            if *status != '#' {
                continue;
            }

            let neighbors = &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .iter()
                .filter_map(|c| map.get(c))
                .filter(|c| **c == '#')
                .count();

            if *neighbors == 4 {
                debug!("({x}, {y}) -> {}", x * y);
                intersections.insert((x, y));
            }
        }
    }

    // for y in 0..=max_y {
    //     for x in 0..max_x {
    //         if intersections.contains(&(x, y)) {
    //             print!("O");
    //         } else {
    //             let status = map.get(&(x, y)).unwrap();
    //             let out = match status {
    //                 '#' => "\u{2588}",
    //                 '.' => " ",
    //                 '^' => "^",
    //                 _ => panic!("{}", status),
    //             };
    //             print!("{}", status);
    //         }
    //     }
    //     println!();
    // }

    let alignment_sum = sum_alignment_parameters(intersections);

    PartAnswer::new(alignment_sum, start.elapsed().unwrap())
}

fn part_two(program: &str) -> PartAnswer {
    /*

        A  : 65
        B  : 66
        C  : 67
        ,  : 44
        \n : 10

        L  : 76
        R  : 82

        6  : 54
        8  : 56
        12 : 49 50

        y : 121
        n : 110

        A B A A B C B C C B

        A : l 12 r 8 l 6 r 8 l 6
        B : r 8 l 12 l 12 r 8
        C : l 6 r 6 l 12

    */

    let start = SystemTime::now();

    let mut computer = Computer::from_program(program);

    computer.set(0, 2);

    let inputs = [65, 66, 65, 65, 66, 67, 66, 67, 67, 66];
    itertools::Itertools::intersperse(inputs.iter().copied(), 44).collect_vec();

    for input in inputs {
        computer.push_input(input);
    }
    computer.push_input(10);

    // A
    computer.push_input(76); // L
    computer.push_input(44); // ,
    computer.push_input(49); // 1
    computer.push_input(50); // 2
    computer.push_input(44); // ,
    computer.push_input(82); // R
    computer.push_input(44); // ,
    computer.push_input(56); // 8
    computer.push_input(44); // ,
    computer.push_input(76); // L
    computer.push_input(44); // ,
    computer.push_input(54); // 6
    computer.push_input(44); // ,
    computer.push_input(82); // R
    computer.push_input(44); // ,
    computer.push_input(56); // 8
    computer.push_input(44); // ,
    computer.push_input(76); // L
    computer.push_input(44); // ,
    computer.push_input(54); // 6
    computer.push_input(10); // \n

    // B
    computer.push_input(82); // R
    computer.push_input(44); // ,
    computer.push_input(56); // 8
    computer.push_input(44); // ,
    computer.push_input(76); // L
    computer.push_input(44); // ,
    computer.push_input(49); // 1
    computer.push_input(50); // 2
    computer.push_input(44); // ,
    computer.push_input(76); // L
    computer.push_input(44); // ,
    computer.push_input(49); // 1
    computer.push_input(50); // 2
    computer.push_input(44); // ,
    computer.push_input(82); // R
    computer.push_input(44); // ,
    computer.push_input(56); // 8
    computer.push_input(10); // \n

    // C
    computer.push_input(76); // L
    computer.push_input(44); // ,
    computer.push_input(54); // 6
    computer.push_input(44); // ,
    computer.push_input(82); // R
    computer.push_input(44); // ,
    computer.push_input(54); // 6
    computer.push_input(44); // ,
    computer.push_input(76); // L
    computer.push_input(44); // ,
    computer.push_input(49); // 1
    computer.push_input(50); // 2
    computer.push_input(10); // \n

    computer.push_input(110); // y
    computer.push_input(10); // \n

    computer.step_until_halt();

    while let Some(output) = computer.get_output() {
        // not 46
        if output > 127 {
            return PartAnswer::new(output, start.elapsed().unwrap());
        }
    }

    PartAnswer::default()
}

fn sum_alignment_parameters<I>(intersections: I) -> usize
where
    I: IntoIterator<Item = (usize, usize)>,
{
    intersections.into_iter().map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_alignment_parameters() {
        assert_eq!(
            sum_alignment_parameters([(2, 2), (2, 4), (6, 4), (10, 4)]),
            76
        );
    }
}
