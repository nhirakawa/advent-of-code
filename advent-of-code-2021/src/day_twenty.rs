use std::collections::{HashMap, HashSet, VecDeque};

use common::prelude::*;
use log::trace;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-20.txt");
    let scanner_output = parse_scanner_output(input);

    let part_one = part_one(&scanner_output);
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one(scanner_output: &ScannerOutput) -> PartAnswer {
    let start = SystemTime::now();
    let enhanced = enhance_twice(scanner_output);

    let solution = enhanced.count_lit_pixels();

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn enhance_twice(scanner_output: &ScannerOutput) -> Image {
    let mut before = scanner_output.image_input.clone();

    for _ in 0..2 {
        let mut after = HashMap::new();

        let mut seen = HashSet::new();

        for x in (before.min_x - 2)..=(before.max_x + 2) {
            for y in (before.min_y - 2)..=(before.max_y + 2) {
                if !seen.insert((x, y)) {
                    continue;
                }

                trace!("visiting {},{}", x, y);

                let pixels = vec![
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                    (x - 1, y),
                    (x, y),
                    (x + 1, y),
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                ];

                let mut to_index = Vec::new();

                for pixel in &pixels {
                    to_index.push(before.get(pixel));
                }

                let index = to_usize(&to_index);

                let new_pixel = scanner_output.enhancement_algorithm[index];
                after.insert((x, y), new_pixel);
            }
        }
        before = Image::new(after);
    }

    before
}

fn to_usize(pixels: &[Pixel]) -> usize {
    let mut out = 0;

    for (index, pixel) in pixels.iter().enumerate() {
        let value = match pixel {
            Pixel::Dark => 0,
            Pixel::Light => 1,
        };

        out |= value << (pixels.len() - index - 1);
    }

    out
}

struct ScannerOutput {
    image_input: Image,
    enhancement_algorithm: Vec<Pixel>,
}

#[derive(Debug, Clone, Copy)]
enum Pixel {
    Dark,
    Light,
}

#[derive(Debug, Clone)]
struct Image {
    data: HashMap<(isize, isize), Pixel>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Image {
    fn new(data: HashMap<(isize, isize), Pixel>) -> Image {
        let mut min_x = isize::MAX;
        let mut max_x = 0;
        let mut min_y = isize::MAX;
        let mut max_y = 0;

        for (x, y) in data.keys().copied() {
            min_x = min_x.min(x);
            max_x = max_x.max(x);

            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }

        Image {
            data,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn get(&self, coord: &(isize, isize)) -> Pixel {
        self.data.get(coord).copied().unwrap_or(Pixel::Dark)
    }

    fn count_lit_pixels(&self) -> usize {
        self.data
            .values()
            .copied()
            .filter(|p| match p {
                Pixel::Light => true,
                Pixel::Dark => false,
            })
            .count()
    }
}

fn parse_scanner_output(i: &str) -> ScannerOutput {
    all_consuming(terminated(
        map(
            separated_pair(image_enhancement_algorithm, tag("\n\n"), image_input),
            |(enhancement_algorithm, image_input)| ScannerOutput {
                enhancement_algorithm,
                image_input,
            },
        ),
        multispace0,
    ))(i)
    .unwrap()
    .1
}

fn image_input(i: &str) -> IResult<&str, Image> {
    map(separated_list1(tag("\n"), many1(pixel)), |rows| {
        let mut out = HashMap::new();

        for (y, row) in rows.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                out.insert((x as isize, y as isize), *pixel);
            }
        }

        Image::new(out)
    })(i)
}

fn image_enhancement_algorithm(i: &str) -> IResult<&str, Vec<Pixel>> {
    many1(pixel)(i)
}

fn pixel(i: &str) -> IResult<&str, Pixel> {
    alt((dark, light))(i)
}

fn dark(i: &str) -> IResult<&str, Pixel> {
    value(Pixel::Dark, tag("."))(i)
}

fn light(i: &str) -> IResult<&str, Pixel> {
    value(Pixel::Light, tag("#"))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_usize() {
        let pixels = many1(pixel)("...#...#.").unwrap().1;
        assert_eq!(to_usize(&pixels), 34);
    }
}
