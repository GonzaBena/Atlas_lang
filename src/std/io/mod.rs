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
                    "{}",
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

    functions.insert(
        "dprint".to_string(),
        StdFunc::new("dprint".to_string(), |args| {
            // println!("args: {args:?}");
            let mut txt = String::new();
            let len = args.len();
            for arg in args {
                txt.push_str(&format!(
                    "{:?}",
                    if arg.value.is_some() {
                        arg.value.unwrap()
                    } else {
                        arg.default_value.unwrap()
                    }
                ));
                if len > 1 {
                    txt.push_str(" ");
                }
            }
            println!("{}", txt.trim());
            Ok(Token::Void)
        }),
    );

    functions.insert(
        "dpprint".to_string(),
        StdFunc::new("dpprint".to_string(), |args| {
            let mut txt = String::new();
            let len = args.len();
            for arg in args {
                if arg.var_type.is_numeric() {
                    let value = if arg.value.is_some() {
                        *(arg.value).clone().unwrap()
                    } else {
                        *(arg.default_value).clone().unwrap()
                    };

                    let divide: &dyn Fn(String, bool) -> String = &|num, double| {
                        let mut result = String::new();
                        let mut count = 0;

                        // Recorremos el string al revés, agregando puntos cada 3 caracteres
                        let chars = num.chars().rev();
                        for c in chars {
                            if count > 0 && count % 3 == 0 {
                                if count == 3 && double {
                                    result.push(',');
                                } else {
                                    result.push('.');
                                }
                            }
                            result.push(c);
                            count += 1;
                        }

                        result.chars().rev().collect() // Revertimos el resultado final
                    };

                    match value {
                        Token::Int32(num) => txt.push_str(&divide(num.to_string(), false)),
                        Token::Int64(num) => txt.push_str(&divide(num.to_string(), false)),
                        Token::Double(num) => txt.push_str(&divide(num.to_string(), true)),
                        Token::Number(num) => txt.push_str(&divide(num.to_string(), false)),
                        _ => (),
                    }
                } else {
                    txt.push_str(&format!(
                        "{:?}",
                        if arg.value.is_some() {
                            arg.value.unwrap()
                        } else {
                            arg.default_value.unwrap()
                        }
                    ));
                }
                if len > 1 {
                    txt.push_str(" ");
                }
            }
            println!("{}", txt.trim());
            Ok(Token::Void)
        }),
    );

    functions
}
