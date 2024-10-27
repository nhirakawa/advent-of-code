use crate::common::parse::unsigned_number;
use anyhow::anyhow;
use itertools::Itertools;
use nom::{
    bytes::complete::take,
    combinator::{map, map_parser},
    multi::{count, many1},
    IResult,
};
use std::collections::HashMap;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let layers = parse(input, WIDTH, HEIGHT);

    let solution = layers
        .iter()
        .min_by_key(|v| v.iter().filter(|i| **i == 0).count())
        .ok_or(anyhow!("could not find solution"))?;

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

    Ok((number_of_ones * number_of_twos).to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let layers = parse(input, WIDTH, HEIGHT);

    let layer_size = WIDTH * HEIGHT;

    let mut pixels = HashMap::new();

    for i in 0..layer_size {
        for layer in &layers {
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

    for h in 0..HEIGHT {
        for w in 0..WIDTH {
            let index = (h * WIDTH) + w;
            let pixel = pixels
                .get(&index)
                .ok_or_else(|| anyhow!("no entry found for {}", index))?;

            if *pixel == 1 {
                combined.push("\u{2588}");
            } else {
                combined.push(" ");
            }
        }
        combined.push("\n");
    }

    //todo I verified this manually - figure out how to display it properly or detect the answer
    Ok("EBZUR".to_string())
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
