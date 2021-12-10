use common::{prelude::AdventOfCodeError, result_logger::log_result};

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

pub fn run_all() -> Result<(), AdventOfCodeError> {
    for i in 1..=25 {
        run_day(i)?;
    }

    Ok(())
}

pub fn run_day(day: u8) -> Result<(), AdventOfCodeError> {
    let result = match day {
        1 => day_one::run(),
        2 => day_two::run(),
        3 => day_three::run(),
        4 => day_four::run(),
        5 => day_five::run(),
        6 => day_six::run(),
        7 => day_seven::run(),
        8 => day_eight::run(),
        9 => day_nine::run(),
        10 => day_ten::run(),
        11 => day_eleven::run(),
        12 => day_twelve::run(),
        13 => day_thirteen::run(),
        14 => day_fourteen::run(),
        15 => day_fifteen::run(),
        16 => day_sixteen::run(),
        17 => day_seventeen::run(),
        18 => day_eighteen::run(),
        19 => day_nineteen::run(),
        20 => day_twenty::run(),
        21 => day_twenty_one::run(),
        22 => day_twenty_two::run(),
        23 => day_twenty_three::run(),
        24 => day_twenty_four::run(),
        25 => day_twenty_five::run(),
        _ => panic!(),
    }?;

    log_result(2020, day, result);

    Ok(())
}
