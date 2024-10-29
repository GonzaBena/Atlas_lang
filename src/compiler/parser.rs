use crate::error::{lexic_errors::LexicError, parse_errors::ParseError};

use super::{
    identifier::IdentifierTable,
    tokens::{
        keywords::Keyword,
        operand::Operand,
        operation::Operation,
        operator::Operator,
        token::{Number, Token},
    },
};

#[derive(Debug)]
pub struct Program<'a> {
    pub statements: Vec<Operand<'a>>,
    pub identifier_table: Box<IdentifierTable<'a>>,
}

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn get_comments(&self) -> Vec<String> {
        println!("comments: {:?}", self.tokens);
        self.tokens
            .iter()
            .filter(|t| match t {
                Token::Comment(_) => true,
                _ => false,
            })
            .map(|x| x.to_string())
            .collect()
    }

    // Función principal para iniciar el parsing
    pub fn parse(&mut self) -> Result<Program<'a>, ParseError> {
        let mut identifier_table: IdentifierTable<'a> = IdentifierTable::new();

        let mut results = Vec::new();
        let lines: Vec<&[Token<'a>]> = self.tokens.split(|x| *x == Token::NewLine).collect();

        for line in lines {
            let mut line_parser = Parser {
                tokens: line,
                position: 0,
            };

            // Intentar parsear una asignación primero
            if let Some(Token::Keyword(k)) = line_parser.tokens.get(0) {
                if k == &Keyword::Let || k == &Keyword::Const {
                    let result = line_parser.parse_assignment(&mut identifier_table);
                    match result {
                        Ok(r) => match r {
                            Operand::Identifier(_, val) => {
                                results.push(*val.unwrap());
                            }
                            b => {
                                results.push(b);
                            }
                        },
                        Err(e) => {
                            panic!("Error al parsear la línea: {:?}", e);
                        }
                    }
                    continue;
                }
            }

            let result = line_parser.expresion(&mut identifier_table).unwrap();
            results.push(result);
        }

        Ok(Program {
            statements: results,
            identifier_table: Box::new(identifier_table.clone()),
        })
    }

    fn parse_assignment(
        &mut self,
        table: &mut IdentifierTable<'a>,
    ) -> Result<Operand<'a>, ParseError> {
        // Keyword let or const followed by an identifier is expected
        match &self.tokens[self.position] {
            Token::Keyword(Keyword::Let) => {
                self.position += 1; // Consume the keyword

                // read the identifier
                let identifier = if let Token::Identifier(name) = &self.tokens[self.position] {
                    name.clone()
                } else {
                    return Err(ParseError::SyntaxError(format!(
                        "Expected identifier at position {}",
                        self.position
                    )));
                };
                self.position += 1; // Consume the identifier

                // Expect assignment operator '='
                match self.consume_operator(&Operator::Asign) {
                    Ok(true) => {
                        // if the next token is a new line, Throw error because the identifier needs a value
                        if let Token::NewLine = self.tokens[self.position] {
                            return Err(ParseError::SyntaxError(format!(
                                "Expected value after identifier at position {}",
                                self.position
                            )));
                        }

                        // Parse the right-hand expression
                        let expr = self.expresion(table)?;

                        // Resolve the expression and insert into the identifier table
                        let value = expr.resolve(table).unwrap();
                        table.insert(identifier.clone(), value.clone());
                        return Ok(Operand::Identifier(
                            Box::leak(identifier.into_boxed_str()),
                            Some(Box::new(Operand::from_token(value))),
                        ));
                    }
                    Ok(false) => {
                        return Err(ParseError::SyntaxError(format!(
                            "Expected '=' at position {}",
                            self.position
                        )));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Token::Identifier(identifier) => {
                self.position += 1; // Consume the identifier

                // Expect assignment operator '='
                if self.consume_operator(&Operator::Asign)? {
                    // if there is a asignation operator, it's mean that the identifier is already declared
                    if let Some(v) = table.get(identifier) {
                        return Ok(Operand::Identifier(
                            Box::leak(identifier.clone().into_boxed_str()),
                            Some(Box::new(Operand::from_token(v.clone()))),
                        ));
                    } else {
                        return Err(ParseError::UndefinedVariable(format!(
                            "Doesn't exists the variable {}.\nYou need to define this variable before assign a value",
                            identifier.to_string()
                        )));
                    }
                } else {
                    // Parse the right-hand expression
                    let expr = self.expresion(table)?;
                    return Ok(expr);
                }
            }
            _ => {
                // it's mean that the keyword doesn't exist
                return Err(ParseError::SyntaxError(format!(
                    "Expected 'let' at position {}",
                    self.position
                )));
            }
        }
    }

    #[allow(dead_code)]
    fn peek_operator(&self, op: &Operator) -> bool {
        self.tokens.get(self.position + 1).map_or(
            false,
            |token| matches!(token, Token::Operator(current_op) if current_op == op),
        )
    }

    /// Consumes the specified operator if it matches the current token.
    fn consume_operator(&mut self, op: &Operator) -> Result<bool, ParseError> {
        if let Some(Token::Operator(current_op)) = self.tokens.get(self.position) {
            if current_op == op {
                self.position += 1; // Consume the operator
                if self.position >= self.tokens.len() {
                    return Err(ParseError::SyntaxError(format!(
                        "Expected value at position {}",
                        self.position
                    )));
                }
                return Ok(true);
            }
        }
        Ok(false)
    }

    // Función principal para manejar la expresión (suma y resta)
    fn expresion(&mut self, table: &mut IdentifierTable<'a>) -> Result<Operand<'a>, ParseError> {
        let mut node = self.term(table)?;

        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Operator(op) if *op == Operator::Add || *op == Operator::Sub => {
                    let operator = op.clone();
                    self.position += 1; // Consume the operator
                    let right = self.term(table)?;
                    node = Operand::Operation(Operation {
                        operator,
                        left: Box::new(node),
                        right: Box::new(right),
                    });
                }
                _ => break,
            }
        }

        Ok(node)
    }

    // Maneja multiplicación y división
    fn term(&mut self, table: &mut IdentifierTable<'a>) -> Result<Operand<'a>, ParseError> {
        let mut node = self.factor(table)?;

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
                    let right = self.factor(table)?;
                    node = Operand::Operation(Operation {
                        operator,
                        left: Box::new(node),
                        right: Box::new(right),
                    });
                }
                _ => break,
            }
        }

        Ok(node)
    }

    // Maneja números y paréntesis
    fn factor(&mut self, table: &mut IdentifierTable<'a>) -> Result<Operand<'a>, ParseError> {
        if self.position >= self.tokens.len() {
            return Ok(Operand::End);
        }

        match &self.tokens[self.position] {
            Token::Number(n) => {
                self.position += 1; // Consume the number
                Ok(Operand::Number(n.clone()))
            }
            Token::String(s) => {
                self.position += 1; // Consume the string
                Ok(Operand::String(s.clone()))
            }
            Token::Keyword(k) => {
                self.position += 1; // Consume the keyword
                match k {
                    Keyword::Let => {
                        let result = self.parse_assignment(table)?;
                        Ok(result)
                    }
                    Keyword::True => Ok(Operand::Boolean(true)),
                    Keyword::False => Ok(Operand::Boolean(false)),
                    _ => Err(ParseError::SyntaxError(format!(
                        "Unexpected keyword at position {}: {}",
                        self.position, k
                    ))),
                }
            }
            Token::Identifier(name) => {
                self.position += 1; // Consume the identifier
                if let Some(&Token::Operator(Operator::Asign)) = self.tokens.get(self.position) {
                    // Handle assignment
                    let expr = self.expresion(table)?;
                    Ok(Operand::Identifier(
                        Box::leak(name.clone().into_boxed_str()),
                        Some(Box::new(expr)),
                    ))
                } else {
                    // Handle variable usage
                    if let Some(value) = table.get(name) {
                        Ok(Operand::Identifier(
                            Box::leak(name.clone().into_boxed_str()),
                            Some(Box::new(Operand::from_token(value))),
                        ))
                    } else {
                        Err(ParseError::UndefinedVariable(name.to_string()))
                    }
                }
            }
            Token::StartParenthesis => {
                self.position += 1; // Consume '('
                let expr = self.expresion(table)?;
                match self.tokens.get(self.position) {
                    Some(Token::EndParenthesis) => {
                        self.position += 1; // Consume ')'
                        Ok(expr)
                    }
                    _ => Err(ParseError::SyntaxError("Expected ')'".to_string())),
                }
            }
            Token::Operator(op) if *op == Operator::Add || *op == Operator::Sub => {
                let operator = op.clone();
                self.position += 1; // Consume the unary operator
                let operand = self.factor(table)?;
                // Handle unary operations if necessary
                Ok(Operand::Operation(Operation {
                    operator,
                    left: Box::new(Operand::Number(Number::Int(0))), // Assuming unary operation
                    right: Box::new(operand),
                }))
            }
            Token::Comment(_) | Token::Operator(_) | Token::NewLine | Token::EOF => {
                self.position += 1; // Consumir comentario
                self.factor(table)
            }
            v => {
                println!("v: {:?}", v);
                panic!(
                    "{}",
                    LexicError::SyntaxError(format!(
                        "Unexpected token in position {}: {}",
                        self.position, self.tokens[self.position]
                    ))
                )
            }
        }
    }
}
