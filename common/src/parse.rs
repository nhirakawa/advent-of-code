use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, space0},
    combinator::{map, map_opt},
    sequence::{delimited, preceded},
    IResult,
};
use std::ops::Neg;

pub type ParseResult<'a, O> = IResult<&'a str, O, nom::error::VerboseError<&'a str>>;

pub fn number<T: Neg<Output = T> + FromStr>(i: &str) -> IResult<&str, T> {
    alt((negative_number, unsigned_number))(i)
}

pub fn unsigned_number<T: FromStr>(i: &str) -> IResult<&str, T> {
    map_opt(digit1, |s: &str| s.parse::<T>().ok())(i)
}

pub fn negative_number<T: Neg<Output = T> + FromStr>(i: &str) -> IResult<&str, T> {
    map(preceded(tag("-"), unsigned_number), T::neg)(i)
}

pub fn whitespace<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn spaces<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    delimited(space0, inner, space0)
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
