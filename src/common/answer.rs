use std::fmt;
use std::fmt::Display;

pub struct PixelatedString {
    s: String,
}

impl PixelatedString {
    pub fn new<I>(i: I) -> PixelatedString
    where
        I: Into<String>,
    {
        let s = i.into();
        PixelatedString { s }
    }
}

impl Display for PixelatedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{}\n", self.s)
    }
}
