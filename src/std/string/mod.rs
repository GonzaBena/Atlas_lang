use super::StdFunc;
use std::collections::HashMap;

pub fn string_functions() -> HashMap<String, StdFunc> {
    let functions = HashMap::new();

    // // Ejemplo de función: convertir a mayúsculas
    // functions.insert(
    //     "to_upper".to_string(),
    //     StdFunc::new("to_upper".to_string(), |args| {
    //         if let Some(arg) = args.get(0) {
    //             if let Token::String(s) = arg.va
    //             Ok(s.to_uppercase().into())
    //         } else {
    //             Err("to_upper requires a string argument".to_string())
    //         }
    //     }),
    // );

    // functions.insert(
    //     "length".to_string(),
    //     StdFunc::new("lenght".to_string(), |args| {
    //         if let Some(Token::String(s)) = args.get(0) {
    //             Ok((s.len() as f64).into())
    //         } else {
    //             Err("length requires a string argument".to_string())
    //         }
    //     }),
    // );

    functions
}
