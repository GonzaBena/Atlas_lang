use super::{elements::token::Token, types::Types};

#[derive(Debug, PartialEq, Clone)]
pub struct Variable<'a> {
    pub(crate) name: String,
    pub(crate) var_type: Types,
    pub(crate) value: Box<Token<'a>>,
    pub(crate) scope: usize,
}

#[allow(dead_code)]
impl<'a> Variable<'a> {
    pub fn new(name: String, var_type: Types, value: Token<'a>, scope: usize) -> Self {
        Self {
            name,
            var_type,
            value: Box::new(value),
            scope,
        }
    }

    pub fn to_token(&self) -> &Token<'_> {
        &*self.value
    }
}
