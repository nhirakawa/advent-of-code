extern crate nom;

mod answer;
mod day_five;
mod day_four;
mod day_one;
mod day_six;
mod day_three;
mod day_two;

use answer::{AdventOfCodeError, AnswerWithTiming};

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

    Ok(())
}

fn log_result(day: u8, answers: (AnswerWithTiming, AnswerWithTiming)) {
    let ((part_one_solution, part_one_timing), (part_two_solution, part_two_timing)) = answers;
    println!(
        "day {}, part {}: {} ({} ms)",
        day,
        1,
        part_one_solution,
        part_one_timing.as_millis()
    );
    println!(
        "day {}, part {}: {} ({} ms)",
        day,
        2,
        part_two_solution,
        part_two_timing.as_millis()
    );
}
