use crate::common::base::{Day, Part};

extern crate nom;

pub mod day_eight;
pub mod day_eighteen;
pub mod day_eleven;
pub mod day_fifteen;
pub mod day_five;
pub mod day_four;
pub mod day_fourteen;
pub mod day_nine;
pub mod day_nineteen;
pub mod day_one;
pub mod day_seven;
pub mod day_seventeen;
pub mod day_six;
pub mod day_sixteen;
pub mod day_ten;
pub mod day_thirteen;
pub mod day_three;
pub mod day_twelve;
pub mod day_twenty;
pub mod day_twenty_five;
pub mod day_twenty_four;
pub mod day_twenty_one;
pub mod day_twenty_three;
pub mod day_twenty_two;
pub mod day_two;

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
        (Day::Day24, Part::PartOne) => Some(day_twenty_four::part_one),
        (Day::Day24, Part::PartTwo) => Some(day_twenty_four::part_two),
        (Day::Day25, Part::PartOne) => Some(day_twenty_five::part_one),
        _ => None,
    }
}
