use std::fmt::{Debug, Display};

use super::{keyword::Keyword, operation::Operation, operator::Operator};
use crate::{
    compiler::{error::parse_error::ParseError, function::Argument, types::Types},
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
    Type(Types),
    Argument(Argument<'a>),

    // Datatypes
    Int32(Int32),
    Double(Double),
    String(String),
    Boolean(bool),
    // Function(&'a str),

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
            Token::Separator(op) => write!(f, "{op}"),
            _ => write!(f, "{}", String::from("hola")),
        }
    }
}

impl PartialEq for Token<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Boolean(b1), Token::Boolean(b2)) => b1 == b2,
            (Token::Int32(i1), Token::Int32(i2)) => i1 == i2,
            (Token::Double(d1), Token::Double(d2)) => d1 == d2,
            (Token::String(s1), Token::String(s2)) => s1 == s2,
            (Token::Identifier(id1), Token::Identifier(id2)) => id1 == id2,
            (Token::Operation(op1), Token::Operation(op2)) => op1 == op2,
            (Token::Keyword(k1), Token::Keyword(k2)) => k1 == k2,
            (Token::Operator(op1), Token::Operator(op2)) => op1 == op2,
            (Token::StartParenthesis, Token::StartParenthesis) => true,
            (Token::EndParenthesis, Token::EndParenthesis) => true,
            (Token::StartBracket, Token::StartBracket) => true,
            (Token::EndBracket, Token::EndBracket) => true,
            (Token::StartBrace, Token::StartBrace) => true,
            (Token::EndBrace, Token::EndBrace) => true,
            (Token::NewLine, Token::NewLine) => true,
            (Token::EOF, Token::EOF) => true,
            (Token::Void, Token::Void) => true,
            (Token::Separator(sep1), Token::Separator(sep2)) => sep1 == sep2,
            (Token::Type(sep1), Token::Type(sep2)) => sep1 == sep2,
            _ => false, // Si no hay coincidencia exacta, los tokens son diferentes
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
impl From<String> for Token<'_> {
    fn from(value: String) -> Self {
        Token::String(value)
    }
}

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
