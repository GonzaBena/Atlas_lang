use std::{error::Error, fmt};

#[derive(Debug)]
pub enum LexicError {
    InvalidIdentifier(String),
    InvalidKeyword(String),
    InvalidNumber(String),
}

impl fmt::Display for LexicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier(txt) => write!(f, "InvalidIdentifier: {txt}"),
            Self::InvalidKeyword(txt) => write!(f, "InvalidKeyword: {txt}"),
            Self::InvalidNumber(txt) => write!(f, "InvalidNumber: {txt}"),
            // _ => write!(f, ""),
        }
    }
}

impl Error for LexicError {}
