use core::str;
use std::{num::ParseIntError, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0, space0},
    combinator::{all_consuming, map, map_res},
    error::ParseError,
    sequence::{delimited, preceded, terminated},
    AsChar, IResult, Parser,
};
use nom_language::error::VerboseError;
use std::ops::Neg;

pub type ParseResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

pub fn number<T: Neg<Output = T> + FromStr<Err = ParseIntError>>(i: &str) -> IResult<&str, T> {
    alt((negative_number, unsigned_number)).parse(i)
}

pub fn unsigned_number<T: FromStr<Err = ParseIntError>>(i: &str) -> IResult<&str, T> {
    map_res(digit1, |s: &str| s.parse().map_err(anyhow::Error::from)).parse(i)
}

pub fn negative_number<T: Neg<Output = T> + FromStr<Err = ParseIntError>>(
    i: &str,
) -> IResult<&str, T> {
    map(preceded(tag("-"), unsigned_number), T::neg).parse(i)
}

pub fn whitespace<F, O>(inner: F) -> impl Fn(&str) -> IResult<&str, O>
where
    F: Fn(&str) -> IResult<&str, O>,
{
    move |input| delimited(multispace0, &inner, multispace0).parse(input)
}

pub fn finish<I, O, E, P>(parser: P, input: I) -> anyhow::Result<O>
where
    I: nom::Input,
    <I as nom::Input>::Item: AsChar,
    P: Parser<I, Output = O, Error = E>,
    E: std::fmt::Debug + ParseError<I>,
{
    match all_consuming(terminated(parser, multispace0)).parse(input) {
        Ok((_, output)) => Ok(output),
        Err(e) => Err(anyhow::anyhow!("Parse error: {:?}", e)),
    }
}

pub fn spaces<F, O>(inner: F) -> impl Fn(&str) -> IResult<&str, O>
where
    F: Fn(&str) -> IResult<&str, O>,
{
    move |input| delimited(space0, &inner, space0).parse(input)
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
