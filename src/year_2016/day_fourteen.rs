use anyhow::bail;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::env;
use std::iter::successors;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    nth_key(input, KeyAlgorithm::Simple, 64)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    nth_key(input, KeyAlgorithm::Extended, 64)
}

fn nth_key(salt: &str, key_algorithm: KeyAlgorithm, n: usize) -> anyhow::Result<usize> {
    let mut hashes = hashes(salt, key_algorithm);
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

            if valid_keys == n {
                return Ok(hash.index);
            }
        }
    }

    bail!("No solution found")
}

#[derive(Debug, Copy, Clone)]
enum KeyAlgorithm {
    Simple,
    Extended,
}

struct Hash {
    index: usize,
    /// The hexadecimal representation of the key
    #[allow(unused)]
    key: String,
    /// The first char that is repeated 3 times in a row, if present
    same_char: Option<char>,
    /// All characters that are repeated 5 times in a row
    repeated_chars: HashSet<char>,
}

impl Hash {
    fn new(salt: &str, index: usize, key_algorithm: KeyAlgorithm) -> Hash {
        let content = format!("{salt}{index}");
        let digest = md5::compute(content);

        let initial = format!("{digest:0x}");
        let n = match key_algorithm {
            KeyAlgorithm::Simple => 0,
            KeyAlgorithm::Extended => 2016,
        };

        let key = successors(Some(initial), |key| {
            Some(format!("{:0x}", md5::compute(key)))
        })
        .nth(n)
        .unwrap();

        let mut same_char = None;

        let mut repeated_chars = HashSet::new();

        for (a, b, c, d, e) in key.chars().tuple_windows() {
            if a == b && b == c && c == d && d == e {
                repeated_chars.insert(a);
            }

            if same_char.is_none()
                && ((a == b && b == c) || (b == c && c == d) || (c == d && d == e))
            {
                same_char = Some(c);
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

fn hashes(salt: &str, key_algorithm: KeyAlgorithm) -> impl Iterator<Item = Hash> {
    successors(Some(0), |n| Some(*n + 1)).map(move |index| Hash::new(salt, index, key_algorithm))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_hash() {
        let hash = Hash::new("abc", 0, KeyAlgorithm::Simple);
        assert_eq!(hash.key, "577571be4de9dcce85a041ba0410f29f");
    }

    #[test]
    fn test_extended_hash() {
        let hash = Hash::new("abc", 0, KeyAlgorithm::Extended);
        assert_eq!(hash.key, "a107ff634856bb300138cac6568c0f24");
    }

    #[test]
    fn test_nth_key_simple() {
        let first_key = nth_key("abc", KeyAlgorithm::Simple, 1).unwrap();
        assert_eq!(first_key, 39);

        let later_key = nth_key("abc", KeyAlgorithm::Simple, 64).unwrap();
        assert_eq!(later_key, 22728);
    }

    #[ignore = "takes 30+ seconds to run"]
    #[test]
    fn slow_test_nth_key_extended() {
        let first_key = nth_key("abc", KeyAlgorithm::Extended, 1).unwrap();
        assert_eq!(first_key, 10);

        let later_key = nth_key("abc", KeyAlgorithm::Extended, 64).unwrap();
        assert_eq!(later_key, 22551);
    }
}
