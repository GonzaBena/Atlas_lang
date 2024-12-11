use std::{error::Error, fmt};

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseError<'a> {
    /// Indicate that the file doesn't correctly finish
    UndefinedEOF,
    SyntaxError(&'a str),
    UndefinedVariable(&'a str),
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UndefinedEOF => write!(
                f,
                "The End of File couldn't be defined. \nThe file could be damaged or corrupted. "
            ),
            Self::SyntaxError(txt) => write!(f, "SyntaxError: {txt}"),
            v => write!(f, "{v}"),
        }
    }
}

impl Error for ParseError<'_> {}
