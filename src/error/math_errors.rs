use std::error::Error as Err;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
#[allow(dead_code)]
pub enum MathError {
    // every token which is not a number, operator, or identifier
    InvalidToken(String),

    // when the syntax is not correct. For example, when there is an operator at the end of the input
    InvalidSyntax(String),

    // when the operand is not a number or an identifier or a string which can be converted to a number
    InvalidOperand(String),

    // when the operator is not valid
    InvalidOperator(String),

    InvalidOperation(String),

    InvalidNumber(String),

    InvalidString(String),

    InvalidEnd(String),

    ZeroDivision(String),

    UndefinedVariable(String),
}

impl Display for MathError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            MathError::InvalidToken(token) => write!(f, "Invalid token: {}", token),
            MathError::InvalidSyntax(syntax) => write!(f, "Invalid syntax: {}", syntax),
            MathError::InvalidOperand(operand) => write!(f, "Invalid operand: {}", operand),
            MathError::InvalidOperator(operator) => write!(f, "Invalid operator: {}", operator),
            MathError::InvalidNumber(number) => write!(f, "Invalid number: {}", number),
            MathError::InvalidString(string) => write!(f, "Invalid string: {}", string),
            MathError::InvalidEnd(end) => write!(f, "Invalid end: {}", end),
            MathError::ZeroDivision(division) => write!(f, "Zero division: {}", division),
            MathError::InvalidOperation(operation) => write!(f, "Invalid operation: {}", operation),
            MathError::UndefinedVariable(variable) => write!(f, "Undefined variable: {}", variable),
        }
    }
}

impl Err for MathError {}
