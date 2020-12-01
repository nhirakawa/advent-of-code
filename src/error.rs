use std::io;
use std::num;

use num::ParseIntError;

#[derive(Debug)]
pub enum AdventOfCodeError {
    CannotOpenFile(String),
    CannotParseInteger(ParseIntError),
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
