use crate::compiler::elements::token::Token;

use super::StdFunc;
use std::collections::HashMap;

pub fn io_functions() -> HashMap<String, StdFunc> {
    let mut functions = HashMap::new();

    // Ejemplo de función: print
    functions.insert(
        "print".to_string(),
        StdFunc::new("print".to_string(), |args| {
            let mut txt = String::new();
            for arg in args {
                txt.push_str(&format!(
                    " {}",
                    if arg.value.is_some() {
                        arg.value.unwrap()
                    } else {
                        arg.default_value.unwrap()
                    }
                ));
            }
            println!("{}", txt.trim());
            Ok(Token::Void)
        }),
    );

    functions
}
