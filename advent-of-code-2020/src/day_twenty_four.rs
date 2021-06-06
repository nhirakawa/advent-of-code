use common::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::value,
    multi::{many1, separated_list1},
    IResult,
};
use std::{collections::HashSet, ops::Add};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-24.txt");
    let parse_start = SystemTime::now();
    let tile_pointers = parse_tile_pointers(input);
    let parse_duration = parse_start.elapsed().unwrap();

    let part_one = part_one(&tile_pointers, parse_duration);
    let part_two = part_two(&tile_pointers, parse_duration);

    Ok((part_one, part_two))
}

fn part_one(tile_pointers: &Vec<TilePointer>, parse_duration: Duration) -> PartAnswer {
    let start = SystemTime::now();

    let tile_states = get_initial_state(tile_pointers);
    let number_of_black = tile_states.len();

    let elapsed = start.elapsed().unwrap();

    (number_of_black as u64, elapsed + parse_duration).into()
}

fn part_two(tile_pointers: &[TilePointer], parse_duration: Duration) -> PartAnswer {
    let start = SystemTime::now();

    let mut art = TileFloorArtExhibit::new(tile_pointers);

    for _ in 0..100 {
        art.another_day();
    }

    let elapsed = start.elapsed().unwrap();

    (art.len() as u64, elapsed + parse_duration).into()
}

fn get_initial_state(tile_pointers: &[TilePointer]) -> HashSet<CubeCoordinates> {
    let mut tile_states = HashSet::new();

    for pointer in tile_pointers {
        let mut current = CubeCoordinates::new(0, 0, 0);

        for direction in pointer {
            let offset = match direction {
                Direction::Northwest => (0, 1, -1),
                Direction::West => (-1, 1, 0),
                Direction::Southwest => (-1, 0, 1),
                Direction::Southeast => (0, -1, 1),
                Direction::East => (1, -1, 0),
                Direction::Northeast => (1, 0, -1),
            };

            let offset = offset.into();

            current = &current + &offset;
        }

        if tile_states.contains(&current) {
            tile_states.remove(&current);
        } else {
            tile_states.insert(current);
        }
    }

    tile_states
}

#[derive(Debug, PartialEq)]
struct TileFloorArtExhibit {
    black_tiles: HashSet<CubeCoordinates>,
}

impl TileFloorArtExhibit {
    fn new(tile_pointers: &[TilePointer]) -> TileFloorArtExhibit {
        let mut black_tiles = HashSet::new();

        for pointer in tile_pointers {
            let mut current = CubeCoordinates::new(0, 0, 0);

            for direction in pointer {
                let offset = match direction {
                    Direction::Northwest => (0, 1, -1),
                    Direction::West => (-1, 1, 0),
                    Direction::Southwest => (-1, 0, 1),
                    Direction::Southeast => (0, -1, 1),
                    Direction::East => (1, -1, 0),
                    Direction::Northeast => (1, 0, -1),
                };

                let offset = offset.into();

                current = &current + &offset;
            }

            if black_tiles.contains(&current) {
                black_tiles.remove(&current);
            } else {
                black_tiles.insert(current);
            }
        }

        Self { black_tiles }
    }

    fn len(&self) -> usize {
        self.black_tiles.len()
    }

    fn another_day(&mut self) {
        let tiles_to_consider: HashSet<CubeCoordinates> = self
            .black_tiles
            .iter()
            .map(|tile| {
                let mut me_and_adjacent = vec![];
                me_and_adjacent.push(*tile);
                me_and_adjacent.extend(tile.adjacent());

                me_and_adjacent
            })
            .flatten()
            .collect();

        let mut new_black_tiles = self.black_tiles.clone();

        for tile_to_consider in &tiles_to_consider {
            let adjacent = tile_to_consider.adjacent();

            let black_adjacent_tiles: HashSet<&CubeCoordinates> =
                adjacent.intersection(&self.black_tiles).collect();

            let black_adjacent_tiles = black_adjacent_tiles.len();

            if self.black_tiles.contains(&tile_to_consider) {
                // current tile is black
                if black_adjacent_tiles == 0 || black_adjacent_tiles > 2 {
                    new_black_tiles.remove(tile_to_consider);
                }
            } else {
                // current tile is white
                if black_adjacent_tiles == 2 {
                    new_black_tiles.insert(*tile_to_consider);
                }
            };
        }

        self.black_tiles = new_black_tiles;
    }
}

type TilePointer = Vec<Direction>;

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
struct CubeCoordinates {
    x: i64,
    y: i64,
    z: i64,
}

impl CubeCoordinates {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn adjacent(&self) -> HashSet<Self> {
        vec![
            (0, 1, -1).into(),
            (-1, 1, 0).into(),
            (-1, 0, 1).into(),
            (0, -1, 1).into(),
            (1, -1, 0).into(),
            (1, 0, -1).into(),
        ]
        .iter()
        .map(|other| self + other)
        .collect()
    }
}

impl From<(i64, i64, i64)> for CubeCoordinates {
    fn from(tuple: (i64, i64, i64)) -> CubeCoordinates {
        let (x, y, z) = tuple;

        CubeCoordinates { x, y, z }
    }
}

impl Add<&CubeCoordinates> for &CubeCoordinates {
    type Output = CubeCoordinates;

    fn add(self, other: &CubeCoordinates) -> Self::Output {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;

        Self::Output { x, y, z }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

fn parse_tile_pointers(input: &str) -> Vec<TilePointer> {
    tile_pointers(input).map(|(_, pointers)| pointers).unwrap()
}

fn tile_pointers(i: &str) -> IResult<&str, Vec<TilePointer>> {
    separated_list1(tag("\n"), tile_pointer)(i)
}

fn tile_pointer(i: &str) -> IResult<&str, TilePointer> {
    many1(direction)(i)
}

fn direction(i: &str) -> IResult<&str, Direction> {
    let east = value(Direction::East, tag("e"));
    let southeast = value(Direction::Southeast, tag("se"));
    let southwest = value(Direction::Southwest, tag("sw"));
    let west = value(Direction::West, tag("w"));
    let northwest = value(Direction::Northwest, tag("nw"));
    let northeast = value(Direction::Northeast, tag("ne"));

    alt((southeast, southwest, northeast, northwest, east, west))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_pointer() {
        assert_eq!(
            tile_pointer("nwwswee"),
            Ok((
                "",
                vec![
                    Direction::Northwest,
                    Direction::West,
                    Direction::Southwest,
                    Direction::East,
                    Direction::East
                ]
            ))
        );
    }

    #[test]
    fn test_another_day() {
        let tiles = parse_tile_pointers("sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew");
        let mut art = TileFloorArtExhibit::new(&tiles);

        art.another_day();
        assert_eq!(art.len(), 15);

        art.another_day();
        assert_eq!(art.len(), 12);

        art.another_day();
        assert_eq!(art.len(), 25);

        art.another_day();
        assert_eq!(art.len(), 14);
    }
}
