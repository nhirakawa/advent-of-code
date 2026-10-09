use itertools::Itertools;
use log::debug;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (_max_x, _max_y, grid) = parse_grid(input);

    let mut total_price = 0;
    let mut regions: Vec<Region> = vec![];

    for (coordinate, plot) in &grid {
        let mut is_in_another_region = false;
        for region in &regions {
            if region.contains(coordinate) {
                is_in_another_region = true;
                break;
            }
        }

        if is_in_another_region {
            debug!(
                "Skipping coordinate {:?} as it is in another region",
                coordinate
            );
            continue;
        }

        let now = Instant::now();
        let (region, search_size) = flood_search(*coordinate, &grid)?;
        let elapsed = now.elapsed();
        debug!(
            "Took {elapsed:?} to discover {} coordinates ({search_size} searched) starting from {coordinate:?}",
            region.len()
        );

        let mut sorted_region = region.coordinates.iter().copied().collect_vec();
        sorted_region.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

        debug!(
            "Region found at {coordinate:?} with plot {plot} (len:{}): {sorted_region:?}",
            sorted_region.len()
        );

        total_price += region.area() * region.perimeter();
        regions.push(region);
    }

    debug!("Found {} regions", regions.len());

    Ok(total_price)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (_max_x, _max_y, grid) = parse_grid(input);

    let mut regions: Vec<Region> = vec![];

    for coordinate in grid.keys() {
        let mut is_in_another_region = false;
        for region in &regions {
            if region.contains(coordinate) {
                is_in_another_region = true;
                break;
            }
        }

        if is_in_another_region {
            debug!(
                "Skipping coordinate {:?} as it is in another region",
                coordinate
            );
            continue;
        }

        let (region, _) = flood_search(*coordinate, &grid)?;

        regions.push(region);
    }

    debug!("Found {} regions", regions.len());

    let mut total_price = 0;

    for region in regions {
        let number_of_sides = region.count_sides();

        total_price += region.area() * number_of_sides;
    }

    Ok(total_price)
}

fn flood_search(coordinate: Coordinate, grid: &Grid) -> anyhow::Result<(Region, usize)> {
    let plot = grid
        .get(&coordinate)
        .ok_or_else(|| anyhow::anyhow!("No plot found at coordinate {coordinate:?}"))?;

    debug!("Flood search at {:?} ({plot})", coordinate);

    let mut visited = HashSet::new();
    let mut seen = HashSet::new();

    let mut to_visit = VecDeque::new();
    to_visit.push_back(coordinate);

    let mut queue_count = 0;

    while let Some(current) = to_visit.pop_front() {
        if !seen.insert(current) {
            continue;
        }

        queue_count += 1;

        if let Some(current_plot) = grid.get(&current) {
            if current_plot != plot {
                continue;
            }

            visited.insert(current);

            let potential_neighbor_coordinates = [
                (current.0 + 1, current.1),
                (current.0 - 1, current.1),
                (current.0, current.1 + 1),
                (current.0, current.1 - 1),
            ];

            let neighboring_plots = potential_neighbor_coordinates
                .iter()
                .filter_map(|c| grid.get(c).map(|p| (c, *p)))
                .filter(|(_, p)| p == plot)
                .collect_vec();

            for (neighbor, _) in neighboring_plots {
                if !seen.contains(neighbor) {
                    to_visit.push_back(*neighbor);
                }
            }
        }
    }

    Ok((Region::new(*plot, visited), queue_count))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum EdgeOrientation {
    Top,
    Right,
    Bottom,
    Left,
}

type Coordinate = (isize, isize);
type Grid = HashMap<Coordinate, char>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Region {
    plot: char,
    coordinates: HashSet<Coordinate>,
}

impl Region {
    pub fn new<I: IntoIterator<Item = Coordinate>>(plot: char, items: I) -> Region {
        Region {
            plot,
            coordinates: items.into_iter().collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.coordinates.len()
    }

    pub fn area(&self) -> usize {
        self.coordinates.len()
    }

    pub fn perimeter(&self) -> usize {
        let mut perimeter = 0;

        for coordinate in &self.coordinates {
            let neighboring_coordinates = [
                (coordinate.0 + 1, coordinate.1),
                (coordinate.0 - 1, coordinate.1),
                (coordinate.0, coordinate.1 + 1),
                (coordinate.0, coordinate.1 - 1),
            ];

            for neighbor in neighboring_coordinates {
                if !self.coordinates.contains(&neighbor) {
                    perimeter += 1;
                }
            }
        }

        perimeter
    }

    pub fn count_sides(&self) -> usize {
        let mut edges = HashSet::new();

        for coordinate in &self.coordinates {
            let has_neighbor_above = self.coordinates.contains(&(coordinate.0, coordinate.1 - 1));

            if !has_neighbor_above {
                edges.insert((*coordinate, EdgeOrientation::Top));
            }

            let has_neighbor_below = self.coordinates.contains(&(coordinate.0, coordinate.1 + 1));

            if !has_neighbor_below {
                edges.insert((*coordinate, EdgeOrientation::Bottom));
            }

            let has_neighbor_left = self.coordinates.contains(&(coordinate.0 - 1, coordinate.1));

            if !has_neighbor_left {
                edges.insert((*coordinate, EdgeOrientation::Left));
            }

            let has_neighbor_right = self.coordinates.contains(&(coordinate.0 + 1, coordinate.1));

            if !has_neighbor_right {
                edges.insert((*coordinate, EdgeOrientation::Right));
            }
        }

        let mut edge_regions: Vec<HashSet<(Coordinate, EdgeOrientation)>> = vec![];

        for edge in &edges {
            let mut is_in_another_region = false;
            for region in &edge_regions {
                if region.contains(edge) {
                    is_in_another_region = true;
                    break;
                }
            }

            if is_in_another_region {
                continue;
            }

            let region = bfs_edges(edge.0, edge.1, &edges);
            edge_regions.push(region);
        }

        edge_regions.len()
    }

    pub fn contains(&self, other_coordinate: &Coordinate) -> bool {
        self.coordinates.contains(other_coordinate)
    }

    #[allow(dead_code)]
    pub fn intersection(&self, other: &Region) -> HashSet<Coordinate> {
        if self.plot != other.plot {
            return HashSet::new();
        }

        self.coordinates
            .intersection(&other.coordinates)
            .copied()
            .collect()
    }
}

fn bfs_edges(
    start_coordinate: Coordinate,
    orientation: EdgeOrientation,
    edges: &HashSet<(Coordinate, EdgeOrientation)>,
) -> HashSet<(Coordinate, EdgeOrientation)> {
    let mut visited = HashSet::new();
    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start_coordinate, orientation));

    while let Some(current) = to_visit.pop_front() {
        if !seen.insert(current) {
            continue;
        }

        if !edges.contains(&current) {
            continue;
        }

        if orientation != current.1 {
            continue;
        }

        if visited.insert(current) {
            let potential_neighbor_coordinates = [
                (current.0 .0 + 1, current.0 .1),
                (current.0 .0 - 1, current.0 .1),
                (current.0 .0, current.0 .1 + 1),
                (current.0 .0, current.0 .1 - 1),
            ];

            for neighbor in potential_neighbor_coordinates {
                if !seen.contains(&(neighbor, orientation)) {
                    to_visit.push_back((neighbor, orientation));
                }
            }
        }
    }

    visited
}

fn parse_grid(input: &str) -> (isize, isize, Grid) {
    let mut grid = Grid::new();
    let mut x = 1;
    let mut y = 1;
    for line in input.lines() {
        x = 1;
        for c in line.chars() {
            grid.insert((x, y), c);
            x += 1;
        }
        y += 1;
    }
    (x - 1, y - 1, grid)
}
