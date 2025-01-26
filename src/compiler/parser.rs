use crate::compiler::{
    function::{Argument, Function},
    types::Types,
};
use crate::std::debug::DEBUG_LIST;

use super::{
    elements::{keyword::Keyword, operation::Operation, operator::Operator, token::Token},
    error::parse_error::ParseError,
    function_table::{Func, FunctionTable},
    variable::Variable,
    variable_table::VariableTable,
};
use std::{cell::RefCell, rc::Rc};

/// This struct is in charge of manage the logic and semantic
#[derive(Debug, Clone)]
pub struct Parser {
    /// List of tokens to parse
    tokens: Vec<Token>,
    position: usize,
    scope: usize,
    variables: Rc<RefCell<VariableTable>>,
    functions: Rc<RefCell<FunctionTable>>,
}

#[allow(dead_code)]
impl Parser {
    // MARK: Creation
    pub fn new(
        tokens: Vec<Token>,
        variables: Option<Rc<RefCell<VariableTable>>>,
        functions: Option<Rc<RefCell<FunctionTable>>>,
    ) -> Self {
        // Using into_iter I can take the ownership and avoid clone
        let tokens: Vec<Token> = tokens.into_iter().filter(|x| *x != Token::EOF).collect();
        Parser {
            tokens: tokens.into_iter().filter(|x| *x != Token::EOF).collect(),
            position: 0,
            scope: 0,
            variables: variables.unwrap_or(Rc::new(RefCell::new(VariableTable::new()))),
            functions: functions.unwrap_or(Rc::new(RefCell::new(FunctionTable::new()))),
        }
    }

    fn internal_new(
        tokens: Vec<Token>,
        scope: usize,
        variables: Rc<RefCell<VariableTable>>,
        functions: Rc<RefCell<FunctionTable>>,
    ) -> Self {
        Parser {
            tokens: tokens.into_iter().filter(|x| *x != Token::EOF).collect(),
            position: 0,
            scope,
            variables,
            functions,
        }
    }

    // MARK: Get
    pub fn get_variables(&self) -> Vec<(String, Variable)> {
        self.variables
            .borrow()
            .variables
            .iter()
            .map(|(key, var)| (key.clone(), var.clone()))
            .collect()
    }

    pub fn get_variable_table(&self) -> VariableTable {
        (*self.variables.borrow()).clone()
    }

    pub fn get_function_table(&self) -> FunctionTable {
        (*self.functions.borrow()).clone()
    }

    pub fn get_functions(&self) -> Vec<(String, Function)> {
        let mut result = vec![];

        for (key, variable) in self.functions.borrow().functions.iter() {
            result.push((key.to_owned(), variable.to_owned()));
        }

        result
    }

    // MARK: Parse
    pub fn parse(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut results = Vec::new();

        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            if *token == Token::EOF {
                continue;
            }

            match token {
                Token::Keyword(Keyword::Var) => match self.assignment() {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Parsing error: {:?}", err);
                        self.recover_from_error();
                    }
                },
                Token::Keyword(Keyword::Function) => {
                    if let Err(err) = self.function_assignment() {
                        eprintln!("Parsing error in function definition: {:?}", err);
                        self.recover_from_error();
                    }
                }
                Token::Operator(op) if op.is_assignation() => match self.assignment() {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Parsing error: {:?}", err);
                        self.recover_from_error();
                    }
                },
                _ => {
                    let result = self.resolve()?;
                    if result != Token::Void && result != Token::EOF {
                        let data = match result {
                            Token::Operation(mut op) => op.resolve()?,
                            v => v,
                        };
                        results.push(data);
                    }
                }
            }

            self.position += 1;
        }

        Ok(results)
    }

    fn resolve(&mut self) -> Result<Token, ParseError> {
        let mut node = self.term()?;

        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                // Manejo de operadores matemáticos simples
                Token::Operator(op) if matches!(op, Operator::Add | Operator::Sub) => {
                    let operator = op.clone();
                    self.position += 1; // Consume el operador
                    let right = self.term()?;
                    node = Token::Operation(Operation::new(operator, node, right));
                }

                // Manejo de operadores de asignación (+=, -=, *=, /=)
                Token::Operator(op) if op.is_assignation() => {
                    self.position += 1; // Consume el operador

                    // let right = self.term()?;

                    // Aplicar la asignación a una variable existente
                    if let Token::Identifier(identifier) = &node {
                        let mut table = self.variables.borrow_mut();
                        if let Ok(_) = table.get_mut(identifier) {
                            node = Token::Identifier(identifier.clone()); // Devuelve la variable actualizada
                        } else {
                            return Err(ParseError::UndefinedVariable(identifier.to_string()));
                        }
                    } else {
                        return Err(ParseError::SyntaxError(format!(
                            "Assignment operator '{}' used incorrectly",
                            node
                        )));
                    }
                }

                _ => break,
            }
        }

        Ok(node)
    }

    fn term(&mut self) -> Result<Token, ParseError> {
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
                    let right = self.factor().map_err(|_| {
                        ParseError::SyntaxError("Expected an operand after the operator".into())
                    })?;
                    node = Token::Operation(Operation::new(operator, node, right));
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn factor(&mut self) -> Result<Token, ParseError> {
        if self.position >= self.tokens.len() {
            return Ok(Token::Void);
        }

        match &self.tokens[self.position] {
            Token::Int32(n) => {
                self.position += 1; // Consume the number
                Ok(Token::Int32(*n))
            }

            Token::Double(n) => {
                self.position += 1; // Consume the number
                Ok(Token::Double(*n))
            }

            Token::EOF => {
                self.position += 1; // Consume the number
                return self.factor();
            }

            Token::NewLine => {
                self.position += 1; // Consume the new line
                while let Some(Token::NewLine) = self.tokens.get(self.position) {
                    self.position += 1;
                }
                return self.factor();
            }

            Token::StartBrace => {
                self.position += 1; // Consume the brace
                self.scope += 1;
                while let Some(Token::NewLine) = self.tokens.get(self.position) {
                    self.position += 1;
                }
                return self.factor();
            }

            Token::EndBrace => {
                self.position += 1; // Consume the brace
                self.scope -= 1;
                self.variables.borrow_mut().pop_scope(self.scope + 1);
                while let Some(Token::NewLine) = self.tokens.get(self.position) {
                    self.position += 1;
                }
                return self.factor();
            }

            Token::String(s) => {
                self.position += 1; // Consume the string
                Ok(Token::String(s.clone()))
            }

            Token::Str(s) => {
                self.position += 1; // Consume the string
                Ok(Token::Str(s.clone()))
            }

            Token::Keyword(k) => {
                self.position += 1; // Consume the keyword
                match k {
                    Keyword::Var => {
                        self.assignment()?;
                        Ok(Token::EOF)
                    }
                    Keyword::True => Ok(Token::Boolean(true)),
                    Keyword::False => Ok(Token::Boolean(false)),
                    _ => {
                        let msg = format!(
                            "Unexpected keyword at position {}: {}",
                            self.position.clone(),
                            k
                        );
                        Err(ParseError::SyntaxError(msg))
                    }
                }
            }

            Token::Identifier(var) => {
                self.position += 1;
                let var_name = var.to_string();
                self.process_identifier(&var_name)
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

            Token::Type(types) => {
                self.position += 1;
                Ok(Token::Type(types.clone()))
            }

            v => {
                let msg = format!("Unexpected token in position {}: {:?}", self.position, v);
                return Err(ParseError::SyntaxError(msg));
            }
        }
    }

    // MARK: Errors
    fn recover_from_error(&mut self) {
        let sync_tokens = [
            Token::NewLine,
            Token::Separator(';'),
            Token::StartBrace,
            Token::EndBrace,
        ];

        while self.position < self.tokens.len() {
            if sync_tokens.contains(&self.tokens[self.position]) {
                self.position += 1;
                break;
            }
            self.position += 1;
        }

        eprintln!("Error recovered at position: {}", self.position);
    }

    // MARK: Assign
    fn assignment(&mut self) -> Result<(), ParseError> {
        let new_var = self.consume_keyword_var();
        let identifier = self.consume_identifier(new_var)?;

        let var_type = self.consume_type().unwrap_or(Types::Inferred);

        let operator = match self.consume_assignment_operator() {
            Some(op) => op,
            None => return Ok(()), // Si no hay operador, la asignación es inválida.
        };
        let value_token = self.resolve()?;
        let inferred_type = Types::inferred(&value_token)?;

        if operator.is_assignation() {
            self.validate_existing_identifier()?;
            let mut table = self.variables.borrow_mut();
            let var = table.get_mut(&identifier)?;

            let existing_value = *var.value.clone();
            let new_value = match value_token.clone() {
                Token::Operation(mut expr) => expr.resolve()?,
                value => operator.execute(existing_value, value)?,
            };

            let inferred_type = Types::inferred(&new_value)?;

            if inferred_type == var.var_type {
                var.value = Box::new(new_value);
            } else {
                return Err(ParseError::TypeError(format!(
                    "Cannot assign type '{}' to variable '{}'",
                    inferred_type, identifier
                )));
            }
            let mut var = var.clone();
            table.update(identifier.as_ref(), &mut var)?;
        }

        if operator.is_assignation() {
            self.validate_existing_identifier()?;
            let mut table = self.variables.borrow_mut();
            let var = table.get_mut(&identifier)?;

            let existing_value = *var.value.clone();
            let new_value = match value_token.clone() {
                Token::Operation(mut expr) => expr.resolve()?,
                value => operator.execute(existing_value, value)?,
            };

            let inferred_type = Types::inferred(&new_value)?;

            if inferred_type == var.var_type {
                var.value = Box::new(new_value);
            } else {
                return Err(ParseError::TypeError(format!(
                    "Cannot assign type '{}' to variable '{}'",
                    inferred_type, identifier
                )));
            }
            let mut var = var.clone();
            table.update(identifier.as_ref(), &mut var)?;
        }

        if operator.is_assignation() {
            self.handle_variable_reassignment(identifier, value_token, operator, new_var)
        } else {
            self.handle_variable_declaration(identifier, value_token, inferred_type, var_type)
        }
    }

    fn handle_variable_reassignment(
        &mut self,
        identifier: String,
        value_token: Token,
        operator: Operator,
        new_var: bool,
    ) -> Result<(), ParseError> {
        let mut table = self.variables.borrow_mut();
        if let Ok(var) = table.get_mut(&identifier) {
            let new_value = match value_token {
                Token::Operation(mut expr) => expr.resolve()?,
                value => operator.execute(*var.value.clone(), value)?,
            };

            let inferred_type = Types::inferred(&new_value)?;
            let mut var = var.clone();
            if inferred_type == var.var_type {
                var.value = Box::new(new_value);
            } else {
                var.var_type = inferred_type;
                var.value = Box::new(new_value);
            }

            table.update(identifier.as_ref(), &mut var)?;
            Ok(())
        } else if new_var {
            Err(ParseError::UndefinedVariable(format!(
                "Variable {} not defined",
                identifier
            )))
        } else {
            Err(ParseError::UndefinedVariable(format!(
                "Variable {} not defined",
                identifier
            )))
        }
    }

    fn handle_variable_declaration(
        &mut self,
        identifier: String,
        value_token: Token,
        inferred_type: Types,
        mut var_type: Types,
    ) -> Result<(), ParseError> {
        let mut table = self.variables.borrow_mut();

        let new_value = if inferred_type != var_type {
            if inferred_type.is_integer() && var_type.is_integer() {
                Types::transform(value_token, var_type)?.0
            } else if inferred_type.is_float() && var_type.is_float() {
                Types::transform(value_token, var_type)?.0
            } else if var_type == Types::Inferred {
                var_type = inferred_type;
                value_token
            } else {
                return Err(ParseError::TypeError(format!(
                    "The type of '{}' must be <{var_type}> but it's <{}>.",
                    identifier,
                    Types::from(value_token)
                )));
            }
        } else {
            value_token
        };

        let variable = Variable::new(identifier.clone(), var_type, new_value, self.scope);
        table.insert(identifier.as_ref(), variable)?;

        Ok(())
    }

    // MARK: Consume
    fn consume_keyword_var(&mut self) -> bool {
        if let Token::Keyword(Keyword::Var) = self.tokens[self.position] {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn consume_identifier(&mut self, new_var: bool) -> Result<String, ParseError> {
        match self.tokens.get(self.position) {
            Some(Token::Identifier(name)) => {
                self.position += 1;
                Ok(name.to_string())
            }
            _ => {
                if new_var {
                    Err(ParseError::SyntaxError(
                        "Unexpected an identifier before assignment operator".into(),
                    ))
                } else {
                    Ok("".into())
                }
            }
        }
    }

    fn consume_type(&mut self) -> Option<Types> {
        if let Some(Token::Separator(':')) = self.tokens.get(self.position) {
            self.position += 1;
            if let Some(Token::Type(tipo)) = self.tokens.get(self.position) {
                self.position += 1;
                return Some(tipo.clone());
            }
        }
        None
    }

    fn consume_assignment_operator(&mut self) -> Option<Operator> {
        match self.tokens.get(self.position) {
            Some(Token::Operator(op)) => {
                self.position += 1;
                Some(op.clone())
            }
            _ => None,
        }
    }

    // MARK: Auxiliary
    fn validate_existing_identifier(&mut self) -> Result<String, ParseError> {
        match self.tokens.get(self.position - 2) {
            Some(Token::Identifier(id)) => Ok(id.to_string()),
            _ => Err(ParseError::SyntaxError(
                "Expected an identifier before assignment operator".into(),
            )),
        }
    }

    // MARK: Functions
    fn function_assignment(&mut self) -> Result<(), ParseError> {
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
                return Err(ParseError::SyntaxError("Bad Function Definition".into()));
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
                "Expected '{' after function arguments".into(),
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

        let mut arg_array: Vec<Argument> = vec![];
        for x in arguments
            .split(|x| *x == Token::Separator(','))
            .map(|x| x.to_owned())
            .collect::<Vec<Vec<Token>>>()
            .iter()
        {
            if let [name_slice, var_type_slice] = x
                .split(|x| *x == Token::Separator(':'))
                .collect::<Vec<&[Token]>>()
                .as_slice()
            {
                // Procesa los elementos de forma segura
                let name = name_slice.first().ok_or_else(|| {
                    ParseError::SyntaxError("Argument name missing in function definition".into())
                })?;

                if let Token::Identifier(_) = name {
                } else {
                    return Err(ParseError::SyntaxError("Invalid argument name".into()));
                };

                if var_type_slice.len() > 1 {
                    if var_type_slice
                        .iter()
                        .find(|x| **x == Token::Operator(Operator::Assign))
                        .is_some()
                    {
                        let var_type = var_type_slice.first().ok_or_else(|| {
                            ParseError::SyntaxError(
                                "Argument type missing in function definition".into(),
                            )
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
                                "Argument assignation operator missing in function definition"
                                    .into(),
                            ));
                        }

                        let name = if let Token::Identifier(id) = name {
                            id
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Invalid Name of atribbute".to_string(),
                            ));
                        };

                        let var_type = if let Token::Type(my_type) = var_type {
                            my_type.clone()
                        } else {
                            return Err(ParseError::SyntaxError(
                                "Invalid Type of atribbute".to_string(),
                            ));
                        };

                        arg_array.push(Argument::new(
                            name.clone(),
                            var_type,
                            Some(Box::new(value)),
                            None,
                        ));
                    } else {
                        return Err(ParseError::SyntaxError("Invalid argument type".into()));
                    }
                } else {
                    let var_type = var_type_slice.first().ok_or_else(|| {
                        ParseError::SyntaxError(
                            "Argument type missing in function definition".into(),
                        )
                    })?;

                    let name = if let Token::Identifier(id) = name {
                        id
                    } else {
                        return Err(ParseError::SyntaxError(
                            "Invalid Name of atribbute".to_string(),
                        ));
                    };

                    let var_type = if let Token::Type(my_type) = var_type {
                        my_type.clone()
                    } else {
                        return Err(ParseError::SyntaxError(
                            "Invalid Type of atribbute".to_string(),
                        ));
                    };

                    arg_array.push(Argument::new(name.clone(), var_type, None, None));
                }
            } else {
                return Err(ParseError::SyntaxError("Invalid argument format".into()));
            }
        }
        let mut table = self.functions.borrow_mut();
        if let Ok(_) = table.get(identifier.as_ref()) {
            return Err(ParseError::DefinedFunction(identifier.to_string()));
        } else {
            table.insert(
                identifier.as_ref(),
                Function::new(identifier.clone(), Types::Void, arg_array, content, 0),
            )?;
        }
        return Ok(());
    }

    fn process_function_call(&self, var: &str, args: Vec<Argument>) -> Result<Token, ParseError> {
        let func = if let Ok(function) = self.functions.borrow().get(var) {
            function
        } else {
            return Err(ParseError::UndefinedFunction(
                "This function doesn't exist.".into(),
            ));
        };
        match func {
            Func::Std(std_func) => {
                let result = if DEBUG_LIST.contains(&std_func.name.as_str()) {
                    let mut vars = vec![];
                    for (_, var) in self.variables.borrow().variables.iter() {
                        vars.push(var.details());
                    }
                    let params = vec![Argument::new(
                        "variables".into(),
                        Types::List,
                        None,
                        Some(Box::new(Token::List(
                            vars.iter().map(|x| Token::Str(x.clone().into())).collect(),
                        ))),
                    )];
                    std_func.call(params)
                } else {
                    std_func.call(args)
                };
                if let Err(err) = result {
                    return Err(ParseError::FunctionExecution(err));
                }
                return Ok(result.unwrap());
            }
            Func::User(func) => {
                let result = func.call(args, self.variables.clone(), self.functions.clone());

                if let Err(err) = result {
                    return Err(ParseError::FunctionExecution(err.to_string()));
                }
                return Ok(result.unwrap());
            }
        }
    }

    fn process_identifier(&mut self, var: &str) -> Result<Token, ParseError> {
        let args = self.is_function_call()?;
        if args.0 {
            return self.process_function_call(var, args.1);
        }

        let (is_assignment, op) = self.is_assignment()?;
        self.position += 1;
        if is_assignment {
            let var = self.validate_existing_identifier()?;
            let right = self.term()?;

            // Aplicar la asignación a una variable existente
            let mut table = self.variables.borrow_mut();
            if let Ok(var) = table.get_mut(&var) {
                let new_value = op.execute(*var.value.clone(), right)?;
                let inferred_type = Types::inferred(&new_value)?;

                if inferred_type == var.var_type {
                    var.value = Box::new(new_value);
                } else {
                    return Err(ParseError::TypeError(format!(
                        "Cannot assign type '{}' to variable '{}'",
                        inferred_type, var.name
                    )));
                }
                let mut var = var.clone();
                let id = var.name.clone();

                table.update(&id, &mut var)?;
            } else {
                return Err(ParseError::UndefinedVariable(var));
            }
            return Ok(Token::Void);
        }

        if let Some(variable) = self.variables.borrow_mut().get(var).ok() {
            return Ok(*variable.value.clone());
        }

        self.handle_undefined_variable_or_type(var)
    }

    fn is_function_call(&mut self) -> Result<(bool, Vec<Argument>), ParseError> {
        if let Some(Token::StartParenthesis) = self.tokens.get(self.position) {
            self.position += 1;
            let args = Parser::get_args(
                &self.tokens,
                &mut self.position,
                self.variables.borrow().clone(),
                self.functions.borrow().clone(),
            )?;

            Ok((true, args))
        } else {
            Ok((false, vec![]))
        }
    }

    fn is_assignment(&mut self) -> Result<(bool, Operator), ParseError> {
        if let Some(Token::Operator(op)) = self.tokens.get(self.position) {
            if op.is_assignation() {
                return Ok((true, op.clone()));
            }
        }
        Ok((false, Operator::Null))
    }

    fn handle_undefined_variable_or_type(&self, var: &str) -> Result<Token, ParseError> {
        if let Some(Token::Separator(':')) = self.tokens.get(self.position - 2) {
            Err(ParseError::UndefinedType(format!(
                "The type '{var}' doesn't exist."
            )))
        } else {
            Err(ParseError::UndefinedVariable(format!(
                "The variable '{var}' doesn't exist."
            )))
        }
    }

    /// For any transformation you want to make to the arguments, you must change the body of this function.
    fn get_args(
        tokens: &[Token],
        position: &mut usize,
        variables: VariableTable,
        functions: FunctionTable,
    ) -> Result<Vec<Argument>, ParseError> {
        let mut result = vec![];
        let mut scopes = 1;
        while *position < tokens.len() {
            let token = &tokens[*position];
            if *token == Token::EndParenthesis && scopes == 1 {
                *position += 1;
                break;
            }
            if *token == Token::StartParenthesis {
                scopes += 1;
            }

            result.push(token.clone());
            *position += 1;
        }
        println!("args: {result:?}");
        let mut data: Vec<Argument> = vec![];
        for res in result.split(|x| *x.to_string() == Token::Separator(',').to_string()) {
            let mut parser = Parser::internal_new(
                res.to_vec(),
                0,
                Rc::new(RefCell::new(variables.clone())),
                Rc::new(RefCell::new(functions.clone())),
            );
            let result = parser.parse()?;
            if !result.is_empty() {
                data.push(Argument::from(result[0].clone()));
            }
        }
        Ok(data)
    }
}

impl PartialEq for Parser {
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
                scope: 0,
                variables: Rc::new(RefCell::new(VariableTable::new())),
                functions: Rc::new(RefCell::new(FunctionTable::new()))
            }
        )
    }

    #[test]
    fn parse_test() {
        let mut lex: Lexer<'static> = Lexer::new("var hola = 10\n");
        let tokens = lex.lex();
        let mut parser: Parser = Parser::new(tokens, None, None);
        let parse = parser.parse().unwrap();

        assert_eq!(parse, vec![])
    }
}
