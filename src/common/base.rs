use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum Year {
    Year2015,
    Year2016,
    Year2017,
    Year2018,
    Year2019,
    Year2020,
    Year2021,
    Year2022,
}

impl Year {
    pub fn iter() -> impl DoubleEndedIterator<Item = Year> {
        [
            Year::Year2015,
            Year::Year2016,
            Year::Year2017,
            Year::Year2018,
            Year::Year2019,
            Year::Year2020,
            Year::Year2021,
            Year::Year2022,
        ]
        .iter()
        .copied()
    }

    pub fn as_u32(&self) -> u32 {
        match self {
            Year::Year2015 => 2015,
            Year::Year2016 => 2016,
            Year::Year2017 => 2017,
            Year::Year2018 => 2018,
            Year::Year2019 => 2019,
            Year::Year2020 => 2020,
            Year::Year2021 => 2021,
            Year::Year2022 => 2022,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Year::Year2015 => "2015",
            Year::Year2016 => "2016",
            Year::Year2017 => "2017",
            Year::Year2018 => "2018",
            Year::Year2019 => "2019",
            Year::Year2020 => "2020",
            Year::Year2021 => "2021",
            Year::Year2022 => "2022",
        }
    }
}

impl FromStr for Year {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2015" => Ok(Year::Year2015),
            "2016" => Ok(Year::Year2016),
            "2017" => Ok(Year::Year2017),
            "2018" => Ok(Year::Year2018),
            "2019" => Ok(Year::Year2019),
            "2020" => Ok(Year::Year2020),
            "2021" => Ok(Year::Year2021),
            "2022" => Ok(Year::Year2022),
            _ => Err(anyhow::anyhow!("Invalid year {s}")),
        }
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u32())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl Day {
    pub fn as_u8(&self) -> u8 {
        match self {
            Day::Day1 => 1,
            Day::Day2 => 2,
            Day::Day3 => 3,
            Day::Day4 => 4,
            Day::Day5 => 5,
            Day::Day6 => 6,
            Day::Day7 => 7,
            Day::Day8 => 8,
            Day::Day9 => 9,
            Day::Day10 => 10,
            Day::Day11 => 11,
            Day::Day12 => 12,
            Day::Day13 => 13,
            Day::Day14 => 14,
            Day::Day15 => 15,
            Day::Day16 => 16,
            Day::Day17 => 17,
            Day::Day18 => 18,
            Day::Day19 => 19,
            Day::Day20 => 20,
            Day::Day21 => 21,
            Day::Day22 => 22,
            Day::Day23 => 23,
            Day::Day24 => 24,
            Day::Day25 => 25,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Day::Day1 => "1",
            Day::Day2 => "2",
            Day::Day3 => "3",
            Day::Day4 => "4",
            Day::Day5 => "5",
            Day::Day6 => "6",
            Day::Day7 => "7",
            Day::Day8 => "8",
            Day::Day9 => "9",
            Day::Day10 => "10",
            Day::Day11 => "11",
            Day::Day12 => "12",
            Day::Day13 => "13",
            Day::Day14 => "14",
            Day::Day15 => "15",
            Day::Day16 => "16",
            Day::Day17 => "17",
            Day::Day18 => "18",
            Day::Day19 => "19",
            Day::Day20 => "20",
            Day::Day21 => "21",
            Day::Day22 => "22",
            Day::Day23 => "23",
            Day::Day24 => "24",
            Day::Day25 => "25",
        }
    }

    pub fn iter() -> impl DoubleEndedIterator<Item = Day> {
        [
            Day::Day1,
            Day::Day2,
            Day::Day3,
            Day::Day4,
            Day::Day5,
            Day::Day6,
            Day::Day7,
            Day::Day8,
            Day::Day9,
            Day::Day10,
            Day::Day11,
            Day::Day12,
            Day::Day13,
            Day::Day14,
            Day::Day15,
            Day::Day16,
            Day::Day17,
            Day::Day18,
            Day::Day19,
            Day::Day20,
            Day::Day21,
            Day::Day22,
            Day::Day23,
            Day::Day24,
            Day::Day25,
        ]
        .iter()
        .copied()
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Day::Day1),
            "2" => Ok(Day::Day2),
            "3" => Ok(Day::Day3),
            "4" => Ok(Day::Day4),
            "5" => Ok(Day::Day5),
            "6" => Ok(Day::Day6),
            "7" => Ok(Day::Day7),
            "8" => Ok(Day::Day8),
            "9" => Ok(Day::Day9),
            "10" => Ok(Day::Day10),
            "11" => Ok(Day::Day11),
            "12" => Ok(Day::Day12),
            "13" => Ok(Day::Day13),
            "14" => Ok(Day::Day14),
            "15" => Ok(Day::Day15),
            "16" => Ok(Day::Day16),
            "17" => Ok(Day::Day17),
            "18" => Ok(Day::Day18),
            "19" => Ok(Day::Day19),
            "20" => Ok(Day::Day20),
            "21" => Ok(Day::Day21),
            "22" => Ok(Day::Day22),
            "23" => Ok(Day::Day23),
            "24" => Ok(Day::Day24),
            "25" => Ok(Day::Day25),
            _ => Err(anyhow::anyhow!("Invalid day {s}")),
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u8())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Part {
    PartOne,
    PartTwo,
}

// pub struct Solution {
//     pub year: Year,
//     pub day: Day,
//     pub parts: Parts,
// }

// impl Solution {
//     pub fn new(year: Year, day: Day, parts: Parts) -> Self {
//         Self { year, day, parts }
//     }

//     pub fn year(&self) -> &Year {
//         &self.year
//     }

//     pub fn day(&self) -> &Day {
//         &self.day
//     }
// }
