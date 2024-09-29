use crate::common::{answer::*, result_logger};

mod day_one;
mod day_three;
mod day_two;

pub fn run_all() -> Result<(), AdventOfCodeError> {
    for i in 1..=3 {
        run_day(i)?;
    }

    Ok(())
}

pub fn run_day(day: u8) -> Result<(), AdventOfCodeError> {
    let result = match day {
        1 => day_one::run()?,
        2 => day_two::run()?,
        3 => day_three::run()?,
        _ => panic!("unimplemented"),
    };

    result_logger::log_result(2017, day, result);

    Ok(())
}
