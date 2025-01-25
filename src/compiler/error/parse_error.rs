use std::{error::Error, fmt};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ParseError {
    /// Indicate that the file doesn't correctly finish
    UndefinedEOF,
    SyntaxError(String),
    UndefinedVariable(String),
    UndefinedFunction(String),
    UndefinedType(String),
    InvalidType(String),
    TypeError(String),
    DefinedVariable(String),
    DefinedFunction(String),
    FunctionExecution(String),
    InvalidTypeConvertion(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UndefinedEOF => write!(
                f,
                "The End of File couldn't be defined. \nThe file could be damaged or corrupted. "
            ),
            Self::SyntaxError(txt) => write!(f, "SyntaxError: {txt}"),
            Self::InvalidType(txt) => {
                write!(f, "InvalidTypeError: The type '{txt}' doesn't exists.")
            }
            Self::DefinedVariable(txt) => write!(
                f,
                "DefinedVariableError: The variable '{txt}' already was defined."
            ),

            Self::DefinedFunction(txt) => write!(
                f,
                "DefinedFunctionError: The function '{txt}' already was defined."
            ),
            v => write!(f, "{v}"),
        }
    }
}

impl Error for ParseError {}
