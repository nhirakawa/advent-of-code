use std::fmt;
use std::fmt::Display;

pub struct PixelatedString {
    s: String,
}

impl PixelatedString {
    pub fn new<I>(i: I) -> PixelatedString
    where
        I: Into<String>,
    {
        let s = i.into();
        PixelatedString { s }
    }
}

impl Display for PixelatedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{}\n", self.s)
    }
}

#[macro_export]
macro_rules! make_part_fn {
    ($f:expr) => {
        |input: &str| $f(input).map(|result| result.to_string())
    };
}

#[macro_export]
macro_rules! advent_year {
    ($year:literal) => {
        $crate::advent_year!($year, []);
    };
    ($year:literal, [$($extra_mod:ident),*]) => {
        use crate::common::base::{Day, Part};
        use crate::make_part_fn;

        mod day_one;
        mod day_two;
        mod day_three;
        mod day_four;
        mod day_five;
        mod day_six;
        mod day_seven;
        mod day_eight;
        mod day_nine;
        mod day_ten;
        mod day_eleven;
        mod day_twelve;
        mod day_thirteen;
        mod day_fourteen;
        mod day_fifteen;
        mod day_sixteen;
        mod day_seventeen;
        mod day_eighteen;
        mod day_nineteen;
        mod day_twenty;
        mod day_twenty_one;
        mod day_twenty_two;
        mod day_twenty_three;
        mod day_twenty_four;
        mod day_twenty_five;
        
        $(pub mod $extra_mod;)*

        pub fn solution(day: Day, part: Part) -> Option<fn(&str) -> anyhow::Result<String>> {
            match (day, part) {
                (Day::Day1, Part::PartOne) => Some(make_part_fn!(day_one::part_one)),
                (Day::Day1, Part::PartTwo) => Some(make_part_fn!(day_one::part_two)),
                (Day::Day2, Part::PartOne) => Some(make_part_fn!(day_two::part_one)),
                (Day::Day2, Part::PartTwo) => Some(make_part_fn!(day_two::part_two)),
                (Day::Day3, Part::PartOne) => Some(make_part_fn!(day_three::part_one)),
                (Day::Day3, Part::PartTwo) => Some(make_part_fn!(day_three::part_two)),
                (Day::Day4, Part::PartOne) => Some(make_part_fn!(day_four::part_one)),
                (Day::Day4, Part::PartTwo) => Some(make_part_fn!(day_four::part_two)),
                (Day::Day5, Part::PartOne) => Some(make_part_fn!(day_five::part_one)),
                (Day::Day5, Part::PartTwo) => Some(make_part_fn!(day_five::part_two)),
                (Day::Day6, Part::PartOne) => Some(make_part_fn!(day_six::part_one)),
                (Day::Day6, Part::PartTwo) => Some(make_part_fn!(day_six::part_two)),
                (Day::Day7, Part::PartOne) => Some(make_part_fn!(day_seven::part_one)),
                (Day::Day7, Part::PartTwo) => Some(make_part_fn!(day_seven::part_two)),
                (Day::Day8, Part::PartOne) => Some(make_part_fn!(day_eight::part_one)),
                (Day::Day8, Part::PartTwo) => Some(make_part_fn!(day_eight::part_two)),
                (Day::Day9, Part::PartOne) => Some(make_part_fn!(day_nine::part_one)),
                (Day::Day9, Part::PartTwo) => Some(make_part_fn!(day_nine::part_two)),
                (Day::Day10, Part::PartOne) => Some(make_part_fn!(day_ten::part_one)),
                (Day::Day10, Part::PartTwo) => Some(make_part_fn!(day_ten::part_two)),
                (Day::Day11, Part::PartOne) => Some(make_part_fn!(day_eleven::part_one)),
                (Day::Day11, Part::PartTwo) => Some(make_part_fn!(day_eleven::part_two)),
                (Day::Day12, Part::PartOne) => Some(make_part_fn!(day_twelve::part_one)),
                (Day::Day12, Part::PartTwo) => Some(make_part_fn!(day_twelve::part_two)),
                (Day::Day13, Part::PartOne) => Some(make_part_fn!(day_thirteen::part_one)),
                (Day::Day13, Part::PartTwo) => Some(make_part_fn!(day_thirteen::part_two)),
                (Day::Day14, Part::PartOne) => Some(make_part_fn!(day_fourteen::part_one)),
                (Day::Day14, Part::PartTwo) => Some(make_part_fn!(day_fourteen::part_two)),
                (Day::Day15, Part::PartOne) => Some(make_part_fn!(day_fifteen::part_one)),
                (Day::Day15, Part::PartTwo) => Some(make_part_fn!(day_fifteen::part_two)),
                (Day::Day16, Part::PartOne) => Some(make_part_fn!(day_sixteen::part_one)),
                (Day::Day16, Part::PartTwo) => Some(make_part_fn!(day_sixteen::part_two)),
                (Day::Day17, Part::PartOne) => Some(make_part_fn!(day_seventeen::part_one)),
                (Day::Day17, Part::PartTwo) => Some(make_part_fn!(day_seventeen::part_two)),
                (Day::Day18, Part::PartOne) => Some(make_part_fn!(day_eighteen::part_one)),
                (Day::Day18, Part::PartTwo) => Some(make_part_fn!(day_eighteen::part_two)),
                (Day::Day19, Part::PartOne) => Some(make_part_fn!(day_nineteen::part_one)),
                (Day::Day19, Part::PartTwo) => Some(make_part_fn!(day_nineteen::part_two)),
                (Day::Day20, Part::PartOne) => Some(make_part_fn!(day_twenty::part_one)),
                (Day::Day20, Part::PartTwo) => Some(make_part_fn!(day_twenty::part_two)),
                (Day::Day21, Part::PartOne) => Some(make_part_fn!(day_twenty_one::part_one)),
                (Day::Day21, Part::PartTwo) => Some(make_part_fn!(day_twenty_one::part_two)),
                (Day::Day22, Part::PartOne) => Some(make_part_fn!(day_twenty_two::part_one)),
                (Day::Day22, Part::PartTwo) => Some(make_part_fn!(day_twenty_two::part_two)),
                (Day::Day23, Part::PartOne) => Some(make_part_fn!(day_twenty_three::part_one)),
                (Day::Day23, Part::PartTwo) => Some(make_part_fn!(day_twenty_three::part_two)),
                (Day::Day24, Part::PartOne) => Some(make_part_fn!(day_twenty_four::part_one)),
                (Day::Day24, Part::PartTwo) => Some(make_part_fn!(day_twenty_four::part_two)),
                (Day::Day25, Part::PartOne) => Some(make_part_fn!(day_twenty_five::part_one)),
                (Day::Day25, Part::PartTwo) => None,
            }
        }
    };
}

#[macro_export]
macro_rules! advent_year_12 {
    ($year:literal) => {
        $crate::advent_year_12!($year, []);
    };
    ($year:literal, [$($extra_mod:ident),*]) => {
        use crate::common::base::{Day, Part};
        use crate::make_part_fn;

        mod day_one;
        mod day_two;
        mod day_three;
        mod day_four;
        mod day_five;
        mod day_six;
        mod day_seven;
        mod day_eight;
        mod day_nine;
        mod day_ten;
        mod day_eleven;
        mod day_twelve;

        $(pub mod $extra_mod;)*

        pub fn solution(day: Day, part: Part) -> Option<fn(&str) -> anyhow::Result<String>> {
            match (day, part) {
                (Day::Day1, Part::PartOne) => Some(make_part_fn!(day_one::part_one)),
                (Day::Day1, Part::PartTwo) => Some(make_part_fn!(day_one::part_two)),
                (Day::Day2, Part::PartOne) => Some(make_part_fn!(day_two::part_one)),
                (Day::Day2, Part::PartTwo) => Some(make_part_fn!(day_two::part_two)),
                (Day::Day3, Part::PartOne) => Some(make_part_fn!(day_three::part_one)),
                (Day::Day3, Part::PartTwo) => Some(make_part_fn!(day_three::part_two)),
                (Day::Day4, Part::PartOne) => Some(make_part_fn!(day_four::part_one)),
                (Day::Day4, Part::PartTwo) => Some(make_part_fn!(day_four::part_two)),
                (Day::Day5, Part::PartOne) => Some(make_part_fn!(day_five::part_one)),
                (Day::Day5, Part::PartTwo) => Some(make_part_fn!(day_five::part_two)),
                (Day::Day6, Part::PartOne) => Some(make_part_fn!(day_six::part_one)),
                (Day::Day6, Part::PartTwo) => Some(make_part_fn!(day_six::part_two)),
                (Day::Day7, Part::PartOne) => Some(make_part_fn!(day_seven::part_one)),
                (Day::Day7, Part::PartTwo) => Some(make_part_fn!(day_seven::part_two)),
                (Day::Day8, Part::PartOne) => Some(make_part_fn!(day_eight::part_one)),
                (Day::Day8, Part::PartTwo) => Some(make_part_fn!(day_eight::part_two)),
                (Day::Day9, Part::PartOne) => Some(make_part_fn!(day_nine::part_one)),
                (Day::Day9, Part::PartTwo) => Some(make_part_fn!(day_nine::part_two)),
                (Day::Day10, Part::PartOne) => Some(make_part_fn!(day_ten::part_one)),
                (Day::Day10, Part::PartTwo) => Some(make_part_fn!(day_ten::part_two)),
                (Day::Day11, Part::PartOne) => Some(make_part_fn!(day_eleven::part_one)),
                (Day::Day11, Part::PartTwo) => Some(make_part_fn!(day_eleven::part_two)),
                (Day::Day12, Part::PartOne) => Some(make_part_fn!(day_twelve::part_one)),
                (Day::Day12, Part::PartTwo) => None,
                _ => None,
            }
        }
    };
}
