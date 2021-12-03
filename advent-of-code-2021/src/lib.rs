mod day_one;
mod day_two;

use common::prelude::*;
use common::result_logger;

pub fn run_all() -> Result<(), AdventOfCodeError> {
    for i in 1..=2 {
        run_day(i)?;
    }

    Ok(())
}

pub fn run_day(day: u8) -> Result<(), AdventOfCodeError> {
    let result = match day {
        1 => day_one::run()?,
        2 => day_two::run()?,
        _ => panic!("unimplemented"),
    };

    result_logger::log_result(2018, day, result);

    Ok(())
}