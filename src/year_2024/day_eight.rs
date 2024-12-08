use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let Grid {
        nodes,
        max_x,
        max_y,
    } = parse(input);

    let mut antinodes = HashSet::new();

    for coordinates in nodes.values() {
        for coordinate in coordinates {
            for other in coordinates {
                if coordinate == other {
                    continue;
                }

                if let Some(antinode) =
                    coordinates_along_slope(*coordinate, *other, (max_x, max_y)).nth(2)
                {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    Ok(antinodes.len().to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let Grid {
        nodes,
        max_x,
        max_y,
    } = parse(input);

    let mut antinodes = HashSet::new();

    for coordinates in nodes.values() {
        for coordinate in coordinates {
            for other in coordinates {
                if coordinate == other {
                    continue;
                }

                antinodes.extend(coordinates_along_slope(*coordinate, *other, (max_x, max_y)));
            }
        }
    }

    Ok(antinodes.len().to_string())
}

fn coordinates_along_slope(
    start: Coordinate,
    next: Coordinate,
    bounds: (isize, isize),
) -> impl Iterator<Item = Coordinate> {
    let (slope_x, slope_y) = (next.0 - start.0, next.1 - start.1);

    successors(Some(start), move |current| {
        let antinode = (current.0 + slope_x, current.1 + slope_y);

        if (antinode.0 < 0 || antinode.0 > bounds.0) || (antinode.1 < 0 || antinode.1 > bounds.1) {
            None
        } else {
            Some(antinode)
        }
    })
}

type Coordinate = (isize, isize);

struct Grid {
    nodes: HashMap<String, HashSet<Coordinate>>,
    max_x: isize,
    max_y: isize,
}

fn parse(input: &str) -> Grid {
    let mut nodes = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        max_y = max_y.max(y as isize);

        for (x, c) in line.chars().enumerate() {
            max_x = max_x.max(x as isize);

            if c != '.' {
                nodes.insert((x as isize, y as isize), c.to_string());
            }
        }
    }

    let mut node_to_coordinates: HashMap<String, HashSet<Coordinate>> = HashMap::new();

    for (coordinate, node) in nodes {
        node_to_coordinates
            .entry(node)
            .or_default()
            .insert(coordinate);
    }

    Grid {
        nodes: node_to_coordinates,
        max_x,
        max_y,
    }
}
