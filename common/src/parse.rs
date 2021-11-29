use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{map, map_opt},
    error::ParseError,
    sequence::{delimited, preceded},
    IResult,
};
use std::ops::Neg;

pub fn number<T: Neg<Output = T> + FromStr>(i: &str) -> IResult<&str, T> {
    alt((negative_number, unsigned_number))(i)
}

pub fn unsigned_number<T: FromStr>(i: &str) -> IResult<&str, T> {
    map_opt(digit1, |s: &str| s.parse::<T>().ok())(i)
}

pub fn negative_number<T: Neg<Output = T> + FromStr>(i: &str) -> IResult<&str, T> {
    map(preceded(tag("-"), unsigned_number), T::neg)(i)
}

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_number() {
        assert_eq!(negative_number("-100"), Ok(("", -100)));
    }

    #[test]
    fn test_unsigned_number() {
        assert_eq!(unsigned_number("42"), Ok(("", 42)));
    }

    #[test]
    fn test_number() {
        assert_eq!(negative_number("-100"), Ok(("", -100)));
        assert_eq!(unsigned_number("42"), Ok(("", 42)));
    }
}
