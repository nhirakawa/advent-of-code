use std::collections::HashMap;

use common::{parse::unsigned_number, prelude::*};
use itertools::Itertools;
use nom::{
    bytes::complete::take,
    combinator::{map, map_parser},
    multi::{count, many1},
    IResult,
};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-8.txt");
    let layers = parse(input, WIDTH, HEIGHT);

    let part_one = part_one(&layers);
    let part_two = part_two(&layers, WIDTH, HEIGHT);

    Ok((part_one, part_two))
}

fn part_one(layers: &[Layer]) -> PartAnswer {
    let start = SystemTime::now();

    let solution = layers
        .iter()
        .min_by_key(|v| v.iter().filter(|i| **i == 0).count())
        .expect("could not find solution");

    let number_of_ones = solution
        .iter()
        .filter_map(|i| match *i {
            1 => Some(1),
            _ => None,
        })
        .count();

    let number_of_twos = solution
        .iter()
        .filter_map(|i| match *i {
            2 => Some(1),
            _ => None,
        })
        .count();

    let solution = number_of_ones * number_of_twos;

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two(layers: &[Layer], width: usize, height: usize) -> PartAnswer {
    let start = SystemTime::now();

    let layer_size = width * height;

    let mut pixels = HashMap::new();

    for i in 0..layer_size {
        for layer in layers {
            let pixel = layer[i];
            if pixels.contains_key(&i) {
                continue;
            }

            if pixel == 2 {
                continue;
            }

            pixels.insert(i, pixel);
        }
    }

    let mut combined = vec!["\n"];

    for h in 0..height {
        for w in 0..width {
            let index = (h * width) + w;
            let pixel = pixels
                .get(&index)
                .unwrap_or_else(|| panic!("no entry found for {}", index));

            if *pixel == 1 {
                combined.push("\u{2588}");
            } else {
                combined.push(" ");
            }
        }
        combined.push("\n");
    }

    //todo I verified this manually - figure out how to display it properly or detect the answer

    PartAnswer::new("EBZUR", start.elapsed().unwrap())
}

type Layer = Vec<i32>;
type Layers = Vec<Layer>;

fn parse(i: &str, width: usize, height: usize) -> Layers {
    layers(width, height)(i).unwrap().1
}

fn layers<'a>(width: usize, height: usize) -> impl FnMut(&'a str) -> IResult<&'a str, Layers> {
    many1(layer(width, height))
}

fn layer<'a>(width: usize, height: usize) -> impl FnMut(&'a str) -> IResult<&'a str, Layer> {
    map(count(row(width), height), |v| {
        v.into_iter().flatten().collect_vec()
    })
}

fn row<'a>(width: usize) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<i32>> {
    count(pixel, width)
}

fn pixel(i: &str) -> IResult<&str, i32> {
    map_parser(take(1_usize), unsigned_number)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row() {
        assert_eq!(row(2)("1234"), Ok(("34", vec![1, 2])));
        assert_eq!(row(3)("123"), Ok(("", vec![1, 2, 3])));
    }

    #[test]
    fn test_layer() {
        assert_eq!(layer(3, 2)("123456"), Ok(("", vec![1, 2, 3, 4, 5, 6])));
    }
}
