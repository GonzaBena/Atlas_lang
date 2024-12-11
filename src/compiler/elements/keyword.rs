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
    EOF,
    True,
    False,
}

// impl ToString for Keyword {
//     fn to_string(&self) -> String {
//         match *self {
//             Keyword::Var => String::from("Var"),
//             Keyword::Const => String::from("Const"),
//             Keyword::For => String::from("For"),
//             Keyword::While => String::from("While"),
//             Keyword::If => String::from("If"),
//             Keyword::Else => String::from("Else"),
//             Keyword::EOF => String::from("EOF"),
//             Keyword::True => String::from("True"),
//             Keyword::False => String::from("False"),
//             // _ => String::new(),
//         }
//     }
// }

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Keyword::Var => write!(f, "Var"),
            Keyword::Const => write!(f, "Const"),
            Keyword::For => write!(f, "For"),
            Keyword::While => write!(f, "While"),
            Keyword::If => write!(f, "If"),
            Keyword::Else => write!(f, "Else"),
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
            _ => Err(LexicError::InvalidKeyword(format!(
                "the word '{s}' isn't a Keyword."
            ))),
        };
    }
}
