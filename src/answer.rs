use num::ParseIntError;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::num;
use std::time::Duration;

pub type AdventOfCodeResult<T: Display + Default, U: Display + Default> =
    Result<(PartAnswer<T>, PartAnswer<U>), AdventOfCodeError>;

#[derive(Default)]
pub struct PartAnswer<T: Display + Default> {
    duration: Duration,
    answer: T,
}

impl<T: Display + Default> PartAnswer<T> {
    pub fn new(answer: T, duration: Duration) -> PartAnswer<T> {
        PartAnswer { answer, duration }
    }

    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    pub fn get_answer(&self) -> &T {
        &self.answer
    }
}

impl<I: Into<u64>> From<(I, Duration)> for PartAnswer<u64> {
    fn from(tuple: (I, Duration)) -> PartAnswer<u64> {
        let answer = tuple.0.into();
        let duration = tuple.1;

        PartAnswer { answer, duration }
    }
}

impl<I: Into<String>> From<(I, Duration)> for PartAnswer<String> {
    fn from(tuple: (I, Duration)) -> PartAnswer<String> {
        let answer = tuple.0.into();
        let duration = tuple.1;

        PartAnswer { answer, duration }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AdventOfCodeError {
    CannotOpenFile(String),
    CannotParseInteger(ParseIntError),
    NomParseError,
    CannotGetChar,
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
        }
    }
}
