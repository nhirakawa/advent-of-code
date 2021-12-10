use num::ParseIntError;
use std::fmt;
use std::fmt::Debug;

use std::fmt::Display;
use std::io;
use std::num;
use std::time::Duration;

pub type AdventOfCodeResult = Result<(PartAnswer, PartAnswer), AdventOfCodeError>;

#[derive(Default)]
pub struct PartAnswer {
    duration: Duration,
    answer: String,
}

impl PartAnswer {
    pub fn new<T: Display>(answer: T, duration: Duration) -> PartAnswer {
        let answer = format!("{}", answer);
        PartAnswer { duration, answer }
    }

    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    pub fn get_answer(&self) -> &str {
        &self.answer
    }
}

impl<I: Into<u64>> From<(I, Duration)> for PartAnswer {
    fn from(tuple: (I, Duration)) -> PartAnswer {
        let answer: String = tuple.0.into().to_string();
        let duration = tuple.1;

        PartAnswer { duration, answer }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AdventOfCodeError {
    Unimplemented,
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
            AdventOfCodeError::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}
