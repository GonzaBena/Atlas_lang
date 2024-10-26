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

    pub fn tokenizer(&mut self) -> Result<Vec<Token<'a>>, LexicError> {
        // cut until the first '#'
        let mut tokens: Vec<Token<'a>> = Vec::new();

        while let Some(&c) = self.input.peek() {
            match c {
                // MARK: Identifiers
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = self.consume_identifier();
                    tokens.push(Token::Identifier(identifier));
                }
                // MARK: Strings
                '\'' | '"' => {
                    let string = self.consume_string(c)?;
                    tokens.push(Token::String(string));
                }
                // MARK: Comments
                '#' => {
                    let comment = self.consume_comment();
                    let comment: &'a str = Box::leak(comment.into_boxed_str());
                    tokens.push(Token::Comment(comment));
                }
                // MARK: Numbers
                '0'..='9' | '.' => {
                    let number = self.consume_number()?;
                    tokens.push(Token::Number(number));
                }
                // MARK: Operators
                '+' | '-' | '*' | '/' | '%' | '=' => {
                    // Handle operator parsing
                    let operator = self.consume_operator(c)?;
                    tokens.push(Token::Operator(operator));
                }
                // MARK: Parenthesis
                '(' => {
                    self.input.next(); // Consume '('
                    tokens.push(Token::StartParenthesis);
                }
                ')' => {
                    self.input.next(); // Consume ')'
                    tokens.push(Token::EndParenthesis);
                }
                // MARK: NewLine
                '\n' => {
                    self.input.next(); // Consume '\n'
                    tokens.push(Token::NewLine);
                }
                // MARK: Carriage Return (handle '\r\n')
                '\r' => {
                    self.input.next(); // Consume '\r'
                    if let Some(&'\n') = self.input.peek() {
                        self.input.next(); // Consume '\n'
                        tokens.push(Token::NewLine);
                    }
                }
                // MARK: Whitespace
                ' ' | '\t' => {
                    self.input.next(); // Consume space or tab
                }
                // MARK: Unexpected Characters
                _ => {
                    return Err(LexicError::SyntaxError(format!(
                        "Unexpected character: '{}'",
                        c
                    )));
                }
            }
        }

        // Después de recorrer todos los caracteres, verifica si hay un número restante
        // if !current_number.is_empty() {
        //     if let Ok(num) = Number::from_str(current_number.as_str()) {
        //         tokens.push(Token::Number(num));
        //     }
        // }
        tokens.push(Token::EOF); // Opcional: end of input
        Ok(tokens)
    }

    fn consume_identifier(&mut self) -> String {
        // let start_pos = self.input.clone();
        let mut identifier = String::new();
        while let Some(&c_inner) = self.input.peek() {
            if c_inner.is_alphanumeric() || c_inner == '_' {
                identifier.push(c_inner);
                self.input.next();
            } else {
                break;
            }
        }
        identifier
    }

    fn consume_string(&mut self, quote: char) -> Result<String, LexicError> {
        let mut string = String::new();
        self.input.next(); // Consume the opening quote

        while let Some(&c) = self.input.peek() {
            if c == quote {
                self.input.next(); // Consume the closing quote
                return Ok(string);
            } else if c == '\\' {
                self.input.next(); // Consume '\\'
                if let Some(&escaped_char) = self.input.peek() {
                    match escaped_char {
                        'n' => {
                            string.push('\n');
                        }
                        't' => {
                            string.push('\t');
                        }
                        '\\' => {
                            string.push('\\');
                        }
                        '"' => {
                            string.push('"');
                        }
                        '\'' => {
                            string.push('\'');
                        }
                        _ => {
                            return Err(LexicError::SyntaxError(format!(
                                "Invalid escape character: \\{}",
                                escaped_char
                            )));
                        }
                    }
                    self.input.next(); // Consume the escaped character
                } else {
                    return Err(LexicError::SyntaxError(
                        "Unterminated escape sequence in string".to_string(),
                    ));
                }
            } else {
                string.push(c);
                self.input.next(); // Consume the character
            }
        }

        Err(LexicError::UnterminatedString)
    }

    fn consume_comment(&mut self) -> String {
        let mut comment = String::new();
        self.input.next(); // Consume '#'

        while let Some(&c) = self.input.peek() {
            if c == '\n' {
                break;
            } else {
                comment.push(c);
                self.input.next(); // Consume the character
            }
        }

        comment.trim().to_string()
    }

    fn consume_number(&mut self) -> Result<Number, LexicError> {
        let mut number_str = String::new();
        let mut has_decimal = false;

        while let Some(&c) = self.input.peek() {
            if c.is_digit(10) {
                number_str.push(c);
                self.input.next();
            } else if c == '.' && !has_decimal {
                has_decimal = true;
                number_str.push(c);
                self.input.next();
            } else {
                break;
            }
        }

        match Number::from_str(&number_str) {
            Ok(num) => Ok(num),
            Err(_) => Err(LexicError::NumberError(format!(
                "Invalid number format: '{}'",
                number_str
            ))),
        }
    }

    fn consume_operator(&mut self, first_char: char) -> Result<Operator, LexicError> {
        match first_char {
            '*' => {
                self.input.next(); // Consume '*'
                if let Some(&'*') = self.input.peek() {
                    self.input.next(); // Consume second '*'
                    Ok(Operator::Pow)
                } else {
                    Ok(Operator::Mul)
                }
            }
            '/' => {
                self.input.next(); // Consume '/'
                if let Some(&'/') = self.input.peek() {
                    self.input.next(); // Consume '/'
                    Ok(Operator::DivInt)
                } else {
                    Ok(Operator::Div)
                }
            }
            '=' => {
                self.input.next(); // Consume '='
                if let Some(&'=') = self.input.peek() {
                    self.input.next(); // Consume '='
                    Ok(Operator::Equal)
                } else {
                    Ok(Operator::Asign)
                }
            }
            '+' => {
                self.input.next(); // Consume '+'
                Ok(Operator::Add)
            }
            '-' => {
                self.input.next(); // Consume '-'
                Ok(Operator::Sub)
            }
            '%' => {
                self.input.next(); // Consume '%'
                Ok(Operator::Mod)
            }
            _ => Err(LexicError::SyntaxError(format!(
                "Unknown operator: '{}'",
                first_char
            ))),
        }
    }
}
