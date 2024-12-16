use crate::compiler::{
    function::{Argument, Function},
    types::Types,
};

use super::{
    elements::{keyword::Keyword, operation::Operation, operator::Operator, token::Token},
    error::parse_error::ParseError,
    function_table::{Func, FunctionTable},
    variable::Variable,
    variable_table::VariableTable,
};
use std::{cell::RefCell, rc::Rc, str::FromStr};

/// This struct is in charge of manage the logic and semantic
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Parser<'a> {
    /// List of tokens to parse
    tokens: Vec<Token<'a>>,
    position: usize,
    variables: Rc<RefCell<VariableTable<'a>>>,
    functions: Rc<RefCell<FunctionTable<'a>>>,
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    pub fn new(
        tokens: Vec<Token<'a>>,
        variables: Option<Rc<RefCell<VariableTable<'a>>>>,
        functions: Option<Rc<RefCell<FunctionTable<'a>>>>,
    ) -> Self {
        let tokens: Vec<Token<'a>> = tokens
            .iter()
            .filter(|x| **x != Token::EOF)
            .map(|x| x.to_owned())
            .collect();
        Parser {
            tokens,
            position: 0,
            variables: variables.unwrap_or(Rc::new(RefCell::new(VariableTable::new()))),
            functions: functions.unwrap_or(Rc::new(RefCell::new(FunctionTable::new()))),
        }
    }

    fn internal_new(
        tokens: Vec<Token<'a>>,
        variables: Rc<RefCell<VariableTable<'a>>>,
        functions: Rc<RefCell<FunctionTable<'a>>>,
    ) -> Self {
        Parser {
            tokens,
            position: 0,
            variables,
            functions,
        }
    }

    pub fn get_variables(&self) -> Vec<(String, Variable<'a>)> {
        let mut result = vec![];

        for (key, variable) in self.variables.borrow().variables.iter() {
            result.push((key.to_owned(), variable.to_owned()));
        }

        result
    }

    pub fn get_variable_table(&self) -> VariableTable<'a> {
        (*self.variables.borrow()).clone()
    }

    pub fn get_function_table(&self) -> FunctionTable<'a> {
        (*self.functions.borrow()).clone()
    }

    pub fn get_functions(&self) -> Vec<(String, Function<'a>)> {
        let mut result = vec![];

        for (key, variable) in self.functions.borrow().functions.iter() {
            result.push((key.to_owned(), variable.to_owned()));
        }

        result
    }

    pub fn parse(&mut self) -> Result<Vec<Token<'a>>, ParseError<'a>> {
        let mut results = Vec::new();
        // let lines: Vec<&[Token<'a>]> = self
        //     .tokens
        //     .split(|t| *t == Token::NewLine)
        //     .filter(|x| !x.is_empty())
        //     .collect();

        let tokens: Vec<Token<'a>> = self
            .tokens
            .iter()
            .filter(|x| **x != Token::EOF)
            .map(|x| x.to_owned())
            .collect();
        if let Some(Token::Keyword(Keyword::Var)) = tokens.get(0) {
            self.assignment()?;
        } else if let Some(Token::Keyword(Keyword::Function)) = tokens.get(0) {
            self.function_assignment()?;
        } else if let Some(Token::Operator(op)) = tokens.get(1) {
            if op.is_assignation() {
                self.assignment()?;
            } else {
                let result = self.resolve()?;
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
            let result = self.resolve()?;
            if result != Token::Void {
                if let Token::Operation(mut operation) = result {
                    let operation_result = operation.resolve().unwrap();
                    results.push(operation_result);
                }
            }
        }

        Ok(results)
    }

    fn function_assignment(&mut self) -> Result<(), ParseError<'a>> {
        self.position += 1;
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

        match self.tokens.get(self.position) {
            Some(Token::StartParenthesis) => {
                self.position += 1; // Consume el identificador
            }
            _ => {
                // Si no hay identificador, no podemos seguir
                return Err(ParseError::SyntaxError("Bad Function Definition"));
            }
        };

        let mut arguments = vec![];
        while let Some(tok) = self.tokens.get(self.position) {
            if *tok == Token::EndParenthesis {
                self.position += 1; // Avanza más allá del `EndParenthesis`
                break;
            }
            arguments.push(tok.clone());
            self.position += 1;
        }

        // Verifica que `StartBrace` esté después de los argumentos
        if self.tokens.get(self.position) != Some(&Token::StartBrace) {
            return Err(ParseError::SyntaxError(
                "Expected '{' after function arguments",
            ));
        }
        self.position += 1; // Consume el `StartBrace`

        let mut content = vec![];
        while let Some(tok) = self.tokens.get(self.position) {
            if *tok == Token::EndBrace {
                self.position += 1; // Consume el `EndBrace`
                break;
            }
            content.push(tok.clone());
            self.position += 1;
        }

        let mut arg_array: Vec<Argument<'a>> = vec![];
        for x in arguments
            .split(|x| *x == Token::Separator(','))
            .map(|x| x.to_owned())
            .collect::<Vec<Vec<Token<'a>>>>()
            .iter()
        {
            if let [name_slice, var_type_slice] = x
                .split(|x| *x == Token::Separator(':'))
                .collect::<Vec<&[Token<'a>]>>()
                .as_slice()
            {
                // println!("x: {x:?}");
                // Procesa los elementos de forma segura
                let name = name_slice.first().ok_or_else(|| {
                    ParseError::SyntaxError("Argument name missing in function definition")
                })?;

                if let Token::Identifier(_) = name {
                } else {
                    return Err(ParseError::SyntaxError("Invalid argument name"));
                };

                if var_type_slice.len() > 1 {
                    if var_type_slice
                        .iter()
                        .find(|x| **x == Token::Operator(Operator::Assign))
                        .is_some()
                    {
                        let var_type = var_type_slice.first().ok_or_else(|| {
                            ParseError::SyntaxError("Argument type missing in function definition")
                        })?;

                        let operator = if let Some(Token::Operator(op)) = var_type_slice.get(1) {
                            op.clone()
                        } else {
                            Operator::Null
                        };

                        let value = if let Some(token) = var_type_slice.get(2) {
                            token.clone()
                        } else {
                            Token::Void
                        };

                        if value != Token::Void && operator != Operator::Assign {
                            return Err(ParseError::SyntaxError(
                                "Argument assignation operator missing in function definition",
                            ));
                        }

                        println!("name: {name:?}, var_type: {var_type:?}, operator: {operator:?}, value: {value:?}");
                        let name = if let Token::Identifier(id) = name {
                            id
                        } else {
                            panic!("Invalid Name of atribbute");
                        };

                        let var_type = if let Token::Type(my_type) = var_type {
                            my_type.clone()
                        } else {
                            panic!("Invalid type of atribbute");
                        };

                        arg_array.push(Argument::new(name, var_type, Some(Box::new(value)), None));
                    } else {
                        return Err(ParseError::SyntaxError("Invalid argument type"));
                    }
                } else {
                    let var_type = var_type_slice.first().ok_or_else(|| {
                        ParseError::SyntaxError("Argument type missing in function definition")
                    })?;

                    let name = if let Token::Identifier(id) = name {
                        id
                    } else {
                        panic!("Invalid Name of atribbute");
                    };

                    let var_type = if let Token::Type(my_type) = var_type {
                        my_type.clone()
                    } else {
                        panic!("Invalid type of atribbute");
                    };

                    arg_array.push(Argument::new(name, var_type, None, None));
                }
            } else {
                return Err(ParseError::SyntaxError("Invalid argument format"));
            }
        }
        println!("arguments: {arg_array:?}");
        let mut table = self.functions.borrow_mut();
        if let Ok(_) = table.get(identifier) {
            return Err(ParseError::DefinedFunction(identifier));
        } else {
            table.insert(
                identifier,
                Function::new(identifier, Types::Void, arg_array, content, 0),
            )?;
            println!("table: {:?}", *table);
        }
        return Ok(());
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
                        *variable.value = operator.execute(*var.value.clone(), value);
                    }
                    value => {
                        variable = var.clone();
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
                let var_type = Types::from_str(&value.to_string())?;
                variable = Variable::new(identifier.to_string(), var_type, value.clone(), 0);
            }
            value => {
                let var_type = Types::from_str(&value.to_string())?;
                variable = Variable::new(identifier.to_string(), var_type, value.clone(), 0);
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

            Token::NewLine => {
                self.position += 1; // Consume the number
                while let Some(Token::NewLine) = self.tokens.get(self.position) {
                    self.position += 1;
                }
                return self.factor();
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
                self.position += 1;
                if let Some(Token::StartParenthesis) = &self.tokens.get(self.position) {
                    // if let Some(Token::StartBrace) = &self.tokens.get(self.position) {
                    //     return Err(ParseError::SyntaxError(
                    //         "You must use Func keyword to define functions.",
                    //     ));
                    // }

                    let mut args = vec![];
                    let mut scopes = 1;

                    while let Some(token) = self.tokens.get(self.position) {
                        if *token == Token::EndParenthesis && scopes == 1 {
                            self.position += 1;
                            break;
                        }
                        if *token == Token::StartParenthesis {
                            scopes += 1;
                        }
                        args.push(token.clone());
                        self.position += 1;
                    }

                    let func = if let Ok(function) = self.functions.borrow().get(var) {
                        function
                    } else {
                        return Err(ParseError::UndefinedFunction(
                            "This function doesn't exist.",
                        ));
                    };

                    match func {
                        Func::Std(std_func) => {
                            let result = std_func.call(args);
                            if let Err(err) = result {
                                return Err(ParseError::FunctionExecution(Box::leak(
                                    err.to_string().into_boxed_str(),
                                )));
                            }
                            return Ok(result.unwrap());
                        }
                        Func::User(func) => {
                            let result =
                                func.call(args, self.variables.clone(), self.functions.clone());
                            if let Err(err) = result {
                                return Err(ParseError::FunctionExecution(Box::leak(
                                    err.to_string().into_boxed_str(),
                                )));
                            }
                            return Ok(result.unwrap());
                        }
                    }
                    return Ok(Token::Void);
                } else if let Ok(variable) = self.variables.borrow_mut().get(&var) {
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

    fn get_args(&mut self) -> Vec<Token<'a>> {
        let mut result = vec![];
        let mut scopes = 1;

        while let Some(token) = self.tokens.get(self.position) {
            if *token == Token::EndParenthesis && scopes == 1 {
                self.position += 1;
                break;
            }
            if *token == Token::StartParenthesis {
                scopes += 1;
            }
            result.push(token.clone());
            self.position += 1;
        }

        result
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
        let parser = Parser::new(vec![], None, None);
        assert_eq!(
            parser,
            Parser {
                tokens: vec![],
                position: 0,
                variables: Rc::new(RefCell::new(VariableTable::new())),
                functions: Rc::new(RefCell::new(FunctionTable::new()))
            }
        )
    }

    #[test]
    fn parse_test() {
        let mut lex: Lexer<'static> = Lexer::new("var hola = 10\n");
        let tokens = lex.lex();
        let mut parser: Parser<'_> = Parser::new(tokens, None, None);
        let parse = parser.parse().unwrap();

        assert_eq!(parse, vec![])
    }
}
