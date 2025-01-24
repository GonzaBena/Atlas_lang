use super::{elements::token::Token, types::Types};

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) var_type: Types,
    pub(crate) value: Box<Token>,
    pub(crate) scope: usize,
}

#[allow(dead_code)]
impl Variable {
    pub fn new(name: String, var_type: Types, value: Token, scope: usize) -> Self {
        Self {
            id: 0,
            name,
            var_type,
            value: Box::new(value),
            scope,
        }
    }

    pub fn to_token(&self) -> &Token {
        &*self.value
    }

    pub fn details(&self) -> String {
        let id = if self.id == 0 {
            "".to_string()
        } else {
            format!("{} - ", self.id)
        };
        format!(
            "{}{}: {:?} = {} | scope: {}",
            &id, self.name, self.var_type, self.value, self.scope
        )
    }

    pub(crate) fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}
