use std::collections::{HashMap, HashSet};

use common::math;
use common::prelude::*;
use log::trace;

type Data = i32;
type Asteroid = (Data, Data);

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-10.txt");
    let asteroids = parse_asteroids(input);

    let part_one = part_one(&asteroids);
    let part_two = part_two(&asteroids);

    Ok((part_one, part_two))
}

fn part_one(asteroids: &HashSet<(Data, Data)>) -> PartAnswer {
    let start = SystemTime::now();

    let best_location = find_best_location(asteroids);

    let max = normalize_directions(&best_location, asteroids).len();

    PartAnswer::new(max, start.elapsed().unwrap())
}

fn part_two(asteroids: &HashSet<(Data, Data)>) -> PartAnswer {
    let start = SystemTime::now();

    let base = find_best_location(asteroids);

    let (x, y) = vaporize(&base, asteroids, 200);

    PartAnswer::new((x * 100) + y, start.elapsed().unwrap())
}

fn vaporize(
    base: &(Data, Data),
    asteroids: &HashSet<(Data, Data)>,
    number_of_asteroids: usize,
) -> (Data, Data) {
    let asteroids_by_normalized_vector = group_asteroids_by_normalized_vector(base, asteroids);

    let mut asteroids_with_angle = Vec::new();

    for (key, value) in asteroids_by_normalized_vector {
        let angle = calculate_angle(&(0, 0), &key);

        trace!("({}) {:?}", angle, value);

        for (index, value) in value.into_iter().enumerate() {
            let modifier = (index * 360) as f32;

            trace!("({}) -> ({})", angle, angle + modifier);

            let angle_and_coordinates = (angle + modifier, value);

            asteroids_with_angle.push(angle_and_coordinates);
        }
    }

    asteroids_with_angle.sort_by(|(k1, _), (k2, _)| k1.partial_cmp(k2).unwrap());

    for (index, (angle, asteroid)) in asteroids_with_angle.iter().enumerate() {
        trace!(
            "[{}] {:?} is at angle of {} degrees",
            index,
            asteroid,
            angle
        );
    }

    asteroids_with_angle
        .get(number_of_asteroids - 1)
        .unwrap()
        .clone()
        .1
}

fn group_asteroids_by_normalized_vector(
    source: &(Data, Data),
    asteroids: &HashSet<(Data, Data)>,
) -> HashMap<(Data, Data), Vec<(Data, Data)>> {
    let mut asteroids_by_normalized_vector = HashMap::new();

    // group asteroids by vector
    for asteroid in asteroids {
        if asteroid == source {
            continue;
        }

        let normalized_direction = normalize_direction(source, asteroid);

        trace!(
            "{:?} -> {:?} => {:?}",
            source,
            asteroid,
            normalized_direction
        );

        if asteroids_by_normalized_vector
            .get(&normalized_direction)
            .is_none()
        {
            let value = Vec::new();

            asteroids_by_normalized_vector.insert(normalized_direction.clone(), value);
        }

        asteroids_by_normalized_vector
            .get_mut(&normalized_direction)
            .unwrap()
            .push(asteroid.clone());
    }

    for (_, asteroids) in asteroids_by_normalized_vector.iter_mut() {
        asteroids.sort_by(|(a, b), (x, y)| {
            let subtracted_vector_1 = (source.0 - a, source.1 - b);
            let subtracted_vector_1 = (subtracted_vector_1.0 as f32, subtracted_vector_1.1 as f32);

            let norm_1 = ((subtracted_vector_1.0 * subtracted_vector_1.0)
                + (subtracted_vector_1.1 * subtracted_vector_1.1))
                .sqrt();

            let subtracted_vector_2 = (source.0 - x, source.1 - y);
            let subtracted_vector_2 = (subtracted_vector_2.0 as f32, subtracted_vector_2.1 as f32);

            let norm_2 = ((subtracted_vector_2.0 * subtracted_vector_2.0)
                + (subtracted_vector_2.1 * subtracted_vector_2.1))
                .sqrt();

            trace!("{:?} -> {:?} => {:?}", source, (a, b), norm_1);
            trace!("{:?} -> {:?} => {:?}", source, (x, y), norm_2);

            norm_1.partial_cmp(&norm_2).unwrap()
        });
    }

    asteroids_by_normalized_vector
}

fn find_best_location(asteroids: &HashSet<(Data, Data)>) -> (Data, Data) {
    asteroids
        .iter()
        .max_by_key(|asteroid| normalize_directions(*asteroid, asteroids).len())
        .unwrap()
        .clone()
}

fn normalize_directions(
    source: &(Data, Data),
    asteroids: &HashSet<(Data, Data)>,
) -> HashSet<(Data, Data)> {
    let mut normalized_vectors = HashSet::new();

    for asteroid in asteroids {
        if asteroid == source {
            continue;
        }

        let normalized_vector = normalize_direction(source, asteroid);

        normalized_vectors.insert(normalized_vector);
    }

    normalized_vectors
}

fn normalize_direction(source: &(Data, Data), target: &(Data, Data)) -> (Data, Data) {
    let delta_x = target.0 - source.0;
    let delta_y = target.1 - source.1;

    let divisor = math::gcd(delta_x, delta_y) as Data;

    let x = delta_x / divisor * divisor.signum();
    let y = delta_y / divisor * divisor.signum();

    (x, y)
}

/*
 * There's probably a better way to solve this
 * We want angles to increase clockwise
 * We also want to rotate the plane (since upper-left in puzzle-space is (0,0))
 * Experimentally, this seems to work
 */
fn calculate_angle(source: &Asteroid, target: &Asteroid) -> f32 {
    let subtracted_vector = (source.0 - target.0, source.1 - target.1);
    let subtracted_vector = (subtracted_vector.0 as f32, subtracted_vector.1 as f32);

    let mut angle = subtracted_vector.1.atan2(subtracted_vector.0).to_degrees() - 90.0;

    // println!(
    //     "[{:?} {:?}] subtracted vector {:?} ({})",
    //     source, target, subtracted_vector, angle
    // );

    while angle < 0.0 {
        // println!("adding 360 degrees");
        angle += 360.0;
    }

    while angle >= 360.0 {
        // println!("removing 360 degrees");
        angle -= 360.0;
    }

    angle
}

fn parse_asteroids(i: &str) -> HashSet<Asteroid> {
    let mut x = 0;
    let mut y = 0;

    let mut asteroids = HashSet::new();

    for c in i.chars() {
        if c == '\n' {
            x = 0;
            y += 1;
        } else {
            if c == '#' {
                asteroids.insert((x, y));
            }

            x += 1;
        }
    }

    asteroids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_asteroids() {
        let input = ".#..#\n.....\n#####\n....#\n...##";

        let asteroids = parse_asteroids(input);

        assert_eq!(
            asteroids,
            vec![
                (1, 0),
                (4, 0),
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 2),
                (4, 2),
                (4, 3),
                (3, 4),
                (4, 4)
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_normalize_direction() {
        let source = (3, 4);
        let target = (1, 0);

        let normalized = normalize_direction(&source, &target);
        assert_eq!(normalized, (-1, -2));

        let normalized = normalize_direction(&target, &source);
        assert_eq!(normalized, (1, 2));

        let target = (2, 2);

        let normalized = normalize_direction(&source, &target);
        assert_eq!(normalized, (-1, -2));
    }

    #[test]
    fn test_angle() {
        let source = (3, 4);

        let target = (3, 0);
        let angle = calculate_angle(&source, &target);
        assert_eq!(angle, 0.0);

        let target = (3, 1);
        let angle = calculate_angle(&source, &target);
        assert_eq!(angle, 0.0);

        let target = (4, 3);
        let angle = calculate_angle(&source, &target);
        assert_eq!(angle, 45.0);

        let target = (3, 5);
        let angle = calculate_angle(&source, &target);
        assert_eq!(angle, 180.0);

        let target = (4, 4);
        let angle = calculate_angle(&source, &target);
        assert_eq!(angle, 90.0);

        let target = (2, 4);
        let angle = calculate_angle(&source, &target);
        assert_eq!(angle, 270.0);

        let source = (8, 3);

        let target = (8, 1);
        let angle = calculate_angle(&source, &target);
        assert_eq!(angle, 0.0);
    }

    #[test]
    fn test_group_asteroids_by_normalized_vector() {
        let source = (8, 3);

        let asteroids = vec![(8, 1), (8, 0), (9, 2)].into_iter().collect();

        let grouped = group_asteroids_by_normalized_vector(&source, &asteroids);

        assert_eq!(grouped.get(&(0, -1)), Some(&vec![(8, 1), (8, 0)]));
    }

    #[test]
    fn test_vaporize_small() {
        let asteroids = vec![(8, 3), (8, 1), (8, 0), (9, 2)].into_iter().collect();

        let source = (8, 3);

        let first_vaporized = vaporize(&source, &asteroids, 1);
        assert_eq!(first_vaporized, (8, 1));

        let second_vaporized = vaporize(&source, &asteroids, 2);
        assert_eq!(second_vaporized, (9, 2));

        let third_vaporized = vaporize(&source, &asteroids, 3);
        assert_eq!(third_vaporized, (8, 0));
    }

    #[test]
    fn test_vaporize() {
        let input = ".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....X...###..\n..#.#.....#....##";
        let asteroids = parse_asteroids(input);

        let base = (8, 3);

        let first_vaporized = vaporize(&base, &asteroids, 1);
        assert_eq!(first_vaporized, (8, 1));
    }

    #[test]
    fn test_vaporize_large() {
        let input = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";
        let asteroids = parse_asteroids(input);

        let base = (11, 13);

        let first_vaporized = vaporize(&base, &asteroids, 1);
        assert_eq!(first_vaporized, (11, 12));

        let second_vaporized = vaporize(&base, &asteroids, 2);
        assert_eq!(second_vaporized, (12, 1));

        let third_vaporized = vaporize(&base, &asteroids, 3);
        assert_eq!(third_vaporized, (12, 2));

        let tenth_vaporized = vaporize(&base, &asteroids, 10);
        assert_eq!(tenth_vaporized, (12, 8));

        let twentieth_vaporized = vaporize(&base, &asteroids, 20);
        assert_eq!(twentieth_vaporized, (16, 0));

        let fiftieth_vaporized = vaporize(&base, &asteroids, 50);
        assert_eq!(fiftieth_vaporized, (16, 9));

        let hundredth_vaporized = vaporize(&base, &asteroids, 100);
        assert_eq!(hundredth_vaporized, (10, 16));

        let hundred_ninety_ninth_vaporized = vaporize(&base, &asteroids, 199);
        assert_eq!(hundred_ninety_ninth_vaporized, (9, 6));

        let two_hundredth_vaporized = vaporize(&base, &asteroids, 200);
        assert_eq!(two_hundredth_vaporized, (8, 2));

        let two_hundred_first_vaporized = vaporize(&base, &asteroids, 201);
        assert_eq!(two_hundred_first_vaporized, (10, 9));

        let two_hundred_ninety_ninth_vaporized = vaporize(&base, &asteroids, 299);
        assert_eq!(two_hundred_ninety_ninth_vaporized, (11, 1));
    }
}
