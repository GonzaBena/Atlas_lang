use std::str::FromStr;

use super::{elements::token::Token, error::parse_error::ParseError};

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Types {
    Int32,
    Double,
    String,
    Str,
    Boolean,
    Void,
    Function,
}

#[allow(dead_code)]
impl Types {
    pub fn is_integer(&self) -> bool {
        match self {
            Self::Int32 => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Self::Double => true,
            _ => false,
        }
    }

    pub fn is_numeric(&self) -> bool {
        self.is_float() || self.is_integer()
    }
}

impl FromStr for Types {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Boolean" => Ok(Self::Boolean),
            "Int32" => Ok(Self::Int32),
            "Double" => Ok(Self::Double),
            "String" => Ok(Self::String),
            "Str" => Ok(Self::Str),
            "Void" => Ok(Self::Void),
            "Function" => Ok(Self::Function),
            _ => Err(ParseError::InvalidType("invalid type".into())),
        }
    }
}

impl From<&Token<'_>> for Types {
    fn from(value: &Token<'_>) -> Self {
        match value {
            Token::Boolean(_) => Self::Boolean,
            Token::Int32(_) => Self::Int32,
            Token::Double(_) => Self::Double,
            Token::String(_) => Self::String,
            Token::Str(_) => Self::Str,
            Token::Void => Self::Void,
            // Token::Function => Self::Function,
            _ => Self::Void,
        }
    }
}

impl From<Token<'_>> for Types {
    fn from(value: Token<'_>) -> Self {
        match value {
            Token::Boolean(_) => Self::Boolean,
            Token::Int32(_) => Self::Int32,
            Token::Double(_) => Self::Double,
            Token::String(_) => Self::String,
            // Token::Str => Self::Str,
            Token::Void => Self::Void,
            // Token::Function => Self::Function,
            _ => Self::Void,
        }
    }
}
