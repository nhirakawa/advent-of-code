use anyhow::{anyhow, bail};
use itertools::Itertools;
use log::debug;
use std::ops::RangeInclusive;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let mut blocks = parse(input)?;

    while !is_compact(&blocks) {
        blocks = do_compaction(blocks)?;
    }

    let mut checksum = 0;

    for (index, block) in blocks.into_iter().enumerate() {
        if let Block::File { id } = block {
            checksum += index * id;
        }
    }

    Ok(checksum.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let mut blocks = parse(input)?;

    let ids = blocks
        .iter()
        .filter_map(|block| match block {
            Block::File { id } => Some(*id),
            _ => None,
        })
        .unique()
        .collect_vec();

    for id in ids.iter().rev() {
        let range_of_block = range_of_block(*id, &blocks);
        let size = range_of_block.end() - range_of_block.start() + 1;

        debug!(
            "Moving file {} with size {} ({:?})",
            id, size, range_of_block
        );

        if let Some(range) = find_free_space(&range_of_block, &blocks) {
            debug!("Found free space: {:?}", range);
            blocks = move_file(*id, range_of_block, range, &blocks);
        } else {
            debug!("No free space found for file {}", id);
        }
    }

    let mut checksum = 0;

    for (index, block) in blocks.into_iter().enumerate() {
        if let Block::File { id } = block {
            checksum += index * id;
        }
    }

    Ok(checksum.to_string())
}

fn move_file(
    id: usize,
    file_indexes: RangeInclusive<usize>,
    free_space_indexes: RangeInclusive<usize>,
    blocks: &Blocks,
) -> Blocks {
    let mut updated_blocks = vec![];

    for (index, block) in blocks.iter().enumerate() {
        if file_indexes.contains(&index) {
            updated_blocks.push(Block::Free);
        } else if free_space_indexes.contains(&index) {
            updated_blocks.push(Block::File { id });
        } else {
            updated_blocks.push(*block);
        }
    }

    updated_blocks
}

/// Returns (true, blocks) if compaction was performed, (false, blocks) otherwise
fn do_compaction(blocks: Blocks) -> anyhow::Result<Blocks> {
    let index_of_first_free_block =
        index_of_first_free_block(&blocks).ok_or(anyhow!("No free blocks found"))?;
    let index_of_last_file_block =
        index_of_last_file_block(&blocks).ok_or(anyhow!("No file blocks found"))?;

    let file_block_id = match blocks.get(index_of_last_file_block) {
        Some(Block::File { id }) => *id,
        _ => bail!("Expected file block at index {index_of_last_file_block}"),
    };

    let mut updated_blocks = vec![];

    for (index, block) in blocks.into_iter().enumerate() {
        if index == index_of_first_free_block {
            updated_blocks.push(Block::File { id: file_block_id });
        } else if index == index_of_last_file_block {
            updated_blocks.push(Block::Free);
        } else {
            updated_blocks.push(block);
        }
    }

    Ok(updated_blocks)
}

fn is_compact(blocks: &Blocks) -> bool {
    blocks.iter().tuple_windows().all(|(first, second, third)| {
        matches!(
            (first, second, third),
            (Block::File { .. }, Block::File { .. }, Block::File { .. })
                | (Block::File { .. }, Block::File { .. }, Block::Free { .. })
                | (Block::File { .. }, Block::Free { .. }, Block::Free { .. })
                | (Block::Free { .. }, Block::Free { .. }, Block::Free { .. })
        )
    })
}

fn range_of_block(id: usize, blocks: &Blocks) -> RangeInclusive<usize> {
    let start = blocks
        .iter()
        .position(|block| matches!(block, Block::File { id: block_id } if *block_id == id))
        .expect("Block not found");

    let end = blocks
        .iter()
        .rposition(|block| matches!(block, Block::File { id: block_id } if *block_id == id))
        .expect("Block not found");

    start..=end
}

fn find_free_space(
    file_range: &RangeInclusive<usize>,
    blocks: &Blocks,
) -> Option<RangeInclusive<usize>> {
    let size = file_range.end() - file_range.start() + 1;
    let free_spaces = find_free_spaces(blocks);

    free_spaces
        .into_iter()
        .filter(|range| range.start() < file_range.start())
        .find(|range| range.end() - range.start() + 1 >= size)
        .map(|range| *range.start()..=(range.start() + size - 1))
}

fn find_free_spaces(blocks: &Blocks) -> Vec<RangeInclusive<usize>> {
    let mut in_free_block = false;
    let mut start = 0;
    let mut ranges = vec![];

    for (index, block) in blocks.iter().enumerate() {
        if in_free_block {
            if block.is_file() {
                in_free_block = false;
                let end = index - 1;
                let range = start..=end;
                ranges.push(range);
            }
        } else if block.is_free() {
            in_free_block = true;
            start = index;
        }
    }

    ranges
}

fn index_of_first_free_block(blocks: &Blocks) -> Option<usize> {
    blocks.iter().position(|block| block.is_free())
}

fn index_of_last_file_block(blocks: &Blocks) -> Option<usize> {
    blocks.iter().rposition(|block| block.is_file())
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Block {
    Free,
    File { id: usize },
}

impl Block {
    fn is_free(&self) -> bool {
        matches!(self, Block::Free { .. })
    }

    fn is_file(&self) -> bool {
        matches!(self, Block::File { .. })
    }
}

type Blocks = Vec<Block>;

fn parse(input: &str) -> anyhow::Result<Blocks> {
    let mut blocks = vec![];

    let mut is_file = true;
    let mut id = 0;

    for (index, c) in input.char_indices() {
        let size = c
            .to_digit(10)
            .map(|d| d as usize)
            .ok_or_else(|| anyhow!("Could not parse size from '{c}' at index {index}"))?;
        if is_file {
            for _ in 0..size {
                blocks.push(Block::File { id });
            }
            id += 1;
        } else {
            for _ in 0..size {
                blocks.push(Block::Free);
            }
        }

        is_file = !is_file;
    }

    Ok(blocks)
}
