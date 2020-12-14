extern crate nom;

mod answer;
mod day_eight;
mod day_eleven;
mod day_five;
mod day_four;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_ten;
mod day_thirteen;
mod day_three;
mod day_twelve;
mod day_two;
mod prelude;

use prelude::*;

fn main() -> Result<(), AdventOfCodeError> {
    let day_one = day_one::run()?;
    log_result(1, day_one);

    let day_two = day_two::run()?;
    log_result(2, day_two);

    let day_three = day_three::run()?;
    log_result(3, day_three);

    let day_four = day_four::run()?;
    log_result(4, day_four);

    let day_five = day_five::run()?;
    log_result(5, day_five);

    let day_six = day_six::run()?;
    log_result(6, day_six);

    let day_seven = day_seven::run()?;
    log_result(7, day_seven);

    let day_eight = day_eight::run()?;
    log_result(8, day_eight);

    let day_nine = day_nine::run()?;
    log_result(9, day_nine);

    let day_ten = day_ten::run()?;
    log_result(10, day_ten);

    let day_eleven = day_eleven::run()?;
    log_result(11, day_eleven);

    let day_twelve = day_twelve::run()?;
    log_result(12, day_twelve);

    let day_thirteen = day_thirteen::run()?;
    log_result(13, day_thirteen);

    Ok(())
}

fn log_result(day: u8, answers: (PartAnswer, PartAnswer)) {
    let (part_one, part_two) = answers;

    match part_one {
        Err(e) => println!("day {}, part 1: {:#?}", day, e),
        Ok((solution, timing)) => println!(
            "day {}, part 1: {} ({} ms)",
            day,
            solution,
            timing.as_millis()
        ),
    }

    match part_two {
        Err(e) => println!("day {}, part 2: {:#?}", day, e),
        Ok((solution, timing)) => println!(
            "day {}, part 2 {} ({} ms)",
            day,
            solution,
            timing.as_millis()
        ),
    }
}
