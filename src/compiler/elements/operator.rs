use super::token::Token;

/// Represents all possible operators in the language, including:
/// - Assignment operators (e.g., `=`),
/// - Arithmetic operators (e.g., `+`, `-`, `*`, `/`),
/// - Logical operators (e.g., `&&`, `||`),
/// - Comparison operators (e.g., `==`, `!=`, `<`, `>`), and others.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Operator {
    /// Assignation
    Assign,

    /// Addition
    Add,

    /// Subtraction
    Sub,

    /// Multiplication
    Mul,

    /// Divition
    Div,

    /// Integer Divition
    DivInt,

    /// Module
    Mod,
}

impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::Assign => String::from("="),
            Operator::Add => String::from("+"),
            Operator::Sub => String::from("-"),
            Operator::Mul => String::from("*"),
            Operator::Div => String::from("/"),
            Operator::DivInt => String::from("//"),
            Operator::Mod => String::from("%"),
        }
    }
}

impl Operator {
    pub fn execute<'a>(&self, left: Token<'a>, right: Token<'a>) -> Token<'a> {
        let mut left = left;
        let mut right = right;
        if let Token::Operation(mut op) = left {
            left = op.resolve().unwrap();
        }
        if let Token::Operation(mut op) = right {
            right = op.resolve().unwrap();
        }
        match self {
            Self::Add => match (left, right) {
                (Token::Int32(num1), Token::Int32(num2)) => Token::to_number(num1 + num2),
                _ => Token::EOF,
            },
            _ => left,
        }
    }
}
