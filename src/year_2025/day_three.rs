use std::{fmt::Display, str::FromStr};

use anyhow::bail;

pub fn part_one(input: &str) -> anyhow::Result<impl ToString> {
    let battery_banks = input.parse::<BatteryBanks>()?;
    Ok(battery_banks.get_total_output_joltage(2))
}

pub fn part_two(input: &str) -> anyhow::Result<impl ToString> {
    let battery_banks = input.parse::<BatteryBanks>()?;
    Ok(battery_banks.get_total_output_joltage(12))
}

struct BatteryBanks(Vec<BatteryBank>);

impl BatteryBanks {
    fn get_total_output_joltage(&self, num_batteries: usize) -> usize {
        self.0
            .iter()
            .map(|battery_bank| battery_bank.get_largest_joltage(num_batteries))
            .map(|u| u as usize)
            .sum()
    }
}

impl FromStr for BatteryBanks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut battery_banks = Vec::new();

        for line in s.trim().lines() {
            battery_banks.push(line.parse()?);
        }

        Ok(BatteryBanks(battery_banks))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct BatteryBank(Vec<u8>);

impl BatteryBank {
    fn get_largest_joltage(&self, num_batteries: usize) -> u64 {
        let mut largest_joltage = Vec::new();
        let mut remaining = &self.0[..];
        for end in (0..num_batteries).rev() {
            let (largest, remaining_after_extraction) = find_largest_remaining(remaining, end);
            largest_joltage.push(largest);
            remaining = remaining_after_extraction;
        }

        unify(&largest_joltage)
    }
}

/// Searches for the largest element in `slice` that still allows choosing `min_remaining_length` elements
/// Example:
///   - returns `4` from `[4, 3, 2, 5]` with `3` remaining elements
fn find_largest_remaining(slice: &[u8], min_remaining_length: usize) -> (u8, &[u8]) {
    let end = slice.len() - 1 - min_remaining_length;

    // Index of the current max element
    let mut max_index = 0usize;
    let mut max_element = 0u8;

    for (i, &element) in slice.iter().enumerate().take(end + 1) {
        if element > max_element {
            max_index = i;
            max_element = element;
        }
    }

    (max_element, &slice[max_index + 1..])
}

fn unify(digits: &[u8]) -> u64 {
    let mut sum = 0;

    for (index, digit) in digits.iter().enumerate() {
        sum += *digit as u64 * 10u64.pow(digits.len() as u32 - index as u32 - 1)
    }

    sum
}

impl FromStr for BatteryBank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut batteries = Vec::new();

        for b in s.as_bytes() {
            if !(0x30..=0x39).contains(b) {
                bail!("Invalid joltage - {}", b);
            }

            batteries.push(b - 0x30);
        }

        Ok(BatteryBank(batteries))
    }
}

impl Display for BatteryBank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.0.iter().map(|u| u.to_string()).collect();
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_battery_bank_parse() {
        assert_eq!(
            "987654321111111".parse::<BatteryBank>().unwrap(),
            BatteryBank(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1])
        )
    }

    #[test]
    fn test_battery_bank_max_joltage() {
        assert_eq!(
            "987654321111111"
                .parse::<BatteryBank>()
                .unwrap()
                .get_largest_joltage(2),
            98
        );
        assert_eq!(
            "811111111111119"
                .parse::<BatteryBank>()
                .unwrap()
                .get_largest_joltage(2),
            89
        );
        assert_eq!(
            "234234234234278"
                .parse::<BatteryBank>()
                .unwrap()
                .get_largest_joltage(2),
            78
        );
        assert_eq!(
            "818181911112111"
                .parse::<BatteryBank>()
                .unwrap()
                .get_largest_joltage(2),
            92
        );
    }

    #[test]
    fn test_unify() {
        assert_eq!(unify(&[9, 8]), 98);
        assert_eq!(unify(&[8, 9]), 89);
        assert_eq!(unify(&[7, 8]), 78);
        assert_eq!(unify(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1]), 987654321111);
    }

    #[test]
    fn test_find_largest_remaining() {
        let nums = vec![4, 5, 1, 3, 2];
        assert_eq!(find_largest_remaining(&nums, 4), (4, &nums[1..]));
    }
}
