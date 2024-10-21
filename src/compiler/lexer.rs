use crate::{
    compiler::token::{Number, Token},
    error::lexic_errors::LexicError,
};
use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

use super::token::Operator;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn tokenizer(&mut self) -> Vec<Token> {
        // cut until the first '#'
        let mut tokens: Vec<Token> = Vec::new();
        let mut numero_actual = String::new();

        while let Some(&c) = self.input.peek() {
            match c {
                // MARK: Identifiers
                'a'..='z' | 'A'..='Z' | '_' => {
                    // Reconocer Identificadores
                    numero_actual.clear();
                    while let Some(&c_inner) = self.input.peek() {
                        if c_inner.is_alphanumeric() || c_inner == '_' {
                            numero_actual.push(c_inner);
                            self.input.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Identifier(numero_actual.clone()));
                }
                // MARK: Strings
                '\'' | '"' => {
                    // Cadena de texto
                    let mut string = String::new();
                    let quote = c;
                    self.input.next(); // Consumir la comilla inicial
                    while let Some(&c) = self.input.peek() {
                        if c == quote {
                            self.input.next(); // Consumir la comilla final
                            break;
                        } else {
                            string.push(c);
                            self.input.next(); // Consumir el carácter
                        }
                    }
                    tokens.push(Token::String(string));
                }
                // MARK: Comments
                '#' => {
                    // Comentario
                    let mut comment = String::new();
                    self.input.next(); // Consumir '#'
                    while let Some(&c) = self.input.peek() {
                        if c == '\n' {
                            break;
                        } else {
                            comment.push(c);
                            self.input.next(); // Consumir el carácter
                        }
                    }
                    tokens.push(Token::Comment(comment));
                }
                // MARK: Numbers
                '0'..='9' | '.' | ',' => {
                    // Acummulate the number
                    numero_actual.push(c);
                    self.input.next(); // Consume digit
                }
                // MARK: Operators
                '+' | '-' | '*' | '/' | '%' | '=' => {
                    // If there is a number accumulated, add it as a token
                    if !numero_actual.is_empty() {
                        if let Ok(num) = Number::from_str(&numero_actual) {
                            tokens.push(Token::Number(num));
                        }
                        numero_actual.clear();
                    }

                    // verify if the operator is part of a number
                    if c == '-' {
                        if let Some(&next_c) = self.input.peek() {
                            if next_c.is_numeric() {
                                numero_actual.push(c);
                                self.input.next(); // Consume digit
                                continue;
                            }
                        }
                    }

                    // Manejar operadores de uno y dos caracteres
                    let operando = match c {
                        '*' => {
                            self.input.next(); // Consumir '*'
                            if let Some(&next_c) = self.input.peek() {
                                if next_c == '*' {
                                    self.input.next(); // Consumir segundo '*'
                                    Operator::Pow
                                } else {
                                    Operator::Mul
                                }
                            } else {
                                Operator::Mul
                            }
                        }
                        '/' => {
                            self.input.next(); // Consumir '/'
                            if let Some(&next_c) = self.input.peek() {
                                if next_c == '=' {
                                    self.input.next(); // Consumir segundo '/'
                                    Operator::DivInt
                                } else {
                                    Operator::Div
                                }
                            } else {
                                Operator::Div
                            }
                        }
                        '=' => {
                            self.input.next(); // Consumir '='
                            if let Some(&next_c) = self.input.peek() {
                                if next_c == '=' {
                                    self.input.next(); // Consumir segundo '='
                                    Operator::Equal
                                } else {
                                    Operator::Asign
                                }
                            } else {
                                Operator::Asign
                            }
                        }
                        '+' => {
                            self.input.next(); // Consumir '+'
                            Operator::Add
                        }
                        '-' => {
                            self.input.next(); // Consumir '-'
                            Operator::Sub
                        }
                        '%' => {
                            self.input.next(); // Consumir el operador
                            Operator::Mod
                        }
                        _ => unreachable!(),
                    };
                    tokens.push(Token::Operator(operando));
                }
                // MARK: Parenthesis
                '(' => {
                    // Si hay un número acumulado, agregarlo como token
                    if !numero_actual.is_empty() {
                        if let Ok(num) = Number::from_str(numero_actual.as_str()) {
                            tokens.push(Token::Number(num));
                        } else {
                            panic!(
                                "{}",
                                LexicError::NumberError(
                                    "Invalid number, the numbers need to be from 0 to 9"
                                        .to_string()
                                )
                            );
                        }
                        numero_actual.clear();
                    }

                    tokens.push(Token::StartParenthesis);
                    self.input.next(); // Consumir '('
                }
                ')' => {
                    // if there is a number accumulated, add it as a token
                    if !numero_actual.is_empty() {
                        if let Ok(num) = Number::from_str(numero_actual.as_str()) {
                            tokens.push(Token::Number(num));
                        } else {
                            panic!(
                                "{}",
                                LexicError::NumberError(
                                    "Invalid number, the numbers need to be from 0 to 9"
                                        .to_string()
                                )
                            );
                        }
                        numero_actual.clear();
                    }

                    tokens.push(Token::EndParenthesis);
                    self.input.next(); // Consume ')'
                }
                // MARK: NewLine
                '\n' => {
                    if !numero_actual.is_empty() {
                        if let Ok(num) = Number::from_str(numero_actual.as_str()) {
                            tokens.push(Token::Number(num));
                        }
                        numero_actual.clear();
                    }
                    tokens.push(Token::NewLine);
                    self.input.next(); // Consume '\n'
                }
                // MARK: WhiteSpace
                '\r' => {
                    // manage possible '\r\n' in windows
                    self.input.next(); // Consum '\r'
                    if let Some(&'\n') = self.input.peek() {
                        tokens.push(Token::NewLine);
                        self.input.next(); // Consume '\n'
                    }
                }
                ' ' | '\t' => {
                    // Ignorar espacios en blanco
                    if !numero_actual.is_empty() {
                        if let Ok(num) = Number::from_str(numero_actual.as_str()) {
                            tokens.push(Token::Number(num));
                        } else {
                            panic!(
                                "{}",
                                LexicError::NumberError(
                                    "Invalid number, the numbers need to be from 0 to 9"
                                        .to_string()
                                )
                            );
                        }
                        numero_actual.clear();
                    }
                    self.input.next(); // Consume espacio
                }
                _ => {
                    panic!(
                        "{}",
                        LexicError::SyntaxError(format!("Unexpected character: {}", c))
                    );
                }
            }
        }

        // Después de recorrer todos los caracteres, verifica si hay un número restante
        if !numero_actual.is_empty() {
            if let Ok(num) = Number::from_str(numero_actual.as_str()) {
                tokens.push(Token::Number(num));
            }
        }

        tokens.push(Token::EOF); // Opcional: end of input
        tokens
    }
}
