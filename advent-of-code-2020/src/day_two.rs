use common::prelude::*;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1, newline},
    combinator::{map_res, value},
    multi::many1,
    sequence::tuple,
    IResult,
};

pub fn run() -> AdventOfCodeResult<u64, u64> {
    let input = include_str!("../input/day-2.txt");

    let passwords = parse_into_unvalidated_passwords(input)?;

    let answer_one = validate(&passwords, validate_part_one);
    let answer_two = validate(&passwords, validate_part_two);

    Ok((answer_one, answer_two))
}

fn validate_part_one(
    unvalidated_password: &UnvalidatedPassword,
) -> Result<bool, AdventOfCodeError> {
    let mut target_counter = 0;

    for c in unvalidated_password.password.chars() {
        if c == unvalidated_password.target {
            target_counter += 1;
        }
    }

    Ok(target_counter >= unvalidated_password.lower_limit
        && target_counter <= unvalidated_password.upper_limit)
}

fn validate_part_two(
    unvalidated_password: &UnvalidatedPassword,
) -> Result<bool, AdventOfCodeError> {
    let at_first_position = unvalidated_password
        .password
        .chars()
        .nth(unvalidated_password.lower_limit - 1)
        .ok_or(AdventOfCodeError::CannotGetChar)?;

    let at_second_position = unvalidated_password
        .password
        .chars()
        .nth(unvalidated_password.upper_limit - 1)
        .ok_or(AdventOfCodeError::CannotGetChar)?;

    let is_at_first_position = at_first_position == unvalidated_password.target;
    let is_at_second_posi9tion = at_second_position == unvalidated_password.target;

    Ok(is_at_first_position ^ is_at_second_posi9tion)
}

fn validate<F>(passwords: &Vec<UnvalidatedPassword>, validator: F) -> PartAnswer<u64>
where
    F: Fn(&UnvalidatedPassword) -> Result<bool, AdventOfCodeError>,
{
    let start = SystemTime::now();
    let mut counter: u64 = 0;

    for password in passwords {
        let is_valid = validator(password).unwrap_or(false);

        if is_valid {
            counter += 1;
        }
    }

    let elapsed = start.elapsed().unwrap();

    (counter, elapsed).into()
}

fn parse_into_unvalidated_passwords(
    input: &str,
) -> Result<Vec<UnvalidatedPassword>, AdventOfCodeError> {
    let result = many1(unvalidated_password)(input);

    let result = result.map_err(|_err| AdventOfCodeError::NomParseError);

    let (_, passwords) = result?;

    Ok(passwords)
}

fn unvalidated_password(i: &str) -> IResult<&str, UnvalidatedPassword> {
    let mut parser = tuple((
        bounds,
        space,
        target,
        tag(":"),
        space,
        password,
        line_ending,
    ));

    let (remaining, ((lower, upper), _, target, _, _, password, _)) = parser(i)?;

    let unvalidated_password = UnvalidatedPassword::new(lower, upper, target, password);

    Ok((remaining, unvalidated_password))
}

fn bounds(i: &str) -> IResult<&str, (usize, usize)> {
    let mut parser = tuple((integer, tag("-"), integer));

    let (remaining, (lower, _, upper)) = parser(i)?;

    Ok((remaining, (lower, upper)))
}

fn password(i: &str) -> IResult<&str, &str> {
    alpha1(i)
}

fn target(i: &str) -> IResult<&str, char> {
    anychar(i)
}

fn integer(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}

fn space(i: &str) -> IResult<&str, ()> {
    value((), tag(" "))(i)
}

fn line_ending(i: &str) -> IResult<&str, ()> {
    value((), newline)(i)
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

    #[test]
    fn test_answers() {
        let (part_one, part_two) = run().unwrap();

        assert_eq!(*part_one.get_answer(), 560);
        assert_eq!(*part_two.get_answer(), 303);
    }
}
