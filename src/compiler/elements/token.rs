use std::fmt::{Debug, Display};

use super::{keyword::Keyword, operation::Operation, operator::Operator};
use crate::{
    compiler::error::parse_error::ParseError,
    types::basic::number::{double::Double, int32::Int32},
};

/// Represent each possible token which you can use.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Token<'a> {
    // Basic
    /// used to represent variables or functions
    Identifier(&'a str),
    Keyword(Keyword),
    Operation(Operation<'a>),
    Operator(Operator),

    // Datatypes
    Int32(Int32),
    Double(Double),
    String(String),
    Boolean(bool),
    // Function(&'a str),

    // Others
    StartParenthesis,
    EndParenthesis,
    NewLine,
    EOF,
    Void,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(id) => write!(f, "{}", String::from(*id)),
            Token::Keyword(keyword) => write!(f, "{}", String::from(keyword.to_string())),
            Token::Int32(_) => write!(f, "{}", String::from("Int32")),
            Token::Double(_) => write!(f, "{}", String::from("Double")),
            Token::NewLine => write!(f, "{}", String::from("New Line")),
            Token::EOF => write!(f, "{}", String::from("EOF")),
            Token::Void => write!(f, "{}", String::from("Void")),
            Token::Operator(op) => write!(f, "{}", String::from(op.to_string())),
            _ => write!(f, "{}", String::from("hola")),
        }
    }
}

impl PartialEq for Token<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Boolean(b1), Token::Boolean(b2)) => b1 == b2,
            (Token::Int32(b1), Token::Int32(b2)) => b1 == b2,
            (Token::Double(b1), Token::Double(b2)) => b1 == b2,
            (Token::String(b1), Token::String(b2)) => b1 == b2,
            (Token::Identifier(b1), Token::Identifier(b2)) => b1 == b2,
            (Token::Operation(b1), Token::Operation(b2)) => {
                b1.left == b2.left && b1.right == b2.right && b1.operator == b2.operator
            }
            (Token::Keyword(b1), Token::Keyword(b2)) => b1.to_string() == b2.to_string(),
            (Token::Operator(b1), Token::Operator(b2)) => b1 == b2,

            (t1, t2) => t1.to_string() == t2.to_string(),
        }
    }
}

#[allow(dead_code)]
impl<'a> Token<'a> {
    pub fn to_number<T>(num: T, num_type: &str) -> Token<'a>
    where
        T: ToString,
    {
        let id: String = num.to_string();

        match num_type {
            "Int32" => match id.parse::<f32>() {
                Ok(value) => Token::Int32(Int32::new(value as i32)),
                Err(_) => Token::EOF, // Si no se puede parsear, devolvemos un Token::EOF
            },
            "Double" => match id.parse::<f64>() {
                Ok(value) => Token::Double(value.into()),
                Err(_) => Token::EOF,
            },
            _ => {
                return if id.contains('.') {
                    Token::to_number(&id, "Double")
                } else {
                    Token::to_number(&id, "Int32")
                };
            } // Tipo no soportado
        }
    }

    pub fn resolve(self) -> Result<Token<'a>, ParseError<'a>> {
        match self {
            Token::Operation(mut operation) => operation.resolve(),
            v => Ok(v),
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
impl From<i32> for Token<'_> {
    fn from(value: i32) -> Self {
        Token::Int32(value.into())
    }
}

impl From<u32> for Token<'_> {
    fn from(value: u32) -> Self {
        Token::Int32(value.into())
    }
}

impl From<f64> for Token<'_> {
    fn from(value: f64) -> Self {
        Token::Double(value.into())
    }
}

impl<'a> From<bool> for Token<'a> {
    fn from(value: bool) -> Self {
        Token::Boolean(value)
    }
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(value: &'a str) -> Self {
        Token::Identifier(value)
    }
}
