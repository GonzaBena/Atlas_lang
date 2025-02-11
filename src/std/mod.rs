use crate::compiler::{elements::token::Token, function::Argument};
use std::{collections::HashMap, fmt::Debug, sync::Arc};

pub(crate) mod debug;
pub(crate) mod io;
pub(crate) mod math;
pub(crate) mod string;
pub(crate) mod types;

pub fn standard_library() -> HashMap<String, StdFunc> {
    let mut functions = HashMap::new();

    // Agregar funciones de los submódulos
    functions.extend(io::io_functions());
    functions.extend(math::math_functions());
    functions.extend(string::string_functions());
    functions.extend(types::types_functions());
    functions.extend(debug::debug_functions());

    functions
}

#[derive(Clone)]
pub struct StdFunc {
    pub name: String,
    pub execute: Arc<dyn Fn(Vec<Argument>) -> Result<Token, String> + Send + Sync>,
}

impl Debug for StdFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl StdFunc {
    pub fn new<F>(name: String, func: F) -> Self
    where
        F: 'static + Fn(Vec<Argument>) -> Result<Token, String> + Send + Sync,
    {
        Self {
            name,
            execute: Arc::new(func),
        }
    }

    pub fn call(&self, args: Vec<Argument>) -> Result<Token, String> {
        (self.execute)(args)
    }
}
