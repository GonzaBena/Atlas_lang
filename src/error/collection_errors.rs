use std::error::Error as Err;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq)]
pub enum CollectionErrors {
    IndexOutOfBounds(String),
}

impl Display for CollectionErrors {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            CollectionErrors::IndexOutOfBounds(msg) => {
                write!(f, "Index Out of Bounds: {}", msg)
            } // CollectionErrors::NumberError(number) => write!(f, "NumberError: {}", number),
              // CollectionErrors::StringError(string) => write!(f, "StringError: {}", string),
              // CollectionErrors::EOFError(end) => write!(f, "EOFError: {}", end),
              // CollectionErrors::OperatorError(operator) => write!(f, "OperatorError: {}", operator),
              // CollectionErrors::SyntaxError(syntax) => write!(f, "SyntaxError: {}", syntax),
              // CollectionErrors::OperandError(operand) => write!(f, "OperandError: {}", operand),
              // CollectionErrors::UnterminatedString => write!(f, "UnterminatedString"),
        }
    }
}

impl Err for CollectionErrors {}
