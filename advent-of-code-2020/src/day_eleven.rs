use common::prelude::*;
use std::collections::HashMap;

type Coordinate = (i32, i32);

#[derive(Debug, PartialEq, Copy, Clone)]
enum PositionType {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-11.txt");
    let layout = parse_layout(input);

    let part_one = part_one(&layout);
    let part_two = part_two(&layout);

    Ok((part_one, part_two))
}

fn part_one(layout: &HashMap<Coordinate, PositionType>) -> PartAnswer {
    let start = SystemTime::now();

    let stabilized = run_until_stabilized(layout, 4, part_one_inner);

    let (_, stabilized) = stabilized;

    let answer = count_occupied_seats(&stabilized);

    let elapsed = start.elapsed().unwrap();

    (answer, elapsed).into()
}

fn part_one_inner<'a>(
    coordinate: &'a Coordinate,
    layout: &'a HashMap<Coordinate, PositionType>,
) -> Vec<PositionType> {
    let immediate_neighbors = vec![
        layout.get(&(coordinate.0 + 1, coordinate.1)), // right
        layout.get(&(coordinate.0 - 1, coordinate.1)), // left
        layout.get(&(coordinate.0 + 1, coordinate.1 + 1)), // upper right
        layout.get(&(coordinate.0 - 1, coordinate.1 + 1)), // upper left
        layout.get(&(coordinate.0 + 1, coordinate.1 - 1)), // lower right
        layout.get(&(coordinate.0 - 1, coordinate.1 - 1)), // lower left
        layout.get(&(coordinate.0, coordinate.1 + 1)), // upper
        layout.get(&(coordinate.0, coordinate.1 - 1)), // lower
    ];

    immediate_neighbors.into_iter().flatten().copied().collect()
}

fn part_two(layout: &HashMap<Coordinate, PositionType>) -> PartAnswer {
    let start = SystemTime::now();

    let (_, answer) = run_until_stabilized(layout, 5, part_two_inner);
    let answer = count_occupied_seats(&answer);

    let elapsed = start.elapsed().unwrap();

    (answer, elapsed).into()
}

fn part_two_inner(
    coordinate: &Coordinate,
    layout: &HashMap<Coordinate, PositionType>,
) -> Vec<PositionType> {
    let mut positions = Vec::new();

    let deltas = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (0, -1),
    ];

    for delta in deltas {
        let first_seat_in_line = get_first_seat_in_line(coordinate, layout, delta);

        positions.push(first_seat_in_line);
    }

    positions.into_iter().flatten().collect()
}

fn get_first_seat_in_line(
    coordinate: &Coordinate,
    layout: &HashMap<Coordinate, PositionType>,
    delta: (i32, i32),
) -> Option<PositionType> {
    let current = (coordinate.0 + delta.0, coordinate.1 + delta.1);

    return match layout.get(&current) {
        None => None,
        Some(PositionType::Floor) => get_first_seat_in_line(&current, layout, delta),
        Some(position_type) => Some(*position_type),
    };
}

fn count_occupied_seats(layout: &HashMap<Coordinate, PositionType>) -> u64 {
    let mut counter = 0;

    for position_type in layout.values() {
        counter += match position_type {
            PositionType::OccupiedSeat => 1,
            _ => 0,
        }
    }

    counter
}

fn run_until_stabilized(
    layout: &HashMap<Coordinate, PositionType>,
    occupied_seats_to_flip: u32,
    f: fn(&Coordinate, &HashMap<Coordinate, PositionType>) -> Vec<PositionType>,
) -> (u32, HashMap<Coordinate, PositionType>) {
    let mut before = layout.clone();
    let mut counter = 0;

    loop {
        counter += 1;
        let after = run_one_iteration(&before, occupied_seats_to_flip, f);

        if after == before {
            return (counter, after);
        } else {
            before = after;
        }
    }
}

fn run_one_iteration<'a, F>(
    layout: &'a HashMap<Coordinate, PositionType>,
    occupied_seats_to_flip: u32,
    f: F,
) -> HashMap<Coordinate, PositionType>
where
    F: Fn(&'a Coordinate, &'a HashMap<Coordinate, PositionType>) -> Vec<PositionType>,
{
    let mut after = HashMap::new();

    for (coordinate, position_type) in layout {
        let neighbors = f(coordinate, layout);

        let next_position_type =
            get_new_position(position_type, &neighbors, occupied_seats_to_flip);

        after.insert(*coordinate, next_position_type);
    }

    after
}

fn get_new_position(
    position_type: &PositionType,
    neighbors: &[PositionType],
    occupied_seats_to_flip: u32,
) -> PositionType {
    if *position_type == PositionType::Floor {
        return PositionType::Floor;
    }

    let mut occupied_seats = 0;

    for neighbor in neighbors {
        if *neighbor == PositionType::OccupiedSeat {
            occupied_seats += 1;
        }
    }

    if *position_type == PositionType::OccupiedSeat && occupied_seats >= occupied_seats_to_flip {
        PositionType::EmptySeat
    } else if *position_type == PositionType::EmptySeat && occupied_seats == 0 {
        PositionType::OccupiedSeat
    } else {
        *position_type
    }
}

fn parse_layout(input: &str) -> HashMap<Coordinate, PositionType> {
    let mut result = HashMap::new();
    for (y, line) in input.split('\n').enumerate() {
        for (x, position) in line.chars().enumerate() {
            let position_type = match position {
                '#' => Some(PositionType::OccupiedSeat),
                'L' => Some(PositionType::EmptySeat),
                '.' => Some(PositionType::Floor),
                _ => None,
            };

            match position_type {
                Some(position_type) => result.insert((x as i32, y as i32), position_type),
                _ => None,
            };
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_new_position() {
        let new_position = get_new_position(
            &PositionType::OccupiedSeat,
            &[PositionType::EmptySeat, PositionType::EmptySeat],
            2,
        );

        assert_eq!(new_position, PositionType::OccupiedSeat);

        let new_position = get_new_position(
            &PositionType::OccupiedSeat,
            &[PositionType::OccupiedSeat, PositionType::EmptySeat],
            2,
        );

        assert_eq!(new_position, PositionType::OccupiedSeat);

        let new_position = get_new_position(
            &PositionType::OccupiedSeat,
            &[PositionType::OccupiedSeat, PositionType::EmptySeat],
            1,
        );

        assert_eq!(new_position, PositionType::EmptySeat);
    }

    #[test]
    fn test_run_one_iteration() {
        let start = parse_layout("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL");

        let next = run_one_iteration(&start, 4, part_one_inner);

        let expected = parse_layout("#.##.##.##\n#######.##\n#.#.#..#..\n####.##.##\n#.##.##.##\n#.#####.##\n..#.#.....\n##########\n#.######.#\n#.#####.##");

        assert_eq!(next, expected);

        let next = run_one_iteration(&next, 4, part_one_inner);

        let expected = parse_layout("#.LL.L#.##\n#LLLLLL.L#\nL.L.L..L..\n#LLL.LL.L#\n#.LL.LL.LL\n#.LLLL#.##\n..L.L.....\n#LLLLLLLL#\n#.LLLLLL.L\n#.#LLLL.##");

        assert_eq!(next, expected);
    }

    #[test]
    fn test_part_one() {
        let layout = parse_layout("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL");

        let (_, stabilized) = run_until_stabilized(&layout, 4, part_one_inner);

        let expected = parse_layout("#.#L.L#.##\n#LLL#LL.L#\nL.#.L..#..\n#L##.##.L#\n#.#L.LL.LL\n#.#L#L#.##\n..L.L.....\n#L#L##L#L#\n#.LLLLLL.L\n#.#L#L#.##");

        assert_eq!(stabilized, expected);
    }

    #[test]
    fn test_iterations() {
        let layouts = vec![
            parse_layout("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL"),
            parse_layout("#.##.##.##\n#######.##\n#.#.#..#..\n####.##.##\n#.##.##.##\n#.#####.##\n..#.#.....\n##########\n#.######.#\n#.#####.##"),
            parse_layout("#.LL.L#.##\n#LLLLLL.L#\nL.L.L..L..\n#LLL.LL.L#\n#.LL.LL.LL\n#.LLLL#.##\n..L.L.....\n#LLLLLLLL#\n#.LLLLLL.L\n#.#LLLL.##"),
            parse_layout("#.##.L#.##\n#L###LL.L#\nL.#.#..#..\n#L##.##.L#\n#.##.LL.LL\n#.###L#.##\n..#.#.....\n#L######L#\n#.LL###L.L\n#.#L###.##"),
            parse_layout("#.#L.L#.##\n#LLL#LL.L#\nL.L.L..#..\n#LLL.##.L#\n#.LL.LL.LL\n#.LL#L#.##\n..L.L.....\n#L#LLLL#L#\n#.LLLLLL.L\n#.#L#L#.##"),
            parse_layout("#.#L.L#.##\n#LLL#LL.L#\nL.#.L..#..\n#L##.##.L#\n#.#L.LL.LL\n#.#L#L#.##\n..L.L.....\n#L#L##L#L#\n#.LLLLLL.L\n#.#L#L#.##"),
            parse_layout("#.#L.L#.##\n#LLL#LL.L#\nL.#.L..#..\n#L##.##.L#\n#.#L.LL.LL\n#.#L#L#.##\n..L.L.....\n#L#L##L#L#\n#.LLLLLL.L\n#.#L#L#.##")
        ];

        for window in layouts.windows(2) {
            let before = &window[0];
            let expected_after = &window[1];

            let actual_after = run_one_iteration(&before, 4, part_one_inner);

            assert_eq!(&actual_after, expected_after);
        }
    }

    #[test]
    fn test_count_occupied_seats() {
        let layout = parse_layout("#.#L.L#.##\n#LLL#LL.L#\nL.#.L..#..\n#L##.##.L#\n#.#L.LL.LL\n#.#L#L#.##\n..L.L.....\n#L#L##L#L#\n#.LLLLLL.L\n#.#L#L#.##");

        let count = count_occupied_seats(&layout);

        assert_eq!(count, 37);
    }

    #[test]
    fn test_get_first_seat_in_line() {
        let layout = parse_layout(".......#.\n...#.....\n.#.......\n.........\n..#L....#\n....#....\n.........\n#........\n...#.....");
        let first_seat = get_first_seat_in_line(&(3, 4), &layout, (1, -1));
        assert_eq!(first_seat, Some(PositionType::OccupiedSeat));

        let layout = parse_layout(".............\n.L.L.#.#.#.#.\n.............");
        let first_seat = get_first_seat_in_line(&(1, 1), &layout, (1, 0));
        assert_eq!(first_seat, Some(PositionType::EmptySeat));

        let first_seat = get_first_seat_in_line(&(1, 1), &layout, (0, 1));
        assert_eq!(first_seat, None);

        let layout = parse_layout(".##.##.\n#.#.#.#\n##...##\n...L...\n##...##\n#.#.#.#\n.##.##.");
        let first_seat = get_first_seat_in_line(&(4, 4), &layout, (1, 1));
        assert_eq!(first_seat, None);
    }
}
