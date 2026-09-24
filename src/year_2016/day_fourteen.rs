use anyhow::{anyhow, bail};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::env;
use std::iter::successors;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let mut hashes = hashes(input);

    let mut hashes_window = VecDeque::with_capacity(1000);

    while hashes_window.len() < 1000 {
        hashes_window.push_back(hashes.next().unwrap());
    }

    let mut valid_keys = 0;

    let max_iterations = env::var("ITERATIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1_000_000);

    while let Some(hash) = hashes_window.pop_front() {
        if hash.index >= max_iterations {
            bail!("Too many iterations");
        }

        while hashes_window.len() < 1000 {
            hashes_window.push_back(hashes.next().unwrap());
        }

        if is_valid_key(&hash, hashes_window.iter()) {
            valid_keys += 1;

            if valid_keys == 64 {
                return Ok(hash.index);
            }
        }
    }

    bail!("No solution found")
}

pub fn part_two(_input: &str) -> anyhow::Result<impl ToString> {
    Err::<usize, _>(anyhow!("Not implemented"))
}

struct Hash {
    index: usize,
    /// The hexadecimal representation of the key
    key: String,
    /// The first char that is repeated 3 times in a row, if present
    same_char: Option<char>,
    /// All characters that are repeated 5 times in a row
    repeated_chars: HashSet<char>,
}

impl Hash {
    fn new(salt: &str, index: usize) -> Hash {
        let content = format!("{salt}{index}");
        let digest = md5::compute(content);
        let key = format!("{digest:0x}");

        let mut same_char = None;

        let mut repeated_chars = HashSet::new();

        for (a, b, c, d, e) in key.chars().tuple_windows() {
            if a == b && b == c && c == d && d == e {
                repeated_chars.insert(a);
            }

            if same_char.is_none() {
                if a == b && b == c {
                    same_char = Some(c);
                } else if b == c && c == d {
                    same_char = Some(c);
                } else if c == d && d == e {
                    same_char = Some(c);
                }
            }
        }

        Hash {
            index,
            key,
            same_char,
            repeated_chars,
        }
    }
}

fn hashes(salt: &str) -> impl Iterator<Item = Hash> {
    successors(Some(0), |n| Some(*n + 1)).map(|index| Hash::new(salt, index))
}

fn is_valid_key<'a>(hash: &Hash, next_hashes: impl Iterator<Item = &'a Hash>) -> bool {
    if let Some(repeat) = hash.same_char {
        for next in next_hashes {
            if next.repeated_chars.contains(&repeat) {
                return true;
            }
        }
    }

    false
}
