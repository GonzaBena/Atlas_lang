use crate::compiler::elements::token::Token;

use super::StdFunc;
use std::collections::HashMap;

pub fn io_functions() -> HashMap<String, StdFunc> {
    let mut functions = HashMap::new();

    // Ejemplo de función: print
    functions.insert(
        "print".to_string(),
        StdFunc::new("print".to_string(), |args| {
            for arg in args {
                println!("{}", arg);
            }
            Ok(Token::Void)
        }),
    );

    functions
}
