use crate::compiler::token::{Number, Token};

use super::lexer::parse_expression;

#[derive(Debug)]
pub struct AST {
    // The root node of the AST
    root: Vec<Token>,
}

impl AST {
    pub fn new() -> Self {
        AST {
            // Initialize the root node of the AST
            root: vec![Token::Number(Number::Int(0))],
        }
    }

    pub fn from_expression(expr: &str) -> Result<Self, String> {
        let root = parse_expression(expr)?;
        Ok(AST { root })
    }

    pub fn expresion(&self) -> Vec<Token> {
        self.root.clone()
    }
}
