use std::collections::HashMap;
use std::fmt::Display;

use super::tokens::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierTable<'a> {
    table: HashMap<String, Token<'a>>,
}

impl<'a> IdentifierTable<'a> {
    pub fn new() -> Self {
        IdentifierTable {
            table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, identifier: String, value: Token<'a>) {
        self.table.insert(identifier, value);
    }

    pub fn get(&self, identifier: &str) -> Option<Token<'a>> {
        if let Some(value) = self.table.get(identifier) {
            return Some(value.clone());
        } else {
            return None;
        }
    }
}

impl Display for IdentifierTable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut table = String::new();
        table.push_str("{\n");
        for (key, value) in &self.table {
            table.push_str(&format!("  {}: {},\n", key, value));
        }
        table = table.trim().to_string() + "\n}";

        write!(f, "{}", table)
    }
}
