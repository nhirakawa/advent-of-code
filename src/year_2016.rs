use crate::common::base::{Day, Part};

mod day_one;

pub fn solution(day: Day, part: Part) -> Option<fn(&str) -> anyhow::Result<String>> {
    match (day, part) {
        (Day::Day1, Part::PartOne) => Some(day_one::part_one),
        (Day::Day1, Part::PartTwo) => Some(day_one::part_two),
        _ => None,
    }
}
