use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ParseError {
    /// Indicate that the file doesn't correctly finish
    UndefinedEOF,
    SyntaxError(String),
    UndefinedVariable(String),
}

impl fmt::Display for ParseError {
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

impl Error for ParseError {}
