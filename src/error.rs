use num::ParseIntError;
use std::io;
use std::num;

#[derive(Debug, PartialEq)]
pub enum AdventOfCodeError {
    CannotOpenFile(String),
    CannotParseInteger(ParseIntError),
    NomParseError,
    CannotGetChar,
    NoAnswerFoundPartOne,
    NoAnswerFoundPartTwo,
}

impl From<io::Error> for AdventOfCodeError {
    fn from(err: io::Error) -> AdventOfCodeError {
        AdventOfCodeError::CannotOpenFile(err.to_string())
    }
}

impl From<num::ParseIntError> for AdventOfCodeError {
    fn from(err: num::ParseIntError) -> AdventOfCodeError {
        AdventOfCodeError::CannotParseInteger(err)
    }
}
