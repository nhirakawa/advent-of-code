use anyhow::bail;
use bitvec::{array::BitArray, bitarr};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (keys, locks) = parse(input)?;

    let number_of_combinations = keys.len() * locks.len();

    println!("Number of keys: {}", keys.len());
    println!("Number of locks: {}", locks.len());
    println!("Number of combinations: {}", number_of_combinations);

    let mut count = 0;

    for key in &keys {
        for lock in &locks {
            if are_compatible(key, lock) {
                count += 1;
            }
        }
    }

    Ok(count)
}

fn are_compatible(key: &Key, lock: &Lock) -> bool {
    for i in 0..35 {
        if key.0[i] && lock.0[i] {
            return false;
        }
    }

    true
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Key(BitArray);

impl Key {
    fn new() -> Self {
        Key(bitarr![0; 35])
    }

    fn set(&mut self, index: usize) {
        self.0.set(index, true);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Lock(BitArray);

impl Lock {
    fn new() -> Self {
        Lock(bitarr![0; 35])
    }

    fn set(&mut self, index: usize) {
        self.0.set(index, true);
    }
}

type Keys = Vec<Key>;
type Locks = Vec<Lock>;

fn parse(input: &str) -> anyhow::Result<(Keys, Locks)> {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for block in input.split("\n\n") {
        if block.starts_with(".....") && block.ends_with("#####") {
            let key = parse_key(block)?;
            keys.push(key);
        } else if block.starts_with("#####") && block.ends_with(".....") {
            let lock = parse_lock(block)?;
            locks.push(lock);
        } else {
            bail!("Invalid block");
        }
    }

    Ok((keys, locks))
}

fn parse_key(block: &str) -> anyhow::Result<Key> {
    let mut key = Key::new();

    let chars = block.chars().filter(|c| *c != '\n').collect::<Vec<_>>();

    for char in chars.iter().enumerate() {
        if *char.1 == '#' {
            key.set(char.0);
        }
    }

    Ok(key)
}

fn parse_lock(block: &str) -> anyhow::Result<Lock> {
    let mut lock = Lock::new();

    let chars = block.chars().filter(|c| *c != '\n').collect::<Vec<_>>();

    for char in chars.iter().enumerate() {
        if *char.1 == '#' {
            lock.set(char.0);
        }
    }

    Ok(lock)
}
