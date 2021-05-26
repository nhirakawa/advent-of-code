extern crate clap;
extern crate nom;

use clap::{App, Arg, SubCommand};
use common::prelude::*;

fn main() -> Result<(), AdventOfCodeError> {
    let matches = App::new("Advent of Code")
        .version("0.1.0")
        .author("Nick Hirakawa <nickhirakawa@gmail.com>")
        .about("Advent of Code solutions")
        .subcommand(
            SubCommand::with_name("2020").arg(
                Arg::with_name("day")
                    .index(1)
                    .takes_value(false)
                    .validator(is_day_of_month),
            ),
        )
        .get_matches();

    if matches.is_present("2020") {
        let matches = matches.subcommand_matches("2020").unwrap();

        if let Some(day) = matches.value_of("day").and_then(|s| s.parse::<u8>().ok()) {
            run_2020_day(day)?;
        } else {
            run_2020()?;
        }
    } else {
        run_2020()?;
    }

    Ok(())
}

fn is_day_of_month(val: String) -> Result<(), String> {
    u8::from_str_radix(val.as_str(), 10)
        .map_err(|_| format!("Could not parse {}", val))
        .and_then(|d| {
            if d >= 1 && d <= 25 {
                Ok(())
            } else {
                Err(format!("{} is not between 1 and 25", val))
            }
        })
}

fn run_2020_day(day: u8) -> Result<(), AdventOfCodeError> {
    let result = match day {
        1 => advent_of_code_2020::day_one::run(),
        2 => advent_of_code_2020::day_two::run(),
        3 => advent_of_code_2020::day_three::run(),
        4 => advent_of_code_2020::day_four::run(),
        5 => advent_of_code_2020::day_five::run(),
        6 => advent_of_code_2020::day_six::run(),
        7 => advent_of_code_2020::day_seven::run(),
        8 => advent_of_code_2020::day_eight::run(),
        9 => advent_of_code_2020::day_nine::run(),
        10 => advent_of_code_2020::day_ten::run(),
        11 => advent_of_code_2020::day_eleven::run(),
        12 => advent_of_code_2020::day_twelve::run(),
        13 => advent_of_code_2020::day_thirteen::run(),
        14 => advent_of_code_2020::day_fourteen::run(),
        15 => advent_of_code_2020::day_fifteen::run(),
        16 => advent_of_code_2020::day_sixteen::run(),
        17 => advent_of_code_2020::day_seventeen::run(),
        18 => advent_of_code_2020::day_eighteen::run(),
        19 => advent_of_code_2020::day_nineteen::run(),
        20 => advent_of_code_2020::day_twenty::run(),
        21 => advent_of_code_2020::day_twenty_one::run(),
        22 => advent_of_code_2020::day_twenty_two::run(),
        23 => advent_of_code_2020::day_twenty_three::run(),
        24 => advent_of_code_2020::day_twenty_four::run(),
        25 => advent_of_code_2020::day_twenty_five::run(),
        _ => panic!(),
    }?;

    log_result(day, result);

    Ok(())
}

fn run_2020() -> Result<(), AdventOfCodeError> {
    let day_one = advent_of_code_2020::day_one::run()?;
    log_result(1, day_one);

    let day_two = advent_of_code_2020::day_two::run()?;
    log_result(2, day_two);

    let day_three = advent_of_code_2020::day_three::run()?;
    log_result(3, day_three);

    let day_four = advent_of_code_2020::day_four::run()?;
    log_result(4, day_four);

    let day_five = advent_of_code_2020::day_five::run()?;
    log_result(5, day_five);

    let day_six = advent_of_code_2020::day_six::run()?;
    log_result(6, day_six);

    let day_seven = advent_of_code_2020::day_seven::run()?;
    log_result(7, day_seven);

    let day_eight = advent_of_code_2020::day_eight::run()?;
    log_result(8, day_eight);

    let day_nine = advent_of_code_2020::day_nine::run()?;
    log_result(9, day_nine);

    let day_ten = advent_of_code_2020::day_ten::run()?;
    log_result(10, day_ten);

    let day_eleven = advent_of_code_2020::day_eleven::run()?;
    log_result(11, day_eleven);

    let day_twelve = advent_of_code_2020::day_twelve::run()?;
    log_result(12, day_twelve);

    let day_thirteen = advent_of_code_2020::day_thirteen::run()?;
    log_result(13, day_thirteen);

    let day_fourteen = advent_of_code_2020::day_fourteen::run()?;
    log_result(14, day_fourteen);

    let day_fifteen = advent_of_code_2020::day_fifteen::run()?;
    log_result(15, day_fifteen);

    let day_sixteen = advent_of_code_2020::day_sixteen::run()?;
    log_result(16, day_sixteen);

    let day_seventeen = advent_of_code_2020::day_seventeen::run()?;
    log_result(17, day_seventeen);

    let day_eighteen = advent_of_code_2020::day_eighteen::run()?;
    log_result(18, day_eighteen);

    let day_nineteen = advent_of_code_2020::day_nineteen::run()?;
    log_result(19, day_nineteen);

    let day_twenty = advent_of_code_2020::day_twenty::run()?;
    log_result(20, day_twenty);

    let day_twenty_one = advent_of_code_2020::day_twenty_one::run()?;
    log_result(21, day_twenty_one);

    let day_twenty_two = advent_of_code_2020::day_twenty_two::run()?;
    log_result(22, day_twenty_two);

    let day_twenty_three = advent_of_code_2020::day_twenty_three::run()?;
    log_result(23, day_twenty_three);

    let day_twenty_four = advent_of_code_2020::day_twenty_four::run()?;
    log_result(24, day_twenty_four);

    let day_twenty_five = advent_of_code_2020::day_twenty_five::run()?;
    log_result(25, day_twenty_five);

    Ok(())
}

fn log_result(day: u8, answers: (PartAnswer, PartAnswer)) {
    let (part_one, part_two) = answers;

    println!(
        "day {}, part 1: {} ({:?} ms)",
        day,
        part_one.get_answer(),
        part_one.get_duration().as_millis()
    );
    println!(
        "day {}, part 2: {} ({:?} ms)",
        day,
        part_two.get_answer(),
        part_two.get_duration().as_millis()
    );
}