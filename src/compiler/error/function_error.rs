use std::{error::Error, fmt};

#[derive(Debug)]
#[allow(dead_code)]
pub enum FunctionError {
    /// Indicate that the file doesn't correctly finish
    UndefinedEOF,
    InvalidNumberOfArgs(String),
    ExecutionError(String),
    DifferentReturnType(String),
}

impl fmt::Display for FunctionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UndefinedEOF => write!(
                f,
                "The End of File couldn't be defined. \nThe file could be damaged or corrupted. "
            ),
            v => write!(f, "{v}"),
        }
    }
}

impl Error for FunctionError {}
