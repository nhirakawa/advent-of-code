use crate::common::parse::griderator;
use anyhow::bail;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let tiles = parse_tiles(input, Robots::One)?;

    if tiles.keys.is_empty() {
        bail!("No keys");
    }

    let key_graph = build_key_graph(&tiles)?;
    let target = all_keys(&tiles)?;

    dijkstra(&tiles, &key_graph, target)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let tiles = parse_tiles(input, Robots::Four)?;

    if tiles.keys.is_empty() {
        bail!("No keys");
    }

    let key_graph = build_key_graph(&tiles)?;
    let target = all_keys(&tiles)?;

    dijkstra(&tiles, &key_graph, target)
}

fn all_keys(tiles: &Tiles) -> anyhow::Result<Keychain> {
    tiles
        .keys
        .keys()
        .try_fold(Keychain::default(), |acc, &c| acc.or(c))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QueueState {
    distance: usize,
    positions: Vec<Position>,
    keys: Keychain,
}

impl PartialOrd for QueueState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

fn dijkstra(tiles: &Tiles, key_graph: &KeyGraph, target: Keychain) -> anyhow::Result<usize> {
    let mut heap = BinaryHeap::new();
    let mut best: HashMap<(Vec<Position>, Keychain), usize> = HashMap::new();

    let start = tiles.start.clone();
    best.insert((start.clone(), Keychain::default()), 0);
    heap.push(QueueState {
        distance: 0,
        positions: start,
        keys: Keychain::default(),
    });

    while let Some(QueueState {
        distance,
        positions,
        keys,
    }) = heap.pop()
    {
        if keys == target {
            return Ok(distance);
        }

        if best
            .get(&(positions.clone(), keys))
            .is_some_and(|&best_dist| distance > best_dist)
        {
            continue;
        }

        for (robot, &position) in positions.iter().enumerate() {
            let Some(edges) = key_graph.graph.get(&position) else {
                continue;
            };

            for &(next_position, edge_distance, required) in edges {
                if !keys.contains_all(required) {
                    continue;
                }

                let Some(Tile::Key(key_char)) = tiles.tiles.get(&next_position).copied() else {
                    bail!("Expected key tile at {next_position:?}");
                };

                if keys.contains(key_char) {
                    continue;
                }

                let new_keys = keys.or(key_char)?;
                let new_distance = distance + edge_distance;
                let mut new_positions = positions.clone();
                new_positions[robot] = next_position;
                let state_key = (new_positions.clone(), new_keys);

                if best
                    .get(&state_key)
                    .is_none_or(|&best_dist| new_distance < best_dist)
                {
                    best.insert(state_key, new_distance);
                    heap.push(QueueState {
                        distance: new_distance,
                        positions: new_positions,
                        keys: new_keys,
                    });
                }
            }
        }
    }

    bail!("No solution found")
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Keychain(u32);

impl Keychain {
    fn or(&self, c: char) -> anyhow::Result<Self> {
        if c.is_ascii_lowercase() {
            let shift = c as u8 - b'a';
            Ok(Self(self.0 | 1 << shift))
        } else {
            bail!("Invalid char '{c}'")
        }
    }

    fn contains_all(&self, required: Keychain) -> bool {
        self.0 & required.0 == required.0
    }

    fn contains(&self, c: char) -> bool {
        c.is_ascii_lowercase() && self.0 & (1 << (c as u8 - b'a')) != 0
    }
}

impl TryFrom<HashSet<char>> for Keychain {
    type Error = anyhow::Error;

    fn try_from(keys: HashSet<char>) -> Result<Self, Self::Error> {
        let mut keychain = 0;

        for key in keys {
            if key.is_ascii_lowercase() {
                let shift = key as u8 - b'a';
                keychain |= 1 << shift;
            } else {
                bail!("Invalid key '{key}'");
            }
        }

        Ok(Keychain(keychain))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position((isize, isize));

impl Position {
    fn next(&self) -> [Position; 4] {
        let (x, y) = self.0;
        [
            (x + 1, y).into(),
            (x - 1, y).into(),
            (x, y + 1).into(),
            (x, y - 1).into(),
        ]
    }

    fn up_left(&self) -> Self {
        let (x, y) = self.0;
        (x - 1, y - 1).into()
    }

    fn up_right(&self) -> Self {
        let (x, y) = self.0;
        (x + 1, y - 1).into()
    }

    fn down_right(&self) -> Self {
        let (x, y) = self.0;
        (x + 1, y + 1).into()
    }

    fn down_left(&self) -> Self {
        let (x, y) = self.0;
        (x - 1, y + 1).into()
    }
}

impl From<(isize, isize)> for Position {
    fn from(value: (isize, isize)) -> Self {
        Self(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Space,
    Key(char),
    Door(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tiles {
    start: Vec<Position>,
    tiles: HashMap<Position, Tile>,
    keys: HashMap<char, Position>,
}

#[derive(Debug, Copy, Clone)]
enum Robots {
    One,
    Four,
}

fn parse_tiles(s: &str, robots: Robots) -> anyhow::Result<Tiles> {
    let mut center: Option<Position> = None;
    let mut tiles = HashMap::new();
    let mut keys = HashMap::new();

    for (position, c) in griderator(s) {
        let position = position.into();

        let tile = match c {
            '#' => Tile::Wall,
            '.' | '@' => Tile::Space,
            'a'..='z' => Tile::Key(c),
            'A'..='Z' => Tile::Door(c),
            _ => bail!("Invalid tile: '{c}'"),
        };

        tiles.insert(position, tile);

        if let Tile::Key(key) = tile {
            keys.insert(key, position);
        }

        if c == '@' {
            if center.is_some() {
                bail!("Duplicate start found");
            }
            center = Some(position);
        }
    }

    let Some(center) = center else {
        bail!("No start found");
    };

    let start = match robots {
        Robots::One => vec![center],
        Robots::Four => {
            for wall in std::iter::once(center).chain(center.next()) {
                tiles.insert(wall, Tile::Wall);
            }

            vec![
                center.up_left(),
                center.up_right(),
                center.down_right(),
                center.down_left(),
            ]
        }
    };

    Ok(Tiles { start, tiles, keys })
}

struct KeyGraph {
    /// A graph from a position (either a start or a key) to all other keys in the map (position of key, distance to key, prerequisite keys)
    graph: HashMap<Position, Vec<(Position, usize, Keychain)>>,
}

fn build_key_graph(tiles: &Tiles) -> anyhow::Result<KeyGraph> {
    let mut graph = HashMap::new();

    for start in &tiles.start {
        let adjacent = bfs(*start, tiles)?;
        graph.insert(*start, adjacent);
    }

    for key_position in tiles.keys.values() {
        let adjacent = bfs(*key_position, tiles)?;
        graph.insert(*key_position, adjacent);
    }

    Ok(KeyGraph { graph })
}

fn bfs(start: Position, tiles: &Tiles) -> anyhow::Result<Vec<(Position, usize, Keychain)>> {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0, Keychain::default()));

    let mut seen = HashSet::new();

    let mut adjacent = Vec::new();

    while let Some((current, distance, keys)) = queue.pop_front() {
        if !seen.insert(current) {
            continue;
        }

        for next in current.next() {
            if let Some(tile) = tiles.tiles.get(&next) {
                match tile {
                    Tile::Wall => {
                        continue;
                    }
                    Tile::Space => {
                        queue.push_back((next, distance + 1, keys));
                    }
                    Tile::Key(_key) => {
                        adjacent.push((next, distance + 1, keys));
                        queue.push_back((next, distance + 1, keys));
                    }
                    Tile::Door(door) => {
                        queue.push_back((next, distance + 1, keys.or(door.to_ascii_lowercase())?));
                    }
                }
            }
        }
    }

    Ok(adjacent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = "#########\n#b.A.@.a#\n#########";
        assert_eq!(part_one(input).unwrap().to_string(), "8");
    }

    #[test]
    fn test_keychain_from_str() {
        let keychain =
            Keychain::try_from(['a', 's', 'd', 'f'].into_iter().collect::<HashSet<_>>()).unwrap();
        assert_eq!(keychain.0, 1 << 0 | 1 << 3 | 1 << 5 | 1 << 18);
    }

    #[test]
    fn test_parse_tiles() {
        let tiles = parse_tiles("#########\n#b.A.@.a#\n#########", Robots::One).unwrap();

        assert_eq!(tiles.start, vec![(5, 1).into()]);
        assert_eq!(tiles.keys.get(&'a').copied().unwrap(), (7, 1).into());
        assert_eq!(tiles.keys.get(&'b').copied().unwrap(), (1, 1).into());
    }

    #[test]
    fn test_bfs_finds_key() {
        let tiles = parse_tiles("#####\n#@.a#\n#####", Robots::One).unwrap();

        let adjacent = bfs(tiles.start[0], &tiles).unwrap();

        assert_eq!(adjacent, vec![((3, 1).into(), 2, Keychain::default())]);
    }

    #[test]
    fn test_bfs_finds_key_behind_door() {
        let tiles = parse_tiles("#########\n#b.A.@.a#\n#########", Robots::One).unwrap();

        let adjacent = bfs(tiles.start[0], &tiles).unwrap();

        let needs_a = Keychain::default().or('a').unwrap();

        assert_eq!(
            adjacent,
            vec![
                ((7, 1).into(), 2, Keychain::default()),
                ((1, 1).into(), 4, needs_a),
            ]
        );
    }
}
