use std::collections::{HashMap, HashSet, VecDeque};

use common::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-18.txt");

    let tiles = parse_tiles(input);

    let part_one = part_one(&tiles);
    let part_two = part_two(&tiles);

    Ok((part_one, part_two))
}

fn part_one(tiles: &HashMap<(usize, usize), TileType>) -> PartAnswer {
    todo!()
}

fn part_two(tiles: &HashMap<(usize, usize), TileType>) -> PartAnswer {
    todo!()
}

fn parse_tiles(s: &str) -> HashMap<(usize, usize), TileType> {
    let mut result = HashMap::new();

    let mut x = 0;
    let mut y = 0;

    for c in s.chars() {
        if c == '\n' {
            x = 0;
            y += 1;
        } else {
            let tile_type = match c {
                '#' => TileType::Wall,
                '.' => TileType::Open,
                '@' => TileType::Start,
                'a'..='z' => TileType::Key(c),
                'A'..='Z' => TileType::Door(c),
                _ => unreachable!("unsupported character {}", c),
            };

            result.insert((x, y), tile_type);

            x += 1;
        }
    }

    result
}

struct Tunnels {
    map: HashMap<Coordinate, TileType>,
    keys: HashSet<Coordinate>,
    doors: HashSet<Coordinate>,
    start: Coordinate,
    costs: HashMap<Coordinate, Costs>,
}

impl Tunnels {
    fn new(map: HashMap<(usize, usize), TileType>) -> Tunnels {
        let keys = map
            .iter()
            .filter_map(|(c, tile_type)| if tile_type.is_key() { Some(c) } else { None })
            .copied()
            .collect();

        let doors = map
            .iter()
            .filter_map(|(c, tile_type)| if tile_type.is_door() { Some(c) } else { None })
            .copied()
            .collect();

        let start = map
            .iter()
            .filter_map(|(c, tile_type)| if tile_type.is_start() { Some(c) } else { None })
            .next()
            .copied()
            .unwrap();

        let mut all_significant_coordinates = HashSet::new();
        all_significant_coordinates.insert(start);
        all_significant_coordinates.extend(&doors);
        all_significant_coordinates.extend(&keys);

        let costs = calculate_costs(all_significant_coordinates, &map);

        Tunnels {
            map,
            keys,
            doors,
            start,
            costs,
        }
    }
}

fn calculate_costs<I>(
    coordinates: I,
    map: &HashMap<Coordinate, TileType>,
) -> HashMap<Coordinate, Costs>
where
    I: IntoIterator<Item = Coordinate>,
{
    let mut costs = HashMap::new();

    for coordinate in coordinates {
        todo!()
    }

    costs
}

// change this to dfs, so that we know the exact path that we've taken so far
fn bfs(coordinate: &Coordinate, map: &HashMap<Coordinate, TileType>) -> Costs {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    let mut costs = HashMap::new();

    queue.push_back((*coordinate, 0));

    while let Some((to_check, cost)) = queue.pop_front() {
        // if we've already visited, move to the next node
        if !seen.insert(to_check) {
            continue;
        }

        if let Some(tile_type) = map.get(&to_check) {
            if tile_type.is_significant() {
                costs.insert(coordinate, cost);
            }

            todo!()
        }
    }

    costs
}

fn neighbors(coordinate: &Coordinate, map: &HashMap<Coordinate, TileType>) -> Vec<Coordinate> {
    let (x, y) = coordinate;

    let mut neighbors = Vec::with_capacity(4);

    if *x > 0 {
        if let Some(tile_type) = map.get(&(x - 1, *y)) {
            if tile_type.
        }
    }

    todo!()
}

type Coordinate = (usize, usize);
type Costs = HashMap<(usize, usize), usize>;

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
    tile_type: TileType,
}

#[derive(Debug, Copy, Clone)]
enum TileType {
    Wall,
    Open,
    Start,
    Key(char),
    Door(char),
}

impl TileType {
    fn is_key(&self) -> bool {
        match self {
            TileType::Key(_) => true,
            _ => false,
        }
    }

    fn is_door(&self) -> bool {
        match self {
            TileType::Door(_) => true,
            _ => false,
        }
    }

    fn is_start(&self) -> bool {
        match self {
            TileType::Start => true,
            _ => false,
        }
    }

    fn is_significant(&self) -> bool {
        self.is_key() || self.is_door() || self.is_start()
    }

    fn is_wall(&self) -> bool {
        match self {
            TileType::Wall => true,
            _ => false
        }
    }
}

impl ToString for TileType {
    fn to_string(&self) -> String {
        match self {
            TileType::Wall => "#".to_string(),
            TileType::Open => " ".to_string(),
            TileType::Start => "@".to_string(),
            TileType::Key(c) => c.to_string(),
            TileType::Door(c) => c.to_string(),
        }
    }
}
