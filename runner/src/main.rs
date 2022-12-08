extern crate clap;

use clap::{App, Arg};
use common::prelude::*;
use env_logger::Env;

fn main() -> Result<(), AdventOfCodeError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    let matches = App::new("Advent of Code")
        .version("0.1.0")
        .author("Nick Hirakawa <nickhirakawa@gmail.com>")
        .about("Advent of Code solutions")
        .arg(Arg::with_name("year").index(1).possible_values(&[
            "2022", "2021", "2020", "2019", "2018", "2017", "2016", "2015",
        ]))
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
                2022 => advent_of_code_2022::run_day(day)?,
                2021 => advent_of_code_2021::run_day(day)?,
                2020 => advent_of_code_2020::run_day(day)?,
                2019 => advent_of_code_2019::run_day(day)?,
                2018 => advent_of_code_2018::run_day(day)?,
                2017 => advent_of_code_2017::run_day(day)?,
                2016 => advent_of_code_2016::run_day(day)?,
                2015 => advent_of_code_2015::run_day(day)?,
                _ => panic!(),
            }
        } else {
            match year {
                2022 => advent_of_code_2022::run_all()?,
                2021 => advent_of_code_2021::run_all()?,
                2020 => advent_of_code_2020::run_all()?,
                2019 => advent_of_code_2019::run_all()?,
                2018 => advent_of_code_2018::run_all()?,
                2017 => advent_of_code_2017::run_all()?,
                2016 => advent_of_code_2016::run_all()?,
                2015 => advent_of_code_2015::run_all()?,
                _ => panic!(),
            }
        }
    } else {
        advent_of_code_2022::run_all()?;
        advent_of_code_2021::run_all()?;
        advent_of_code_2020::run_all()?;
        advent_of_code_2019::run_all()?;
        advent_of_code_2018::run_all()?;
        advent_of_code_2017::run_all()?;
        advent_of_code_2016::run_all()?;
        advent_of_code_2015::run_all()?;
    }

    Ok(())
}
