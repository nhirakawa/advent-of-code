extern crate nom;

mod answer;
mod day_eight;
mod day_eighteen;
mod day_eleven;
mod day_fifteen;
mod day_five;
mod day_four;
mod day_fourteen;
mod day_nine;
mod day_nineteen;
mod day_one;
mod day_seven;
mod day_seventeen;
mod day_six;
mod day_sixteen;
mod day_ten;
mod day_thirteen;
mod day_three;
mod day_twelve;
mod day_twenty;
mod day_twenty_one;
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

    let day_fourteen = day_fourteen::run()?;
    log_result(14, day_fourteen);

    let day_fifteen = day_fifteen::run()?;
    log_result(15, day_fifteen);

    let day_sixteen = day_sixteen::run()?;
    log_result(16, day_sixteen);

    let day_seventeen = day_seventeen::run()?;
    log_result(17, day_seventeen);

    let day_eighteen = day_eighteen::run()?;
    log_result(18, day_eighteen);

    let day_nineteen = day_nineteen::run()?;
    log_result(19, day_nineteen);

    let day_twenty = day_twenty::run()?;
    log_result(20, day_twenty);

    let day_twenty_one = day_twenty_one::run()?;
    log_result(21, day_twenty_one);

    Ok(())
}

fn log_result(day: u8, answers: (PartAnswer, PartAnswer)) {
    let (part_one, part_two) = answers;

    println!(
        "day {}, part 1: {} ({} μs)",
        day,
        part_one.0,
        part_one.1.as_micros()
    );
    println!(
        "day {}, part 2: {} ({} μs)",
        day,
        part_two.0,
        part_two.1.as_micros()
    );
}
