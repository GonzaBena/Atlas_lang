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
    End,
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
            Operand::End => false,
        };

        let right_valid = match &*self.right {
            Operand::Number(_) => true,
            Operand::Operation(op) => op.is_valid(),
            Operand::End => false,
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
        }
    }

    pub fn is_valid(&self) -> bool {
        return match self {
            Operand::Number(_) => true,
            Operand::Operation(op) => op.is_valid(),
            Operand::End => true,
        };
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
        if self.posición >= self.tokens.len() {
            return Operand::End;
            // panic!("Expresión incompleta");
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
            Token::Comment(_) | Token::Operand(_) => {
                self.posición += 1; // Consumir comentario
                self.factor()
            }
            _ => panic!(
                "Token inesperado en la posición {}: {:?}",
                self.posición, self.tokens[self.posición]
            ),
        }
    }
}
