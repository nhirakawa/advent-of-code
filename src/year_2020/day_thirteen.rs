use crate::common::math::{self, Congruence};

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let (timestamp, bus_schedule) = parse_bus_schedule(input);

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

    Ok(min_bus_wait * min_bus_id)
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let (_, bus_schedule) = parse_bus_schedule(input);
    Ok(solve_congruences(&bus_schedule))
}

fn solve_congruences(schedule: &[BusTiming]) -> u64 {
    // bus i departs at t + index, so t ≡ -index (mod id)
    let congruences = schedule.iter().map(|bus| Congruence {
        residue: -(bus.index as i64),
        modulus: bus.id,
    });

    math::chinese_remainder(congruences).expect("bus IDs should be pairwise coprime and their product should fit in an i64") as u64
}

#[derive(Debug, PartialEq)]
struct BusTiming {
    id: i64,      // n
    index: usize, // a
}

fn parse_bus_schedule(s: &str) -> (u64, Vec<BusTiming>) {
    let mut lines = s.split('\n');
    let timestamp = lines
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let bus_timings = lines.next().unwrap_or("");
    let mut bus_timings: Vec<BusTiming> = bus_timings
        .split(',')
        .enumerate()
        .filter_map(|(index, raw_id)| raw_id.parse::<i64>().map(|id| (index, id)).ok())
        .map(|(index, id)| BusTiming { id, index })
        .collect();

    sort_bus_schedule(&mut bus_timings);

    (timestamp, bus_timings)
}

fn sort_bus_schedule(schedule: &mut [BusTiming]) {
    schedule.sort_by_key(|bus| bus.id);
    schedule.reverse();
}

#[cfg(test)]
mod test {
    use super::*;

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
