use error::AdventOfCodeError;

mod day_one;
mod error;

fn main() -> Result<(), AdventOfCodeError> {
    let (day_one_part_one, day_two_part_two) = day_one::run()?;

    println!("day 1, part 1 {}", day_one_part_one);
    println!("day 1, part 2 {}", day_two_part_two);

    Ok(())
}
