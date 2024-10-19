use std::error::Error as Err;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
#[allow(dead_code)]
pub enum LexicError {
    NumberError(String),
    StringError(String),
    EOFError(String),
    OperatorError(String),
    SyntaxError(String),
    OperandError(String),
}

impl Display for LexicError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            LexicError::NumberError(number) => write!(f, "NumberError: {}", number),
            LexicError::StringError(string) => write!(f, "StringError: {}", string),
            LexicError::EOFError(end) => write!(f, "EOFError: {}", end),
            LexicError::OperatorError(operator) => write!(f, "OperatorError: {}", operator),
            LexicError::SyntaxError(syntax) => write!(f, "SyntaxError: {}", syntax),
            LexicError::OperandError(operand) => write!(f, "OperandError: {}", operand),
        }
    }
}

impl Err for LexicError {}
