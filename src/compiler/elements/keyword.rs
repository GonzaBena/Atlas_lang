use std::{fmt::Display, str::FromStr};

use crate::compiler::error::lexic_error::LexicError;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Keyword {
    Var,
    Const,
    For,
    While,
    If,
    Else,
    Function,
    EOF,
    True,
    False,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Keyword::Var => write!(f, "Var"),
            Keyword::Const => write!(f, "Const"),
            Keyword::For => write!(f, "For"),
            Keyword::While => write!(f, "While"),
            Keyword::If => write!(f, "If"),
            Keyword::Else => write!(f, "Else"),
            Keyword::Function => write!(f, "Function"),
            Keyword::EOF => write!(f, "EOF"),
            Keyword::True => write!(f, "True"),
            Keyword::False => write!(f, "False"),
            // _ => String::new(),
        }
    }
}

impl FromStr for Keyword {
    type Err = LexicError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.trim() {
            "var" => Ok(Keyword::Var),
            "const" => Ok(Keyword::Const),
            "for" => Ok(Keyword::For),
            "while" => Ok(Keyword::While),
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "EOF" => Ok(Keyword::EOF),
            "func" => Ok(Keyword::Function),
            _ => Err(LexicError::InvalidKeyword(format!(
                "the word '{s}' isn't a Keyword."
            ))),
        };
    }
}
