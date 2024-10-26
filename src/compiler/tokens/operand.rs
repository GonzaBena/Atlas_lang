use crate::{
    compiler::identifier::IdentifierTable,
    compiler::tokens::{
        operation::Operation,
        token::{Number, Token},
    },
    error::math_errors::MathError,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Operand<'a> {
    Number(Number),
    String(String),
    Boolean(bool),
    Identifier(&'a str, Option<Box<Operand<'a>>>),
    Operation(Operation<'a>),
    End,
}

impl<'a> Operand<'a> {
    pub fn resolve(&self, table: &IdentifierTable<'a>) -> Result<Token<'a>, MathError> {
        match self {
            Operand::Number(n) => Ok(Token::Number(n.clone())),
            Operand::Operation(op) => op.resolve(table),
            Operand::String(s) => Ok(Token::String(s.clone())),
            Operand::End => Ok(Token::Number(Number::Int(0))),
            Operand::Identifier(name, _operand) => {
                if let Some(value) = table.get(name) {
                    Ok(value.clone())
                } else {
                    Err(MathError::UndefinedVariable(name.to_string()))
                }
            }
            Operand::Boolean(b) => Ok(Token::Bool(*b)),
        }
    }

    #[allow(dead_code)]
    fn to_token(&self, table: &'a IdentifierTable) -> Result<Token<'a>, MathError> {
        match self {
            Operand::Number(n) => Ok(Token::Number(n.clone())),
            Operand::Operation(op) => op.resolve(table),
            Operand::End => Ok(Token::EOF),
            Operand::String(s) => Ok(Token::String(s.clone())),
            Operand::Identifier(_, v) => {
                if let Some(value) = v {
                    value.resolve(table)
                } else {
                    Ok(Token::EOF)
                }
            }
            Operand::Boolean(b) => Ok(Token::Bool(*b)),
        }
    }

    pub fn from_token(token: Token) -> Operand {
        match token {
            Token::Number(n) => Operand::Number(n),
            Token::String(s) => Operand::String(s),
            Token::Bool(b) => Operand::Boolean(b),
            v => Operand::String(v.to_string()),
        }
    }
}
