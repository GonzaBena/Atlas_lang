use std::collections::HashMap;

use super::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierTable {
    table: HashMap<String, Token>,
}

impl IdentifierTable {
    pub fn new() -> Self {
        IdentifierTable {
            table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, identifier: String, value: Token) {
        self.table.insert(identifier, value);
    }

    pub fn get(&self, identifier: &str) -> Option<Token> {
        if let Some(value) = self.table.get(identifier) {
            return Some(value.clone());
        } else {
            return None;
        }
    }
}
