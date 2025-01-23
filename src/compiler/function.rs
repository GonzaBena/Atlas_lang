use std::{cell::RefCell, fmt::Debug, rc::Rc, str::FromStr, sync::Arc};

use crate::compiler::error::parse_error::ParseError;

use super::{
    elements::token::Token, error::function_error::FunctionError, function_table::FunctionTable,
    parser::Parser, types::Types, variable::Variable, variable_table::VariableTable,
};

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub struct Argument {
    pub(crate) name: Arc<str>,
    pub(crate) var_type: Types,
    pub(crate) default_value: Option<Box<Token>>,
    pub(crate) value: Option<Box<Token>>,
}

impl<'a> From<Token> for Argument {
    fn from(value: Token) -> Self {
        Self {
            name: "".into(),
            var_type: Types::from(&value),
            default_value: Some(Box::new(value.clone())),
            value: Some(Box::new(value)),
        }
    }
}

impl Argument {
    pub fn new(
        name: Arc<str>,
        var_type: Types,
        default_value: Option<Box<Token>>,
        value: Option<Box<Token>>,
    ) -> Self {
        Self {
            name,
            var_type,
            default_value,
            value,
        }
    }

    pub fn as_var(&self, scope: usize) -> Variable {
        Variable::new(
            self.name.to_string(),
            self.var_type.clone(),
            if self.value.is_some() {
                *self.value.clone().unwrap()
            } else {
                *self.default_value.clone().unwrap()
            },
            scope,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    name: Arc<str>,
    return_type: Types,
    args: Vec<Argument>,
    content: Vec<Token>,
    scope: usize,
    predefined: Option<Arc<str>>, // Usa un identificador único (nombre)
}

#[allow(dead_code)]
impl Function {
    pub fn new(
        name: Arc<str>,
        return_type: Types,
        args: Vec<Argument>,
        content: Vec<Token>,
        scope: usize,
    ) -> Self {
        Self {
            name,
            return_type,
            args,
            content,
            scope,
            predefined: None,
        }
    }

    pub fn new_predefined(name: Arc<str>, return_type: Types) -> Self {
        Self {
            name: name.clone(),
            return_type,
            args: vec![], // Las funciones predefinidas no necesitan argumentos explícitos
            content: vec![], // Tampoco necesitan un cuerpo
            scope: 0,     // El scope no importa aquí
            predefined: Some(name),
        }
    }

    fn num_args_with_default_value(&self) -> usize {
        return self
            .args
            .iter()
            .filter(|x| x.default_value.is_some())
            .count();
    }

    fn num_args_required(&self) -> usize {
        self.args.len() - self.num_args_with_default_value()
    }

    pub fn call(
        &self,
        arguments: Vec<Argument>,
        variables: Rc<RefCell<VariableTable>>,
        functions: Rc<RefCell<FunctionTable>>,
    ) -> Result<Token, FunctionError> {
        if arguments.len() > self.args.len() {
            return Err(FunctionError::InvalidNumberOfArgs(format!(
                "The function accept only {} arguments",
                self.args.len()
            )));
        }
        if arguments.len() < self.num_args_required() {
            return Err(FunctionError::InvalidNumberOfArgs(format!(
                "The function required only {} arguments",
                self.args.len()
            )));
        }
        let mut args_to_variables: Vec<Variable> = vec![];
        for var in &self.args {
            args_to_variables.push(var.as_var(self.scope));
        }
        let mut var_table: VariableTable = (*variables.borrow_mut()).clone();
        for arg in args_to_variables.iter_mut() {
            if let Ok(var) = variables.borrow().get(&arg.name) {
                let _ = var_table.update(&var.name, arg);
            } else {
                let _ = var_table.insert(&arg.name, arg.clone());
            }
        }

        let mut parser: Parser = Parser::new(
            self.content.clone(),
            Some(Rc::new(RefCell::new(var_table))),
            Some(functions),
        );
        let parse: Result<Vec<Token>, ParseError> = parser.parse();
        if parse.is_err() {
            return Err(FunctionError::ExecutionError(format!("r")));
        }
        let parse = parse.unwrap();
        if self.return_type == Types::Void {
            return Ok(Token::Void);
        }
        let result = parse[0].clone();
        let result_type = Types::from_str(&result.to_string());

        if result_type.is_err() || result_type.clone().unwrap() != self.return_type {
            return Err(FunctionError::DifferentReturnType(format!(
                " the return type is {:?} and you are returning {:?}",
                self.return_type, result_type
            )));
        }

        Ok(parse[0].clone())
    }
}
