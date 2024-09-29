use std::collections::{HashMap, HashSet};
use std::time::SystemTime;
use crate::common::answer::*;
use log::{debug, trace};
use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::many1, IResult};
use crate::common::parse::finish;

/**
 * NOTES
 *
 * I use a coordinate system where
 * - x takes on a value [0, 6] and increases right
 * - y takes on a value [0, infinity) and increases up
 *
 * Spaces (0, 0) to (0, 6) can be occupied by rocks; everything else is out of bounds
 *
 * One optimization to make is to only keep the highest rock particle for each x position
 * The rocks below cannot (as of part 1) influence a falling rock
 */

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("input/day-17.txt");

    let part_one = part_one(input);
    let part_two = part_two(input);

    Ok((part_one, part_two))
}

fn part_one(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let wind_directions = parse(input);

    let mut game = TetrisGame::new(wind_directions);

    for _ in 0..2022 {
        game.add_rock();
    }

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(game.highest_y, elapsed)
}

fn part_two(input: &str) -> PartAnswer {
    let start = SystemTime::now();

    let wind_directions = parse(input);

    let mut game = TetrisGame::new(wind_directions);

    let mut first_appearances = HashMap::new();

    // the height of the rock tower after that many rocks have fallen
    let mut heights_after_rocks_fallen = HashMap::new();

    let mut cycle_start_index = usize::MAX;

    for index in 0..10_000 {
        let current_shape = game.next_shape_type;
        let shape_index = match current_shape {
            ShapeType::Horizontal => 0,
            ShapeType::Cross => 1,
            ShapeType::El => 2,
            ShapeType::Vertical => 3,
            ShapeType::Square => 4,
        };

        let jet_stream_index = game.current_wind_index;
        game.add_rock();

        let height = game.highest_y;

        heights_after_rocks_fallen.insert(index + 1, height);

        debug!(
            "Rock {}, shape {shape_index}, jet stream {jet_stream_index}, height {height}",
            index + 1
        );

        if index < 250 {
            continue;
        }

        if let Some(first_iteration) = first_appearances.get(&(shape_index, jet_stream_index)) {
            trace!("Last saw (shape_index, jet_stream_index) at {first_iteration}");

            cycle_start_index = cycle_start_index.min(*first_iteration);
        } else {
            first_appearances.insert((shape_index, jet_stream_index), index + 1);
        }
    }

    let end_of_cycle = first_appearances.values().max().unwrap();
    let length_of_cycle = end_of_cycle - cycle_start_index + 1;
    let warmup = cycle_start_index - 1;

    debug!("Cycle starts with rock {cycle_start_index}");
    debug!("Cycle ends after rock {end_of_cycle}");
    debug!("Cycle is {length_of_cycle} rocks long");
    debug!("Cycle starts after {warmup} rocks fallen");

    let number_of_cycles = (1_000_000_000_000 - warmup) / length_of_cycle;

    let height_before_start_of_cycle = heights_after_rocks_fallen[&(cycle_start_index - 1)];
    let height_after_end_of_cycle = heights_after_rocks_fallen[&(end_of_cycle)];

    debug!("Height at start of cycle: {}", height_before_start_of_cycle);
    debug!("Height at end of cycle: {}", height_after_end_of_cycle);

    let cycle_delta = height_after_end_of_cycle - height_before_start_of_cycle;
    debug!("Each cycle adds {cycle_delta} units of height");
    debug!("Number of cycles required: {number_of_cycles}");

    let warmup_plus_many_cycles = warmup + (number_of_cycles * length_of_cycle);

    debug!(
        "{number_of_cycles} cycles of {length_of_cycle} rocks plus {warmup} warmup is {warmup_plus_many_cycles} rocks fallen"
    );
    let rocks_remaining = 1_000_000_000_000 - warmup_plus_many_cycles;
    debug!("{rocks_remaining} rocks remaining for 1_000_000_000",);

    let warmup_height = heights_after_rocks_fallen[&warmup];
    let cycles_height = number_of_cycles as isize * cycle_delta;
    let height_partway_through_cycle =
        heights_after_rocks_fallen[&(cycle_start_index + rocks_remaining - 1)] - warmup_height;
    debug!("Height at {rocks_remaining} rocks into cycle is {height_partway_through_cycle}");

    let height = warmup_height + cycles_height + height_partway_through_cycle;
    debug!("Height after 1_000_000_000 rocks is {height}");

    let elapsed = start.elapsed().unwrap();

    PartAnswer::new(height, elapsed)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct TetrisGame {
    occupied_cells: HashSet<(isize, isize)>,
    highest_y: isize,
    next_shape_type: ShapeType,
    wind_directions: Vec<WindDirection>,
    current_wind_index: usize,
}

impl TetrisGame {
    fn new(wind_directions: Vec<WindDirection>) -> TetrisGame {
        let occupied_cells = HashSet::new();
        let highest_y = 0;
        let next_shape_type = ShapeType::initial();
        let current_wind_index = 0;

        TetrisGame {
            occupied_cells,
            highest_y,
            next_shape_type,
            wind_directions,
            current_wind_index,
        }
    }

    fn add_rock(&mut self) {
        debug!("Dropping {:?}", self.next_shape_type);
        let current_rock: Shape = self.next_shape_type.into();
        let mut current_rock = current_rock.with_delta((2, self.highest_y + 4));

        loop {
            let wind_direction = self.next_direction();

            debug!("-- wind is blowing {:?}", wind_direction);

            current_rock = self.blow_with_wind(&current_rock, wind_direction);

            if self.has_settled(&current_rock) {
                debug!("-- rock has settled");
                break;
            }

            debug!("-- rock is moving down");
            current_rock = current_rock.with_delta((0, -1));
        }

        self.occupied_cells.extend(current_rock.offsets);
        self.highest_y = self.occupied_cells.iter().map(|(_, y)| *y).max().unwrap();
        debug!("-- new highest y is {}", self.highest_y);
        self.next_shape_type = self.next_shape_type.next();
    }

    fn next_direction(&mut self) -> WindDirection {
        let wind_direction = self.wind_directions.get(self.current_wind_index).unwrap();

        self.current_wind_index = (self.current_wind_index + 1) % self.wind_directions.len();

        *wind_direction
    }

    fn blow_with_wind(&self, shape: &Shape, direction: WindDirection) -> Shape {
        let (min_x, max_x) = shape.min_max_x();

        if min_x == 0 && direction == WindDirection::Left {
            return shape.clone();
        }

        if max_x == 6 && direction == WindDirection::Right {
            return shape.clone();
        }

        match direction {
            WindDirection::Left => {
                if min_x == 0 {
                    shape.clone()
                } else {
                    let would_collide = shape
                        .offsets
                        .iter()
                        .map(|(x, y)| (*x - 1, *y))
                        .any(|cell| self.occupied_cells.contains(&cell));

                    if would_collide {
                        shape.clone()
                    } else {
                        shape.with_delta((-1, 0))
                    }
                }
            }
            WindDirection::Right => {
                if max_x == 6 {
                    shape.clone()
                } else {
                    let would_collide = shape
                        .offsets
                        .iter()
                        .map(|(x, y)| (*x + 1, *y))
                        .any(|cell| self.occupied_cells.contains(&cell));

                    if would_collide {
                        shape.clone()
                    } else {
                        shape.with_delta((1, 0))
                    }
                }
            }
        }
    }

    fn has_settled(&self, shape: &Shape) -> bool {
        if shape.offsets.iter().any(|(_, y)| *y == 1) {
            debug!("---- rock has settled at bottom");
            return true;
        }

        let settled = shape
            .offsets
            .iter()
            .map(|(x, y)| (*x, *y - 1))
            .any(|cell| self.occupied_cells.contains(&cell));

        debug!("---- settled - {settled}");

        settled
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum WindDirection {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum ShapeType {
    Horizontal,
    Cross,
    El,
    Vertical,
    Square,
}

impl ShapeType {
    fn initial() -> ShapeType {
        ShapeType::Horizontal
    }

    fn next(&self) -> ShapeType {
        match self {
            ShapeType::Horizontal => ShapeType::Cross,
            ShapeType::Cross => ShapeType::El,
            ShapeType::El => ShapeType::Vertical,
            ShapeType::Vertical => ShapeType::Square,
            ShapeType::Square => ShapeType::Horizontal,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Shape {
    offsets: Vec<(isize, isize)>,
    bottom_edge: Vec<(isize, isize)>,
    height: isize,
}

impl Shape {
    fn new(offsets: Vec<(isize, isize)>, bottom_edge: Vec<(isize, isize)>) -> Shape {
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;

        for (_, y) in &offsets {
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }

        let height = max_y - min_y;

        Shape {
            offsets,
            height,
            bottom_edge,
        }
    }

    fn with_delta(&self, delta: (isize, isize)) -> Shape {
        let (delta_x, delta_y) = delta;
        let offsets = self
            .offsets
            .iter()
            .map(|(x, y)| (*x + delta_x, *y + delta_y))
            .collect();

        Shape::new(offsets, self.bottom_edge.clone())
    }

    fn min_max_x(&self) -> (isize, isize) {
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;

        for (x, _) in &self.offsets {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
        }

        (min_x, max_x)
    }
}

impl From<ShapeType> for Shape {
    fn from(shape_type: ShapeType) -> Shape {
        match shape_type {
            ShapeType::Horizontal => Shape::new(
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            ),
            ShapeType::Vertical => Shape::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)], vec![(0, 0)]),
            ShapeType::Square => {
                Shape::new(vec![(0, 0), (1, 0), (0, 1), (1, 1)], vec![(0, 0), (1, 0)])
            }
            ShapeType::El => Shape::new(
                vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
                vec![(0, 0), (1, 0), (2, 0)],
            ),
            ShapeType::Cross => Shape::new(
                vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
                vec![(0, 1), (1, 0), (2, 1)],
            ),
        }
    }
}

fn parse(i: &str) -> Vec<WindDirection> {
    finish(wind_directions)(i).unwrap().1
}

fn wind_directions(i: &str) -> IResult<&str, Vec<WindDirection>> {
    many1(wind_direction)(i)
}

fn wind_direction(i: &str) -> IResult<&str, WindDirection> {
    alt((left, right))(i)
}

fn left(i: &str) -> IResult<&str, WindDirection> {
    value(WindDirection::Left, tag("<"))(i)
}

fn right(i: &str) -> IResult<&str, WindDirection> {
    value(WindDirection::Right, tag(">"))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_example() {
        let mut game = TetrisGame::new(parse(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"));

        game.add_rock();

        assert_eq!(
            game.occupied_cells,
            vec![(2, 1), (3, 1), (4, 1), (5, 1)].into_iter().collect()
        );

        game.add_rock();

        assert_eq!(
            game.occupied_cells,
            vec![
                (2, 1),
                (3, 1),
                (4, 1),
                (5, 1),
                (3, 2),
                (2, 3),
                (3, 3),
                (4, 3),
                (3, 4)
            ]
            .into_iter()
            .collect()
        );
    }
}
