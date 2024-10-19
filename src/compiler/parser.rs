use crate::error::{lexic_errors::LexicError, math_errors::MathError};

use super::{
    identifier::IdentifierTable,
    token::{Number, Operator, Token},
};

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Operand {
    Number(Number),
    String(String),
    Boolean(bool),
    Identifier(String, Box<Operand>),
    Operation(Operation),
    End,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operation {
    operator: Operator,
    left: Box<Operand>,
    right: Box<Operand>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Operand>,
    pub identifier_table: IdentifierTable,
}

#[allow(dead_code)]
impl Operation {
    /// this function resolves the operation
    pub fn resolve(&self) -> Token {
        match self.operator {
            Operator::Add => self.left.resolve() + self.right.resolve(),
            Operator::Sub => self.left.resolve() - self.right.resolve(),
            Operator::Mul => self.left.resolve() * self.right.resolve(),
            Operator::Div => {
                if self.right.resolve() == Token::Number(Number::Int(0)) {
                    panic!(
                        "{}",
                        MathError::ZeroDivision(
                            "Division by zero isn't mathematically possible".to_string()
                        )
                    );
                }
                self.left.resolve() / self.right.resolve()
            }
            Operator::DivInt => (self.left.resolve() / self.right.resolve()).floor(),
            Operator::Mod => self.left.resolve() % self.right.resolve(),
            Operator::Pow => self.left.resolve().pow(self.right.resolve()),
            Operator::Greater => Token::Bool(self.left.resolve() > self.right.resolve()),
            Operator::Less => Token::Bool(self.left.resolve() < self.right.resolve()),
            Operator::GreaterEqual => Token::Bool(self.left.resolve() >= self.right.resolve()),
            Operator::LessEqual => Token::Bool(self.left.resolve() <= self.right.resolve()),
            Operator::Equal => Token::Bool(self.left.resolve() == self.right.resolve()),
            Operator::NotEqual => Token::Bool(self.left.resolve() != self.right.resolve()),
            Operator::Asign => {
                // Asignar el valor de la derecha a la variable de la izquierda
                // let mut variables = VARIABLES.lock().unwrap();
                // variables.insert(self.left.resolve().to_string(), self.right.resolve());
                Token::EOF
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

impl Operand {
    pub fn resolve(&self) -> Token {
        match self {
            Operand::Number(n) => Token::Number(n.clone()),
            Operand::Operation(op) => op.resolve(),
            Operand::End => Token::Number(Number::Int(0)),
            Operand::String(s) => Token::String(s.clone()),
            Operand::Identifier(_, v) => v.to_token(),
            Operand::Boolean(b) => Token::Bool(*b),
        }
    }

    fn to_token(&self) -> Token {
        match self {
            Operand::Number(n) => Token::Number(n.clone()),
            Operand::Operation(op) => op.resolve(),
            Operand::End => Token::EOF,
            Operand::String(s) => Token::String(s.clone()),
            Operand::Identifier(_, v) => match (**v).clone() {
                Operand::Identifier(i, _) => Token::Identifier(i),
                _ => v.to_token(),
            },
            Operand::Boolean(b) => Token::Bool(*b),
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

pub struct Parser<'a> {
    tokens: &'a [Token],
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
    pub fn parse(&mut self) -> Program {
        let mut identifier_table = IdentifierTable::new();

        let lines: Vec<&[Token]> = self.tokens.split(|x| x == &Token::NewLine).collect();
        // for line in lines {
        //     println!("Line: {:?}", line);
        // }
        let mut results = Vec::new();

        for line in lines {
            println!("Line: {:?}", line);

            // Crear un nuevo parser para cada línea
            let mut line_parser = Parser {
                tokens: line,
                position: 0,
                // Copiar otros campos necesarios del parser original
            };

            // Intentar parsear una asignación primero
            if let Some(t) = line_parser.tokens.get(line_parser.position) {
                match t {
                    Token::Identifier(_) => {
                        if line_parser.peek_operator(&Operator::Asign) {
                            let result = line_parser.parse_assignment(&identifier_table);
                            println!("Assignment: {:?}", result);
                            match result.clone() {
                                Operand::Identifier(i, v) => {
                                    identifier_table.insert(i, (*v).to_token());
                                }
                                _ => {}
                            }
                            // IdentifierTable::insert(result, result.get_right());
                            results.push(result);
                            continue;
                        }
                    }
                    _ => {}
                }
                if line_parser.peek_operator(&Operator::Asign) {
                    let result = line_parser.parse_assignment(&identifier_table);
                    results.push(result);
                    continue;
                }
            }

            let result = line_parser.expresion(&mut identifier_table);
            if result != Operand::End {
                results.push(result);
            } else {
                panic!("Error al parsear la línea: {:?}", line);
            }
        }

        Program {
            statements: results,
            identifier_table,
        }
    }

    fn parse_assignment(&mut self, table: &IdentifierTable) -> Operand {
        // Obtener el identificador
        let identifier = if let Token::Identifier(ref name) = self.tokens[self.position] {
            name.clone()
        } else {
            panic!(
                "Se esperaba un identificador en la posición {}",
                self.position
            );
        };
        self.position += 1; // Consumir el identificador
        println!("Identifier: {:?}", identifier);
        // Obtener el operador de asignación
        if let Some(v) = self.tokens.get(self.position) {
            if let Token::Operator(op) = v {
                if *op != Operator::Asign {
                    panic!(
                        "{}",
                        LexicError::SyntaxError(format!(
                            "waiting for '=' in position {}",
                            self.position
                        ))
                    );
                }
            } else {
                panic!(
                    "{}",
                    LexicError::SyntaxError(format!(
                        "waiting for '=' in position {}",
                        self.position
                    ))
                );
            }
        } else {
            panic!(
                "{}",
                LexicError::SyntaxError(format!("waiting for '=' in position {}", self.position))
            );
        }
        self.position += 1; // Consumir '='

        // Parsear la expresión derecha de la asignación
        let expr = self.expresion(&table);
        println!("Expr: {:?}", expr.resolve());

        Operand::Identifier(identifier, Box::new(expr))
    }

    fn peek_operator(&self, op: &Operator) -> bool {
        if self.position + 1 > self.tokens.len() {
            return false;
        }
        match self.tokens.get(self.position + 1) {
            Some(v) => match v {
                Token::Operator(o) if o == op => true,
                _ => false,
            },
            _ => false,
        }
    }

    // Función principal para manejar la expresión (suma y resta)
    fn expresion(&mut self, table: &IdentifierTable) -> Operand {
        let mut nodo = self.término(&table);

        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Operator(op) if *op == Operator::Add || *op == Operator::Sub => {
                    self.position += 1; // Consumir el operador
                    let derecho = self.término(&table);
                    nodo = Operand::Operation(Operation {
                        operator: op.clone(),
                        left: Box::new(nodo),
                        right: Box::new(derecho),
                    });
                }
                _ => break,
            }
        }

        nodo
    }

    // Maneja multiplicación y división
    fn término(&mut self, table: &IdentifierTable) -> Operand {
        let mut nodo = self.factor(&table);

        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Operator(op) => {
                    self.position += 1; // Consumir el operador
                    let derecho = self.factor(&table);
                    if derecho == Operand::End {
                        break;
                    }
                    nodo = Operand::Operation(Operation {
                        operator: op.clone(),
                        left: Box::new(nodo),
                        right: Box::new(derecho),
                    });
                }
                _ => break,
            }
        }

        nodo
    }

    // Maneja números y paréntesis
    fn factor(&mut self, table: &IdentifierTable) -> Operand {
        if self.position >= self.tokens.len() {
            return Operand::End;
            // panic!("Expresión incompleta");
        }

        match &self.tokens[self.position] {
            Token::Number(n) => {
                let nodo = Operand::Number(n.clone());
                self.position += 1; // Consumir el número
                nodo
            }
            Token::String(s) => {
                let nodo = Operand::String(s.clone());
                self.position += 1; // Consumir el número
                nodo
            }
            Token::Identifier(i) => {
                let value = table.get(i).unwrap();
                let nodo = Operand::Identifier(i.clone(), Box::new(Operand::from_token(value)));
                self.position += 1; // Consumir el número
                nodo
            }
            Token::StartParenthesis => {
                self.position += 1; // Consumir '('

                let nodo = self.expresion(&table);

                if self.position >= self.tokens.len() {
                    panic!(
                        "{}",
                        LexicError::SyntaxError(format!("'(' left in position {}", self.position))
                    );
                }

                match &self.tokens[self.position] {
                    Token::EndParenthesis => {
                        self.position += 1; // Consumir ')'
                        nodo
                    }
                    _ => {
                        panic!(
                            "{}",
                            LexicError::SyntaxError(format!(
                                "')' left in position {}",
                                self.position
                            ))
                        );
                    }
                }
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
