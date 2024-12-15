use crate::types::basic::number::int32::Int32;

use super::token::Token;

/// Represents all possible operators in the language, including:
/// - Assignment operators (e.g., `=`),
/// - Arithmetic operators (e.g., `+`, `-`, `*`, `/`),
/// - Logical operators (e.g., `&&`, `||`),
/// - Comparison operators (e.g., `==`, `!=`, `<`, `>`), and others.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    /// Assignation
    Assign,

    /// Addition
    Add,

    /// Addition and Assignation
    AddAssign,

    /// Subtraction
    Sub,

    /// Subtraction and Assignation
    SubAssign,

    /// Multiplication
    Mul,

    /// Multiplication and Assignation
    MulAssign,

    /// Power
    Pow,

    /// Power and Assignation
    PowAssign,

    /// Divition
    Div,

    /// Divition and Assignation
    DivAssign,

    /// Integer Divition
    DivInt,

    /// Integer Divition and Assignation
    DivIntAssign,

    /// Module
    Mod,

    /// Module and Assignation
    ModAssign,
}

impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::Assign => String::from("="),
            Operator::Add => String::from("+"),
            Operator::AddAssign => String::from("+="),
            Operator::Sub => String::from("-"),
            Operator::SubAssign => String::from("-="),
            Operator::Mul => String::from("*"),
            Operator::MulAssign => String::from("*="),
            Operator::Pow => String::from("**"),
            Operator::PowAssign => String::from("**="),
            Operator::Div => String::from("/"),
            Operator::DivAssign => String::from("/="),
            Operator::DivInt => String::from("//"),
            Operator::DivIntAssign => String::from("//="),
            Operator::Mod => String::from("%"),
            Operator::ModAssign => String::from("%="),
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
            Self::Add | Self::AddAssign => match (left, right) {
                (Token::Int32(num1), Token::Int32(num2)) => Token::to_number(num1 + num2, "Int32"),
                (Token::Double(num1), Token::Double(num2)) => {
                    Token::to_number(num1 + num2, "Double")
                }
                (Token::Double(num1), Token::Int32(num2)) => {
                    Token::to_number(num1 + num2, "Double")
                }
                (Token::Int32(num1), Token::Double(num2)) => {
                    Token::to_number(num1 + num2, "Double")
                }
                _ => Token::EOF,
            },

            Self::Sub | Self::SubAssign => match (left, right) {
                (Token::Int32(num1), Token::Int32(num2)) => Token::to_number(num1 - num2, "Int32"),
                (Token::Double(num1), Token::Double(num2)) => {
                    Token::to_number(num1 - num2, "Double")
                }
                (Token::Double(num1), Token::Int32(num2)) => {
                    Token::to_number(num1 - num2, "Double")
                }
                (Token::Int32(num1), Token::Double(num2)) => {
                    Token::to_number(num1 - num2, "Double")
                }
                _ => Token::EOF,
            },

            Self::Mul | Self::MulAssign => match (left, right) {
                (Token::Int32(num1), Token::Int32(num2)) => Token::to_number(num1 * num2, "int32"),
                (Token::Double(num1), Token::Double(num2)) => {
                    Token::to_number(num1 * num2, "Double")
                }
                (Token::Double(num1), Token::Int32(num2)) => {
                    Token::to_number(num1 * num2, "Double")
                }
                (Token::Int32(num1), Token::Double(num2)) => {
                    Token::to_number(num1 * num2, "Double")
                }
                _ => Token::EOF,
            },

            Self::Pow | Self::PowAssign => match (left, right) {
                (Token::Int32(num1), Token::Int32(num2)) => {
                    let mut result = num1;

                    if *num2 <= 1 {
                        return Token::Int32(result);
                    }

                    for _ in 2..=*num2 {
                        result *= num1;
                        println!("Result: {result:?}");
                    }
                    return Token::Int32(result);
                }
                _ => Token::EOF,
            },

            Self::Div | Self::DivAssign => {
                return match (left, right) {
                    (Token::Int32(num1), Token::Int32(num2)) => {
                        Token::to_number(num1 / num2, "Int32")
                    }
                    (Token::Double(num1), Token::Double(num2)) => {
                        Token::to_number(num1 / num2, "Double")
                    }
                    (Token::Double(num1), Token::Int32(num2)) => {
                        Token::to_number(num1 / num2, "Double")
                    }
                    (Token::Int32(num1), Token::Double(num2)) => {
                        Token::to_number(num1 / num2, "Double")
                    }
                    _ => Token::EOF,
                };
            }

            Self::DivInt | Self::DivIntAssign => {
                return match (left, right) {
                    (Token::Int32(num1), Token::Int32(num2)) => {
                        Token::Int32((num1 as Int32 / num2 as Int32).trunc())
                    }
                    _ => Token::EOF,
                };
            }

            Self::Mod | Self::ModAssign => match (left, right) {
                (Token::Int32(num1), Token::Int32(num2)) => Token::Int32(num1 % num2),
                (Token::Double(num1), Token::Double(num2)) => Token::Int32(num1 % num2),
                (Token::Double(num1), Token::Int32(num2)) => Token::Int32(num1 % num2),
                (Token::Int32(num1), Token::Double(num2)) => Token::Int32(num1 % num2),
                _ => Token::EOF,
            },

            _ => todo!("Hola mundo"),
        }
    }

    pub fn is_assignation(&self) -> bool {
        match self {
            Self::AddAssign
            | Self::SubAssign
            | Self::MulAssign
            | Self::DivAssign
            | Self::ModAssign
            | Self::PowAssign
            | Self::DivIntAssign => true,
            _ => false,
        }
    }
}
