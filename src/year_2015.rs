use crate::common::base::{Day, Part};

mod day_eight;
mod day_five;
mod day_four;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
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
        (Day::Day5, Part::PartOne) => Some(day_five::part_one),
        (Day::Day5, Part::PartTwo) => Some(day_five::part_two),
        (Day::Day6, Part::PartOne) => Some(day_six::part_one),
        (Day::Day6, Part::PartTwo) => Some(day_six::part_two),
        (Day::Day7, Part::PartOne) => Some(day_seven::part_one),
        (Day::Day7, Part::PartTwo) => Some(day_seven::part_two),
        (Day::Day8, Part::PartOne) => Some(day_eight::part_one),
        (Day::Day8, Part::PartTwo) => Some(day_eight::part_two),
        (Day::Day9, Part::PartOne) => Some(day_nine::part_one),
        (Day::Day9, Part::PartTwo) => Some(day_nine::part_two),
        _ => None,
    }
}
