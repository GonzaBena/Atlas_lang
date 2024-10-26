use std::error::Error as Err;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseError {
    SyntaxError(String),

    UndefinedVariable(String),

    UnexpectedToken(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            ParseError::SyntaxError(msg) => write!(f, "Syntax Error: {}", msg),
            ParseError::UndefinedVariable(msg) => write!(f, "Undefined Variable: {}", msg),
            ParseError::UnexpectedToken(msg) => write!(f, "Unexpected Token: {}", msg),
        }
    }
}

impl Err for ParseError {}
