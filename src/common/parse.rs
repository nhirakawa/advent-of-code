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

pub fn griderator(input: &str) -> impl Iterator<Item = ((isize, isize), char)> + '_ {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as isize, y as isize), ch))
        })
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

    #[test]
    fn test_griderator_simple() {
        let input = "ab\ncd";
        let result: Vec<_> = griderator(input).collect();
        
        assert_eq!(result, vec![
            ((0, 0), 'a'),
            ((1, 0), 'b'),
            ((0, 1), 'c'),
            ((1, 1), 'd'),
        ]);
    }

    #[test]
    fn test_griderator_empty_lines() {
        let input = "a\n\nb";
        let result: Vec<_> = griderator(input).collect();
        
        assert_eq!(result, vec![
            ((0, 0), 'a'),
            ((0, 2), 'b'),
        ]);
    }

    #[test]
    fn test_griderator_coordinates() {
        let input = "123\n456\n789";
        let result: Vec<_> = griderator(input).collect();
        
        // Verify coordinate system: (0,0) at top-left, x increases right, y increases down
        assert_eq!(result[0], ((0, 0), '1')); // top-left
        assert_eq!(result[2], ((2, 0), '3')); // top-right
        assert_eq!(result[6], ((0, 2), '7')); // bottom-left
        assert_eq!(result[8], ((2, 2), '9')); // bottom-right
    }
}
