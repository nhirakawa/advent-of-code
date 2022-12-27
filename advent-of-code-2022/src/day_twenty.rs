use common::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let part_one = part_one();
    let part_two = part_two();

    Ok((part_one, part_two))
}

fn part_one() -> PartAnswer {
    let start = SystemTime::now();
    let _elapsed = start.elapsed().unwrap();
    PartAnswer::default()
}

fn part_two() -> PartAnswer {
    let start = SystemTime::now();
    let _elapsed = start.elapsed().unwrap();
    PartAnswer::default()
}

fn mix(numbers: &[isize]) -> Vec<isize> {
    let mut mixed: Vec<isize> = numbers.iter().cloned().collect();

    for index in 0..numbers.len() {
        let number_to_mix = numbers[index];
        mixed = mix_number(&mixed, number_to_mix);
    }

    mixed
}

fn mix_number(sequence: &[isize], number_to_mix: isize) -> Vec<isize> {
    let mut updated = Vec::with_capacity(sequence.len());

    let index_of_number_to_mix = sequence
        .iter()
        .enumerate()
        .find_map(|(index, number)| {
            if *number == number_to_mix {
                Some(index)
            } else {
                None
            }
        })
        .unwrap();

    println!(
        "Mixing {} with index {}",
        number_to_mix, index_of_number_to_mix
    );

    let index_of_number_to_mix = index_of_number_to_mix as isize;

    let new_index = index_of_number_to_mix + number_to_mix;

    let new_index = if new_index > 0 {
        new_index as usize % sequence.len()
    } else if new_index < 0 {
        (new_index + (sequence.len() - 1) as isize) as usize % sequence.len()
    } else {
        sequence.len() - 1
    };

    if new_index as isize == index_of_number_to_mix {
        return sequence.iter().cloned().collect();
    }

    println!("  New index {}", new_index);

    for (index, number) in sequence.iter().enumerate() {
        if *number == number_to_mix {
            println!("  skipping {} from input", number);
            continue;
        }

        println!("  inserting {}", number);
        updated.push(*number);

        if index == new_index {
            println!("  inserting {} at index {}", number_to_mix, index);
            updated.push(number_to_mix);
        }
    }

    updated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_number() {
        assert_eq!(
            mix_number(&vec![1, 2, -3, 3, -2, 0, 4], 1),
            vec![2, 1, -3, 3, -2, 0, 4]
        );

        assert_eq!(
            mix_number(&vec![2, 1, -3, 3, -2, 0, 4], 2),
            vec![1, -3, 2, 3, -2, 0, 4]
        );

        assert_eq!(
            mix_number(&vec![1, -3, 2, 3, -2, 0, 4], -3),
            vec![1, 2, 3, -2, -3, 0, 4]
        );

        assert_eq!(
            mix_number(&vec![1, 2, -2, -3, 0, 3, 4], -2),
            vec![1, 2, -3, 0, 3, 4, -2]
        );

        assert_eq!(
            mix_number(&vec![1, 2, -3, 0, 3, 4, -2], 0),
            vec![1, 2, -3, 0, 3, 4, -2]
        );

        assert_eq!(
            mix_number(&vec![1, 2, -3, 0, 3, 4, -2], 4),
            vec![1, 2, -3, 4, 0, 3, -2]
        );

        // my test case
        assert_eq!(
            mix_number(&vec![20, 1, 2, 3, 4, 5, 6, 7, 8, 9], 20),
            vec![20, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn test_mix() {
        assert_eq!(
            mix(&vec![1, 2, -3, 3, -2, 0, 4]),
            vec![1, 2, -3, 4, 0, 3, -2]
        );
    }
}
