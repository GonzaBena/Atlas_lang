use crate::error::{lexic_errors::LexicError, parse_errors::ParseError};

use super::{
    identifier::IdentifierTable,
    tokens::operand::Operand,
    tokens::operation::Operation,
    tokens::operator::Operator,
    tokens::token::{Number, Token},
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
            if let Some(Token::Identifier(_)) = line_parser.tokens.get(0) {
                if line_parser.peek_operator(&Operator::Asign) {
                    let result = line_parser.parse_assignment(&mut identifier_table).unwrap();
                    if let Operand::Identifier(..) = result.clone() {
                        if self.peek_operator(&Operator::Asign) {
                            results.push(result);
                            continue;
                        } else {
                            panic!("Error al parsear la línea: {:?}", line);
                        }
                    }
                    continue;
                }
            }

            let result = line_parser.expresion(&identifier_table).unwrap();
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
        if !self.consume_operator(&Operator::Asign)? {
            return Err(ParseError::SyntaxError(format!(
                "Expected '=' after identifier at position {}",
                self.position
            )));
        }

        // Parse the right-hand expression
        let expr = self.expresion(table)?;

        // Resolve the expression and insert into the identifier table
        let value = expr.resolve(table).unwrap();
        table.insert(identifier.clone(), value.clone());

        Ok(Operand::Identifier(
            Box::leak(identifier.into_boxed_str()),
            Some(Box::new(expr)),
        ))
    }

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
                return Ok(true);
            }
        }
        Ok(false)
    }

    // Función principal para manejar la expresión (suma y resta)
    fn expresion(&mut self, table: &IdentifierTable<'a>) -> Result<Operand<'a>, ParseError> {
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
    fn term(&mut self, table: &IdentifierTable<'a>) -> Result<Operand<'a>, ParseError> {
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
    fn factor(&mut self, table: &IdentifierTable<'a>) -> Result<Operand<'a>, ParseError> {
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
                self.factor(&table)
            }
            _ => panic!(
                "{}",
                LexicError::SyntaxError(format!(
                    "Unexpected token in position {}: {}",
                    self.position, self.tokens[self.position]
                ))
            ),
        }
    }
}
