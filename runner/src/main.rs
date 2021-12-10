extern crate clap;

use clap::{App, Arg};
use common::prelude::*;
use common::result_logger::log_result;
use env_logger::Env;

fn main() -> Result<(), AdventOfCodeError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    let matches = App::new("Advent of Code")
        .version("0.1.0")
        .author("Nick Hirakawa <nickhirakawa@gmail.com>")
        .about("Advent of Code solutions")
        .arg(
            Arg::with_name("year")
                .index(1)
                .possible_values(&["2021", "2020", "2019", "2018"]),
        )
        .arg(
            Arg::with_name("day")
                .index(2)
                .possible_values(&[
                    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14",
                    "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25",
                ])
                .requires("year"),
        )
        .get_matches();

    if let Some(year) = matches.value_of("year").and_then(|s| s.parse::<u32>().ok()) {
        if let Some(day) = matches.value_of("day").and_then(|s| s.parse::<u8>().ok()) {
            match year {
                2021 => advent_of_code_2021::run_day(day)?,
                2020 => advent_of_code_2020::run_day(day)?,
                2019 => run_2019_day(day)?,
                2018 => advent_of_code_2018::run_day(day)?,
                _ => panic!(),
            }
        } else {
            match year {
                2021 => advent_of_code_2021::run_all()?,
                2020 => advent_of_code_2020::run_all()?,
                2019 => run_2019()?,
                2018 => advent_of_code_2018::run_all()?,
                _ => panic!(),
            }
        }
    } else {
        advent_of_code_2021::run_all()?;
        advent_of_code_2020::run_all()?;
        run_2019()?;
        advent_of_code_2018::run_all()?;
    }

    Ok(())
}

fn run_2019_day(day: u8) -> Result<(), AdventOfCodeError> {
    let result = match day {
        1 => advent_of_code_2019::day_one::run()?,
        2 => advent_of_code_2019::day_two::run()?,
        3 => advent_of_code_2019::day_three::run()?,
        4 => advent_of_code_2019::day_four::run()?,
        5 => advent_of_code_2019::day_five::run()?,
        6 => advent_of_code_2019::day_six::run()?,
        7 => advent_of_code_2019::day_seven::run()?,
        8 => advent_of_code_2019::day_eight::run()?,
        9 => advent_of_code_2019::day_nine::run()?,
        10 => advent_of_code_2019::day_ten::run()?,
        11 => advent_of_code_2019::day_eleven::run()?,
        12 => advent_of_code_2019::day_twelve::run()?,
        13 => advent_of_code_2019::day_thirteen::run()?,
        _ => panic!(),
    };

    log_result(2019, day, result);

    Ok(())
}

fn run_2019() -> Result<(), AdventOfCodeError> {
    for day in 1..=13 {
        run_2019_day(day)?;
    }

    Ok(())
}
