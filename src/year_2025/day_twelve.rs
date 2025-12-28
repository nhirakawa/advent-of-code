use anyhow::{Context, anyhow, bail};
use std::str::FromStr;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let summary = Summary::from_str(input)?;

    let mut region_count = 0;

    for region in summary.regions() {
        let counts = region.counts();

        let mut shape_area = 0;

        for (index, &count) in counts.iter().enumerate() {
            let shape = summary
                .get_shape(index)
                .ok_or_else(|| anyhow!("no shape at index {index}"))?;

            shape_area += shape.area() * count;
        }

        if shape_area <= region.area() {
            region_count += 1;
        }
    }

    Ok(region_count)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Summary {
    shapes: Vec<Shape>,
    regions: Regions,
}

impl Summary {
    fn get_shape(&self, index: usize) -> Option<&Shape> {
        self.shapes.get(index)
    }

    fn regions(&self) -> impl Iterator<Item = &Region> + '_ {
        self.regions.regions.iter()
    }
}

impl FromStr for Summary {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = s.split("\n\n").peekable();

        let mut shapes = Vec::new();

        while let Some(block) = blocks.next() {
            if blocks.peek().is_some() {
                let shape = Shape::from_str(block)?;
                shapes.push(shape);
            } else {
                let regions = Regions::from_str(block)?;
                return Ok(Summary { shapes, regions });
            }
        }

        bail!("Invalid input")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Shape {
    id: usize,
    shape: [bool; 9],
}

impl Shape {
    #[allow(unused)]
    fn new(id: usize, shape: [bool; 9]) -> Self {
        Self { id, shape }
    }

    fn area(&self) -> usize {
        self.shape.iter().filter(|&&v| v).count()
    }
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().filter(|line| !line.is_empty());

        let first = lines.next().ok_or_else(|| anyhow!("Empty shape"))?;
        let first = first.trim();

        if !first.ends_with(":") {
            bail!("Could not parse ID line: {}", first);
        }

        let index_of_colon = first
            .find(':')
            .ok_or_else(|| anyhow!("Could not find colon"))?;

        let id = first[..index_of_colon].parse::<usize>()?;

        let mut shape = [false; 9];

        let mut index = 0;

        while let Some(line) = lines.next() {
            let line = line.trim();
            for b in line.bytes() {
                if b == b'#' {
                    shape[index] = true;
                } else if b == b'.' {
                    shape[index] = false;
                } else {
                    bail!("Unexpected character: {}", line);
                }
                index += 1;
            }
        }

        Ok(Self { id, shape })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Regions {
    regions: Vec<Region>,
}

impl FromStr for Regions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut regions = Vec::new();

        for line in s.lines().filter(|line| !line.is_empty()) {
            let region = Region::from_str(line)?;
            regions.push(region);
        }

        Ok(Self { regions })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Region {
    wide: usize,
    long: usize,
    counts: Vec<usize>,
}

impl Region {
    #[allow(unused)]
    fn new(wide: usize, long: usize, counts: Vec<usize>) -> Self {
        Self { wide, long, counts }
    }

    fn counts(&self) -> &[usize] {
        &self.counts
    }

    fn area(&self) -> usize {
        self.wide * self.long
    }
}

impl FromStr for Region {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (area, raw_counts) = s.split_once(':').ok_or_else(|| anyhow!("Empty region"))?;

        let (wide, long) = area
            .trim()
            .split_once('x')
            .ok_or_else(|| anyhow!("Could not parse region area - {area}"))?;

        let wide = wide.parse()?;
        let long = long.parse()?;

        let mut counts = Vec::new();

        for raw_count in raw_counts.split(' ').filter(|s| !s.is_empty()) {
            let count = raw_count.parse().with_context(|| format!("{raw_counts}"))?;
            counts.push(count);
        }

        Ok(Region { wide, long, counts })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_from_str() {
        assert_eq!(
            Shape::from_str("0:\n###\n##.\n##.").unwrap(),
            Shape::new(0, [true, true, true, true, true, false, true, true, false])
        );
    }

    #[test]
    fn test_region_from_str() {
        assert_eq!(
            Region::from_str("12x5: 1 0 1 0 2 2").unwrap(),
            Region::new(12, 5, vec![1, 0, 1, 0, 2, 2])
        );
    }
}
