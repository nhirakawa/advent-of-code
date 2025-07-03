use crate::common::parse::unsigned_number;
use nom::{
    bytes::complete::{tag, take},
    combinator::{all_consuming, map_parser},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult, Parser,
};
use std::collections::{HashMap, HashSet, VecDeque};

type Digit = u8;
type Row = Vec<u8>;
type Grid = Vec<Row>;
type Coordinate = (usize, usize);
type HeightMap = HashMap<Coordinate, u8>;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let grid = parse_grid(input);

    let height_map = build_height_map(&grid);
    let lowest_points = find_lowest_points(&height_map);

    Ok(lowest_points
        .iter()
        .map(|coordinate| height_map.get(coordinate))
        .flat_map(Option::into_iter)
        .map(|height| (height + 1) as u32)
        .sum::<u32>())
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let grid = parse_grid(input);

    let height_map = build_height_map(&grid);
    let lowest_points = find_lowest_points(&height_map);

    let mut basin_sizes = Vec::new();

    for point in lowest_points {
        basin_sizes.push(find_basin(&height_map, point).len());
    }

    basin_sizes.sort_unstable();

    Ok(basin_sizes[basin_sizes.len() - 1]
        * basin_sizes[basin_sizes.len() - 2]
        * basin_sizes[basin_sizes.len() - 3])
}

fn build_height_map(grid: &[Row]) -> HeightMap {
    let mut height_map = HashMap::new();

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            let coordinates = (row_index, column_index);
            height_map.insert(coordinates, *column);
        }
    }

    height_map
}

fn find_lowest_points(height_map: &HeightMap) -> HashSet<Coordinate> {
    let mut lowest_points = HashSet::new();

    for ((x, y), height) in height_map.iter() {
        let lower_than_surrounding = vec![
            height_map.get(&(x - 1, *y)),
            height_map.get(&(x + 1, *y)),
            height_map.get(&(*x, y - 1)),
            height_map.get(&(*x, y + 1)),
        ]
        .into_iter()
        .flat_map(|o| o.into_iter())
        .all(|other| other > height);

        if lower_than_surrounding {
            lowest_points.insert((*x, *y));
        }
    }

    lowest_points
}

fn find_basin(height_map: &HeightMap, lowest_point: Coordinate) -> HashSet<Coordinate> {
    let mut queue = VecDeque::from(vec![lowest_point]);
    let mut seen = HashSet::new();

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if !seen.insert((x, y)) {
            continue;
        }

        let surrounding = vec![(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)];

        for coordinate in surrounding {
            if let Some(height) = height_map.get(&coordinate) {
                if *height != 9 {
                    queue.push_back(coordinate);
                }
            }
        }
    }

    seen
}

fn parse_grid(i: &str) -> Grid {
    all_consuming(terminated(grid, tag("\n")))
        .parse(i)
        .unwrap()
        .1
}

fn grid(i: &str) -> IResult<&str, Grid> {
    separated_list1(tag("\n"), row).parse(i)
}

fn row(i: &str) -> IResult<&str, Row> {
    many1(digit).parse(i)
}

fn digit(i: &str) -> IResult<&str, Digit> {
    map_parser(take(1usize), unsigned_number).parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit() {
        assert_eq!(digit("123"), Ok(("23", 1)))
    }
}
