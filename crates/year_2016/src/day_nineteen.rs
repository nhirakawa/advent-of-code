use anyhow::{anyhow, bail};
use std::iter::successors;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let number_of_elves = input.parse::<u32>()?;
    // https://en.wikipedia.org/wiki/Josephus_problem#Bitwise
    let leading_one_index = number_of_elves
        .highest_one()
        .ok_or(anyhow!("{number_of_elves:0b} does not have a leading one"))?;
    let remainder = number_of_elves & !(1 << leading_one_index);
    Ok((remainder << 1) | 1)
}
pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let number_of_elves = input.parse::<u32>()?;
    if number_of_elves <= 1 {
        bail!("Invalid input - {number_of_elves}");
    }

    // Determined by examining winners for n in 2..=50 and deriving the following "function"
    // The winner of n=1 is 1
    // The winner of n=2 is 2
    // The winner of n=3 is 3 (power-of-3)
    // The winner of n=4 is 1 (1+power-of-3)
    // The winner of n=6 is 3 (2*power-of-3)
    // The winner of n=7 is 5 (1+(2*power-of-3))
    let powers_of_three = successors(Some(1u32), |n| n.checked_mul(3));
    let lower_power_of_three = powers_of_three
        .take_while(|&p| p < number_of_elves)
        .last()
        .ok_or(anyhow!(
            "Could not find power-of-three bounds for {number_of_elves}"
        ))?;

    if number_of_elves <= 2 * lower_power_of_three {
        // Step size between elves is 1
        Ok(number_of_elves - lower_power_of_three)
    } else {
        // Step size between elves is 2
        Ok(2 * number_of_elves - 3 * lower_power_of_three)
    }
}

/// Naive O(n^2) simulation of part two: each elf steals from the elf directly across the circle.
/// Returns the 1-based position of the winning elf.
#[cfg(test)]
fn simulate(number_of_elves: u32) -> u32 {
    let mut elves = (1..=number_of_elves).collect::<Vec<_>>();
    let mut current = 0;
    while elves.len() > 1 {
        let across = (current + elves.len() / 2) % elves.len();
        elves.remove(across);
        // if the removed elf was before the current one, everything shifted left and `current`
        // already points at the next elf; otherwise advance
        if across > current {
            current += 1;
        }
        current %= elves.len();
    }
    elves[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate() {
        assert_eq!(simulate(5), 2);
    }
}
