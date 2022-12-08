use common::prelude::*;
use common::result_logger;

mod day_one;

pub fn run_all() -> Result<(), AdventOfCodeError> {
    for i in 1..=1 {
        run_day(i)?;
    }

    Ok(())
}

pub fn run_day(day: u8) -> Result<(), AdventOfCodeError> {
    let result = match day {
        1 => day_one::run()?,
        _ => panic!("unimplemented"),
    };

    result_logger::log_result(2016, day, result);

    Ok(())
}
