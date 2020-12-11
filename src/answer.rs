use num::ParseIntError;
use std::fmt;
use std::io;
use std::num;
use std::time::Duration;

pub type AdventOfCodeResult = Result<(PartAnswer, PartAnswer), AdventOfCodeError>;
pub type AnswerWithTiming = (u64, Duration);
pub type PartAnswer = Result<AnswerWithTiming, AdventOfCodeError>;

#[derive(Debug, PartialEq, Clone)]
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

impl fmt::Display for AdventOfCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdventOfCodeError::CannotOpenFile(s) => write!(f, "{}", s),
            AdventOfCodeError::CannotParseInteger(p) => write!(f, "{}", p),
            AdventOfCodeError::NomParseError => write!(f, "nom parse error"),
            AdventOfCodeError::CannotGetChar => write!(f, "cannot get char"),
            AdventOfCodeError::NoAnswerFoundPartOne => write!(f, "no answer found for part one"),
            AdventOfCodeError::NoAnswerFoundPartTwo => write!(f, "no answer found for part two"),
            _ => write!(f, "unknown error"),
        }
    }
}
