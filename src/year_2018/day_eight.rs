use anyhow::{anyhow, bail};
use itertools::Itertools;
use log::debug;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let bytes = to_bytes(input)?;
    let (_index, root) = parse(&bytes, 0)?;
    Ok(sum_all_metadata(&root))
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let bytes = to_bytes(input)?;
    let (_index, root) = parse(&bytes, 0)?;
    Ok(sum_node_values(&root))
}

fn sum_all_metadata(node: &Node) -> usize {
    let mut sum = 0;

    for child in &node.children {
        sum += sum_all_metadata(child);
    }

    for entry in &node.metadata {
        sum += *entry as usize;
    }

    sum
}

fn sum_node_values(node: &Node) -> usize {
    if node.children.is_empty() {
        let mut sum = 0;
        for entry in &node.metadata {
            sum += *entry as usize;
        }
        sum
    } else {
        let child_indexes = node
            .metadata
            .iter()
            .map(|idx| *idx as usize - 1)
            .collect_vec();

        let mut sum = 0;

        for child_index in child_indexes {
            if let Some(child_node) = node.children.get(child_index) {
                sum += sum_node_values(child_node);
            }
        }
        sum
    }
}

fn to_bytes(s: &str) -> anyhow::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    for token in s.split_ascii_whitespace() {
        let token = token.parse()?;
        bytes.push(token);
    }
    Ok(bytes)
}

fn parse(bytes: &[u8], index: usize) -> anyhow::Result<(usize, Node)> {
    let mut index = index;
    let number_of_children = bytes.get(index).copied().ok_or(anyhow!(
        "Cannot get number_of_children - index {index} out of bounds"
    ))?;

    index += 1;

    let number_of_metadata = bytes.get(index).copied().ok_or(anyhow!(
        "Cannot get number_of_metadata - index {index} out of bounds"
    ))?;

    if number_of_metadata == 0 {
        bail!("Invalid number_of_metadata at index {index}");
    }

    index += 1;

    let mut children = Vec::new();
    for _ in 0..number_of_children {
        let (next_index, child) = parse(bytes, index)?;
        index = next_index;
        children.push(child);
    }

    let mut metadata = Vec::new();
    for _ in 0..number_of_metadata {
        let entry = bytes
            .get(index)
            .copied()
            .ok_or(anyhow!("Cannot get metadata - index {index} out-of-bounds"))?;
        metadata.push(entry);
        index += 1;
    }

    debug!("{metadata:?}");

    Ok((index, Node { children, metadata }))
}

#[derive(Debug, Default)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u8>,
}
