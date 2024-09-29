use std::fmt;
use std::{
    collections::{hash_map::Values, HashMap, HashSet},
    fmt::{Display, Formatter},
};
use std::time::{Duration, SystemTime};
use crate::common::answer::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::delimited,
    sequence::tuple,
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-20.txt");
    let parse_start = SystemTime::now();
    let tiles = parse_tiles(input);

    let parse_duration = parse_start.elapsed().unwrap();

    let part_one = part_one(&tiles, parse_duration);
    let part_two = part_two(&tiles, parse_duration);

    Ok((part_one, part_two))
}

fn part_one(tiles: &Tiles, parse_duration: Duration) -> PartAnswer {
    let start = SystemTime::now();

    let corners = find_corners(tiles);

    let product = corners.0 * corners.1 * corners.2 * corners.3;

    let elapsed = start.elapsed().unwrap();

    (product as u64, elapsed + parse_duration).into()
}

fn part_two(tiles: &Tiles, parse_duration: Duration) -> PartAnswer {
    let start = SystemTime::now();

    let corners = find_corners(tiles);

    let top_left = find_top_left(corners, tiles);

    let tile_layout = get_tile_layout(top_left, tiles);

    let image_string = build_image_string(&tile_layout, tiles);

    let image_tile = pixels(&image_string).unwrap().1;

    let mut image_tile = Tile::new(0, image_tile);

    let monster: HashMap<(usize, usize), ()> = vec![
        ((0, 18), ()),
        ((1, 0), ()),
        ((1, 5), ()),
        ((1, 6), ()),
        ((1, 11), ()),
        ((1, 12), ()),
        ((1, 17), ()),
        ((1, 18), ()),
        ((1, 19), ()),
        ((2, 1), ()),
        ((2, 4), ()),
        ((2, 7), ()),
        ((2, 10), ()),
        ((2, 13), ()),
        ((2, 16), ()),
    ]
    .into_iter()
    .collect();

    let monster_height = 3;
    let monster_width = 20;

    let mut monster_encounters = HashSet::new();
    let mut number_of_monsters_seen = 0;

    let mut seen_coordinates = HashSet::new();

    for i in 0..8 {
        if i % 4 == 0 {
            image_tile.flip();
        }

        image_tile.rotate();

        for delta_x in 0..image_tile.length - monster_height {
            for delta_y in 0..image_tile.length - monster_width {
                let adjusted_monster: HashSet<(usize, usize)> = monster
                    .keys()
                    .map(|(x, y)| (*x + delta_x, *y + delta_y))
                    .collect();

                let mut monster_hits = HashSet::new();

                for coordinates in adjusted_monster {
                    seen_coordinates.insert(coordinates);
                    let pixel = image_tile
                        .pixels
                        .get(&coordinates)
                        .map(|s| s.as_str())
                        .unwrap_or("");

                    if pixel == "#" {
                        monster_hits.insert(coordinates);
                    }
                }

                if monster_hits.len() == monster.len() {
                    number_of_monsters_seen += 1;
                    monster_encounters.extend(monster_hits);
                }
            }
        }

        if number_of_monsters_seen > 0 {
            break;
        }
    }

    let mut rocks: u64 = 0;

    let mut image_tile_with_monster = image_tile.clone();

    for coordinates in &monster_encounters {
        image_tile_with_monster
            .pixels
            .insert(*coordinates, "O".into());
    }

    for (coordinates, pixel) in image_tile.pixels {
        if pixel == "." {
            continue;
        }

        if !monster_encounters.contains(&coordinates) {
            rocks += 1;
        }
    }

    let elapsed = start.elapsed().unwrap();

    (rocks, elapsed + parse_duration).into()
}

#[allow(dead_code)]
fn print_layout(tile_layout: &HashMap<(usize, usize), Tile>, tiles: &Tiles) {
    println!();

    for image_row in 0..tiles.image_width {
        for tile_row_index in 0..tiles.tile_width {
            let mut tile_scanline: Vec<&str> = Vec::new();

            for image_column in 0..tiles.image_width {
                let tile = &tile_layout[&(image_row, image_column)];

                for tile_column_index in 0..tiles.tile_width {
                    tile_scanline.push(&tile.pixels[&(tile_row_index, tile_column_index)]);
                }

                tile_scanline.push(" ");
            }

            println!("{}", tile_scanline.join(""));
        }

        println!();
    }
}

#[allow(dead_code)]
fn print_layout_without_borders(tile_layout: &HashMap<(usize, usize), Tile>, tiles: &Tiles) {
    println!();

    for image_row in 0..tiles.image_width {
        for tile_row_index in 1..tiles.tile_width - 1 {
            let mut tile_scanline: Vec<&str> = Vec::new();

            for image_column in 0..tiles.image_width {
                let tile = &tile_layout[&(image_row, image_column)];

                for tile_column_index in 1..tiles.tile_width - 1 {
                    tile_scanline.push(&tile.pixels[&(tile_row_index, tile_column_index)]);
                }

                tile_scanline.push(" ");
            }

            println!("{}", tile_scanline.join(""));
        }

        println!();
    }
}

#[allow(dead_code)]
fn print_layout_without_borders_and_gaps(
    tile_layout: &HashMap<(usize, usize), Tile>,
    tiles: &Tiles,
) {
    println!();

    for image_row in 0..tiles.image_width {
        for tile_row_index in 1..tiles.tile_width - 1 {
            let mut tile_scanline: Vec<&str> = Vec::new();

            for image_column in 0..tiles.image_width {
                let tile = &tile_layout[&(image_row, image_column)];

                for tile_column_index in 1..tiles.tile_width - 1 {
                    tile_scanline.push(&tile.pixels[&(tile_row_index, tile_column_index)]);
                }
            }

            println!("{}", tile_scanline.join(""));
        }
    }

    println!();
}

fn build_image_string(tile_layout: &HashMap<(usize, usize), Tile>, tiles: &Tiles) -> String {
    let mut rows = Vec::new();

    for image_row in 0..tiles.image_width {
        for tile_row_index in 1..tiles.tile_width - 1 {
            let mut tile_scanline: Vec<&str> = Vec::new();

            for image_column in 0..tiles.image_width {
                let tile = &tile_layout[&(image_row, image_column)];

                for tile_column_index in 1..tiles.tile_width - 1 {
                    tile_scanline.push(&tile.pixels[&(tile_row_index, tile_column_index)]);
                }
            }

            rows.push(tile_scanline.join(""));
        }
    }

    rows.join("\n")
}

fn get_tile_layout(top_left: &Tile, tiles: &Tiles) -> HashMap<(usize, usize), Tile> {
    let mut placed_tiles = HashSet::new();
    let mut tile_set = HashMap::new();

    let mut row: usize = 0;
    let mut column: usize = 0;

    while tile_set.len() != tiles.len() {
        let next_tile = if row == 0 && column == 0 {
            top_left.clone()
        } else {
            let previous_tile: &Tile = if column == 0 {
                &tile_set[&(row - 1, 0)]
            } else {
                &tile_set[&(row, column - 1)]
            };

            let previous_borders = previous_tile.get_borders();
            let border_to_find = if column == 0 {
                previous_borders.bottom
            } else {
                previous_borders.right
            };

            let next_tile_id =
                get_other_tile_with_border(&previous_tile.id, &border_to_find, tiles);
            if next_tile_id == None {
                panic!(
                    "Could not get next tile for tile {} (row {}, column {}, border {})",
                    previous_tile.id, row, column, border_to_find
                );
            }

            let next_tile_id = next_tile_id.unwrap();

            let mut next_tile = tiles.get(&next_tile_id).clone();

            for _ in 0..4 {
                let borders = next_tile.get_borders();
                let border_to_check = if column == 0 {
                    borders.top
                } else {
                    borders.left
                };

                if border_to_check == border_to_find {
                    break;
                }

                next_tile.rotate();
            }

            let borders = next_tile.get_borders();

            let border_to_check = if column == 0 {
                borders.top
            } else {
                borders.left
            };

            if border_to_check != border_to_find {
                next_tile.flip();
            }

            for _ in 0..4 {
                let borders = next_tile.get_borders();
                let border_to_check = if column == 0 {
                    borders.top
                } else {
                    borders.left
                };

                if border_to_check == border_to_find {
                    break;
                }

                next_tile.rotate();
            }

            next_tile
        };

        placed_tiles.insert(next_tile.id);
        tile_set.insert((row, column), next_tile.clone());

        column += 1;

        if column == tiles.image_width {
            column = 0;
            row += 1;
        }
    }

    tile_set
}

fn get_other_tile_with_border(this_tile_id: &usize, border: &str, tiles: &Tiles) -> Option<usize> {
    let forward = tiles.get_tile_ids_with_matching_border(border);
    let reverse = tiles.get_tile_ids_with_matching_border(&reverse(border));

    let all_ids = forward.union(reverse);

    all_ids
        .into_iter()
        .filter(|n| *n != this_tile_id)
        .copied()
        .next()
}

fn find_corners(tiles: &Tiles) -> (usize, usize, usize, usize) {
    let mut output = Vec::with_capacity(4);

    for first in tiles.values() {
        let mut overlapping_borders = 0;

        let borders = first.get_borders();
        let borders = vec![borders.top, borders.left, borders.bottom, borders.right];

        for border in borders {
            if tiles.get_tile_ids_with_matching_border(&border).len() > 1 {
                overlapping_borders += 1;
            }
        }

        if overlapping_borders == 2 {
            output.push(first.id);
        }
    }

    assert!(
        output.len() == 4,
        "Expected 4 corners, found {}",
        output.len()
    );

    (output[0], output[1], output[2], output[3])
}

fn get_common_border_fast(first: &Tile, second: &Tile) -> Option<String> {
    let first_borders = first.get_borders();
    let second_borders = second.get_borders();

    let first_borders_list = vec![
        first_borders.top,
        first_borders.left,
        first_borders.bottom,
        first_borders.right,
    ];
    let second_borders_list = vec![
        second_borders.top,
        second_borders.left,
        second_borders.bottom,
        second_borders.right,
    ];

    for first_border in &first_borders_list {
        for second_border in &second_borders_list {
            if first_border == second_border {
                return Some(first_border.clone());
            }
            let second_border = &reverse(second_border);

            if first_border == second_border {
                return Some(first_border.clone());
            }
        }
    }

    None
}

fn find_top_left(corner_ids: (usize, usize, usize, usize), tiles: &Tiles) -> &Tile {
    let (first, second, third, fourth) = corner_ids;

    let corner_tiles = vec![
        tiles.get(&first),
        tiles.get(&second),
        tiles.get(&third),
        tiles.get(&fourth),
    ];

    for tile in corner_tiles {
        let borders = tile.get_borders();

        let mut has_top_or_left = false;

        for other_tile in tiles.values() {
            if tile.id == other_tile.id {
                continue;
            }

            let common_border = get_common_border_fast(tile, other_tile);

            if common_border.is_none() {
                continue;
            }

            let common_border = common_border.unwrap();

            if common_border == borders.top || common_border == borders.left {
                has_top_or_left = true;
            }
        }

        if !has_top_or_left {
            return tile;
        }
    }

    panic!()
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

#[derive(Debug, PartialEq)]
struct Image {
    pixels: HashMap<(usize, usize), String>,
    width: usize,
    height: usize,
}

impl From<HashMap<(usize, usize), String>> for Image {
    fn from(pixels: HashMap<(usize, usize), String>) -> Image {
        let mut height = 0;
        let mut width = 0;

        for (row, column) in pixels.keys() {
            height = height.max(*row);
            width = width.max(*column);
        }

        Image {
            pixels,
            width,
            height,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Borders {
    top: String,
    bottom: String,
    left: String,
    right: String,
}

#[derive(Debug, PartialEq, Clone)]
struct Tile {
    id: usize,
    pixels: HashMap<(usize, usize), String>,
    length: usize,
}

impl Tile {
    fn new(id: usize, pixels: HashMap<(usize, usize), String>) -> Self {
        let max_row = pixels.keys().map(|(row, _)| row).max().unwrap();
        let max_column = pixels.keys().map(|(_, column)| column).max().unwrap();

        assert!(
            max_row == max_column,
            "max row = {}, max column = {}",
            max_row,
            max_column
        );

        let length = max_row + 1;

        Self { id, pixels, length }
    }

    fn get_borders(&self) -> Borders {
        let mut top = Vec::with_capacity(self.length);
        for y in 0..self.length {
            top.push(self.pixels[&(0, y)].clone());
        }
        let top = top.join("");

        let mut bottom = Vec::with_capacity(self.length);
        for y in 0..self.length {
            bottom.push(self.pixels[&(self.length - 1, y)].clone());
        }
        let bottom = bottom.join("");

        let mut left = Vec::with_capacity(self.length);
        for x in 0..self.length {
            left.push(self.pixels[&(x, 0)].clone());
        }
        let left = left.join("");

        let mut right = Vec::with_capacity(self.length);
        for x in 0..self.length {
            right.push(self.pixels[&(x, self.length - 1)].clone());
        }
        let right = right.join("");

        Borders {
            top,
            bottom,
            left,
            right,
        }
    }

    fn rotate(&mut self) {
        self.pixels = rotate(&self.pixels, self.length, self.length);
    }

    fn flip(&mut self) {
        self.pixels = flip(&self.pixels, self.length, self.length);
    }
}

fn rotate<V: Clone>(
    map: &HashMap<(usize, usize), V>,
    width: usize,
    height: usize,
) -> HashMap<(usize, usize), V> {
    let mut output = HashMap::new();

    for row in 0..width {
        for column in 0..height {
            let pixel_type = map.get(&(row, column));

            if pixel_type.is_none() {
                continue;
            }

            let pixel_type = pixel_type.unwrap().clone();

            let new_coordinates = (height - column - 1, row);

            output.insert(new_coordinates, pixel_type);
        }
    }

    output
}

fn flip<V: Clone>(
    map: &HashMap<(usize, usize), V>,
    width: usize,
    height: usize,
) -> HashMap<(usize, usize), V> {
    let mut output = HashMap::new();

    for row in 0..width {
        for column in 0..height {
            let pixel_type = map.get(&(row, column));
            if pixel_type.is_none() {
                continue;
            }

            let pixel_type = pixel_type.unwrap().clone();

            let new_coordinates = (row, height - column - 1);

            output.insert(new_coordinates, pixel_type);
        }
    }

    output
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut rows: Vec<String> = Vec::with_capacity(self.length);

        for row in 0..self.length {
            let mut current = Vec::with_capacity(self.length);
            for column in 0..self.length {
                let pixel_type = self.pixels[&(row, column)].clone();
                current.push(pixel_type);
            }

            rows.push(current.join(""));
        }

        write!(f, "{}", rows.join("\n"))
    }
}

#[derive(Debug, PartialEq)]
struct Tiles {
    tiles_by_id: HashMap<usize, Tile>,
    tile_ids_by_border: HashMap<String, HashSet<usize>>,
    image_width: usize, // the number of tiles on one side of the image
    tile_width: usize,  // the number of characters on one side of the tile
}

impl Tiles {
    fn get(&self, id: &usize) -> &Tile {
        self.tiles_by_id.get(id).unwrap()
    }

    fn values(&self) -> Values<'_, usize, Tile> {
        self.tiles_by_id.values()
    }

    fn len(&self) -> usize {
        self.tiles_by_id.len()
    }

    fn get_tile_ids_with_matching_border(&self, border: &str) -> &HashSet<usize> {
        self.tile_ids_by_border.get(border).unwrap()
    }
}

impl From<Vec<Tile>> for Tiles {
    fn from(tile_list: Vec<Tile>) -> Self {
        let tile_width = tile_list[0].length;

        let mut tiles_by_id = HashMap::new();

        for tile in &tile_list {
            tiles_by_id.insert(tile.id, tile.clone());
        }

        let mut tile_ids_by_border = HashMap::new();

        for tile in &tile_list {
            let borders = tile.get_borders();
            let borders = vec![borders.top, borders.left, borders.bottom, borders.right];

            for border in borders {
                if !tile_ids_by_border.contains_key(&border) {
                    tile_ids_by_border.insert(border.clone(), HashSet::new());
                }
                tile_ids_by_border.get_mut(&border).unwrap().insert(tile.id);

                let border = reverse(&border);

                if !tile_ids_by_border.contains_key(&border) {
                    tile_ids_by_border.insert(border.clone(), HashSet::new());
                }
                tile_ids_by_border.get_mut(&border).unwrap().insert(tile.id);
            }
        }

        let image_width = (tiles_by_id.len() as f64).sqrt() as usize;

        assert!(image_width * image_width == tiles_by_id.len());

        Self {
            tiles_by_id,
            tile_ids_by_border,
            image_width,
            tile_width,
        }
    }
}

fn parse_tiles(input: &str) -> Tiles {
    tiles(input).map(|(_, tiles)| tiles).unwrap().into()
}

fn tiles(i: &str) -> IResult<&str, Vec<Tile>> {
    separated_list1(tag("\n\n"), tile)(i)
}

fn tile(i: &str) -> IResult<&str, Tile> {
    let tile_header = delimited(tag("Tile "), tile_id, tag(":\n"));

    let tile = tuple((tile_header, pixels));

    map(tile, |(id, pixels)| Tile::new(id, pixels))(i)
}

fn tile_id(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse())(i)
}

fn pixels(i: &str) -> IResult<&str, HashMap<(usize, usize), String>> {
    let tile_rows = separated_list1(tag("\n"), tile_row);

    map(tile_rows, |rows| {
        let mut pixels = HashMap::new();

        for (row_index, row) in rows.into_iter().enumerate() {
            for (column_index, pixel) in row.into_iter().enumerate() {
                let coordinates = (row_index, column_index);

                pixels.insert(coordinates, pixel);
            }
        }

        pixels
    })(i)
}

fn tile_row(i: &str) -> IResult<&str, Vec<String>> {
    many1(pixel)(i)
}

fn pixel(i: &str) -> IResult<&str, String> {
    map(alt((tag("#"), tag("."))), |s: &str| s.into())(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel() {
        assert_eq!(pixel("#"), Ok(("", "#".into())));
        assert_eq!(pixel("."), Ok(("", ".".into())));
    }

    #[test]
    fn test_tile_row() {
        assert_eq!(
            tile_row("#.#"),
            Ok(("", vec!["#".into(), ".".into(), "#".into()]))
        );
    }

    #[test]
    fn test_tile() {
        let expected_pixels = vec![
            ((0, 0), ".".into()),
            ((0, 1), ".".into()),
            ((0, 2), "#".into()),
            ((1, 0), "#".into()),
            ((1, 1), "#".into()),
            ((1, 2), ".".into()),
            ((2, 0), ".".into()),
            ((2, 1), "#".into()),
            ((2, 2), ".".into()),
        ]
        .into_iter()
        .collect();

        assert_eq!(
            tile("Tile 2311:\n..#\n##.\n.#."),
            Ok(("", Tile::new(2311, expected_pixels)))
        );
    }

    #[test]
    fn test_tile_display() {
        let tile = tile("Tile 2311:\n..##.#..#.\n##..#.....\n#...##..#.\n####.#...#\n##.##.###.\n##...#.###\n.#.#.#..##\n..#....#..\n###...#.#.\n..###..###").unwrap().1;

        let display = format!("{}", tile);

        assert_eq!(display, "..##.#..#.\n##..#.....\n#...##..#.\n####.#...#\n##.##.###.\n##...#.###\n.#.#.#..##\n..#....#..\n###...#.#.\n..###..###");
    }

    #[test]
    fn test_tile_rotate() {
        let pixels = vec![
            ((0, 0), "#".into()),
            ((0, 1), ".".into()),
            ((1, 0), ".".into()),
            ((1, 1), ".".into()),
        ]
        .into_iter()
        .collect();

        let mut tile = Tile::new(0, pixels);

        tile.rotate();

        let expected_pixels = vec![
            ((0, 0), ".".into()),
            ((0, 1), ".".into()),
            ((1, 0), "#".into()),
            ((1, 1), ".".into()),
        ]
        .into_iter()
        .collect();

        assert_eq!(tile.pixels, expected_pixels);
    }

    #[test]
    fn test_tile_flip() {
        let pixels = vec![
            ((0, 0), "#".into()),
            ((0, 1), ".".into()),
            ((1, 0), ".".into()),
            ((1, 1), "#".into()),
        ]
        .into_iter()
        .collect();

        let mut tile = Tile::new(0, pixels);

        tile.flip();

        let expected_pixels = vec![
            ((0, 0), ".".into()),
            ((0, 1), "#".into()),
            ((1, 0), "#".into()),
            ((1, 1), ".".into()),
        ]
        .into_iter()
        .collect();

        assert_eq!(tile.pixels, expected_pixels);
    }

    #[test]
    fn test_has_common_borders() {
        let first = vec![
            ((0, 0), "#".into()),
            ((0, 1), "#".into()),
            ((1, 0), ".".into()),
            ((1, 1), ".".into()),
        ]
        .into_iter()
        .collect();
        let first = Tile::new(1, first);

        let second = vec![
            ((0, 0), "#".into()),
            ((0, 1), "#".into()),
            ((1, 0), "#".into()),
            ((1, 1), "#".into()),
        ]
        .into_iter()
        .collect();
        let second = Tile::new(2, second);

        assert_eq!(get_common_border_fast(&first, &second), Some("##".into()));
    }

    #[test]
    fn test_rotate() {
        let map = vec![
            ((0, 0), 1),
            ((1, 0), 2),
            ((2, 0), 3),
            ((0, 1), 4),
            ((1, 1), 5),
            ((2, 1), 6),
        ]
        .into_iter()
        .collect();
        let map = rotate(&map, 3, 2);

        let expected = vec![
            ((0, 0), 4),
            ((0, 1), 5),
            ((0, 2), 6),
            ((1, 0), 1),
            ((1, 1), 2),
            ((1, 2), 3),
        ]
        .into_iter()
        .collect();

        assert_eq!(map, expected);
    }

    #[test]
    fn test_flip() {
        let map = vec![
            ((0, 0), 1),
            ((1, 0), 2),
            ((2, 0), 3),
            ((0, 1), 4),
            ((1, 1), 5),
            ((2, 1), 6),
        ]
        .into_iter()
        .collect();
        let map = flip(&map, 3, 2);

        let expected = vec![
            ((0, 0), 4),
            ((1, 0), 5),
            ((2, 0), 6),
            ((0, 1), 1),
            ((1, 1), 2),
            ((2, 1), 3),
        ]
        .into_iter()
        .collect();

        assert_eq!(map, expected);
    }
}
