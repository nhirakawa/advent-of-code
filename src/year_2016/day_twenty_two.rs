use std::str::FromStr;

use anyhow::{Context, anyhow, bail};
use itertools::Itertools;

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

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
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
