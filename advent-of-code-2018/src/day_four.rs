use std::{cmp::Ordering, collections::HashMap};

use common::{parse::unsigned_number, prelude::*};

use multiset::HashMultiSet;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

pub fn run() -> AdventOfCodeResult {
    let pre_processing_start = SystemTime::now();
    let records = parse_and_sort_records(include_str!("../input/day-4.txt"));
    let minutes_slept_by_guard = get_minutes_slept_by_guard(&records);
    let pre_processing_elapsed = pre_processing_start.elapsed().unwrap();

    let part_one = part_one(&minutes_slept_by_guard, &pre_processing_elapsed);
    let part_two = part_two(&minutes_slept_by_guard, &pre_processing_elapsed);

    Ok((part_one, part_two))
}

fn part_one(
    minutes_slept_by_guard: &HashMap<usize, HashMultiSet<usize>>,
    pre_processing_start: &Duration,
) -> PartAnswer {
    let start = SystemTime::now();

    let (guard_id, minutes) = minutes_slept_by_guard
        .iter()
        .max_by_key(|(_, minutes)| total_minutes_slept(minutes))
        .unwrap();

    let solution = guard_id * highest_frequency(&minutes);

    PartAnswer::new(solution, start.elapsed().unwrap() + *pre_processing_start)
}

fn total_minutes_slept(minutes: &HashMultiSet<usize>) -> usize {
    let mut total = 0;
    for minute in minutes.distinct_elements() {
        total += minutes.count_of(minute);
    }
    total
}

fn highest_frequency(minutes: &HashMultiSet<usize>) -> usize {
    minutes
        .distinct_elements()
        .max_by_key(|minute| minutes.count_of(*minute))
        .cloned()
        .unwrap()
}

fn part_two(
    minutes_slept_by_guard: &HashMap<usize, HashMultiSet<usize>>,
    pre_processing_duration: &Duration,
) -> PartAnswer {
    let start = SystemTime::now();
    let (guard_id, minutes) = minutes_slept_by_guard
        .iter()
        .max_by_key(|(_, minutes)| highest_count(minutes))
        .unwrap();

    let most_slept_minute = minutes
        .distinct_elements()
        .max_by_key(|m| minutes.count_of(*m))
        .cloned()
        .unwrap();
    let solution = guard_id * most_slept_minute;

    PartAnswer::new(
        solution,
        start.elapsed().unwrap() + *pre_processing_duration,
    )
}

fn highest_count(minutes: &HashMultiSet<usize>) -> usize {
    if minutes.is_empty() {
        0
    } else {
        minutes
            .distinct_elements()
            .map(|m| minutes.count_of(m))
            .max()
            .unwrap()
    }
}

fn get_minutes_slept_by_guard(records: &[Record]) -> HashMap<usize, HashMultiSet<usize>> {
    let mut last_guard_id = 0;

    let mut records_by_guard = HashMap::new();

    for record in records {
        match record {
            Record::BeginShift {
                guard_id,
                timestamp: _,
            } => {
                last_guard_id = *guard_id;
                if !records_by_guard.contains_key(guard_id) {
                    records_by_guard.insert(*guard_id, Vec::new());
                }
            }
            _ => {
                if let Some(records) = records_by_guard.get_mut(&last_guard_id) {
                    records.push(record);
                }
            }
        }
    }

    let mut minutes_slept_by_guard = HashMap::new();

    for (guard_id, records) in records_by_guard.iter() {
        if records.is_empty() {
            minutes_slept_by_guard.insert(*guard_id, HashMultiSet::new());
        } else {
            let mut minutes_slept = HashMultiSet::new();
            for i in (0..records.len() - 1).step_by(2) {
                let first = records[i];
                let second = records[i + 1];

                for minute in first.get_timestamp().minute..second.get_timestamp().minute {
                    minutes_slept.insert(minute);
                }
            }
            minutes_slept_by_guard.insert(*guard_id, minutes_slept);
        }
    }

    minutes_slept_by_guard
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
struct Timestamp {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

#[derive(PartialEq, Eq, Debug)]
enum Record {
    BeginShift {
        guard_id: usize,
        timestamp: Timestamp,
    },
    FallAsleep {
        timestamp: Timestamp,
    },
    WakeUp {
        timestamp: Timestamp,
    },
}

impl Record {
    fn get_timestamp(&self) -> &Timestamp {
        match self {
            Record::BeginShift {
                guard_id: _,
                timestamp,
            } => timestamp,
            Record::FallAsleep { timestamp } => timestamp,
            Record::WakeUp { timestamp } => timestamp,
        }
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_timestamp().cmp(&other.get_timestamp())
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_and_sort_records(i: &str) -> Vec<Record> {
    let mut records = all_consuming(records)(i).unwrap().1;

    records.sort();

    records
}

fn records(i: &str) -> IResult<&str, Vec<Record>> {
    terminated(separated_list1(tag("\n"), record), tag("\n"))(i)
}

fn record(i: &str) -> IResult<&str, Record> {
    alt((start_shift, falls_asleep, wakes_up))(i)
}

fn start_shift(i: &str) -> IResult<&str, Record> {
    let guard_id = preceded(tag("Guard #"), unsigned_number);

    map(
        tuple((timestamp, tag(" "), guard_id, tag(" "), tag("begins shift"))),
        |(timestamp, _, guard_id, _, _)| Record::BeginShift {
            guard_id,
            timestamp,
        },
    )(i)
}

fn falls_asleep(i: &str) -> IResult<&str, Record> {
    map(
        separated_pair(timestamp, tag(" "), tag("falls asleep")),
        |(timestamp, _)| Record::FallAsleep { timestamp },
    )(i)
}

fn wakes_up(i: &str) -> IResult<&str, Record> {
    map(
        separated_pair(timestamp, tag(" "), tag("wakes up")),
        |(timestamp, _)| Record::WakeUp { timestamp },
    )(i)
}

fn timestamp(i: &str) -> IResult<&str, Timestamp> {
    map(
        delimited(tag("["), separated_pair(date, tag(" "), time), tag("]")),
        |((year, month, day), (hour, minute))| Timestamp {
            year,
            month,
            day,
            hour,
            minute,
        },
    )(i)
}

fn date(i: &str) -> IResult<&str, (usize, usize, usize)> {
    map(
        tuple((
            unsigned_number,
            tag("-"),
            unsigned_number,
            tag("-"),
            unsigned_number,
        )),
        |(year, _, month, _, day)| (year, month, day),
    )(i)
}

fn time(i: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(unsigned_number, tag(":"), unsigned_number)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_shift() {
        assert_eq!(
            start_shift("[1518-11-01 00:00] Guard #10 begins shift"),
            Ok((
                "",
                Record::BeginShift {
                    timestamp: Timestamp {
                        year: 1518,
                        month: 11,
                        day: 1,
                        hour: 0,
                        minute: 0
                    },
                    guard_id: 10
                }
            ))
        )
    }

    #[test]
    fn test_falls_asleep() {
        assert_eq!(
            falls_asleep("[1518-11-01 00:05] falls asleep"),
            Ok((
                "",
                Record::FallAsleep {
                    timestamp: Timestamp {
                        year: 1518,
                        month: 11,
                        day: 1,
                        hour: 0,
                        minute: 5
                    }
                }
            ))
        )
    }

    #[test]
    fn test_wakes_up() {
        assert_eq!(
            wakes_up("[1518-11-01 00:25] wakes up"),
            Ok((
                "",
                Record::WakeUp {
                    timestamp: Timestamp {
                        year: 1518,
                        month: 11,
                        day: 1,
                        hour: 0,
                        minute: 25
                    }
                }
            ))
        )
    }

    #[test]
    fn test_timestamp() {
        assert_eq!(
            timestamp("[1518-11-01 00:30]"),
            Ok((
                "",
                Timestamp {
                    year: 1518,
                    month: 11,
                    day: 1,
                    hour: 0,
                    minute: 30
                }
            ))
        )
    }

    #[test]
    fn test_date() {
        assert_eq!(date("1518-11-01"), Ok(("", (1518, 11, 1))))
    }

    #[test]
    fn test_time() {
        assert_eq!(time("23:58"), Ok(("", (23, 58))))
    }
}
