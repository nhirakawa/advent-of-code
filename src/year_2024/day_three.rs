use anyhow::anyhow;
use itertools::Itertools;
use log::info;
use regex::Regex;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let re = Regex::new(r"(?m)mul\((?<left>\d+),(?<right>\d+)\)")?;
    let captures = re.captures_iter(input).collect_vec();
    info!("Found {} captures", captures.len());

    let mut sum = 0;

    for capture in captures {
        let left = capture
            .name("left")
            .ok_or(anyhow!("No left operand match"))
            .and_then(|m| m.as_str().parse::<u64>().map_err(anyhow::Error::from))?;
        let right = capture
            .name("right")
            .ok_or(anyhow!("No right operand match"))
            .and_then(|m| m.as_str().parse::<u64>().map_err(anyhow::Error::from))?;

        sum += left * right;
    }

    Ok(sum.to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let re = Regex::new(r"(mul\((?<left>\d+),(?<right>\d+)\)|don't\(\)|do\(\))")?;

    let captures = re.captures_iter(input).collect_vec();

    info!("Found {} captures", captures.len());

    let mut sum = 0;
    let mut enabled = true;

    for capture in captures {
        if capture.get(0).unwrap().as_str() == "do()" {
            enabled = true;
        } else if capture.get(0).unwrap().as_str() == "don't()" {
            enabled = false;
        } else if enabled {
            let left = capture
                .name("left")
                .ok_or(anyhow!("No left operand match"))
                .and_then(|m| m.as_str().parse::<u64>().map_err(anyhow::Error::from))?;
            let right = capture
                .name("right")
                .ok_or(anyhow!("No right operand match"))
                .and_then(|m| m.as_str().parse::<u64>().map_err(anyhow::Error::from))?;

            sum += left * right;
        }
    }

    Ok(sum.to_string())
}
