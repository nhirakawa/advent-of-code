use std::collections::{HashMap, HashSet};

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
    map: HashMap<(usize, usize), TileType>,
    keys: HashSet<(usize, usize)>,
    doors: HashSet<(usize, usize)>,
    start: (usize, usize),
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

        Tunnels {
            map,
            keys,
            doors,
            start,
        }
    }
}

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
