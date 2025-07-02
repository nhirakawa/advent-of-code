use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_opt, multi::separated_list1,
    IResult, Parser,
};

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let modules = parse_input(input);
    Ok(modules
        .iter()
        .map(|module| calculate_fuel(*module))
        .sum::<u32>()
        .to_string())
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let modules = parse_input(input);

    Ok(modules
        .iter()
        .map(|module| calculate_fuel_recursive(*module))
        .sum::<u32>()
        .to_string())
}

fn calculate_fuel(module: u32) -> u32 {
    (module / 3) - 2
}

fn calculate_fuel_recursive(module: u32) -> u32 {
    if module <= 6 {
        0
    } else {
        let cost = calculate_fuel(module);
        cost + calculate_fuel_recursive(cost)
    }
}

fn parse_input(i: &str) -> Vec<u32> {
    let (_, input) = separated_list1(tag("\n"), number).parse(i).unwrap();
    input
}

fn number(i: &str) -> IResult<&str, u32> {
    map_opt(digit1, |s: &str| s.parse::<u32>().ok()).parse(i)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn test_calculate_fuel_recursive() {
        assert_eq!(calculate_fuel_recursive(14), 2);
        assert_eq!(calculate_fuel_recursive(1969), 966);
        assert_eq!(calculate_fuel_recursive(100756), 50346);
    }
}
