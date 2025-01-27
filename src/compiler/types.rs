use std::{fmt, str::FromStr};

use serde::Serialize;

use crate::types::basic::number::{double::Double, float::Float, int32::Int32, int64::Int64};

use super::{elements::token::Token, error::parse_error::ParseError};

#[derive(Debug, Serialize, PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum Types {
    Int32,
    Int64,
    /// High Precision Integer
    HPInt,
    Float,
    Double,
    String,
    Str,
    Boolean,
    Void,
    Function,
    List,
    Inferred,
}

#[allow(dead_code)]
impl Types {
    pub fn is_integer(&self) -> bool {
        match self {
            Self::Int32 | Self::Int64 | Self::HPInt => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Self::Double | Self::Float => true,
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

    pub fn inferred<'a>(value: &Token) -> Result<Self, ParseError> {
        match value {
            Token::Int32(_) => Ok(Self::Int32),
            Token::Int64(_) => Ok(Self::Int64),
            Token::HPInt(_) => Ok(Self::HPInt),
            Token::Double(_) => Ok(Self::Double),
            Token::Float(_) => Ok(Self::Float),
            Token::String(_) => Ok(Self::String),
            Token::Str(_) => Ok(Self::Str),
            Token::Type(types) => Ok(types.clone()),
            Token::Boolean(_) => Ok(Self::Boolean),
            _ => Err(ParseError::InvalidType(format!(
                "the type to '{:?}' doesn't exists.",
                Self::from(value)
            ))),
        }
    }

    pub fn transform(value: Token, to: Self) -> Result<(Token, Self), ParseError> {
        match (to, value) {
            (Types::Int32, Token::Int32(int32)) => Ok((Token::Int32(int32.clone()), Types::Int32)),
            (Types::Int32, Token::Int64(int64)) => {
                Ok((Token::Int32(Int32::from(*int64 as i32)), Types::Int32))
            }
            (Types::Int32, Token::HPInt(int128)) => {
                Ok((Token::Int32(Int32::from(*int128 as i32)), Types::Int32))
            }
            (Types::Int32, Token::Double(double)) => {
                Ok((Token::Int32(Int32::new(*double as i32)), Types::Int32))
            }
            (Types::Int32, Token::String(s)) => Ok((
                Token::Int32(Int32::new(s.chars().map(|c| c as i32).sum::<i32>())),
                Types::Int32,
            )),
            (Types::Int32, Token::Str(s)) => Ok((
                Token::Int32(Int32::new(s.chars().map(|c| c as i32).sum::<i32>())),
                Types::Int32,
            )),
            (Types::Int32, Token::Boolean(b)) => Ok((
                Token::Int32(if b { Int32::new(1) } else { Int32::new(0) }),
                Types::Int32,
            )),
            (Types::Int32, Token::Void) => Ok((Token::Int32(Int32::new(0)), Types::Int32)),

            (Types::Int64, Token::Int32(int32)) => {
                Ok((Token::Int64((*int32 as i64).into()), Types::Int64))
            }
            (Types::Int64, Token::Int64(int64)) => Ok((Token::Int64(int64.clone()), Types::Int64)),
            (Types::Int64, Token::HPInt(int128)) => {
                Ok((Token::Int64((*int128 as i64).into()), Types::Int64))
            }
            (Types::Int64, Token::Double(double)) => {
                Ok((Token::Int64(Int64::new(*double as i64)), Types::Int64))
            }
            (Types::Int64, Token::String(s)) => Ok((
                Token::Int64(Int64::new(s.chars().map(|c| c as i64).sum::<i64>())),
                Types::Int64,
            )),
            (Types::Int64, Token::Str(s)) => Ok((
                Token::Int64(Int64::new(s.chars().map(|c| c as i64).sum::<i64>())),
                Types::Int64,
            )),
            (Types::Int64, Token::Boolean(b)) => Ok((
                Token::Int64(if b { Int64::new(1) } else { Int64::new(0) }),
                Types::Int64,
            )),
            (Types::Int64, Token::Void) => Ok((Token::Int64(Int64::new(0)), Types::Int64)),

            (Types::Float, Token::Int32(int32)) => {
                Ok((Token::Float(Float::new(*int32 as f32)), Types::Float))
            }
            (Types::Float, Token::Int64(int64)) => {
                Ok((Token::Float(Float::new(*int64 as f32)), Types::Float))
            }
            (Types::Float, Token::HPInt(int128)) => {
                Ok((Token::Float(Float::new(*int128 as f32)), Types::Float))
            }
            (Types::Float, Token::Float(double)) => {
                Ok((Token::Float(double.clone()), Types::Float))
            }
            (Types::Float, Token::Double(double)) => {
                Ok((Token::Float(Float::new(*double as f32)), Types::Float))
            }
            (Types::Float, Token::String(s)) => Ok((
                Token::Float(Float::new(
                    s.chars().map(|c| (c as i64) as f32).sum::<f32>(),
                )),
                Types::Float,
            )),
            (Types::Float, Token::Str(s)) => Ok((
                Token::Float(Float::new(
                    s.chars().map(|c| (c as i64) as f32).sum::<f32>(),
                )),
                Types::Float,
            )),
            (Types::Float, Token::Boolean(b)) => Ok((
                Token::Float(if b { Float::new(1.0) } else { Float::new(0.0) }),
                Types::Float,
            )),
            (Types::Float, Token::Void) => Ok((Token::Float(Float::new(0.0)), Types::Float)),

            (Types::Double, Token::Int32(int32)) => {
                Ok((Token::Double(Double::new(*int32 as f64)), Types::Double))
            }
            (Types::Double, Token::Int64(int64)) => {
                Ok((Token::Double(Double::new(*int64 as f64)), Types::Double))
            }
            (Types::Double, Token::HPInt(int128)) => {
                Ok((Token::Double(Double::new(*int128 as f64)), Types::Double))
            }
            (Types::Double, Token::Double(double)) => {
                Ok((Token::Double(double.clone()), Types::Double))
            }
            (Types::Double, Token::Float(double)) => {
                Ok((Token::Double(Double::new(*double as f64)), Types::Double))
            }
            (Types::Double, Token::String(s)) => Ok((
                Token::Double(Double::new(
                    s.chars().map(|c| (c as i64) as f64).sum::<f64>(),
                )),
                Types::Double,
            )),
            (Types::Double, Token::Str(s)) => Ok((
                Token::Double(Double::new(
                    s.chars().map(|c| (c as i64) as f64).sum::<f64>(),
                )),
                Types::Double,
            )),
            (Types::Double, Token::Boolean(b)) => Ok((
                Token::Double(if b {
                    Double::new(1.0)
                } else {
                    Double::new(0.0)
                }),
                Types::Double,
            )),
            (Types::Double, Token::Void) => Ok((Token::Double(Double::new(0.0)), Types::Double)),

            (Types::String, v) => {
                Ok((Token::String(v.clone().str_value().to_string()), Types::Str))
            }
            (Types::Str, v) => Ok((Token::Str(v.str_value().into()), Types::Str)),
            (Types::Boolean, v) => Ok((Token::Boolean(v.as_bool()), Types::Boolean)),
            (Types::Void, _) => Ok((Token::Void, Types::Void)),
            (Types::Function, v) => Ok((v.clone(), Types::Function)),
            (Types::Inferred, v) => Ok((v.clone(), Self::from(v.clone()))),
            _ => Err(ParseError::InvalidTypeConvertion(format!(""))),
        }
    }
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Types::Int32 => write!(f, "Int32"),
            Types::Int64 => write!(f, "Int64"),
            Types::HPInt => write!(f, "HPInt"),
            Types::Float => write!(f, "Float"),
            Types::Double => write!(f, "Double"),
            Types::String => write!(f, "String"),
            Types::Str => write!(f, "Str"),
            Types::Boolean => write!(f, "Boolean"),
            Types::Void => write!(f, "Void"),
            Types::Function => write!(f, "Function"),
            Types::Inferred => write!(f, "Inferred"),
            Types::List => write!(f, "List"),
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
            "HPInt" => Ok(Self::HPInt),
            "Float" => Ok(Self::Float),
            "Double" => Ok(Self::Double),
            "String" => Ok(Self::String),
            "Str" => Ok(Self::Str),
            "Void" => Ok(Self::Void),
            "Function" => Ok(Self::Function),
            _ => Err(ParseError::InvalidType(format!("Invalid type: {s}"))),
        }
    }
}

impl From<&Token> for Types {
    fn from(value: &Token) -> Self {
        match value {
            Token::Boolean(_) => Self::Boolean,
            Token::Int32(_) => Self::Int32,
            Token::Int64(_) => Self::Int64,
            Token::HPInt(_) => Self::HPInt,
            Token::Float(_) => Self::Float,
            Token::Double(_) => Self::Double,
            Token::String(_) => Self::String,
            Token::Str(_) => Self::Str,
            Token::Void => Self::Void,
            // Token::Function => Self::Function,
            _ => Self::Void,
        }
    }
}

impl From<Token> for Types {
    fn from(value: Token) -> Self {
        match value {
            Token::Boolean(_) => Self::Boolean,
            Token::Int32(_) => Self::Int32,
            Token::Int64(_) => Self::Int64,
            Token::HPInt(_) => Self::HPInt,
            Token::Float(_) => Self::Float,
            Token::Double(_) => Self::Double,
            Token::String(_) => Self::String,
            Token::Str(_) => Self::Str,
            Token::Void => Self::Void,
            // Token::Function => Self::Function,
            _ => Self::Void,
        }
    }
}
