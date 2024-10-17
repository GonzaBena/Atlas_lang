use super::ast::Token;
use crate::compiler::ast::Number;
use std::str::FromStr;

/// Parse an expression and build an AST
pub fn parse_expression(expr: &str) -> Result<Vec<Token>, String> {
    // each token is separated by a space
    let binding = expr.split_whitespace().collect::<Vec<&str>>().join("");
    let mut characters = binding.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    let mut numero_actual = String::new();

    while let Some(&c) = characters.peek() {
        match c {
            '0'..='9' | '.' | ',' => {
                // Acummulate the number
                numero_actual.push(c);
                characters.next(); // Consumir el dígito
            }
            '+' | '-' | '*' | '/' => {
                // If there is a number accumulated, add it as a token
                if !numero_actual.is_empty() {
                    // if the num is a float
                    if let Ok(num) = Number::from_str(&numero_actual) {
                        tokens.push(Token::Number(num));
                    } else {
                        panic!("Número inválido: {}", numero_actual);
                    }
                    numero_actual.clear();
                }

                // Manejar operadores de uno y dos caracteres
                let operando = match c {
                    '*' => {
                        characters.next(); // Consumir '*'
                        if let Some(&next_c) = characters.peek() {
                            if next_c == '*' {
                                characters.next(); // Consumir segundo '*'
                                "**".to_string()
                            } else {
                                "*".to_string()
                            }
                        } else {
                            "*".to_string()
                        }
                    }
                    '/' => {
                        characters.next(); // Consumir '/'
                        if let Some(&next_c) = characters.peek() {
                            if next_c == '/' {
                                characters.next(); // Consumir segundo '/'
                                "//".to_string()
                            } else {
                                "/".to_string()
                            }
                        } else {
                            "/".to_string()
                        }
                    }
                    '+' | '-' => {
                        characters.next(); // Consumir el operador
                        c.to_string()
                    }
                    _ => unreachable!(),
                };
                tokens.push(Token::Operand(operando));
            }
            '(' => {
                // Si hay un número acumulado, agregarlo como token
                if !numero_actual.is_empty() {
                    if let Ok(num) = Number::from_str(numero_actual.as_str()) {
                        tokens.push(Token::Number(num));
                    } else {
                        panic!("Número inválido: {}", numero_actual);
                    }
                    numero_actual.clear();
                }

                tokens.push(Token::StartParenthesis);
                characters.next(); // Consumir '('
            }
            ')' => {
                // Si hay un número acumulado, agregarlo como token
                if !numero_actual.is_empty() {
                    if let Ok(num) = Number::from_str(numero_actual.as_str()) {
                        tokens.push(Token::Number(num));
                    } else {
                        panic!("Número inválido: {}", numero_actual);
                    }
                    numero_actual.clear();
                }

                tokens.push(Token::EndParenthesis);
                characters.next(); // Consumir ')'
            }
            ' ' | '\t' | '\n' => {
                // Ignorar espacios en blanco
                if !numero_actual.is_empty() {
                    if let Ok(num) = Number::from_str(numero_actual.as_str()) {
                        tokens.push(Token::Number(num));
                    } else {
                        panic!("Número inválido: {}", numero_actual);
                    }
                    numero_actual.clear();
                }
                characters.next(); // Consumir espacio
            }
            _ => {
                panic!("Carácter inesperado: {}", c);
            }
        }
    }

    println!("Tokens: {:?}", tokens);

    // Después de recorrer todos los caracteres, verifica si hay un número restante
    if !numero_actual.is_empty() {
        if let Ok(num) = Number::from_str(numero_actual.as_str()) {
            tokens.push(Token::Number(num));
        }
    }

    Ok(tokens)
}
