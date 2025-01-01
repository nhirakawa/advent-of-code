use crate::common::base::{Day, Part};

mod day_eight;
mod day_eighteen;
mod day_eleven;
mod day_fifteen;
mod day_five;
mod day_four;
mod day_fourteen;
mod day_nine;
mod day_nineteen;
mod day_one;
mod day_seven;
mod day_seventeen;
mod day_six;
mod day_sixteen;
mod day_ten;
mod day_thirteen;
mod day_three;
mod day_twelve;
mod day_twenty;
mod day_twenty_one;
mod day_twenty_three;
mod day_twenty_two;
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
        (Day::Day10, Part::PartOne) => Some(day_ten::part_one),
        (Day::Day10, Part::PartTwo) => Some(day_ten::part_two),
        (Day::Day11, Part::PartOne) => Some(day_eleven::part_one),
        (Day::Day11, Part::PartTwo) => Some(day_eleven::part_two),
        (Day::Day12, Part::PartOne) => Some(day_twelve::part_one),
        (Day::Day12, Part::PartTwo) => Some(day_twelve::part_two),
        (Day::Day13, Part::PartOne) => Some(day_thirteen::part_one),
        (Day::Day13, Part::PartTwo) => Some(day_thirteen::part_two),
        (Day::Day14, Part::PartOne) => Some(day_fourteen::part_one),
        (Day::Day14, Part::PartTwo) => Some(day_fourteen::part_two),
        (Day::Day15, Part::PartOne) => Some(day_fifteen::part_one),
        (Day::Day15, Part::PartTwo) => Some(day_fifteen::part_two),
        (Day::Day16, Part::PartOne) => Some(day_sixteen::part_one),
        (Day::Day16, Part::PartTwo) => Some(day_sixteen::part_two),
        (Day::Day17, Part::PartOne) => Some(day_seventeen::part_one),
        (Day::Day17, Part::PartTwo) => Some(day_seventeen::part_two),
        (Day::Day18, Part::PartOne) => Some(day_eighteen::part_one),
        (Day::Day18, Part::PartTwo) => Some(day_eighteen::part_two),
        (Day::Day19, Part::PartOne) => Some(day_nineteen::part_one),
        (Day::Day19, Part::PartTwo) => Some(day_nineteen::part_two),
        (Day::Day20, Part::PartOne) => Some(day_twenty::part_one),
        (Day::Day20, Part::PartTwo) => Some(day_twenty::part_two),
        (Day::Day21, Part::PartOne) => Some(day_twenty_one::part_one),
        (Day::Day21, Part::PartTwo) => Some(day_twenty_one::part_two),
        (Day::Day22, Part::PartOne) => Some(day_twenty_two::part_one),
        (Day::Day22, Part::PartTwo) => Some(day_twenty_two::part_two),
        (Day::Day23, Part::PartOne) => Some(day_twenty_three::part_one),
        (Day::Day23, Part::PartTwo) => Some(day_twenty_three::part_two),
        _ => None,
    }
}
