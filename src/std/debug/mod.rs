use crate::compiler::elements::token::Token;

use super::StdFunc;
use colored::Colorize;
use std::collections::HashMap;

pub const DEBUG_LIST: &[&str] = &["showVars"];

pub fn debug_functions() -> HashMap<String, StdFunc> {
    let mut functions = HashMap::new();

    // Ejemplo de función: print
    functions.insert(
        "showVars".to_string(),
        StdFunc::new("showVars".to_string(), |args| {
            if args.len() != 1 && args.iter().find(|x| *x.name == *"variables").is_none() {
                return Err("The showVars function doesn't accept arguments.".to_string());
            }
            let vars = args.iter().find(|x| *x.name == *"variables").unwrap();
            if let Token::List(vars) = *vars.value.clone().unwrap() {
                let mut orders_vars = vars
                    .iter()
                    .map(|x| x.to_string().replace("\"", ""))
                    .collect::<Vec<String>>();

                orders_vars.sort_by_key(|s| {
                    s.split(" - ")
                        .next() // Tomamos la parte antes del guion
                        .and_then(|num| num.parse::<u32>().ok()) // Convertimos a número
                        .unwrap_or(u32::MAX) // En caso de error, asignamos un valor grande para evitar fallos
                });

                for s in &orders_vars {
                    println!("{}", s.blue());
                }
            }

            Ok(Token::Void)
        }),
    );

    functions
}
