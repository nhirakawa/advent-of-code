extern crate clap;

mod common;
mod year_2015;
mod year_2016;
mod year_2017;
mod year_2018;
mod year_2019;
mod year_2020;
mod year_2021;
mod year_2022;

use ansi_term::Color::Red;
use anyhow::Context;
use clap::Command;
use common::base::{Day, Part, Year};
use env_logger::Env;
use itertools::Itertools;
use log::{error, info};
use std::{fmt::Display, iter, time::Duration};

type PartAnswer = (anyhow::Result<String>, Duration);

struct DayRunner {
    year: Year,
    day: Day,
    part_one: Option<fn(&str) -> anyhow::Result<String>>,
    part_two: Option<fn(&str) -> anyhow::Result<String>>,
}

struct DayResult {
    year: Year,
    day: Day,
    parts: anyhow::Result<(Option<PartAnswer>, Option<PartAnswer>)>,
}

enum RunMode {
    All,
    Latest,
    AllYear(Year),
    LatestYear(Year),
    Day(Year, Day),
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("off"))
        .format_timestamp(None)
        .init();

    let matches = Command::new("AdventOfCode")
        .version("1.0")
        .about("Solves Advent of Code problems")
        .subcommand_required(true)
        .subcommand(all_command())
        .subcommand(latest_command())
        .subcommands(Year::iter().map(year_command))
        .get_matches();

    let run_mode = match matches.subcommand() {
        Some(("all", _)) => RunMode::All,
        Some(("latest", _)) => RunMode::Latest,
        Some((year, year_matches)) => {
            let year = year.parse::<Year>().unwrap();

            match year_matches.subcommand() {
                Some(("all", _)) => RunMode::AllYear(year),
                Some(("latest", _)) => RunMode::LatestYear(year),
                Some((day, _)) => {
                    let day = day.parse::<Day>().unwrap();
                    RunMode::Day(year, day)
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };

    let runners = match run_mode {
        RunMode::All => run_all().collect_vec(),
        RunMode::Latest => run_all()
            .filter(|runner| runner.part_one.is_some())
            .last()
            .into_iter()
            .collect_vec(),
        RunMode::AllYear(year) => run_all_for_year(year).collect_vec(),
        RunMode::LatestYear(year) => run_all_for_year(year)
            .filter(|runner| runner.part_one.is_some())
            .last()
            .into_iter()
            .collect_vec(),
        RunMode::Day(year, day) => iter::once(run_day(year, day)).collect_vec(),
    };

    if runners.is_empty() {
        error!("No runners found");
        return Ok(());
    }

    for runner in runners {
        let DayRunner {
            year,
            day,
            part_one,
            part_two,
        } = runner;

        let day_result = run_solution(&year, &day, part_one, part_two);

        let DayResult {
            parts,
            year: _year,
            day: _day,
        } = day_result;

        match parts {
            Ok((part_one, part_two)) => {
                if part_one.is_none() && part_two.is_none() {
                    println!(
                        "{}",
                        Red.paint(format!("No solutions found for year {year}, day {day}"))
                    );
                }

                match part_one {
                    Some((Ok(part_one_solution), part_one_elapsed)) => {
                        println!(
                            "year {}, day {}, part 1: {part_one_solution} ({:?} ms)",
                            &year,
                            &day,
                            part_one_elapsed.as_millis()
                        );
                    }
                    Some((Err(e), _)) => {
                        println!(
                            "{}",
                            Red.paint(format!(
                                "Could not run part 1 for year {year}, day {day} - {e}"
                            ))
                        );
                    }
                    None => {
                        println!(
                            "{}",
                            Red.paint(format!(
                                "No solution found for year {year}, day {day}, part 1"
                            ))
                        );
                    }
                };

                match (part_two, day) {
                    (Some((Ok(part_two_solution), part_two_elapsed)), _) => {
                        println!(
                            "year {}, day {}, part 2: {part_two_solution} ({:?} ms)",
                            &year,
                            &day,
                            part_two_elapsed.as_millis()
                        );
                    }
                    (Some((Err(e), _)), _) => {
                        println!(
                            "{}",
                            Red.paint(format!(
                                "Could not run part 2 for year {year}, day {day} - {e}"
                            ))
                        );
                    }
                    (None, Day::Day25) => {}
                    (None, _) => {
                        println!(
                            "{}",
                            Red.paint(format!(
                                "No solution found for year {year}, day {day}, part 2"
                            ))
                        );
                    }
                };
            }
            Err(e) => println!(
                "{}",
                Red.paint(format!(
                    "Could not read input for year {year}, day {day} - {e}"
                ))
            ),
        }
    }

    Ok(())
}

fn run_all() -> impl Iterator<Item = DayRunner> {
    Year::iter().flat_map(|year| run_all_for_year(year))
}

// fn run_latest() -> Box<dyn Iterator<Item = Option<DayResult>>> {
//     for year in Year::iter().rev() {
//         let day_result = run_latest_for_year(year).into_iter().next();
//         if day_result.is_some() {
//             return Box::new(day_result.into_iter());
//         }
//     }
//     Box::new(iter::empty())
// }

fn run_all_for_year(year: Year) -> impl Iterator<Item = DayRunner> {
    Day::iter().map(move |day| run_day(year.clone(), day))
}

// fn run_latest_for_year(year: Year) -> Box<dyn Iterator<Item = Option<DayResult>>> {
//     for day in Day::iter().rev() {
//         let result = run_day(year, day);
//         if result.is_some() {
//             return Box::new(iter::once(result));
//         }
//     }
//     Box::new(iter::empty())
// }

fn run_day(year: Year, day: Day) -> DayRunner {
    let part_one = match year {
        Year::Year2015 => year_2015::solution(day, Part::PartOne),
        Year::Year2016 => year_2016::solution(day, Part::PartOne),
        Year::Year2017 => year_2017::solution(day, Part::PartOne),
        Year::Year2018 => year_2018::solution(day, Part::PartOne),
        Year::Year2019 => year_2019::solution(day, Part::PartOne),
        Year::Year2020 => year_2020::solution(day, Part::PartOne),
        Year::Year2021 => year_2021::solution(day, Part::PartOne),
        Year::Year2022 => year_2022::solution(day, Part::PartOne),
    };

    let part_two = match year {
        Year::Year2015 => year_2015::solution(day, Part::PartTwo),
        Year::Year2016 => year_2016::solution(day, Part::PartTwo),
        Year::Year2017 => year_2017::solution(day, Part::PartTwo),
        Year::Year2018 => year_2018::solution(day, Part::PartTwo),
        Year::Year2019 => year_2019::solution(day, Part::PartTwo),
        Year::Year2020 => year_2020::solution(day, Part::PartTwo),
        Year::Year2021 => year_2021::solution(day, Part::PartTwo),
        Year::Year2022 => year_2022::solution(day, Part::PartTwo),
    };

    DayRunner {
        year,
        day,
        part_one,
        part_two,
    }
}

fn run_solution(
    year: &Year,
    day: &Day,
    part_one: Option<fn(&str) -> anyhow::Result<String>>,
    part_two: Option<fn(&str) -> anyhow::Result<String>>,
) -> DayResult {
    info!("Running year {year}, day {day}");
    let input = read_input(year, day);

    match input {
        Ok(input) => {
            let part_one = part_one.map(|f| timed(|| f(&input)));
            let part_two = part_two.map(|f| timed(|| f(&input)));

            DayResult {
                year: *year,
                day: *day,
                parts: Ok((part_one, part_two)),
            }
        }
        Err(e) => DayResult {
            year: *year,
            day: *day,
            parts: Err(e),
        },
    }
}

fn read_input(year: &Year, day: &Day) -> anyhow::Result<String> {
    let path = format!("input/year-{}/day-{}.txt", year.as_u32(), day.as_u8());
    std::fs::read_to_string(&path)
        .with_context(|| format!("Could not read {path}"))
        .map_err(anyhow::Error::from)
}

/// Times the execution of a function and returns the result and the elapsed time
/// The result is converted to a string
fn timed<T: Display>(
    fun: impl FnOnce() -> anyhow::Result<T>,
) -> (anyhow::Result<String>, Duration) {
    let start = std::time::Instant::now();
    let result = fun().map(|s| s.to_string());
    let elapsed = start.elapsed();
    (result, elapsed)
}

fn year_command(year: Year) -> Command {
    let year_str = year.as_str();
    Command::new(year_str)
        .about(format!("Run solutions for a year {year_str}"))
        .subcommand_required(true)
        .subcommand(all_command())
        .subcommand(latest_command())
        .subcommands(day_commands())
}

fn day_commands() -> impl IntoIterator<Item = Command> {
    Day::iter().map(day_command)
}

fn day_command(day: Day) -> Command {
    let day_str = day.as_str();
    Command::new(day_str).about(format!("Run solutions for a day {day_str}"))
}

fn all_command() -> Command {
    Command::new("all").about("Run all solutions")
}

fn latest_command() -> Command {
    Command::new("latest").about("Run latest solution")
}
