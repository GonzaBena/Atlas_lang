use super::{
    elements::{keyword::Keyword, operator::Operator},
    error::lexic_error::LexicError,
    types::Types,
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
                'a'..='z' | 'A'..='Z' => {
                    let id = self.cut_identifier();
                    match id {
                        Ok(id) => match id.type_id {
                            IdentifierType::Id => result.push(Token::Identifier(Box::leak(
                                id.value.unwrap().into_boxed_str(),
                            ))),
                            IdentifierType::Keyword => {
                                result.push(Token::Keyword(id.keyword.unwrap()))
                            }
                            IdentifierType::Type => {
                                result.push(Token::Type(id.return_type.unwrap()));
                            }
                        },
                        Err(err) => panic!("{:?}", err),
                    }
                }
                '"' => {
                    self.content.next();
                    let id = self.cut_string();
                    match id {
                        Ok(id) => result.push(Token::String(id)),
                        Err(err) => panic!("{:?}", err),
                    }
                }

                // Numbers
                '0'..='9' | '.' => {
                    let number = self.cut_number();
                    match number {
                        Ok(num) => result.push(Token::to_number(num, Types::Void)),
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

                    if let Some('=') = self.content.peek() {
                        self.content.next();
                        result.push(Token::Operator(Operator::AddAssign));
                    } else {
                        result.push(Token::Operator(Operator::Add));
                    }
                }

                '-' => {
                    self.content.next();

                    if let Some('=') = self.content.peek() {
                        self.content.next();
                        result.push(Token::Operator(Operator::SubAssign));
                    } else {
                        result.push(Token::Operator(Operator::Sub));
                    }
                }

                '*' => {
                    self.content.next();

                    if let Some('=') = self.content.peek() {
                        self.content.next();
                        result.push(Token::Operator(Operator::MulAssign));
                    } else if let Some('*') = self.content.peek() {
                        self.content.next();
                        if let Some('=') = self.content.peek() {
                            self.content.next();
                            result.push(Token::Operator(Operator::PowAssign));
                        } else {
                            result.push(Token::Operator(Operator::Pow));
                        }
                    } else {
                        result.push(Token::Operator(Operator::Mul));
                    }
                }

                '/' => {
                    self.content.next();

                    if let Some('=') = self.content.peek() {
                        self.content.next();
                        result.push(Token::Operator(Operator::DivAssign));
                    } else if let Some('/') = self.content.peek() {
                        self.content.next();
                        if let Some('=') = self.content.peek() {
                            self.content.next();
                            result.push(Token::Operator(Operator::DivIntAssign));
                        } else {
                            result.push(Token::Operator(Operator::DivInt));
                        }
                    } else {
                        result.push(Token::Operator(Operator::Div));
                    }
                }

                '%' => {
                    self.content.next();

                    if let Some('=') = self.content.peek() {
                        self.content.next();
                        result.push(Token::Operator(Operator::ModAssign));
                    } else {
                        result.push(Token::Operator(Operator::Mod));
                    }
                }

                // Others
                ' ' | '\t' => {
                    self.content.next();
                }

                '\n' => {
                    self.content.next();
                    result.push(Token::NewLine);
                }

                ',' => {
                    self.content.next();
                    result.push(Token::Separator(','));
                }

                ':' => {
                    self.content.next();
                    result.push(Token::Separator(':'));
                }

                '(' => {
                    self.content.next();
                    result.push(Token::StartParenthesis);
                }

                ')' => {
                    self.content.next();
                    result.push(Token::EndParenthesis);
                }

                '[' => {
                    self.content.next();
                    result.push(Token::StartBracket);
                }

                ']' => {
                    self.content.next();
                    result.push(Token::EndBracket);
                }

                '{' => {
                    self.content.next();
                    result.push(Token::StartBrace);
                }

                '}' => {
                    self.content.next();
                    result.push(Token::EndBrace);
                }

                _ => {
                    self.content.next();
                    break;
                }
            }
        }
        result.push(Token::EOF);
        return result;
    }

    fn cut_identifier(&mut self) -> Result<Identifier, LexicError> {
        let mut id = String::new();

        while let Some(char) = self.content.peek() {
            if *char == ' ' || !char.is_alphanumeric() {
                break;
            }
            id.push(*char);
            self.content.next();
        }

        if id.trim() == "" {
            return Err(LexicError::InvalidIdentifier(format!(
                "the id '{}' is invalid",
                id
            )));
        }

        if let Ok(types) = Types::from_str(id.as_str()) {
            return Ok(Identifier {
                value: None,
                type_id: IdentifierType::Type,
                keyword: None,
                return_type: Some(types),
            });
        }

        if let Ok(keyword) = Keyword::from_str(id.as_str()) {
            return Ok(Identifier {
                value: None,
                type_id: IdentifierType::Keyword,
                keyword: Some(keyword),
                return_type: None,
            });
        }

        Ok(Identifier {
            value: Some(id),
            type_id: IdentifierType::Id,
            keyword: None,
            return_type: None,
        })
    }

    fn cut_string(&mut self) -> Result<String, LexicError> {
        let mut id = String::new();

        while let Some(char) = self.content.next() {
            if char == '"' {
                break;
            }
            match char {
                '\\' => match self.content.peek() {
                    Some(ch) => {
                        match ch {
                            'r' => {
                                self.content.next();
                                id.push('\r');
                                id.push('\n');
                            }
                            'n' => {
                                self.content.next();
                                id.push('\n')
                            }
                            't' => {
                                self.content.next();
                                id.push('\t')
                            }
                            _ => return Err(LexicError::UnfinalizedString),
                        };
                    }
                    None => return Err(LexicError::UnfinalizedString),
                },
                _ => id.push(char),
            };
        }

        Ok(id)
    }

    fn cut_number(&mut self) -> Result<String, LexicError> {
        let mut id = String::new();

        while let Some(char) = self.content.peek() {
            match *char {
                '0'..='9' | '.' => {
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
    return_type: Option<Types>,
}

enum IdentifierType {
    Id,
    Keyword,
    Type,
}
