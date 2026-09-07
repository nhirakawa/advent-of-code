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
mod year_2023;
mod year_2024;
mod year_2025;

use ansi_term::Color::Red;
use anyhow::Context;
use clap::Command;
use common::base::{Day, Part, Year};
use env_logger::Env;
use itertools::Itertools;
use log::{error, info};
use std::{fmt::Display, iter, time::Duration};

struct PartAnswer {
    part: Part,
    result: anyhow::Result<String>,
    duration: Duration,
}

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

#[derive(Clone, Copy)]
struct TestMode {
    enabled: bool,
}

struct Config {
    run_mode: RunMode,
    test_mode: TestMode,
}

struct TestViolation {
    year: Year,
    day: Day,
    part: Part,
    expected: String,
    actual: String,
}

struct TestResults {
    violations: Vec<TestViolation>,
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("off"))
        .format_timestamp(None)
        .init();

    let config = parse_cli();

    let runners = get_runners(&config.run_mode);

    if runners.is_empty() {
        error!("No runners found");
        return Ok(());
    }

    if config.test_mode.enabled {
        let mut test_results = TestResults {
            violations: Vec::new(),
        };

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

            collect_test_results(&year, &day, &parts, &mut test_results);
        }

        display_test_summary(&test_results);
    } else {
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

            display_results(&year, &day, &parts);
        }
    }

    Ok(())
}

fn run_all() -> impl Iterator<Item = DayRunner> {
    Year::iter().flat_map(run_all_for_year)
}

fn run_all_for_year(year: Year) -> impl Iterator<Item = DayRunner> {
    Day::iter().map(move |day| run_day(year, day))
}

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
        Year::Year2023 => year_2023::solution(day, Part::PartOne),
        Year::Year2024 => year_2024::solution(day, Part::PartOne),
        Year::Year2025 => year_2025::solution(day, Part::PartOne),
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
        Year::Year2023 => year_2023::solution(day, Part::PartTwo),
        Year::Year2024 => year_2024::solution(day, Part::PartTwo),
        Year::Year2025 => year_2025::solution(day, Part::PartTwo),
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
            let part_one = part_one.map(|f| {
                let (result, duration) = timed(|| f(&input));
                PartAnswer {
                    part: Part::PartOne,
                    result,
                    duration,
                }
            });
            let part_two = part_two.map(|f| {
                let (result, duration) = timed(|| f(&input));
                PartAnswer {
                    part: Part::PartTwo,
                    result,
                    duration,
                }
            });

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
    let path = format!("input/year-{year}/day-{day}.txt");
    std::fs::read_to_string(&path).with_context(|| format!("Could not read {path}"))
}

fn read_expected_output(year: &Year, day: &Day, part: &Part) -> anyhow::Result<String> {
    let path = format!("output/year-{year}/day-{day}/part-{part}.txt",);
    std::fs::read_to_string(&path)
        .with_context(|| format!("Could not read expected output from {path}"))
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

fn parse_cli() -> Config {
    let matches = Command::new("AdventOfCode")
        .version("1.0")
        .about("Solves Advent of Code problems")
        .arg(
            clap::Arg::new("test")
                .long("test")
                .help("Compare solutions with expected output from output/ directory")
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand_required(true)
        .subcommand(all_command())
        .subcommands(Year::iter().map(year_command))
        .get_matches();

    let test_mode = TestMode {
        enabled: matches.get_flag("test"),
    };

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

    Config {
        run_mode,
        test_mode,
    }
}

fn get_runners(run_mode: &RunMode) -> Vec<DayRunner> {
    match run_mode {
        RunMode::All => run_all().collect_vec(),
        RunMode::Latest => run_all()
            .filter(|runner| runner.part_one.is_some())
            .last()
            .into_iter()
            .collect_vec(),
        RunMode::AllYear(year) => run_all_for_year(*year).collect_vec(),
        RunMode::LatestYear(year) => run_all_for_year(*year)
            .filter(|runner| runner.part_one.is_some())
            .last()
            .into_iter()
            .collect_vec(),
        RunMode::Day(year, day) => iter::once(run_day(*year, *day)).collect_vec(),
    }
}

fn display_results(
    year: &Year,
    day: &Day,
    parts: &anyhow::Result<(Option<PartAnswer>, Option<PartAnswer>)>,
) {
    match parts {
        Ok((part_one, part_two)) => {
            if part_one.is_none() && part_two.is_none() {
                println!(
                    "{}",
                    Red.paint(format!("No solutions found for year {year}, day {day}"))
                );
            }

            handle_part_display(year, day, part_one);
            handle_part_display(year, day, part_two);
        }
        Err(e) => println!(
            "{}",
            Red.paint(format!(
                "Could not read input for year {year}, day {day} - {e}"
            ))
        ),
    }
}

fn handle_part_display(year: &Year, day: &Day, part: &Option<PartAnswer>) {
    match part {
        Some(PartAnswer {
            part,
            result: Ok(solution),
            duration,
        }) => {
            println!(
                "year {year}, day {day}, part {part}: {solution} ({:?} ms)",
                duration.as_millis()
            );
        }
        Some(PartAnswer {
            part,
            result: Err(e),
            duration: _,
        }) => {
            if e.to_string() != "Not implemented" {
                println!(
                    "{}",
                    Red.paint(format!(
                        "Could not run part {part} for year {year}, day {day} - {e:?}"
                    ))
                );
            }
        }
        None => {
            println!(
                "{}",
                Red.paint(format!("No solution found for year {year}, day {day}"))
            );
        }
    }
}

fn collect_test_results(
    year: &Year,
    day: &Day,
    parts: &anyhow::Result<(Option<PartAnswer>, Option<PartAnswer>)>,
    test_results: &mut TestResults,
) {
    if let Ok((part_one, part_two)) = parts {
        if let Some(PartAnswer {
            part,
            result: Ok(solution),
            duration: _,
        }) = part_one
        {
            collect_test_for_part(year, day, part, solution, test_results);
        }

        if let Some(PartAnswer {
            part,
            result: Ok(solution),
            duration: _,
        }) = part_two
        {
            collect_test_for_part(year, day, part, solution, test_results);
        }
    }
}

fn collect_test_for_part(
    year: &Year,
    day: &Day,
    part: &Part,
    solution: &str,
    test_results: &mut TestResults,
) {
    if let Ok(expected) = read_expected_output(year, day, part)
        && solution.trim() != expected.trim()
    {
        test_results.violations.push(TestViolation {
            year: *year,
            day: *day,
            part: *part,
            expected: expected.trim().to_string(),
            actual: solution.trim().to_string(),
        });
    }
}

fn display_test_summary(test_results: &TestResults) {
    if test_results.violations.is_empty() {
        println!("All tests passed");
    } else {
        println!("Test violations found:");
        for violation in &test_results.violations {
            println!(
                "  ❌ {} day {} part {}: expected '{}', got '{}'",
                violation.year, violation.day, violation.part, violation.expected, violation.actual
            );
        }
    }
}
