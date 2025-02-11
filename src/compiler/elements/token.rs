use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use super::{keyword::Keyword, operation::Operation, operator::Operator};
use crate::{
    compiler::{error::parse_error::ParseError, types::Types},
    types::basic::number::{
        double::Double, float::Float, hpint::HPInt, int32::Int32, int64::Int64,
    },
};

/// Represent each possible token which you can use.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Token {
    // Basic
    /// used to represent variables or functions
    Identifier(Arc<str>),
    Keyword(Keyword),
    Operation(Operation),
    Operator(Operator),
    Type(Types),
    // Argument(Argument<'a>),

    // Datatypes
    Int32(Int32),
    Int64(Int64),
    HPInt(HPInt),
    Float(Float),
    Double(Double),
    String(String),
    Str(Arc<str>),
    Boolean(bool),
    List(Vec<Token>),
    // Function(Arc<str>),

    // Others
    StartParenthesis, // (
    EndParenthesis,   // )
    StartBracket,     // [
    EndBracket,       // ]
    StartBrace,       // {
    EndBrace,         // }
    Separator(char),  // ',', ';'
    NewLine,          // \n
    EOF,              // EOF
    Void,             // void
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Identifier(val1), Token::Identifier(val2)) => val1 == val2,

            (Token::Keyword(val1), Token::Keyword(val2)) => val1 == val2,

            (Token::Operation(val1), Token::Operation(val2)) => val1 == val2,

            (Token::Operator(val1), Token::Operator(val2)) => val1 == val2,

            (Token::Type(val1), Token::Type(val2)) => val1 == val2,

            (Token::Int32(val1), Token::Int32(val2)) => **val1 as i32 == **val2,
            (Token::Int32(val1), Token::Int64(val2)) => **val1 as i64 == **val2,
            (Token::Int32(val1), Token::HPInt(val2)) => **val1 as i128 == **val2,
            (Token::Int32(val1), Token::Float(val2)) => **val1 as f32 == **val2,
            (Token::Int32(val1), Token::Double(val2)) => **val1 == **val2 as i32,
            (Token::Int32(val1), Token::String(val2)) => val1.to_string() == *val2,
            (Token::Int32(val1), Token::Str(val2)) => val1.to_string() == val2.to_string(),
            (Token::Int32(val1), Token::Boolean(val2)) => {
                (**val1 > 0 && *val2) || (**val1 <= 0 && !*val2)
            }

            (Token::Int64(val1), Token::Int32(val2)) => **val1 as i32 == **val2,
            (Token::Int64(val1), Token::Int64(val2)) => **val1 as i64 == **val2,
            (Token::Int64(val1), Token::HPInt(val2)) => **val1 as i128 == **val2,
            (Token::Int64(val1), Token::Float(val2)) => **val1 as f32 == **val2,
            (Token::Int64(val1), Token::Double(val2)) => **val1 == **val2 as i64,
            (Token::Int64(val1), Token::String(val2)) => val1.to_string() == *val2,
            (Token::Int64(val1), Token::Str(val2)) => val1.to_string() == val2.to_string(),
            (Token::Int64(val1), Token::Boolean(val2)) => {
                (**val1 > 0 && *val2) || (**val1 <= 0 && !*val2)
            }

            (Token::HPInt(val1), Token::Int32(val2)) => **val1 as i32 == **val2,
            (Token::HPInt(val1), Token::Int64(val2)) => **val1 as i64 == **val2,
            (Token::HPInt(val1), Token::HPInt(val2)) => **val1 as i128 == **val2,
            (Token::HPInt(val1), Token::Float(val2)) => **val1 as f32 == **val2,
            (Token::HPInt(val1), Token::Double(val2)) => **val1 == **val2 as i128,
            (Token::HPInt(val1), Token::String(val2)) => val1.to_string() == *val2,
            (Token::HPInt(val1), Token::Str(val2)) => val1.to_string() == val2.to_string(),
            (Token::HPInt(val1), Token::Boolean(val2)) => {
                (**val1 > 0 && *val2) || (**val1 <= 0 && !*val2)
            }

            (Token::Float(val1), Token::Int32(val2)) => **val1 as i32 == **val2,
            (Token::Float(val1), Token::Int64(val2)) => **val1 as i64 == **val2,
            (Token::Float(val1), Token::HPInt(val2)) => **val1 as i128 == **val2,
            (Token::Float(val1), Token::Float(val2)) => **val1 as f32 == **val2,
            (Token::Float(val1), Token::Double(val2)) => **val1 == **val2 as f32,
            (Token::Float(val1), Token::String(val2)) => val1.to_string() == *val2,
            (Token::Float(val1), Token::Str(val2)) => val1.to_string() == val2.to_string(),
            (Token::Float(val1), Token::Boolean(val2)) => {
                (**val1 > 0.0 && *val2) || (**val1 <= 0.0 && !*val2)
            }

            (Token::Double(val1), Token::Int32(val2)) => **val1 as i32 == **val2,
            (Token::Double(val1), Token::Int64(val2)) => **val1 as i64 == **val2,
            (Token::Double(val1), Token::HPInt(val2)) => **val1 as i128 == **val2,
            (Token::Double(val1), Token::Float(val2)) => **val1 as f32 == **val2,
            (Token::Double(val1), Token::Double(val2)) => val1 == val2,
            (Token::Double(val1), Token::String(val2)) => val1.to_string() == *val2,
            (Token::Double(val1), Token::Str(val2)) => val1.to_string() == val2.to_string(),
            (Token::Double(val1), Token::Boolean(val2)) => {
                (**val1 > 0.0 && *val2) || (**val1 <= 0.0 && !*val2)
            }

            (Token::String(val1), Token::String(val2)) => val1 == val2,
            (Token::String(val1), Token::Str(val2)) => *val1 == val2.to_string(),

            (Token::Str(val1), Token::String(val2)) => val1.to_string() == *val2,
            (Token::Str(val1), Token::Str(val2)) => val1 == val2,

            (Token::Boolean(val1), Token::Boolean(val2)) => val1 == val2,

            (Token::List(val1), Token::List(val2)) => val1 == val2,

            (Token::StartParenthesis, Token::StartParenthesis) => true,

            (Token::EndParenthesis, Token::EndParenthesis) => true,

            (Token::StartBracket, Token::StartBracket) => true,

            (Token::EndBracket, Token::EndBracket) => true,

            (Token::StartBrace, Token::StartBrace) => true,

            (Token::EndBrace, Token::EndBrace) => true,

            (Token::Separator(val1), Token::Separator(val2)) => val1 == val2,

            (Token::NewLine, Token::NewLine) => true,

            (Token::EOF, Token::EOF) => true,

            (Token::Void, Token::Void) => true,
            _ => false,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(id) => write!(f, "{}", id.to_string()),
            Token::Keyword(keyword) => write!(f, "{}", String::from(keyword.to_string())),
            Token::Int32(num) => write!(f, "{num}"),
            Token::Int64(num) => write!(f, "{num}"),
            Token::HPInt(num) => write!(f, "{num}"),
            Token::Float(num) => write!(f, "{num}"),
            Token::Double(num) => write!(f, "{num}"),
            Token::NewLine => write!(f, "{}", String::from("\n")),
            Token::EOF => write!(f, "{}", String::from("EOF")),
            Token::Void => write!(f, "{}", String::from("Void")),
            Token::Operator(op) => write!(f, "{}", String::from(op.to_string())),
            Token::Separator(op) => write!(f, "{op}"),
            Token::String(string) => {
                if string.len() <= 1 {
                    write!(f, "{}", string.chars().next().unwrap_or_default())
                } else {
                    write!(f, "{string}")
                }
            }
            Token::Str(string) => write!(f, "{string}"),
            _ => write!(f, "{}", String::from("funcion")),
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Token {
    pub fn to_number<T>(num: T, num_type: Types) -> Token
    where
        T: ToString,
    {
        let id: String = num.to_string();

        match num_type {
            Types::Int32 => match id.parse::<i32>() {
                Ok(value) => Token::Int32(Int32::new(value as i32)),
                Err(_) => Token::EOF, // Si no se puede parsear, devolvemos un Token::EOF
            },
            Types::Int64 => match id.parse::<i64>() {
                Ok(value) => Token::Int64(Int64::new(value as i64)),
                Err(_) => Token::EOF, // Si no se puede parsear, devolvemos un Token::EOF
            },
            Types::HPInt => match id.parse::<i64>() {
                Ok(value) => Token::Int64(Int64::new(value as i64)),
                Err(_) => Token::EOF, // Si no se puede parsear, devolvemos un Token::EOF
            },
            Types::Double => match id.parse::<f64>() {
                Ok(value) => Token::Double(Double::new(value)),
                Err(_) => Token::EOF,
            },
            _ => {
                return if id.contains('.') {
                    Token::to_number(&id, Types::Double)
                } else {
                    Token::to_number(&id, Types::Int32)
                };
            } // Tipo no soportado
        }
    }

    pub fn resolve(self) -> Result<Token, ParseError> {
        match self {
            Token::Operation(mut operation) => operation.resolve(),
            v => Ok(v),
        }
    }

    pub fn is_valid_value(&self) -> bool {
        match self {
            Self::Operator(_)
            | Self::EOF
            | Self::StartBrace
            | Self::StartBracket
            | Self::StartParenthesis
            | Self::EndBrace
            | Self::EndBracket
            | Self::EndParenthesis
            | Self::Separator(_)
            | Self::Void
            | Self::NewLine
            | Self::Keyword(_) => false,
            _ => true,
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            Self::Int32(_) | Self::Int64(_) | Self::HPInt(_) | Self::Float(_) | Self::Double(_) => {
                true
            }
            _ => false,
        }
    }
    pub fn to(&self, new_type: Types) -> Result<Token, ParseError> {
        match (self, &new_type) {
            (Token::Int32(int32), Types::Int32) => Ok(Token::Int32(*int32)),
            (Token::Int32(int32), Types::Int64) => Ok(Token::Int64(<Int64>::from(*int32))),
            (Token::Int32(int32), Types::Double) => {
                Ok(Token::Double(Double::new((**int32) as f64)))
            }
            (Token::Int32(int32), Types::String) => Ok(Token::String(int32.to_string())),
            (Token::Int32(int32), Types::Str) => {
                Ok(Token::Str(Arc::from(int32.to_string().as_str())))
            }
            (Token::Int32(int32), Types::Boolean) => Ok(Token::Boolean(*int32 != 0)),

            (Token::Int64(int64), Types::Int32) => match i32::try_from(**int64) {
                Ok(n) => Ok(Token::Int32(Int32::from(n))),
                Err(_) => Err(ParseError::InvalidTypeConvertion(format!(
                    "The number {int64} exceed the limit"
                ))),
            },
            (Token::Int64(int64), Types::Int64) => Ok(Token::Int64(*int64)),
            (Token::Int64(int64), Types::Double) => todo!(),
            (Token::Int64(int64), Types::String) => todo!(),
            (Token::Int64(int64), Types::Str) => {
                Ok(Token::Str(Arc::from(int64.to_string().as_str())))
            }
            (Token::Int64(int64), Types::Boolean) => Ok(Token::Boolean(*int64 != 0)),

            (Token::Double(double), Types::Int32) => todo!(),
            (Token::Double(double), Types::Int64) => todo!(),
            (Token::Double(double), Types::Double) => Ok(Token::Double(*double)),
            (Token::Double(double), Types::String) => todo!(),
            (Token::Double(double), Types::Str) => {
                Ok(Token::Str(Arc::from(double.to_string().as_str())))
            }
            (Token::Double(double), Types::Boolean) => Ok(Token::Boolean(*double != 0.0)),

            (Token::String(string), Types::Int32) => todo!(),
            (Token::String(string), Types::Int64) => todo!(),
            (Token::String(string), Types::Double) => todo!(),
            (Token::String(string), Types::String) => Ok(Token::String(string.to_string())),
            (Token::String(string), Types::Str) => todo!(),
            (Token::String(string), Types::Boolean) => Ok(Token::Boolean(string.is_empty())),

            (Token::Str(string), Types::Int32) => {
                if let Ok(value) = string.parse::<i32>() {
                    Ok(Token::Int32(value.into()))
                } else {
                    Err(ParseError::InvalidTypeConvertion(format!(
                        "You can't convert a '{}' into {}",
                        string, new_type
                    )))
                }
            }
            (Token::Str(string), Types::Int64) => {
                if let Ok(value) = string.parse::<i64>() {
                    Ok(Token::Int64(value.into()))
                } else {
                    Err(ParseError::InvalidTypeConvertion(format!(
                        "You can't convert a {} in {}",
                        Types::inferred(self)?,
                        new_type
                    )))
                }
            }
            (Token::Str(string), Types::Double) => {
                if let Ok(value) = string.parse::<f64>() {
                    Ok(Token::Double(Double::new(value)))
                } else {
                    Err(ParseError::InvalidTypeConvertion(format!(
                        "You can't convert a '{}' into {}",
                        string, new_type
                    )))
                }
            }
            (Token::Str(string), Types::String) => Ok(Token::String(string.to_string())),
            (Token::Str(string), Types::Str) => Ok(Token::Str((*string).clone())),
            (Token::Str(string), Types::Boolean) => match string.as_ref() {
                "true" => Ok(Token::Boolean(true)),
                "false" => Ok(Token::Boolean(false)),
                _ => Ok(Token::Boolean(string.is_empty())),
            },

            (Token::Boolean(bool), Types::Int32) => {
                if *bool {
                    Ok(Token::Int32(1.into()))
                } else {
                    Ok(Token::Int32(0.into()))
                }
            }
            (Token::Boolean(bool), Types::Int64) => {
                if *bool {
                    Ok(Token::Int64(1.into()))
                } else {
                    Ok(Token::Int64(0.into()))
                }
            }
            (Token::Boolean(bool), Types::Double) => {
                if *bool {
                    Ok(Token::Double(Double::new(1.0)))
                } else {
                    Ok(Token::Double(Double::new(0.0)))
                }
            }
            (Token::Boolean(bool), Types::String) => {
                if *bool {
                    Ok(Token::String("true".to_string()))
                } else {
                    Ok(Token::String("false".to_string()))
                }
            }
            (Token::Boolean(bool), Types::Str) => {
                if *bool {
                    Ok(Token::Str(Arc::from("true")))
                } else {
                    Ok(Token::Str(Arc::from("false")))
                }
            }
            (Token::Boolean(bool), Types::Boolean) => Ok(Token::Boolean(*bool)),

            _ => Err(ParseError::InvalidTypeConvertion(format!(
                "You can't convert a {} in {}",
                Types::inferred(self)?,
                new_type
            ))),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Token::Identifier(_) | Token::Keyword(_) => true,
            Token::Int32(int32) => *int32 > 0,
            Token::Int64(int64) => *int64 > 0,
            Token::Double(double) => *double > 0.0,
            Token::String(s) => s.is_empty(),
            Token::Str(s) => s.is_empty(),
            Token::Boolean(b) => *b,
            _ => false,
        }
    }

    pub fn str_value(&self) -> &str {
        match self {
            Token::Identifier(v) => v,
            Token::Keyword(keyword) => Box::leak(keyword.to_string().into_boxed_str()),
            Token::Operation(operation) => {
                Box::leak((operation.operator).to_string().into_boxed_str())
            }
            Token::Operator(operator) => Box::leak(operator.to_string().into_boxed_str()),
            Token::Type(types) => Box::leak((*types).to_string().into_boxed_str()),
            Token::Int32(int32) => Box::leak(int32.to_string().into_boxed_str()),
            Token::Int64(int64) => Box::leak(int64.to_string().into_boxed_str()),
            Token::HPInt(int128) => Box::leak(int128.to_string().into_boxed_str()),
            Token::Double(double) => Box::leak(double.to_string().into_boxed_str()),
            Token::Float(float) => Box::leak(float.to_string().into_boxed_str()),
            Token::String(s) => s,
            Token::Str(s) => s,
            Token::Boolean(v) => {
                if *v {
                    "true"
                } else {
                    "false"
                }
            }
            Token::StartParenthesis => "(",
            Token::EndParenthesis => ")",
            Token::StartBracket => "[",
            Token::EndBracket => "]",
            Token::StartBrace => "{",
            Token::EndBrace => "}",
            Token::Separator(c) => Box::leak(c.to_string().into_boxed_str()),
            Token::NewLine => "\\n",
            Token::EOF => "EOF",
            Token::Void => "Void",
            Token::List(_) => "List",
        }
    }

    pub fn is_assignation(&self) -> bool {
        if let Token::Operator(op) = self {
            return op.is_assignation();
        }
        false
    }
}

// Token Creation using From
impl From<i32> for Token {
    fn from(value: i32) -> Self {
        Token::Int32(value.into())
    }
}

impl From<i64> for Token {
    fn from(value: i64) -> Self {
        Token::Int64(value.into())
    }
}

impl From<i128> for Token {
    fn from(value: i128) -> Self {
        Token::HPInt(value.into())
    }
}

impl From<u32> for Token {
    fn from(value: u32) -> Self {
        Token::Int32(Int32::from(value as i32))
    }
}

impl From<f64> for Token {
    fn from(value: f64) -> Self {
        Token::Double(Double::new(value))
    }
}

impl From<f32> for Token {
    fn from(value: f32) -> Self {
        Token::Float(value.into())
    }
}

impl From<bool> for Token {
    fn from(value: bool) -> Self {
        Token::Boolean(value)
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        Token::Identifier(Arc::from(value))
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        // Convertir String en Arc<str> vía Arc::from(&value[..])
        Token::Identifier(Arc::from(&value[..]))
    }
}
