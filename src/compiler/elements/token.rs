use std::fmt::Display;

use super::{keyword::Keyword, operation::Operation, operator::Operator};
use crate::types::basic::number::int32::Int32;

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

// impl ToString for Token<'_> {
//     fn to_string(&self) -> String {
//         match self {
//             Token::Identifier(id) => String::from(*id),
//             Token::Keyword(keyword) => keyword.to_string(),
//             Token::Int32(num) => num.to_string(),
//             Token::NewLine => String::from("NewLine"),
//             Token::EOF => String::from("EOF"),
//             Token::Void => String::from("Void"),
//             Token::Operator(op) => op.to_string(),
//             _ => String::new(),
//         }
//     }
// }

impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(id) => write!(f, "{}", String::from(*id)),
            Token::Keyword(keyword) => write!(f, "{}", String::from(keyword.to_string())),
            Token::Int32(_) => write!(f, "{}", String::from("Int32")),
            Token::NewLine => write!(f, "{}", String::from("New Line")),
            Token::EOF => write!(f, "{}", String::from("EOF")),
            Token::Void => write!(f, "{}", String::from("Void")),
            Token::Operator(op) => write!(f, "{}", String::from(op.to_string())),
            _ => write!(f, "{}", String::from("")),
        }
    }
}

impl PartialEq for Token<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl<'a> Token<'a> {
    pub fn to_number<T>(num: T) -> Token<'a>
    where
        T: ToString,
    {
        let mut id: String = num.to_string();

        if id.starts_with('-') {
            id = id[1..].to_string();
            if id.starts_with('.') {
                id = format!("-0{id}")
            } else if id.ends_with('.') {
                id = format!("-{id}00")
            } else {
                id = id.replace(',', "");
            }
        } else {
            if id.starts_with('.') {
                id = format!("0{id}")
            } else if id.ends_with('.') {
                id = format!("{id}00")
            } else {
                id = id.replace(',', "");
            }
        }

        // Operate with Negative ways
        if id.starts_with('-') {
            // if let Ok(ni8) = id.parse::<i8>() {
            //     // return Token::Int8(ni8.into());
            // } else if let Ok(ni16) = id.parse::<i16>() {
            //     // return Token::Int16(ni16.into());
            // } else if let Ok(ni32) = id.parse::<i32>() {
            //     return Token::Int32(ni32.into());
            // } else if let Ok(ni64) = id.parse::<i64>() {
            //     // return Token::Int64(ni64.into());
            // } else if let Ok(ni128) = id.parse::<i128>() {
            //     // return Token::Int128(ni128.into());
            // }

            if let Ok(ni32) = id.parse::<i32>() {
                Token::Int32(ni32.into())
            } else {
                Token::Int32(999.into())
            }
        } else {
            // Operate with Positive ways
            // if let Ok(ni8) = id.parse::<u8>() {
            //     // return Token::Int8(ni8.into());
            // } else if let Ok(ni16) = id.parse::<u16>() {
            //     // return Token::Int16(ni16.into());
            // } else if let Ok(ni32) = id.parse::<u32>() {
            //     return Token::Int32(ni32.into());
            // } else if let Ok(ni64) = id.parse::<u64>() {
            //     // return Token::Int64(ni64.into());
            // } else if let Ok(ni128) = id.parse::<u128>() {
            //     // return Token::Int128(ni128.into());
            // }

            if let Ok(ni32) = id.parse::<i32>() {
                Token::Int32(ni32.into())
            } else {
                Token::Int32(999.into())
            }
        }
    }
}
