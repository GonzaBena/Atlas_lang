use crate::compiler::elements::token::Token;

use super::StdFunc;
use colored::Colorize;
use std::collections::HashMap;

pub fn io_functions() -> HashMap<String, StdFunc> {
    let mut functions = HashMap::new();

    // Ejemplo de función: print
    functions.insert(
        "print".to_string(),
        StdFunc::new("print".to_string(), |args| {
            // println!("args: {args:?}");
            let mut txt = String::new();
            for arg in args {
                txt.push_str(&format!(
                    " {}",
                    if arg.value.is_some() {
                        if arg.var_type.is_numeric() {
                            arg.value.unwrap().to_string().blue()
                        } else {
                            arg.value.unwrap().to_string().white()
                        }
                    } else {
                        arg.default_value.unwrap().to_string().red()
                    }
                ));
            }
            println!("{}", txt.trim());
            Ok(Token::Void)
        }),
    );

    functions
}
