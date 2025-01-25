use crate::compiler::elements::token::Token;

use super::StdFunc;
use colored::Colorize;
use std::collections::HashMap;

pub fn types_functions() -> HashMap<String, StdFunc> {
    let mut functions = HashMap::new();

    // Ejemplo de función: print
    functions.insert(
        "typeof".to_string(),
        StdFunc::new("typeof".to_string(), |args| {
            if args.len() != 1 {
                return Err("The typeof function only accept one argument.".to_string());
            }
            let value = args[0].value.as_ref().unwrap();

            match **value {
                Token::Int32(_) => println!("{}", "<type: Int32>".blue()),
                Token::Int64(_) => println!("{}", "<type: Int64>".blue()),
                Token::String(_) => println!("{}", "<type: String>".blue()),
                Token::Str(_) => println!("{}", "<type: Str>".blue()),
                Token::Type(types) => println!("<type: {}>", types.to_string().blue()),
                _ => todo!("implement std::types::types_functions::typeof"),
            }
            Ok(Token::Void)
        }),
    );

    functions
}
