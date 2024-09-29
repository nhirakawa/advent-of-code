use std::collections::HashMap;
use std::time;
use crate::common::answer::*;
use time::SystemTime;

pub fn run() -> AdventOfCodeResult {
    let input = 325489;

    let part_one = part_one(&input);
    let part_two = part_two(&input);

    Ok((part_one, part_two))
}

fn part_one(number: &usize) -> PartAnswer {
    let start = SystemTime::now();

    let answer = distance_to_center(number);

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

fn distance_to_center(number: &usize) -> usize {
    if *number == 1 {
        return 0;
    }

    let corners = Corners::from(*number);

    let upper_corner_bound =
        if (corners.lower_left_corner..=corners.lower_right_corner).contains(number) {
            corners.lower_right_corner
        } else if (corners.upper_left_corner..=corners.lower_left_corner).contains(number) {
            corners.lower_left_corner
        } else if (corners.upper_right_corner..=corners.upper_left_corner).contains(number) {
            corners.upper_left_corner
        } else {
            corners.upper_right_corner
        };

    // sides will always be odd-lengthed
    let distance_from_corner_to_midpoint = (corners.nearest_upper_square_root - 1) / 2;

    // the number that is in the middle of the edge
    let midpoint_number = upper_corner_bound - distance_from_corner_to_midpoint;

    let distance_from_number_to_midpoint = midpoint_number.abs_diff(*number);

    distance_from_number_to_midpoint + ((corners.nearest_upper_square_root - 1) / 2)
}

fn corner(lower_right_corner: usize, multiplier: usize) -> usize {
    lower_right_corner.pow(2) - (multiplier * lower_right_corner) + multiplier
}

#[derive(Debug)]
struct Corners {
    nearest_upper_square_root: usize,
    upper_right_corner: usize,
    upper_left_corner: usize,
    lower_left_corner: usize,
    lower_right_corner: usize,
}

impl Corners {
    fn is_on_bottom_edge(&self, value: usize) -> bool {
        (self.lower_left_corner..=self.lower_right_corner).contains(&value)
    }

    fn is_on_left_edge(&self, value: usize) -> bool {
        (self.upper_left_corner..=self.lower_left_corner).contains(&value)
    }

    fn is_on_top_edge(&self, value: usize) -> bool {
        (self.upper_right_corner..=self.upper_left_corner).contains(&value)
    }
}

impl From<usize> for Corners {
    fn from(value: usize) -> Corners {
        let nearest_upper_square = (value as f64).sqrt() as usize;

        let nearest_upper_square = if nearest_upper_square.pow(2) == value {
            nearest_upper_square
        } else {
            nearest_upper_square + 1
        };

        let nearest_upper_square = if nearest_upper_square % 2 == 0 {
            nearest_upper_square + 1
        } else {
            nearest_upper_square
        };

        let lower_right_corner = nearest_upper_square.pow(2);
        let lower_left_corner = corner(nearest_upper_square.clone(), 1);
        let upper_left_corner = corner(nearest_upper_square.clone(), 2);
        let upper_right_corner = corner(nearest_upper_square, 3);

        Corners {
            nearest_upper_square_root: nearest_upper_square,
            lower_right_corner,
            lower_left_corner,
            upper_left_corner,
            upper_right_corner,
        }
    }
}

fn part_two(number: &usize) -> PartAnswer {
    let start = SystemTime::now();

    let mut values = HashMap::new();

    let mut current_coordinate = (0, 0);

    let mut counter = 1;

    loop {
        if counter >= 100 {
            panic!()
        }

        let value = if values.is_empty() {
            1
        } else {
            let neighbors = neighbors(&current_coordinate);
            neighbors.iter().filter_map(|c| values.get(c)).sum()
        };

        if value > *number {
            let elapsed = start.elapsed().unwrap();
            return PartAnswer::new(value, elapsed);
        }

        values.insert(current_coordinate, value);

        let (x, y) = current_coordinate;

        if is_odd_square(counter) {
            current_coordinate = (x + 1, y);
        } else {
            let corners = Corners::from(counter);

            if corners.is_on_bottom_edge(counter) {
                current_coordinate = (x + 1, y);
            } else if corners.is_on_left_edge(counter) {
                current_coordinate = (x, y - 1);
            } else if corners.is_on_top_edge(counter) {
                current_coordinate = (x - 1, y);
            } else {
                current_coordinate = (x, y + 1);
            }
        }

        counter += 1;
    }
}

fn neighbors(coordinate: &(i32, i32)) -> Vec<(i32, i32)> {
    let (x, y) = *coordinate;

    vec![
        (x + 1, y),
        (x - 1, y),
        (x + 1, y + 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y - 1),
        (x, y + 1),
        (x, y - 1),
    ]
}

fn is_odd_square(possible_square: usize) -> bool {
    if possible_square % 2 == 0 {
        return false;
    }

    let root = (possible_square as f64).sqrt() as usize;

    root.pow(2) == possible_square
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to_center() {
        assert_eq!(distance_to_center(&1), 0);
        assert_eq!(distance_to_center(&12), 3);
        assert_eq!(distance_to_center(&23), 2);
        assert_eq!(distance_to_center(&1024), 31);
    }

    #[test]
    fn test_corners() {
        let corners = Corners::from(2);
        assert_eq!(corners.is_on_bottom_edge(8), true);
        assert_eq!(corners.is_on_bottom_edge(4), false);
        assert_eq!(corners.is_on_bottom_edge(2), false);
        assert_eq!(corners.is_on_left_edge(2), false);
        assert_eq!(corners.is_on_top_edge(2), false);
    }
}
