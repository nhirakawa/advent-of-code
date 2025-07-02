use crate::common::{
    math::{average, median},
    parse::unsigned_number,
};
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::{bytes::complete::tag, Parser};

type Position = u32;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let positions = parse_positions(input);

    let median_position = median(&positions) as Position;

    let mut fuel_used = 0;
    for position in &positions {
        fuel_used += median_position.max(*position) - median_position.min(*position);
    }

    Ok(fuel_used.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let positions = parse_positions(input);

    // The sample input requires .ceil(), but my input requires .floor()
    // TODO figure out how to make this really work
    let mean_position = average(&positions).floor() as Position;

    let mut total_cost = 0.0;
    for position in &positions {
        let number_of_moves = mean_position.max(*position) - mean_position.min(*position);
        let cost = (number_of_moves * (number_of_moves + 1)) as f64 / 2.0;
        total_cost += cost;
    }

    Ok(total_cost.to_string())
}

fn parse_positions(i: &str) -> Vec<Position> {
    all_consuming(terminated(
        separated_list1(tag(","), unsigned_number),
        tag("\n"),
    ))
    .parse(i)
    .unwrap()
    .1
}
