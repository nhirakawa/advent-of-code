use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use common::prelude::*;
use log::{debug, trace};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{all_consuming, map, value},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

const BUFFER: isize = 2;

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

    // let enhanced = Image {
    //     data: enhanced.data,
    //     min_x: enhanced.min_x - 5,
    //     max_x: enhanced.max_x + 5,
    //     min_y: enhanced.min_y - 5,
    //     max_y: enhanced.max_y + 5,
    // };

    println!("{:?}", enhanced);

    let solution = enhanced.count_lit_pixels();

    // 5047 -> too low
    // 5084 -> not correct
    // 5433 -> too high

    PartAnswer::new(solution, start.elapsed().unwrap())
}

fn part_two() -> PartAnswer {
    PartAnswer::default()
}

fn enhance_twice(scanner_output: &ScannerOutput) -> Image {
    let mut before = scanner_output.image_input.clone();

    debug!("\n{:?}", before);

    for iteration in 0..2 {
        let default = if iteration % 2 == 0 {
            Pixel::Dark
        } else {
            Pixel::Light
        };

        let mut after = HashMap::new();

        let mut seen = HashSet::new();

        for x in (before.min_x)..=(before.max_x) {
            for y in (before.min_y)..=(before.max_y) {
                if !seen.insert((x, y)) {
                    continue;
                }

                trace!("visiting {},{}", x, y);

                let pixels = vec![
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y),
                    (x, y),
                    (x + 1, y),
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                ];

                let mut to_index = Vec::new();

                for pixel in &pixels {
                    to_index.push(before.get(pixel, default));
                }

                let index = to_usize(&to_index);

                let new_pixel = scanner_output.enhancement_algorithm[index];
                after.insert((x, y), new_pixel);
            }
        }
        before = Image::new(after);

        debug!("\n{:?}", before);
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

#[derive(Clone)]
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

        min_x -= BUFFER;
        max_x += BUFFER;
        min_y -= BUFFER;
        max_y += BUFFER;

        Image {
            data,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn is_in_buffer_area(&self, coord: &(isize, isize)) -> bool {
        let (x, y) = *coord;

        if x <= self.min_x + BUFFER {
            true
        } else if x >= self.max_x - BUFFER {
            true
        } else if y <= self.min_y + BUFFER {
            true
        } else if y >= self.max_y - BUFFER {
            true
        } else {
            false
        }
    }

    fn get(&self, coord: &(isize, isize), default: Pixel) -> Pixel {
        self.data.get(coord).copied().unwrap_or(default)
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

impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let value = match self.get(&(x, y), Pixel::Dark) {
                    Pixel::Dark => ".",
                    Pixel::Light => "#",
                };

                write!(f, "{}", value)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
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
