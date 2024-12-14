use std::{cell::RefCell, rc::Rc};

use super::{
    elements::{keyword::Keyword, operation::Operation, operator::Operator, token::Token},
    error::parse_error::ParseError,
    Variable, VariableTable,
};

/// This struct is in charge of manage the logic and semantic
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    /// List of tokens to parse
    tokens: Vec<Token<'a>>,
    position: usize,
    variables: Rc<RefCell<VariableTable<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>, variables: Option<Rc<RefCell<VariableTable<'a>>>>) -> Self {
        let tokens: Vec<Token<'a>> = tokens
            .iter()
            .filter(|x| **x != Token::EOF)
            .map(|x| x.to_owned())
            .collect();
        Parser {
            tokens,
            position: 0,
            variables: variables.unwrap_or(Rc::new(RefCell::new(VariableTable::new()))),
        }
    }

    fn internal_new(tokens: Vec<Token<'a>>, variables: Rc<RefCell<VariableTable<'a>>>) -> Self {
        Parser {
            tokens,
            position: 0,
            variables,
        }
    }

    // fn start(&self) -> usize {}

    pub fn parse(&mut self) -> Result<Vec<Token>, ParseError<'a>> {
        let mut results = Vec::new();
        let lines: Vec<&[Token<'a>]> = self
            .tokens
            .split(|t| *t == Token::NewLine)
            .filter(|x| !x.is_empty())
            .collect();

        for line in lines {
            // let aux = end;
            // end = end + line.len();
            // start = aux;
            let tokens: Vec<Token<'a>> = line
                .iter()
                .filter(|x| **x != Token::EOF)
                .map(|x| x.to_owned())
                .collect();
            let mut line_parser: Parser<'_> = Parser::internal_new(tokens, self.variables.clone());

            if let Some(Token::Keyword(Keyword::Var)) = line.get(0) {
                line_parser.assignment()?;
            } else {
                let result = line_parser.resolve()?;
                if result != Token::Void {
                    if let Token::Operation(mut operation) = result {
                        let operation_result = operation.resolve().unwrap();
                        results.push(operation_result);
                    }
                }
            }
        }

        Ok(results)
    }

    fn assignment(&mut self) -> Result<(), ParseError<'a>> {
        self.position += 1; // Consume `var`

        let identifier = match self.tokens.get(self.position) {
            Some(Token::Identifier(name)) => {
                self.position += 1; // Consume el identificador
                name.to_owned()
            }
            _ => {
                // let msg = format!("Expected identifier at position {}", self.position.clone());
                // return Err(ParseError::SyntaxError(Box::leak(msg.into_boxed_str())));
                return Ok(());
            }
        };

        // Verifica el operador de asignación
        match self.tokens.get(self.position) {
            Some(Token::Operator(Operator::Assign)) => self.position += 1,
            _ => {
                // let msg = format!("Expected '=' at position {}", self.position.clone());
                // return Err(ParseError::SyntaxError(Box::leak(msg.into_boxed_str())));
                return Ok(());
            }
        }

        // Resuelve la expresión de la derecha
        match self.resolve()? {
            Token::Operation(mut expr) => {
                let value = expr.resolve()?;
                if value == Token::<'a>::Void {
                    return Err(ParseError::SyntaxError("error"));
                }
                let variable =
                    Variable::new(identifier.to_string(), value.to_string(), value.clone(), 0);

                let _ = self.variables.borrow_mut().insert(identifier, variable);
            }
            value => {
                let variable =
                    Variable::new(identifier.to_string(), value.to_string(), value.clone(), 0);

                // Inserta la variable en la tabla
                let _ = (*self.variables.borrow_mut()).insert(identifier, variable);
            }
        }

        Ok(())
    }

    fn resolve(&mut self) -> Result<Token<'a>, ParseError<'a>> {
        let mut node = self.term()?;
        println!("node1: {:?}", node);

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

    fn term(&mut self) -> Result<Token<'a>, ParseError<'a>> {
        let mut node = self.factor()?;
        println!("node2: {:?}", node);

        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Operator(op)
                    if matches!(
                        op,
                        Operator::Mul | Operator::Div | Operator::Mod | Operator::DivInt
                    ) =>
                {
                    let operator = op.clone();
                    self.position += 1;
                    let right = self.factor()?;
                    node = Token::Operation(Operation::new(operator, node, right));
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn factor(&mut self) -> Result<Token<'a>, ParseError<'a>> {
        if self.position >= self.tokens.len() {
            println!("El fin");
            return Ok(Token::Void);
        }
        println!("Token: {:?}", self.tokens[self.position]);

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
                    _ => {
                        let msg = format!(
                            "Unexpected keyword at position {}: {}",
                            self.position.clone(),
                            k
                        );
                        Err(ParseError::SyntaxError(Box::leak(msg.into_boxed_str())))
                    }
                }
            }

            Token::Identifier(var) => {
                if let Ok(variable) = self.variables.borrow_mut().get(&var) {
                    self.position += 1;
                    return Ok(*variable.value.clone());
                } else {
                    let msg = format!("The variable '{var}' doesn't exists.");
                    Err(ParseError::UndefinedVariable(Box::leak(
                        msg.into_boxed_str(),
                    )))
                }
            }

            Token::Operator(op) if *op == Operator::Add || *op == Operator::Sub => {
                let operator = op.clone();
                self.position += 1; // Consume the unary operator
                let operand = self.factor()?;
                // Handle unary operations if necessary
                Ok(Token::Operation(Operation::new(
                    operator,
                    Token::Void,
                    operand,
                )))
            }

            v => {
                println!("v: {:?}", v);
                let msg = format!(
                    "Unexpected token in position {}: {}",
                    self.position, self.tokens[self.position]
                );
                panic!(
                    "{}",
                    ParseError::SyntaxError(Box::leak(msg.into_boxed_str()))
                )
            }
        }
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
        let parser = Parser::new(vec![], None);
        assert_eq!(
            parser,
            Parser {
                tokens: vec![],
                position: 0,
                variables: Rc::new(RefCell::new(VariableTable::new()))
            }
        )
    }

    #[test]
    fn parse_test() {
        let mut lex: Lexer<'static> = Lexer::new("var hola = 10\n");
        let tokens = lex.lex();
        println!("{:?}", tokens);
        let mut parser: Parser<'_> = Parser::new(tokens, None);
        let parse = parser.parse().unwrap();

        assert_eq!(parse, vec![])
    }
}
