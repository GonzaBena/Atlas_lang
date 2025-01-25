use crate::{compiler::elements::token::Token, utils::format::center};

use super::StdFunc;
use colored::Colorize;
use std::collections::HashMap;

pub const DEBUG_LIST: &[&str] = &["showVars"];

fn lengths_of_vars(vars: Vec<String>) -> [usize; 5] {
    let mut largest_id: usize = 0;
    let mut largest_name: usize = 0;
    let mut largest_type: usize = 0;
    let mut largest_value: usize = 0;
    let mut largest_scope: usize = 0;

    let values = vars
        .iter()
        .map(|x| x.split(" - ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    for key in values {
        if key[0].to_string().len() > largest_id {
            largest_id = key[0].len();
        }
        if key[1].len() > largest_name {
            largest_name = key[1].len();
        }
        if key[2].len() > largest_type {
            largest_type = key[2].len();
        }
        if key[3].len() > largest_value {
            largest_value = key[3].len();
        }
        if key[4].len() > largest_scope {
            largest_scope = key[4].len();
        }
    }
    [
        largest_id,
        largest_name,
        largest_type,
        largest_value,
        largest_scope,
    ]
}

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

                let lenghts = lengths_of_vars(orders_vars.clone());
                let string = format!("| {:width_id$} | {:width_name$} | {:width_type$} | {:width_value$} | {} |",
                        center("ID", lenghts[0]),
                        center("NAME", lenghts[1]/2),
                        center("TYPE", lenghts[2]/2),
                        center("VALUE", lenghts[3]/2),
                        center("SCOPE", lenghts[4] + 3),
                        width_id = lenghts[0] + 5,
                        width_name = lenghts[1] + 5,
                        width_type = lenghts[2] + 5,
                        width_value = lenghts[3] + 5);
                println!("{}", "-".repeat(string.len()).blue());
                println!(
                        "{}", string.blue()
                    );
                println!("{}", "-".repeat(string.len()).blue());
                for s in &orders_vars {
                    let string = s.split(" - ").collect::<Vec<&str>>();
                    let txt = format!(
                        "| {:width_id$} | {:width_name$} | {:width_type$} | {:width_value$} | scope: {:width_scope$} |",
                        string[0],
                        string[1],
                        string[2],
                        string[3],
                        string[4],
                        width_id = lenghts[0] + 5,
                        width_name = lenghts[1] + 5,
                        width_type = lenghts[2] + 5,
                        width_value = lenghts[3] + 5,
                        width_scope = lenghts[4] + 5
                    );
                    println!("{}", txt.blue());
                    println!("{}", "-".repeat(txt.len()).blue());
                }
            }

            Ok(Token::Void)
        }),
    );

    functions
}
