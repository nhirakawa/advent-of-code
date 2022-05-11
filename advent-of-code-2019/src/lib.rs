use common::{prelude::AdventOfCodeError, result_logger};

extern crate multimap;
extern crate multiset;
extern crate nom;

mod computer;
mod day_eight;
mod day_eleven;
mod day_fifteen;
mod day_five;
mod day_four;
mod day_fourteen;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_sixteen;
mod day_ten;
mod day_thirteen;
mod day_three;
mod day_twelve;
mod day_two;

pub fn run_all() -> Result<(), AdventOfCodeError> {
    for i in 1..=16 {
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
        6 => day_six::run()?,
        7 => day_seven::run()?,
        8 => day_eight::run()?,
        9 => day_nine::run()?,
        10 => day_ten::run()?,
        11 => day_eleven::run()?,
        12 => day_twelve::run()?,
        13 => day_thirteen::run()?,
        14 => day_fourteen::run()?,
        15 => day_fifteen::run()?,
        16 => day_sixteen::run()?,
        _ => panic!("unimplemented"),
    };

    result_logger::log_result(2021, day, result);

    Ok(())
}
