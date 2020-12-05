extern crate nom;

mod answer;
mod day_four;
mod day_one;
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
