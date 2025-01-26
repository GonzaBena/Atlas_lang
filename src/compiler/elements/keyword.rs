use std::{fmt::Display, str::FromStr};

use serde::Serialize;

use crate::compiler::error::lexic_error::LexicError;

/// The `Keyword` enum represents various keywords in the Atlas language.
///
/// # Variants
///
/// - `Var`: Represents the `var` keyword.
/// - `Const`: Represents the `const` keyword.
/// - `For`: Represents the `for` keyword.
/// - `While`: Represents the `while` keyword.
/// - `If`: Represents the `if` keyword.
/// - `Else`: Represents the `else` keyword.
/// - `Function`: Represents the `func` keyword.
/// - `EOF`: Represents the end of file.
/// - `True`: Represents the `true` keyword or a true boolean value.
/// - `False`: Represents the `false` keyword or a false boolean value.
///
/// # Example
///
/// ```
/// use crate::compiler::elements::keyword::Keyword;
/// use std::str::FromStr;
///
/// let keyword = Keyword::from_str("var").unwrap();
/// assert_eq!(keyword, Keyword::Var);
/// ```
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
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
