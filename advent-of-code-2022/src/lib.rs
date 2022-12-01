use common::prelude::*;
use common::result_logger;

pub fn run_all() -> Result<(), AdventOfCodeError> {
    for i in 1..=1 {
        run_day(i)?;
    }

    Ok(())
}

pub fn run_day(day: u8) -> Result<(), AdventOfCodeError> {
    let result = match day {
        _ => panic!("unimplemented")
    }

    result_logger::log_result(2022, day, result);

    Ok(())
}
