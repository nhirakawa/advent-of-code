use std::collections::{HashSet, VecDeque};

use common::prelude::*;
use log::debug;
use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-15.txt");

    let part_one = part_one(input);
    let part_two = part_two(input);

    Ok((part_one, part_two))
}

fn part_one(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let sensors = parse(input);

    let total_exclusion_area = total_exclusion_area(&sensors, 2000000);

    let answer = total_exclusion_area.len();

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(answer, elapsed)
}

/**
 * The key insight here is that the one place where a beacon could be lies just out of range of 4 sensors
 * If the beacon were further away from the sensors, the area would be larger than just one square
 * To reduce the runtime, we only need to check the boundaries of each sensor and make sure it doesn't lie within another sensor
 */
fn part_two(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let sensors = parse(input);

    for sensor in &sensors {
        let boundary = get_closest_possible_beacons(sensor);

        for potential_beacon_location in boundary {
            let mut is_out_of_range_of_all_sensors = true;

            for sensor in &sensors {
                if sensor.is_in_range(&potential_beacon_location) {
                    is_out_of_range_of_all_sensors = false;
                    break;
                }
            }

            if is_out_of_range_of_all_sensors {
                let (x, y) = potential_beacon_location;

                let answer = (x * 4_000_000) + y;

                let elapsed = start.elapsed().unwrap();

                return PartAnswer::new(answer, elapsed);
            }
        }
    }

    PartAnswer::default()
}

fn total_exclusion_area(sensors: &[Sensor], y_coordinate: isize) -> HashSet<(isize, isize)> {
    let sensor_locations: HashSet<(isize, isize)> =
        sensors.iter().map(|sensor| sensor.location).collect();

    let beacon_locations: HashSet<(isize, isize)> =
        sensors.iter().map(|sensor| sensor.closest_beacon).collect();

    let mut total_exclusion_area = HashSet::new();

    for sensor in sensors {
        let exclusion_area = sensor.exclusion_area(y_coordinate);

        total_exclusion_area.extend(exclusion_area);
    }

    let sensors_and_beacons: HashSet<(isize, isize)> =
        sensor_locations.union(&beacon_locations).cloned().collect();

    total_exclusion_area
        .difference(&sensors_and_beacons)
        .cloned()
        .collect()
}

/**
 * Returns the closest points to the sensor that _could_ contain beacons
 */
fn get_closest_possible_beacons(sensor: &Sensor) -> HashSet<(isize, isize)> {
    let distance = (sensor.radius + 1) as isize;

    let start = (sensor.location.0, sensor.location.1 + distance);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut seen = HashSet::new();

    while let Some(current) = queue.pop_front() {
        if seen.contains(&current) {
            continue;
        }

        seen.insert(current);

        let potential_next: Vec<(isize, isize)> = vec![
            (current.0 + 1, current.1 + 1),
            (current.0 - 1, current.1 + 1),
            (current.0 + 1, current.1 - 1),
            (current.0 - 1, current.1 - 1),
        ]
        .into_iter()
        .filter(|coordinate| manhattan_distance(&sensor.location, coordinate) == distance as usize)
        .collect();

        for potential in potential_next {
            queue.push_back(potential);
        }
    }

    seen
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Sensor {
    location: (isize, isize),
    closest_beacon: (isize, isize),
    radius: usize,
}

impl Sensor {
    fn new(location: (isize, isize), closest_beacon: (isize, isize)) -> Sensor {
        let radius = manhattan_distance(&location, &closest_beacon);

        Sensor {
            location,
            closest_beacon,
            radius,
        }
    }

    fn is_in_range(&self, other: &(isize, isize)) -> bool {
        manhattan_distance(&self.location, other) <= self.radius
    }

    fn exclusion_area(&self, y_coordinate: isize) -> HashSet<(isize, isize)> {
        let beacon_manhattan_distance = manhattan_distance(&self.location, &self.closest_beacon);

        debug!(
            "{:?} has distance {} from its closest beacon",
            self.location, beacon_manhattan_distance
        );

        let distance_to_target_row = self.location.1.abs_diff(y_coordinate);

        // check that going to target row in straight line is possible
        if distance_to_target_row > beacon_manhattan_distance {
            debug!(
                "{:?} is too far away from row y={y_coordinate}",
                self.location
            );
            return HashSet::new();
        }

        let remaining_distance = beacon_manhattan_distance - distance_to_target_row;

        let mut exclusion_area = HashSet::new();

        for x in 0..=remaining_distance {
            exclusion_area.insert((self.location.0 + x as isize, y_coordinate));
            exclusion_area.insert((self.location.0 - x as isize, y_coordinate));
        }

        exclusion_area
    }
}

fn manhattan_distance(first: &(isize, isize), second: &(isize, isize)) -> usize {
    first.0.abs_diff(second.0) + first.1.abs_diff(second.1)
}

fn parse(i: &str) -> Vec<Sensor> {
    finish(sensors)(i).unwrap().1
}

fn sensors(i: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list1(tag("\n"), sensor)(i)
}

fn sensor(i: &str) -> IResult<&str, Sensor> {
    map(
        tuple((
            tag("Sensor at "),
            location,
            tag(": closest beacon is at "),
            location,
        )),
        |(_, location, _, closest_beacon)| Sensor::new(location, closest_beacon),
    )(i)
}

fn location(i: &str) -> IResult<&str, (isize, isize)> {
    separated_pair(x_coordinate, tag(", "), y_coordinate)(i)
}

fn x_coordinate(i: &str) -> IResult<&str, isize> {
    preceded(tag("x="), number)(i)
}

fn y_coordinate(i: &str) -> IResult<&str, isize> {
    preceded(tag("y="), number)(i)
}
