use super::token::{Number, Token};

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    DivInt,
    Mod,
    Pow,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Number(Number),
    Operation(Operation),
}

#[derive(Debug, PartialEq)]
pub struct Operation {
    operator: Operator,
    left: Box<Operand>,
    right: Box<Operand>,
}

impl Operation {
    /// this function resolves the operation
    pub fn resolve(&self) -> Token {
        match self.operator {
            Operator::Add => self.left.resolve() + self.right.resolve(),
            Operator::Sub => self.left.resolve() - self.right.resolve(),
            Operator::Mul => self.left.resolve() * self.right.resolve(),
            Operator::Div => self.left.resolve() / self.right.resolve(),
            Operator::DivInt => (self.left.resolve() / self.right.resolve()).floor(),
            Operator::Mod => self.left.resolve() % self.right.resolve(),
            Operator::Pow => self.left.resolve().pow(self.right.resolve()),
        }
    }
}

impl Operand {
    pub fn resolve(&self) -> Token {
        match self {
            Operand::Number(n) => Token::Number(n.clone()),
            Operand::Operation(op) => op.resolve(),
        }
    }
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    posición: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            posición: 0,
        }
    }

    // Función principal para iniciar el parsing
    pub fn parse(&mut self) -> Operand {
        self.expresión()
    }

    // Función principal para manejar la expresión (suma y resta)
    fn expresión(&mut self) -> Operand {
        let mut nodo = self.término();

        while self.posición < self.tokens.len() {
            match &self.tokens[self.posición] {
                Token::Operand(op) if op == "+" || op == "-" => {
                    let operador = match op.as_str() {
                        "+" => Operator::Add,
                        "-" => Operator::Sub,
                        _ => unreachable!(),
                    };
                    self.posición += 1; // Consumir el operador
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

        while self.posición < self.tokens.len() {
            match &self.tokens[self.posición] {
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
                    self.posición += 1; // Consumir el operador
                    let derecho = self.factor();
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
        if self.posición >= self.tokens.len() {
            panic!("Expresión incompleta");
        }

        match &self.tokens[self.posición] {
            Token::Number(n) => {
                let nodo = Operand::Number(n.clone());
                self.posición += 1; // Consumir el número
                nodo
            }
            Token::StartParenthesis => {
                self.posición += 1; // Consumir '('

                let nodo = self.expresión();

                if self.posición >= self.tokens.len() {
                    panic!("Falta cerrar el paréntesis");
                }

                match &self.tokens[self.posición] {
                    Token::EndParenthesis => {
                        self.posición += 1; // Consumir ')'
                        nodo
                    }
                    _ => {
                        panic!("Se esperaba ')' en la posición {}", self.posición);
                    }
                }
            }
            _ => panic!(
                "Token inesperado en la posición {}: {:?}",
                self.posición, self.tokens[self.posición]
            ),
        }
    }
}
