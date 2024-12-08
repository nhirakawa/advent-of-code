use crate::common::base::{Day, Part};

mod day_four;
mod day_one;
mod day_three;
mod day_two;

pub fn solution(day: Day, part: Part) -> Option<fn(&str) -> anyhow::Result<String>> {
    match (day, part) {
        (Day::Day1, Part::PartOne) => Some(day_one::part_one),
        (Day::Day1, Part::PartTwo) => Some(day_one::part_two),
        (Day::Day2, Part::PartOne) => Some(day_two::part_one),
        (Day::Day2, Part::PartTwo) => Some(day_two::part_two),
        (Day::Day3, Part::PartOne) => Some(day_three::part_one),
        (Day::Day3, Part::PartTwo) => Some(day_three::part_two),
        (Day::Day4, Part::PartOne) => Some(day_four::part_one),
        (Day::Day4, Part::PartTwo) => Some(day_four::part_two),
        _ => None,
    }
}
