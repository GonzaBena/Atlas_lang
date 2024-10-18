use crate::compiler::token::{Number, Token};
use std::str::FromStr;

/// Parse an expression and build an AST
pub fn parse_expression(expr: &str) -> Result<Vec<Token>, String> {
    // cut until the first '#'
    let sentences = expr.split('#').collect::<Vec<&str>>()[0];

    // each token is separated by a space
    let binding = sentences.split_whitespace().collect::<Vec<&str>>().join("");
    let mut characters = binding.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    let mut numero_actual = String::new();

    while let Some(&c) = characters.peek() {
        match c {
            '#' => {
                // Comentario, ignorar hasta el final de la línea
                let mut comment = String::new();
                while let Some(&c) = characters.peek() {
                    if c == '\n' {
                        break;
                    } else {
                        // if c == '#' && !comment.is_empty() {
                        //     tokens.push(Token::Comment(comment.replace('#', "# ").clone()));
                        //     comment.clear();
                        // }
                        comment.push(c);
                        characters.next(); // Consumir el carácter
                    }
                }
                tokens.push(Token::Comment(comment.replace('#', "# ").clone()));
            }
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

    let comment = expr.split('#').collect::<Vec<&str>>();
    if let Some(com) = comment.last() {
        if !com.is_empty() {
            tokens.push(Token::Comment("#".to_string() + &com.replace('\n', "")));
        }
    }
    // Después de recorrer todos los caracteres, verifica si hay un número restante
    if !numero_actual.is_empty() {
        if let Ok(num) = Number::from_str(numero_actual.as_str()) {
            tokens.push(Token::Number(num));
        }
    }

    Ok(tokens)
}
