use crate::prelude::*;

pub fn run() -> AdventOfCodeResult {
    let input = include_str!("../input/day-13.txt");
    let (timestamp, bus_schedule) = parse_bus_schedule(input);

    let part_one = part_one(timestamp, &bus_schedule);
    let part_two = part_two(&bus_schedule);

    Ok((part_one, part_two))
}

fn part_one(timestamp: u64, bus_schedule: &Vec<BusTiming>) -> PartAnswer {
    let start = SystemTime::now();
    let mut min_bus_wait = u64::MAX;
    let mut min_bus_id = u64::MAX;

    for bus_timing in bus_schedule {
        let time_since_last_stop = timestamp % bus_timing.id as u64;
        let time_until_next_stop = bus_timing.id as u64 - time_since_last_stop;

        if time_until_next_stop < min_bus_wait {
            min_bus_wait = time_until_next_stop;
            min_bus_id = bus_timing.id as u64;
        }
    }

    let elapsed = start.elapsed().unwrap();

    Ok((min_bus_wait * min_bus_id, elapsed))
}

fn part_two(bus_schedule: &Vec<BusTiming>) -> PartAnswer {
    let start = SystemTime::now();
    let solution = solve_congruences(bus_schedule);
    let elapsed = start.elapsed().unwrap();

    Ok((solution, elapsed))
}

// uses Lagrange interpolation
fn solve_congruences(schedule: &Vec<BusTiming>) -> u64 {
    let product_of_all: i64 = schedule.into_iter().map(|b| b.id).product();

    let product_of_all_except_self: Vec<i64> = schedule
        .into_iter()
        .map(|b| product_of_all / b.id)
        .collect();

    let mut sum = 0;

    for i in 0..schedule.len() {
        let bus = &schedule[i];

        let n_i = product_of_all_except_self[i];
        let n = bus.id;

        let (m_i, _) = bezout_coefficients(n_i, n);
        sum += bus.index as i64 * m_i * n_i;
    }

    while sum < 0 {
        sum += product_of_all;
    }

    while sum > 0 {
        sum -= product_of_all
    }

    sum += product_of_all;

    (product_of_all - sum) as u64
}

fn bezout_coefficients(a: i64, b: i64) -> (i64, i64) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let quotient = old_r / r;
        let temp = r;
        r = old_r - (quotient * r);
        old_r = temp;

        let temp = s;
        s = old_s - (quotient * s);
        old_s = temp;

        let temp = t;
        t = old_t - (quotient * t);
        old_t = temp;
    }

    (old_s, old_t)
}

#[derive(Debug, PartialEq)]
struct BusTiming {
    id: i64,      // n
    index: usize, // a
}

fn parse_bus_schedule(s: &str) -> (u64, Vec<BusTiming>) {
    let mut lines = s.split("\n");
    let timestamp = lines
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let bus_timings = lines.next().unwrap_or("");
    let mut bus_timings: Vec<BusTiming> = bus_timings
        .split(",")
        .enumerate()
        .map(|(index, raw_id)| raw_id.parse::<i64>().map(|id| (index, id)).ok())
        .flatten()
        .map(|(index, id)| BusTiming { id, index })
        .collect();

    sort_bus_schedule(&mut bus_timings);

    (timestamp, bus_timings)
}

fn sort_bus_schedule(schedule: &mut Vec<BusTiming>) {
    schedule.sort_by_key(|bus| bus.id);
    schedule.reverse();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bezout_coefficients() {
        assert_eq!(bezout_coefficients(3, 4), (-1, 1));
        assert_eq!(bezout_coefficients(5, 12), (5, -2));
    }

    #[test]
    fn test_solve_congruences() {
        let buses = vec![
            BusTiming { id: 17, index: 0 },
            BusTiming { id: 13, index: 2 },
            BusTiming { id: 19, index: 3 },
        ];

        let solution = solve_congruences(&buses);
        assert_eq!(solution, 3417);

        let buses = vec![
            BusTiming { id: 67, index: 0 },
            BusTiming { id: 7, index: 1 },
            BusTiming { id: 59, index: 2 },
            BusTiming { id: 61, index: 3 },
        ];

        let solution = solve_congruences(&buses);
        assert_eq!(solution, 754018);

        let buses = vec![
            BusTiming { id: 67, index: 0 },
            BusTiming { id: 7, index: 2 },
            BusTiming { id: 59, index: 3 },
            BusTiming { id: 61, index: 4 },
        ];

        let solution = solve_congruences(&buses);
        assert_eq!(solution, 779210);

        let buses = vec![
            BusTiming { id: 67, index: 0 },
            BusTiming { id: 7, index: 1 },
            BusTiming { id: 59, index: 3 },
            BusTiming { id: 61, index: 4 },
        ];

        let solution = solve_congruences(&buses);
        assert_eq!(solution, 1261476);

        let buses = vec![
            BusTiming { id: 1789, index: 0 },
            BusTiming { id: 37, index: 1 },
            BusTiming { id: 47, index: 2 },
            BusTiming { id: 1889, index: 3 },
        ];

        let solution = solve_congruences(&buses);
        assert_eq!(solution, 1202161486);
    }
}
