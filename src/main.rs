use error::AdventOfCodeError;

extern crate nom;

mod day_one;
mod day_three;
mod day_two;
mod error;

fn main() -> Result<(), AdventOfCodeError> {
    let (day_one_part_one, day_two_part_two) = day_one::run()?;

    println!("day 1, part 1: {}", day_one_part_one);
    println!("day 1, part 2: {}", day_two_part_two);

    let (day_two_part_one, day_two_part_two) = day_two::run()?;
    println!("day 2, part 1: {}", day_two_part_one);
    println!("day 2, part 2: {}", day_two_part_two);

    let (day_three_part_one, day_three_part_two) = day_three::run()?;
    println!("day 3, part 1: {}", day_three_part_one);
    println!("day 3, part 2: {}", day_three_part_two);

    Ok(())
}
