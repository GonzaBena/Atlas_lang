use super::StdFunc;
use std::collections::HashMap;

pub fn math_functions() -> HashMap<String, StdFunc> {
    let functions = HashMap::new();

    // Ejemplo de función: suma
    // functions.insert(
    //     "sum".to_string(),
    //     StdFunc::new(|args| {
    //         let result: f64 = args
    //             .iter()
    //             .map(|arg| arg.as_number()) // Asegúrate de que `as_number` sea un método en `Value`
    //             .sum();
    //         Ok(result.into())
    //     }),
    // );

    // functions.insert(
    //     "sqrt".to_string(),
    //     StdFunc::new(|args| {
    //         if let Some(arg) = args.first() {
    //             let num = arg.as_number();
    //             Ok(num.sqrt().into())
    //         } else {
    //             Err("sqrt requires one argument".to_string())
    //         }
    //     }),
    // );

    functions
}
