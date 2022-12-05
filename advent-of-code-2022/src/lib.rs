use common::prelude::*;
use common::result_logger;

mod day_five;
mod day_four;
mod day_one;
mod day_three;
mod day_two;

pub fn run_all() -> Result<(), AdventOfCodeError> {
    for i in 1..=5 {
        run_day(i)?;
    }

    Ok(())
}

pub fn run_day(day: u8) -> Result<(), AdventOfCodeError> {
    let result = match day {
        1 => day_one::run()?,
        2 => day_two::run()?,
        3 => day_three::run()?,
        4 => day_four::run()?,
        5 => day_five::run()?,
        _ => panic!("unimplemented"),
    };

    result_logger::log_result(2022, day, result);

    Ok(())
}
