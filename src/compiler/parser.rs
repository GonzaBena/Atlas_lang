use crate::error::{lexic_errors::LexicError, math_errors::MathError, parse_errors::ParseError};

use super::{
    identifier::IdentifierTable,
    token::{Number, Operator, Token},
};

#[derive(Debug, PartialEq, Clone)]
pub enum Operand<'a> {
    Number(Number),
    String(String),
    Boolean(bool),
    Identifier(&'a str, Option<Box<Operand<'a>>>),
    Operation(Operation<'a>),
    End,
}

impl<'a> Operand<'a> {
    pub fn resolve(&self, table: &IdentifierTable<'a>) -> Result<Token<'a>, MathError> {
        match self {
            Operand::Number(n) => Ok(Token::Number(n.clone())),
            Operand::Operation(op) => op.resolve(table),
            Operand::String(s) => Ok(Token::String(s.clone())),
            Operand::End => Ok(Token::Number(Number::Int(0))),
            Operand::Identifier(name, _operand) => {
                if let Some(value) = table.get(name) {
                    Ok(value.clone())
                } else {
                    Err(MathError::UndefinedVariable(name.to_string()))
                }
            }
            Operand::Boolean(b) => Ok(Token::Bool(*b)),
        }
    }

    #[allow(dead_code)]
    fn to_token(&self, table: &'a IdentifierTable) -> Result<Token<'a>, MathError> {
        match self {
            Operand::Number(n) => Ok(Token::Number(n.clone())),
            Operand::Operation(op) => op.resolve(table),
            Operand::End => Ok(Token::EOF),
            Operand::String(s) => Ok(Token::String(s.clone())),
            Operand::Identifier(_, v) => {
                if let Some(value) = v {
                    value.resolve(table)
                } else {
                    Ok(Token::EOF)
                }
            }
            Operand::Boolean(b) => Ok(Token::Bool(*b)),
        }
    }

    pub fn from_token(token: Token) -> Operand {
        match token {
            Token::Number(n) => Operand::Number(n),
            Token::String(s) => Operand::String(s),
            Token::Bool(b) => Operand::Boolean(b),
            v => Operand::String(v.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operation<'a> {
    operator: Operator,
    left: Box<Operand<'a>>,
    right: Box<Operand<'a>>,
}

#[allow(dead_code)]
impl<'a> Operation<'a> {
    /// this function resolves the operation
    pub fn resolve(&self, table: &IdentifierTable<'a>) -> Result<Token<'a>, MathError> {
        let left = self.left.resolve(table)?;
        let right = self.right.resolve(table)?;

        match self.operator {
            Operator::Add => Ok(left + right),
            Operator::Sub => Ok(left - right),
            Operator::Mul => Ok(left * right),
            Operator::Div => {
                if right == Token::Number(Number::Int(0)) {
                    Err(MathError::ZeroDivision(
                        "Division by zero isn't mathematically possible".to_string(),
                    ))
                } else {
                    Ok(left / right)
                }
            }
            Operator::DivInt => {
                println!("DivInt: {:?} / {:?}", left, right);
                if let Token::Number(f) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Number(f / r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for DivInt in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for DivInt".to_string(),
                    ))
                }
            }
            Operator::Mod => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Number(l % r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for Mod in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for Mod in the left element ".to_string(),
                    ))
                }
            }
            Operator::Pow => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        return match r {
                            Number::Int(i) => Ok(Token::Number(l.pow(i as i32))),
                            Number::Float(f) => Ok(Token::Number(l.powf(f))),
                        };
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for Pow in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for Pow in the left element".to_string(),
                    ))
                }
            }
            Operator::Greater => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Bool(l > r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for Greater in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for Greater in the left element".to_string(),
                    ))
                }
            }
            Operator::Less => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Bool(l < r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for Less in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for Less in the left element".to_string(),
                    ))
                }
            }
            Operator::GreaterEqual => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Bool(l >= r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for GreaterEqual in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for GreaterEqual in the left element".to_string(),
                    ))
                }
            }
            Operator::LessEqual => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Bool(l <= r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for LessEqual in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for LessEqual in the left element".to_string(),
                    ))
                }
            }
            Operator::Equal => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Bool(l == r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for Equal in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for Equal in the left element".to_string(),
                    ))
                }
            }
            Operator::NotEqual => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Bool(l != r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for NotEqual in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for NotEqual in the left element".to_string(),
                    ))
                }
            }
            Operator::Asign => {
                // Asignar el valor de la derecha a la variable de la izquierda
                // let mut variables = VARIABLES.lock().unwrap();
                // variables.insert(self.left.resolve().to_string(), self.right.resolve());
                println!("Asignación: {:?}", right);
                Ok(right)
            }
        }
    }

    fn get_operator(&self) -> &Operator {
        &self.operator
    }

    fn get_left(&self) -> &Operand {
        &self.left
    }

    fn get_right(&self) -> &Operand {
        &self.right
    }

    pub fn is_valid(&self) -> bool {
        let left_valid = match &*self.left {
            Operand::Number(_) => true,
            Operand::Operation(op) => op.is_valid(),
            _ => false,
        };

        let right_valid = match &*self.right {
            Operand::Number(_) => true,
            Operand::Operation(op) => op.is_valid(),
            _ => false,
        };

        if *self.left == Operand::End && *self.right == Operand::End {
            return true;
        }

        return left_valid && right_valid;
    }
}

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
