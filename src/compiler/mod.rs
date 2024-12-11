use std::collections::HashMap;

use elements::token::Token;

pub mod elements;
pub mod error;
pub mod lexer;
pub mod parser;

#[derive(Debug)]
pub struct VariableTable<'a> {
    variables: HashMap<String, Variable<'a>>,
}

#[derive(Debug)]
pub struct Variable<'a> {
    name: String,
    var_type: String,
    value: Box<Token<'a>>,
    scope: usize,
}

impl<'a> VariableTable<'a> {
    pub fn new() -> Self {
        VariableTable {
            variables: HashMap::new(),
        }
    }
}

impl<'a> Variable<'a> {
    pub fn new(name: String, var_type: String, value: Token<'a>, scope: usize) -> Self {
        Self {
            name,
            var_type,
            value: Box::new(value),
            scope,
        }
    }
}
