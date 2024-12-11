use std::{cell::RefCell, rc::Rc};

use super::{
    elements::{keyword::Keyword, operation::Operation, operator::Operator, token::Token},
    error::parse_error::ParseError,
    Variable, VariableTable,
};

/// This struct is in charge of manage the logic and semantic
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Parser<'a> {
    /// List of tokens to parse
    tokens: &'a [Token<'a>],
    position: usize,
    variables: Rc<RefCell<VariableTable<'a>>>,
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>], variables: Option<Rc<RefCell<VariableTable<'a>>>>) -> Self {
        Parser {
            tokens,
            position: 0,
            variables: variables.unwrap_or(Rc::new(RefCell::new(VariableTable::new()))),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Token<'a>>, ParseError> {
        if !self.tokens.ends_with(&[Token::EOF]) {
            return Err(ParseError::UndefinedEOF);
        }

        let mut results = Vec::new();
        let lines: Vec<&[Token<'a>]> = self.tokens.split(|t| *t == Token::NewLine).collect();

        for line in lines {
            self.tokens = line; // Configura los tokens actuales
            self.position = 0;

            // Parsea una línea
            if let Some(Token::Keyword(Keyword::Var)) = self.tokens.get(0) {
                println!("Assign");
                self.assignment()?;
            } else {
                results.push(self.resolve()?);
            }
        }

        Ok(results)
    }

    fn assignment(&mut self) -> Result<(), ParseError> {
        self.position += 1; // Consume `var`

        let identifier = match self.tokens.get(self.position) {
            Some(Token::Identifier(name)) => {
                self.position += 1; // Consume el identificador
                name
            }
            _ => {
                let msg = format!("Expected identifier at position {}", self.position);
                return Err(ParseError::SyntaxError(msg));
            }
        };

        // Verifica el operador de asignación
        match self.tokens.get(self.position) {
            Some(Token::Operator(Operator::Assign)) => self.position += 1,
            _ => {
                let msg = format!("Expected '=' at position {}", self.position);
                return Err(ParseError::SyntaxError(msg));
            }
        }

        // Resuelve la expresión de la derecha
        match self.resolve()? {
            Token::Operation(mut expr) => {
                let value = expr.resolve().unwrap();
                if value == Token::Void {
                    return Err(ParseError::SyntaxError("error".to_string()));
                }
                let variable =
                    Variable::new(identifier.to_string(), value.to_string(), value.clone(), 0);

                // Inserta la variable en la tabla
                self.variables
                    .borrow_mut()
                    .variables
                    .insert(identifier.to_string(), variable);
            }
            value => {
                let variable =
                    Variable::new(identifier.to_string(), value.to_string(), value.clone(), 0);

                // Inserta la variable en la tabla
                self.variables
                    .borrow_mut()
                    .variables
                    .insert(identifier.to_string(), variable);
            }
        }

        Ok(())
    }

    fn resolve(&mut self) -> Result<Token<'a>, ParseError> {
        let mut node = self.term()?;

        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Operator(op) if *op == Operator::Add || *op == Operator::Sub => {
                    let operator = op.clone();
                    self.position += 1; // Consume the operator
                    let right = self.term()?;
                    node = Token::Operation(Operation::new(operator, node, right));
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn term(&mut self) -> Result<Token<'a>, ParseError> {
        let mut node = self.factor()?;

        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Operator(op)
                    if matches!(
                        op,
                        Operator::Mul | Operator::Div | Operator::Mod | Operator::DivInt
                    ) =>
                {
                    let operator = op.clone();
                    self.position += 1; // Consume the operator
                    let right = self.factor()?;
                    node = Token::Operation(Operation::new(operator, node, right));
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn factor(&mut self) -> Result<Token<'a>, ParseError> {
        if self.position >= self.tokens.len() {
            println!("El fin");
            return Ok(Token::Void);
        }

        match &self.tokens[self.position] {
            Token::Int32(n) => {
                self.position += 1; // Consume the number
                Ok(Token::Int32(*n))
            }

            Token::EOF => {
                self.position += 1; // Consume the number
                return Ok(Token::Void);
            }

            Token::String(s) => {
                self.position += 1; // Consume the string
                Ok(Token::String(s.clone()))
            }

            Token::Keyword(k) => {
                self.position += 1; // Consume the keyword
                match k {
                    Keyword::Var => {
                        // let result = self.parse_assignment(table)?;
                        // Ok(result)
                        todo!()
                    }
                    Keyword::True => Ok(Token::Boolean(true)),
                    Keyword::False => Ok(Token::Boolean(false)),
                    _ => Err(ParseError::SyntaxError(format!(
                        "Unexpected keyword at position {}: {}",
                        self.position, k
                    ))),
                }
            }

            v => {
                println!("v: {:?}", v);
                let msg = format!(
                    "Unexpected token in position {}: {}",
                    self.position, self.tokens[self.position]
                );
                panic!("{}", ParseError::SyntaxError(msg))
            }
        }
    }

    pub fn trim(&mut self) {
        let tokens = self.tokens.iter().filter(|x| **x != Token::EOF);
        let mut start = 0;
        let mut end = 0;

        for i in self.tokens {
            if *i != Token::NewLine {
                break;
            }
            start += 1;
        }

        for i in tokens.clone().rev() {
            if *i != Token::NewLine {
                break;
            }
            end += 1;
        }

        let tokens = tokens.map(|x| x.to_owned()).collect::<Vec<_>>();
        let vec = tokens[start..=end].to_vec();
        self.tokens = Box::leak(Box::new(vec));
    }
}

impl<'a> PartialEq for Parser<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.tokens == other.tokens
    }
}

#[cfg(test)]
mod parser_test {
    use crate::compiler::lexer::Lexer;

    use super::*;

    #[test]
    fn new_test() {
        let parser = Parser::new(&[], None);
        assert_eq!(
            parser,
            Parser {
                tokens: &[],
                position: 0,
                variables: Rc::new(RefCell::new(VariableTable::new()))
            }
        )
    }

    #[test]
    fn parse_test() {
        let mut lex = Lexer::new("var hola = 10\n");
        let tokens = lex.lex();
        println!("{:?}", tokens);
        let mut parser = Parser::new(tokens.as_slice(), None);
        let parse = parser.parse().unwrap();

        assert_eq!(parse, vec![])
    }
}
