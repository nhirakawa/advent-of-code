use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let serial_number = input.parse::<i32>()?;
    let power_levels = calculate_power_levels(serial_number, 3);

    let max_top_left = power_levels
        .into_iter()
        .filter(|((_, width), _)| *width == 3)
        .max_by_key(|((_, _), value)| *value)
        .map(|((coordinate, _), _)| coordinate)
        .unwrap();

    Ok(format!("{max_top_left:?}"))
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let serial_number = input.parse::<i32>()?;
    let power_levels = calculate_power_levels(serial_number, 300);

    let ((x, y, width), _) = power_levels
        .into_iter()
        .max_by_key(|(_, value)| *value)
        .map(|(((x, y), width), value)| ((x, y, width), value))
        .unwrap();

    Ok(format!("{x},{y},{width}"))
}

fn calculate_power_levels(serial_number: i32, max_width: i32) -> HashMap<((i32, i32), i32), i32> {
    let mut power_levels = HashMap::new();

    // Initialize the power levels with width=1
    for x in 1..=300 {
        for y in 1..=300 {
            power_levels.insert(((x, y), 1), power_level(x, y, serial_number));
        }
    }

    // Calculate the power levels for squares of increasing width
    for width in 2..=max_width {
        let max_coord = 300 - width + 1;

        let all_coordinates = (1..=max_coord)
            .cartesian_product(1..=max_coord)
            .collect_vec();

        let extension_power_levels = all_coordinates
            .par_iter()
            .map(|(x, y)| {
                let sub_square_sum = power_levels.get(&((*x, *y), width - 1)).copied().unwrap();
                let right_and_bottom_sum = right_and_bottom(*x, *y, width)
                    .iter()
                    .filter_map(|point| power_levels.get(&(*point, 1)))
                    .sum::<i32>();

                let square_sum = sub_square_sum + right_and_bottom_sum;

                (((*x, *y), width), square_sum)
            })
            .collect::<HashMap<_, _>>();

        power_levels.extend(extension_power_levels);
    }

    power_levels
}

/// Returns the right and bottom edge coordinates of the square with width `width` and top-left corner at (x, y)
fn right_and_bottom(x: i32, y: i32, width: i32) -> HashSet<(i32, i32)> {
    let mut coordinates = HashSet::new();

    // right edge (top-to-bottom)
    for delta_y in 0..width {
        coordinates.insert((x + width - 1, y + delta_y));
    }

    // bottom edge (left to right)
    for delta_x in 0..width {
        coordinates.insert((x + delta_x, y + width - 1));
    }

    coordinates
}

fn power_level(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let power_level = rack_id * y;
    let power_level = power_level + serial_number;
    let power_level = power_level * rack_id;
    hundreds_digit(power_level) - 5
}

fn hundreds_digit(i: i32) -> i32 {
    if i < 100 { 0 } else { (i / 100) % 10 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extracts_hundreds_digit() {
        assert_eq!(hundreds_digit(1000), 0);
        assert_eq!(hundreds_digit(4), 0);
        assert_eq!(hundreds_digit(739), 7);
        assert_eq!(hundreds_digit(12345), 3);
    }

    #[test]
    fn it_calculates_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn it_returns_right_and_bottom_coordinates() {
        assert_eq!(
            right_and_bottom(10, 5, 5),
            [
                (14, 5),
                (14, 6),
                (14, 7),
                (14, 8),
                (14, 9),
                (10, 9),
                (11, 9),
                (12, 9),
                (13, 9)
            ]
            .into()
        )
    }

    #[test]
    fn it_calculates_power_levels_for_squares() {
        assert_eq!(
            calculate_power_levels(18, 20)
                .get(&((90, 269), 16))
                .copied()
                .unwrap(),
            113
        );

        assert_eq!(
            calculate_power_levels(42, 14)
                .get(&((232, 251), 12))
                .copied()
                .unwrap(),
            119
        );
    }
}
