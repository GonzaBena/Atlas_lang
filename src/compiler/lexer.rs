use super::{
    elements::{keyword::Keyword, operator::Operator},
    error::lexic_error::LexicError,
};
use crate::compiler::elements::token::Token;
use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

/// Convert both character or set of caracter in a Tokend
#[derive(Debug)]
pub struct Lexer<'a> {
    /// List of each caracter in the text
    content: Peekable<Chars<'a>>,

    /// Text to lexing
    data: &'a str,
}

#[allow(dead_code)]
impl<'a> Lexer<'a> {
    /// Create new Empty lexer
    pub fn new(content: &'a str) -> Lexer<'a> {
        Lexer {
            content: content.chars().peekable(),
            data: content,
        }
    }

    /// Transform each character in a Token
    pub fn lex(&mut self) -> Vec<Token> {
        let mut result: Vec<Token<'a>> = vec![];
        while let Some(char) = self.content.peek() {
            match char {
                // Words
                'a'..='z' => {
                    let id = self.cut_identifier();
                    match id {
                        Ok(id) => match id.type_id {
                            IdentifierType::Id => result.push(Token::Identifier(Box::leak(
                                id.value.unwrap().into_boxed_str(),
                            ))),
                            IdentifierType::Keyword => {
                                result.push(Token::Keyword(id.keyword.unwrap()))
                            }
                        },
                        Err(err) => panic!("{:?}", err),
                    }
                }

                // Numbers
                '0'..='9' | '.' | ',' => {
                    let number = self.cut_number();
                    match number {
                        Ok(num) => result.push(Token::to_number(num)),
                        Err(err) => panic!("{:?}", err),
                    }
                }

                // Operators
                '=' => {
                    self.content.next();
                    result.push(Token::Operator(Operator::Assign));
                }
                '+' => {
                    self.content.next();

                    if let Some('+') = self.content.peek() {
                        result.push(Token::Operator(Operator::Add));
                    }
                    result.push(Token::Operator(Operator::Add));
                }

                // Others
                ' ' => {
                    self.content.next();
                }
                '\n' => {
                    self.content.next();
                    result.push(Token::NewLine);
                }
                _ => {
                    self.content.next();
                    println!("Finalizado");
                    break;
                }
            }
        }
        result.push(Token::EOF);
        return result;
    }

    fn cut_identifier(&mut self) -> Result<Identifier, LexicError> {
        let mut id = String::new();

        while let Some(char) = self.content.next() {
            if char == ' ' {
                break;
            }
            id.push(char);
        }

        if id.trim() == "" {
            return Err(LexicError::InvalidIdentifier(format!(
                "the id '{}' is invalid",
                id
            )));
        }

        if let Ok(keyword) = Keyword::from_str(id.as_str()) {
            return Ok(Identifier {
                value: None,
                type_id: IdentifierType::Keyword,
                keyword: Some(keyword),
            });
        }

        Ok(Identifier {
            value: Some(id),
            type_id: IdentifierType::Id,
            keyword: None,
        })
    }

    fn cut_number(&mut self) -> Result<String, LexicError> {
        let mut id = String::new();

        while let Some(char) = self.content.peek() {
            match *char {
                '0'..='9' | '.' | ',' => {
                    id.push(*char);
                    self.content.next();
                }
                '\n' | '\t' => break,
                _ => break,
            }
        }

        if id.chars().all(|x| x.is_digit(10) || x == '.' || x == ',') {
            return Ok(id);
        }

        Err(LexicError::InvalidNumber(format!(
            "The guess number {id} is an invalid number"
        )))
    }
}

impl PartialEq for Lexer<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.data.to_lowercase().trim() == other.data.to_lowercase().trim()
    }
}

#[cfg(test)]
mod lexer_test {
    use crate::compiler::elements::keyword::Keyword;

    use super::*;

    #[test]
    fn new_test() {
        let lex = Lexer::new("");
        let content = "";
        assert_eq!(
            lex,
            Lexer {
                content: content.chars().peekable(),
                data: content
            }
        )
    }

    #[test]
    fn lex_test() {
        let mut lex = Lexer::new("var hola = 10\n");

        assert_eq!(
            lex.lex(),
            vec![
                Token::Keyword(Keyword::Var),
                Token::Identifier("hola"),
                Token::Operator(Operator::Assign),
                Token::Int32(10.into()),
                Token::NewLine,
                Token::EOF
            ]
        )
    }
}

struct Identifier {
    value: Option<String>,
    type_id: IdentifierType,
    keyword: Option<Keyword>,
}

enum IdentifierType {
    Id,
    Keyword,
}
