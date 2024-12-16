use crate::compiler::elements::token::Token;
use std::{collections::HashMap, fmt::Debug, sync::Arc};

mod io;
mod math;
mod string;

pub fn standard_library() -> HashMap<String, StdFunc> {
    let mut functions = HashMap::new();

    // Agregar funciones de los submódulos
    functions.extend(io::io_functions());
    functions.extend(math::math_functions());
    functions.extend(string::string_functions());

    functions
}

#[derive(Clone)]
pub struct StdFunc {
    pub name: String,
    pub execute: Arc<dyn Fn(Vec<Token>) -> Result<Token, String> + Send + Sync>,
}

impl Debug for StdFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl StdFunc {
    pub fn new<F>(name: String, func: F) -> Self
    where
        F: 'static + Fn(Vec<Token>) -> Result<Token, String> + Send + Sync,
    {
        Self {
            name,
            execute: Arc::new(func),
        }
    }

    pub fn call<'a>(&self, args: Vec<Token<'a>>) -> Result<Token<'a>, String> {
        (self.execute)(args)
    }
}
