// use std::{error::Error, fmt};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexicError {
    #[error("InvalidIdentifier: {0}")]
    InvalidIdentifier(String),

    #[error("InvalidKeyword: {0}")]
    InvalidKeyword(String),

    #[error("InvalidNumber: {0}")]
    InvalidNumber(String),

    #[error("UnfinalizedStringError: You didn't finish the string.")]
    UnfinalizedString,
}

// impl fmt::Display for LexicError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::InvalidIdentifier(txt) => write!(f, "InvalidIdentifier: {txt}"),
//             Self::InvalidKeyword(txt) => write!(f, "InvalidKeyword: {txt}"),
//             Self::InvalidNumber(txt) => write!(f, "InvalidNumber: {txt}"),
//             Self::UnfinalizedString => {
//                 write!(f, "UnfinalizedStringError: You didn't finish the string.")
//             } // _ => write!(f, ""),
//         }
//     }
// }

// impl Error for LexicError {}
