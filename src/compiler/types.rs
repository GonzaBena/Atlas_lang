use std::str::FromStr;

use serde::Serialize;

use super::{elements::token::Token, error::parse_error::ParseError};

#[derive(Debug, Serialize, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Types {
    Int32,
    Int64,
    Double,
    Number,
    String,
    Str,
    Boolean,
    Void,
    Function,
    Inferred,
}

#[allow(dead_code)]
impl Types {
    pub fn is_integer(&self) -> bool {
        match self {
            Self::Int32 | Self::Int64 | Self::Number => true,
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

    pub fn cmp<T>(&self, other: T) -> bool
    where
        T: Into<Types>,
    {
        let other = other.into();
        println!("self: {self:?}, other: {other:?}");
        if self.is_numeric() && other.is_numeric() {
            return true;
        }

        match self {
            Self::Int64 => match other {
                Self::Int32 => true,
                _ => false,
            },
            Self::Double => match other {
                Self::Int32 | Self::Int64 => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl FromStr for Types {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Boolean" => Ok(Self::Boolean),
            "Int32" => Ok(Self::Int32),
            "Int64" => Ok(Self::Int64),
            "Double" => Ok(Self::Double),
            "Number" => Ok(Self::Number),
            "String" => Ok(Self::String),
            "Str" => Ok(Self::Str),
            "Void" => Ok(Self::Void),
            "Function" => Ok(Self::Function),
            _ => Err(ParseError::InvalidType(format!("Invalid type: {s}"))),
        }
    }
}

impl From<&Token<'_>> for Types {
    fn from(value: &Token<'_>) -> Self {
        match value {
            Token::Boolean(_) => Self::Boolean,
            Token::Int32(_) => Self::Int32,
            Token::Int64(_) => Self::Int64,
            Token::Double(_) => Self::Double,
            Token::Number(num) => num.as_type(),
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
