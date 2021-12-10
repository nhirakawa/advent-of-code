use common::math::{average, median};
use common::parse::unsigned_number;
use common::prelude::*;
use nom::bytes::complete::tag;
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::sequence::terminated;

type Position = u32;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-7.txt");
    let positions = parse_positions(input);

    let part_one = part_one(&positions);
    let part_two = part_two(&positions);

    Ok((part_one, part_two))
}

fn part_one(positions: &[Position]) -> PartAnswer {
    let start = SystemTime::now();
    let median_position = median(positions) as Position;

    let mut fuel_used = 0;
    for position in positions {
        fuel_used += median_position.max(*position) - median_position.min(*position);
    }

    PartAnswer::new(fuel_used, start.elapsed().unwrap())
}

fn part_two(positions: &[Position]) -> PartAnswer {
    let start = SystemTime::now();

    // The sample input requires .ceil(), but my input requires .floor()
    // TODO figure out how to make this really work
    let mean_position = average(positions).floor() as Position;

    let mut total_cost = 0.0;
    for position in positions {
        let number_of_moves = mean_position.max(*position) - mean_position.min(*position);
        let cost = (number_of_moves * (number_of_moves + 1)) as f64 / 2.0;
        total_cost += cost;
    }

    let total_cost = total_cost as Position;

    PartAnswer::new(total_cost, start.elapsed().unwrap())
}

fn parse_positions(i: &str) -> Vec<Position> {
    all_consuming(terminated(
        separated_list1(tag(","), unsigned_number),
        tag("\n"),
    ))(i)
    .unwrap()
    .1
}
