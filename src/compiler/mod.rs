use std::{collections::HashMap, vec};

use elements::token::Token;
use error::parse_error::ParseError;

pub mod elements;
pub mod error;
pub mod lexer;
pub mod parser;

#[derive(Debug)]
pub struct VariableTable<'a> {
    variables: HashMap<String, Variable<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub struct Variable<'a> {
    name: String,
    var_type: String,
    value: Box<Token<'a>>,
    scope: usize,
}

#[allow(dead_code)]
impl<'a> VariableTable<'a> {
    pub fn new() -> Self {
        VariableTable {
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Result<&Variable<'a>, ParseError<'a>> {
        if let Some(var) = self.variables.get(key) {
            Ok(var)
        } else {
            Err(ParseError::UndefinedVariable(Box::leak(
                format!("The variable {key} doesn't exists.").into_boxed_str(),
            )))
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Result<&mut Variable<'a>, ParseError<'a>> {
        if let Some(var) = self.variables.get_mut(key) {
            Ok(var)
        } else {
            Err(ParseError::UndefinedVariable(Box::leak(
                format!("The variable {key} doesn't exists.").into_boxed_str(),
            )))
        }
    }

    pub fn insert(&mut self, key: &str, value: Variable<'a>) -> Result<(), ParseError<'a>> {
        if !self.variables.contains_key(key) {
            self.variables.insert(key.to_string(), value);
            Ok(())
        } else if let Some(var) = self.variables.get_key_value(key) {
            if *var.1 == value {
                return Err(ParseError::DefinedVariable("{var.1.name}"));
            }
            self.variables.insert(key.to_string(), value);
            Ok(())
        } else {
            self.variables.insert(key.to_string(), value);
            Ok(())
        }
    }

    pub fn update(
        &mut self,
        key: &str,
        value: &mut Variable<'a>,
    ) -> Result<Variable<'a>, ParseError<'a>> {
        if let Some(mut var) = self.variables.get_mut(key) {
            let aux = var.clone();
            var = value;
            self.variables.insert(key.to_string(), var.clone());
            Ok(aux)
        } else {
            Err(ParseError::UndefinedVariable(Box::leak(
                format!("The variable {key} doesn't exists.").into_boxed_str(),
            )))
        }
    }

    fn delete_one(&mut self, key: &str) -> Result<Variable<'a>, ParseError<'a>> {
        if let Some(var) = self.variables.remove(key) {
            Ok(var)
        } else {
            Err(ParseError::UndefinedVariable(Box::leak(
                format!("The variable {key} doesn't exists.").into_boxed_str(),
            )))
        }
    }

    pub fn delete(&mut self, keys: Vec<&str>) -> (Vec<Variable<'a>>, Vec<String>) {
        let mut result: Vec<Variable<'a>> = vec![];
        let mut not_deleted: Vec<String> = vec![];
        if keys.len() > 1 {
            for key in keys {
                if let Ok(var) = self.delete_one(key) {
                    result.push(var);
                } else {
                    not_deleted.push(key.to_string());
                }
            }
        } else {
            if let Ok(var) = self.delete_one(keys[0]) {
                result.push(var);
            } else {
                not_deleted.push(keys[0].to_string());
            }
        }
        (result, not_deleted)
    }
}

#[allow(dead_code)]
impl<'a> Variable<'a> {
    pub fn new(name: String, var_type: String, value: Token<'a>, scope: usize) -> Self {
        Self {
            name,
            var_type,
            value: Box::new(value),
            scope,
        }
    }

    pub fn to_token(&self) -> &Token<'_> {
        &*self.value
    }
}
