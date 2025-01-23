use super::{elements::token::Token, types::Types};

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub(crate) name: String,
    pub(crate) var_type: Types,
    pub(crate) value: Box<Token>,
    pub(crate) scope: usize,
}

#[allow(dead_code)]
impl Variable {
    pub fn new(name: String, var_type: Types, value: Token, scope: usize) -> Self {
        Self {
            name,
            var_type,
            value: Box::new(value),
            scope,
        }
    }

    pub fn to_token(&self) -> &Token {
        &*self.value
    }
}
