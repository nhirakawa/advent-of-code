use anyhow::anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1, newline},
    combinator::{map_res, value},
    multi::many1,
    IResult, Parser,
};
use std::fmt::Display;

pub fn part_one(input: &str) -> anyhow::Result<String> {
    let passwords = parse_into_unvalidated_passwords(input)?;
    Ok(validate(&passwords, validate_part_one).to_string())
}

fn validate_part_one(unvalidated_password: &UnvalidatedPassword) -> anyhow::Result<bool> {
    let mut target_counter = 0;

    for c in unvalidated_password.password.chars() {
        if c == unvalidated_password.target {
            target_counter += 1;
        }
    }

    Ok(target_counter >= unvalidated_password.lower_limit
        && target_counter <= unvalidated_password.upper_limit)
}

pub fn part_two(input: &str) -> anyhow::Result<String> {
    let passwords = parse_into_unvalidated_passwords(input)?;
    Ok(validate(&passwords, validate_part_two).to_string())
}

fn validate_part_two(unvalidated_password: &UnvalidatedPassword) -> anyhow::Result<bool> {
    let at_first_position = unvalidated_password
        .password
        .chars()
        .nth(unvalidated_password.lower_limit - 1)
        .ok_or(anyhow!(
            "Could not get char at position {}",
            unvalidated_password.lower_limit - 1
        ))?;

    let at_second_position = unvalidated_password
        .password
        .chars()
        .nth(unvalidated_password.upper_limit - 1)
        .ok_or(anyhow!(
            "Could not get char at position {}",
            unvalidated_password.upper_limit - 1
        ))?;

    let is_at_first_position = at_first_position == unvalidated_password.target;
    let is_at_second_posi9tion = at_second_position == unvalidated_password.target;

    Ok(is_at_first_position ^ is_at_second_posi9tion)
}

fn validate<F>(passwords: &[UnvalidatedPassword], validator: F) -> impl Display
where
    F: Fn(&UnvalidatedPassword) -> anyhow::Result<bool>,
{
    let mut counter: u64 = 0;

    for password in passwords {
        let is_valid = validator(password).unwrap_or(false);

        if is_valid {
            counter += 1;
        }
    }

    counter
}

fn parse_into_unvalidated_passwords(input: &str) -> anyhow::Result<Vec<UnvalidatedPassword>> {
    let result = many1(unvalidated_password).parse(input);

    let result = result.map_err(|e| anyhow::Error::from(e.to_owned()));

    let (_, passwords) = result?;

    Ok(passwords)
}

fn unvalidated_password(i: &str) -> IResult<&str, UnvalidatedPassword> {
    let mut parser = (
        bounds,
        space,
        target,
        tag(":"),
        space,
        password,
        line_ending,
    );

    let (remaining, ((lower, upper), _, target, _, _, password, _)) = parser.parse(i)?;

    let unvalidated_password = UnvalidatedPassword::new(lower, upper, target, password);

    Ok((remaining, unvalidated_password))
}

fn bounds(i: &str) -> IResult<&str, (usize, usize)> {
    let mut parser = (integer, tag("-"), integer);

    let (remaining, (lower, _, upper)) = parser.parse(i)?;

    Ok((remaining, (lower, upper)))
}

fn password(i: &str) -> IResult<&str, &str> {
    alpha1(i)
}

fn target(i: &str) -> IResult<&str, char> {
    anychar(i)
}

fn integer(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>()).parse(i)
}

fn space(i: &str) -> IResult<&str, ()> {
    value((), tag(" ")).parse(i)
}

fn line_ending(i: &str) -> IResult<&str, ()> {
    value((), newline).parse(i)
}

#[derive(Debug, PartialEq)]
struct UnvalidatedPassword {
    lower_limit: usize,
    upper_limit: usize,
    target: char,
    password: String,
}

impl UnvalidatedPassword {
    pub fn new<S: Into<String>>(
        lower_limit: usize,
        upper_limit: usize,
        target: char,
        password: S,
    ) -> UnvalidatedPassword {
        UnvalidatedPassword {
            lower_limit,
            upper_limit,
            target,
            password: password.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let expected = UnvalidatedPassword::new(1, 3, 'a', "abcde");
        assert_eq!(unvalidated_password("1-3 a: abcde\n"), Ok(("", expected)))
    }
}
