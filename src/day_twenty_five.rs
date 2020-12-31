use crate::prelude::*;

pub fn run() -> AdventOfCodeResult<u64, u64> {
    let part_one = part_one();
    let part_two = PartAnswer::default();

    Ok((part_one, part_two))
}

fn part_one() -> PartAnswer<u64> {
    let start = SystemTime::now();

    let card_public_key = 12092626;
    let door_public_key = 4707356;

    let door_loop_size = find_loop_size(door_public_key);

    let mut card_encryption_key = 1;
    for _ in 0..door_loop_size {
        card_encryption_key = transform(card_encryption_key, card_public_key);
    }

    let elapsed = start.elapsed().unwrap();

    (card_encryption_key, elapsed).into()
}

fn find_loop_size(public_key: u64) -> usize {
    let mut loop_size = 1;
    let mut current_subject_number = 1;

    loop {
        let new_card_subject_number = transform(current_subject_number, 7);
        if new_card_subject_number == public_key {
            break;
        }

        current_subject_number = new_card_subject_number;
        loop_size += 1;
    }

    loop_size
}

fn transform(old: u64, subject_number: u64) -> u64 {
    (old * subject_number) % 20201227
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_loop_size() {
        let card_public_key = 5764801;
        let card_loop_size = find_loop_size(card_public_key);
        assert_eq!(card_loop_size, 8);

        let mut subject_number = 1;

        for _ in 0..card_loop_size {
            subject_number = transform(subject_number, 7);
        }

        assert_eq!(subject_number, card_public_key);

        let door_public_key = 17807724;
        assert_eq!(find_loop_size(door_public_key), 11);
    }
}
