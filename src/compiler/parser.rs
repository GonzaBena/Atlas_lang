use super::{
    elements::{keyword::Keyword, operation::Operation, operator::Operator, token::Token},
    error::parse_error::ParseError,
    Variable, VariableTable,
};
use std::{cell::RefCell, rc::Rc};

/// This struct is in charge of manage the logic and semantic
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Parser<'a> {
    /// List of tokens to parse
    tokens: Vec<Token<'a>>,
    position: usize,
    variables: Rc<RefCell<VariableTable<'a>>>,
}

#[allow(dead_code)]
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

    pub fn get_variables(&self) -> Vec<(String, Variable<'a>)> {
        let mut result = vec![];

        for (key, variable) in self.variables.borrow().variables.iter() {
            result.push((key.to_owned(), variable.to_owned()));
        }

        result
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, ParseError<'a>> {
        let mut results = Vec::new();
        let lines: Vec<&[Token<'a>]> = self
            .tokens
            .split(|t| *t == Token::NewLine)
            .filter(|x| !x.is_empty())
            .collect();

        for line in lines {
            let tokens: Vec<Token<'a>> = line
                .iter()
                .filter(|x| **x != Token::EOF)
                .map(|x| x.to_owned())
                .collect();
            let mut line_parser: Parser<'_> = Parser::internal_new(tokens, self.variables.clone());
            if let Some(Token::Keyword(Keyword::Var)) = line.get(0) {
                line_parser.assignment()?;
            } else if let Some(Token::Operator(op)) = line.get(1) {
                println!("Operator: {op:?}");
                if op.is_assignation() {
                    line_parser.assignment()?;
                } else {
                    let result = line_parser.resolve()?;
                    if result != Token::Void && result != Token::EOF {
                        if let Token::Operation(mut operation) = result {
                            let operation_result = operation.resolve().unwrap();
                            if operation_result != Token::Void && operation_result != Token::EOF {
                                results.push(operation_result);
                            }
                        }
                    }
                }
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
        // Detectamos si la asignación inicia con 'var'
        let mut new_var = false;
        if let Token::Keyword(Keyword::Var) = self.tokens[self.position] {
            self.position += 1; // Consume `var`
            new_var = true;
        }

        // Esperamos un identificador
        let identifier = match self.tokens.get(self.position) {
            Some(Token::Identifier(name)) => {
                self.position += 1; // Consume el identificador
                name.to_owned()
            }
            _ => {
                // Si no hay identificador, no podemos seguir
                return Ok(());
            }
        };

        // Obtenemos el operador de asignación: puede ser '=' o '+='
        let operator = match self.tokens.get(self.position) {
            Some(Token::Operator(op)) => {
                self.position += 1; // Consume '='
                op.clone()
            }
            // Si no hay operador de asignación válido, retornamos sin error.
            // Podrías retornar un error si lo deseas.
            _ => {
                return Ok(());
            }
        };

        // Resolvemos la expresión del lado derecho
        let value_token = self.resolve()?;
        if operator.is_assignation() {
            // Reasignación con suma: x += valor
            let mut variable: Variable;
            let mut table = self.variables.borrow_mut();
            if let Ok(var) = table.get(&identifier) {
                // Ejecutamos la operación sumando al valor actual
                match value_token {
                    Token::Operation(mut expr) => {
                        let value = expr.resolve()?;
                        if value == Token::Void {
                            return Err(ParseError::SyntaxError(
                                "Se esperaba un valor distinto a Void",
                            ));
                        }
                        variable = var.clone();
                        println!("Pow1");
                        *variable.value = operator.execute(*var.value.clone(), value);
                    }
                    value => {
                        variable = var.clone();
                        println!("Pow2");
                        *variable.value = operator.execute(*var.value.clone(), value);
                    }
                }
            } else if new_var {
                // Si se usó 'var' + '+=', no tiene mucho sentido, pues la variable no existe aún.
                // Podríamos tratar este caso como un error.
                return Err(ParseError::UndefinedVariable(Box::leak(
                    format!("Variable {} not defined", identifier).into_boxed_str(),
                )));
            } else {
                // Si no se usó var y la variable no existe, error.
                return Err(ParseError::UndefinedVariable(Box::leak(
                    format!("Variable {} not defined", identifier).into_boxed_str(),
                )));
            }
            let _ = table.update(identifier, &mut variable);
            return Ok(());
        }

        let variable: Variable;

        match value_token {
            Token::Operation(mut expr) => {
                let value = expr.resolve().unwrap();
                if value == Token::Void {
                    return Err(ParseError::SyntaxError("error"));
                }
                variable =
                    Variable::new(identifier.to_string(), value.to_string(), value.clone(), 0);
            }
            value => {
                variable =
                    Variable::new(identifier.to_string(), value.to_string(), value.clone(), 0);
            }
        }
        self.variables
            .borrow_mut()
            .variables
            .insert(identifier.to_string(), variable);

        Ok(())
    }

    fn resolve(&mut self) -> Result<Token<'a>, ParseError<'a>> {
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

    fn term(&mut self) -> Result<Token<'a>, ParseError<'a>> {
        let mut node = self.factor()?;

        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Operator(op)
                    if matches!(
                        op,
                        Operator::Mul
                            | Operator::Div
                            | Operator::Mod
                            | Operator::DivInt
                            | Operator::Pow
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

            // for positive or negative numbers
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
                let msg = format!("Unexpected token in position {}: {}", self.position, v);
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
        let mut parser: Parser<'_> = Parser::new(tokens, None);
        let parse = parser.parse().unwrap();

        assert_eq!(parse, vec![])
    }
}
