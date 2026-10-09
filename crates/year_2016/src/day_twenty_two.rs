use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

use anyhow::{Context, anyhow, bail};
use itertools::Itertools;

use common::{
    base::{Day, Year},
    debug,
};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let nodes = Nodes::from_str(input)?;

    let mut viable_pairs = 0;

    for node in &nodes {
        for other in &nodes {
            if node.x == other.x && node.y == other.y {
                continue;
            }

            if node.used == 0 {
                continue;
            }

            if other.available < node.used {
                continue;
            }

            viable_pairs += 1;
        }
    }

    Ok(viable_pairs)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let nodes = Nodes::from_str(input)?;
    let nodes = nodes.into_iter().copied().sorted().collect_vec();
    let nodes = Nodes(nodes);

    debug::write(Year::Year2016, Day::Day22, "grid.txt", format!("{nodes}"))?;

    let empty = nodes
        .into_iter()
        .find(|node| node.used == 0)
        .ok_or(anyhow!("No empty node"))?;
    let max_x = nodes
        .into_iter()
        .map(|node| node.x)
        .max()
        .ok_or(anyhow!("No nodes"))?;

    let open: HashSet<(u32, u32)> = nodes
        .into_iter()
        .filter(|node| node.used <= empty.size)
        .map(|node| (node.x, node.y))
        .collect();

    // The shuffle below circles the empty node through rows 0 and 1, so both must be free of walls
    for y in 0..=1 {
        for x in 0..=max_x {
            if !open.contains(&(x, y)) {
                bail!("Expected rows 0 and 1 to be open, but ({x}, {y}) is a wall");
            }
        }
    }

    // Move the empty node next to the goal data
    let to_goal = bfs(&open, (empty.x, empty.y), (max_x - 1, 0))?;

    // Swap the empty node into the goal's position, pulling the goal data left one
    let swap = 1;

    // Each remaining step left costs 4 moves to circle the empty node back in front of the goal, plus 1 swap
    let shuffle = 5 * (max_x - 1);

    Ok(to_goal + swap + shuffle)
}

fn bfs(open: &HashSet<(u32, u32)>, start: (u32, u32), target: (u32, u32)) -> anyhow::Result<u32> {
    let mut seen = HashSet::from([start]);
    let mut queue = VecDeque::from([(start, 0)]);

    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == target {
            return Ok(steps);
        }

        let neighbors = [
            x.checked_sub(1).map(|x| (x, y)),
            Some((x + 1, y)),
            y.checked_sub(1).map(|y| (x, y)),
            Some((x, y + 1)),
        ];

        for neighbor in neighbors.into_iter().flatten() {
            if open.contains(&neighbor) && seen.insert(neighbor) {
                queue.push_back((neighbor, steps + 1));
            }
        }
    }

    bail!("Could not reach {target:?}")
}

#[derive(Clone, PartialEq, Eq)]
struct Nodes(Vec<Node>);

impl FromStr for Nodes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = Vec::new();

        for line in s.lines() {
            if !line.starts_with("/dev/grid") {
                continue;
            }

            let node = Node::from_str(line).with_context(|| format!("Could not parse {line}"))?;
            nodes.push(node);
        }

        Ok(Nodes(nodes))
    }
}

impl Display for Nodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let empty = self
            .0
            .iter()
            .find(|node| node.used == 0)
            .ok_or(std::fmt::Error)?;

        let mut row = 0;

        for node in &self.0 {
            if node.y != row {
                row = node.y;
                writeln!(f)?;
            }

            if node.used == 0 {
                write!(f, "_")?;
            } else if node.used > empty.size {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }

        Ok(())
    }
}

impl<'a> IntoIterator for &'a Nodes {
    type Item = &'a Node;

    type IntoIter = std::slice::Iter<'a, Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Node {
    x: u32,
    y: u32,
    size: u32,
    used: u32,
    available: u32,
    use_percentage: u32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_ascii_whitespace().collect_vec();

        if tokens.len() != 5 {
            bail!("Expected 5 tokens - {tokens:?}");
        }

        let node_name = tokens[0];
        let node_name = node_name
            .strip_prefix("/dev/grid/node-")
            .ok_or(anyhow!("Could not strip device path prefix"))?;

        let (x, y) = node_name
            .split_once("-")
            .ok_or(anyhow!("Could not split x and y"))?;

        let x = x
            .strip_prefix("x")
            .ok_or(anyhow!("Could not strip x from '{x}'"))?;
        let y = y
            .strip_prefix("y")
            .ok_or(anyhow!("Could not strip y from '{y}'"))?;

        let x = x
            .parse()
            .with_context(|| format!("Could not parse x '{x}'"))?;
        let y = y
            .parse()
            .with_context(|| format!("Could not parse y '{y}'"))?;

        let size = tokens[1];
        let size = parse_size(size).with_context(|| format!("Could not parse size '{size}'"))?;

        let used = tokens[2];
        let used = parse_size(used).with_context(|| format!("Could not parse used '{used}'"))?;

        let available = tokens[3];
        let available = parse_size(available)
            .with_context(|| format!("Could not parse available '{available}'"))?;

        let use_percentage = tokens[4];
        let use_percentage = parse_percentage(use_percentage)
            .with_context(|| format!("Could not parse use_percentage '{use_percentage}'"))?;

        Ok(Node {
            x,
            y,
            size,
            used,
            available,
            use_percentage,
        })
    }
}

fn parse_size(s: &str) -> anyhow::Result<u32> {
    s.strip_suffix("T")
        .ok_or(anyhow!("Could not strip unit from '{s}'"))
        .and_then(|s| s.parse().map_err(anyhow::Error::from))
}

fn parse_percentage(s: &str) -> anyhow::Result<u32> {
    s.strip_suffix("%")
        .ok_or(anyhow!("Could not strip % from '{s}'"))
        .and_then(|s| s.parse().map_err(anyhow::Error::from))
}
