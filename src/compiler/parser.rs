use crate::error::{lexic_errors::LexicError, math_errors::MathError};

use super::token::{Number, Token};
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    DivInt,
    Mod,
    Pow,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Operand {
    Number(Number),
    String(String),
    Identifier(String),
    Operation(Operation),
    End,
}

#[derive(Debug, PartialEq)]
pub struct Operation {
    operator: Operator,
    left: Box<Operand>,
    right: Box<Operand>,
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
            Operand::Identifier(i) => Token::Identifier(i.clone()),
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
    pub fn parse(&mut self) -> Operand {
        // Intentar parsear una asignación primero
        if let Some(t) = self.tokens.get(self.position) {
            match t {
                Token::Identifier(_) => {
                    if self.peek_operator("=") {
                        return self.parse_assignment();
                    }
                }
                _ => {}
            }
            if self.peek_operator("=") {
                return self.parse_assignment();
            }
        }
        let result = self.expresion();
        if result != Operand::End {
            return result;
        } else {
            panic!(
                "{}",
                LexicError::SyntaxError(format!(
                    "Unexpected token in position {}: {}",
                    self.position,
                    self.tokens[self.position - 1]
                ))
            );
        }
    }

    fn parse_assignment(&mut self) -> Operand {
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

        // Obtener el operador de asignación
        if let Some(v) = self.tokens.get(self.position) {
            if let Token::Operand(op) = v {
                if op != "=" {
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
        let expr = self.expresion();

        Operand::Operation(Operation {
            operator: Operator::Equal,
            left: Box::new(Operand::Identifier(identifier)),
            right: Box::new(expr),
        })
    }

    fn peek_operator(&self, op: &str) -> bool {
        if self.position + 1 > self.tokens.len() {
            return false;
        }
        match self.tokens.get(self.position + 1) {
            Some(v) => match v {
                Token::Operand(o) if o == op => {
                    println!("peek_operator: {:?}", o);
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }

    // Función principal para manejar la expresión (suma y resta)
    fn expresion(&mut self) -> Operand {
        let mut nodo = self.término();

        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Operand(op) if op == "+" || op == "-" => {
                    let operador = match op.as_str() {
                        "+" => Operator::Add,
                        "-" => Operator::Sub,
                        _ => unreachable!(),
                    };
                    self.position += 1; // Consumir el operador
                    let derecho = self.término();
                    nodo = Operand::Operation(Operation {
                        operator: operador,
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
    fn término(&mut self) -> Operand {
        let mut nodo = self.factor();

        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Operand(op)
                    if op == "*" || op == "/" || op == "**" || op == "//" || op == "%" =>
                {
                    let operador = match op.as_str() {
                        "**" => Operator::Pow,
                        "*" => Operator::Mul,
                        "//" => Operator::DivInt,
                        "/" => Operator::Div,
                        "%" => Operator::Mod,
                        _ => unreachable!(),
                    };
                    self.position += 1; // Consumir el operador
                    let derecho = self.factor();
                    if derecho == Operand::End {
                        break;
                    }
                    nodo = Operand::Operation(Operation {
                        operator: operador,
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
    fn factor(&mut self) -> Operand {
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
            Token::StartParenthesis => {
                self.position += 1; // Consumir '('

                let nodo = self.expresion();

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
            Token::Comment(_) | Token::Operand(_) | Token::NewLine | Token::EOF => {
                self.position += 1; // Consumir comentario
                self.factor()
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
