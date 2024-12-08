use std::collections::HashMap;

use itertools::Itertools;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let mut counts_by_index: HashMap<usize, HashMap<char, usize>> = HashMap::new();

    for line in input.trim().lines() {
        for (index, c) in line.chars().enumerate() {
            let count = counts_by_index.entry(index).or_default();
            *count.entry(c).or_insert(0) += 1;
        }
    }

    let mut most_frequent_character_by_index = counts_by_index
        .iter()
        .filter_map(|(index, counts)| {
            counts
                .iter()
                .max_by_key(|(_, count)| *count)
                .map(|(c, _)| (*index, *c))
        })
        .collect_vec();

    most_frequent_character_by_index.sort_by_key(|(index, _)| *index);

    Ok(most_frequent_character_by_index
        .iter()
        .map(|(_, c)| *c)
        .collect::<String>())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let mut counts_by_index: HashMap<usize, HashMap<char, usize>> = HashMap::new();

    for line in input.trim().lines() {
        for (index, c) in line.chars().enumerate() {
            let count = counts_by_index.entry(index).or_default();
            *count.entry(c).or_insert(0) += 1;
        }
    }

    let mut most_frequent_character_by_index = counts_by_index
        .iter()
        .filter_map(|(index, counts)| {
            counts
                .iter()
                .min_by_key(|(_, count)| *count)
                .map(|(c, _)| (*index, *c))
        })
        .collect_vec();

    most_frequent_character_by_index.sort_by_key(|(index, _)| *index);

    Ok(most_frequent_character_by_index
        .iter()
        .map(|(_, c)| *c)
        .collect::<String>())
}
