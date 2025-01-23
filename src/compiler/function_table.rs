use std::collections::HashMap;

use super::{error::parse_error::ParseError, function::Function};
use crate::std::{standard_library, StdFunc};

#[derive(Debug, Clone)]
pub struct FunctionTable {
    pub(crate) functions: HashMap<String, Function>,
    pub(crate) std: HashMap<String, StdFunc>,
}

pub enum Func {
    User(Function),
    Std(StdFunc),
}

#[allow(dead_code)]
impl FunctionTable {
    pub fn new() -> Self {
        FunctionTable {
            functions: HashMap::new(),
            std: standard_library(),
        }
    }

    pub fn get(&self, key: &str) -> Result<Func, ParseError> {
        if let Some(var) = self.std.get(key) {
            Ok(Func::Std(var.clone()))
        } else if let Some(var) = self.functions.get(key) {
            Ok(Func::User(var.clone()))
        } else {
            Err(ParseError::UndefinedVariable(format!(
                "The function {key} doesn't exists."
            )))
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Result<&mut Function, ParseError> {
        if let Some(var) = self.functions.get_mut(key) {
            Ok(var)
        } else {
            Err(ParseError::UndefinedVariable(format!(
                "The function {key} doesn't exists."
            )))
        }
    }

    pub fn insert(&mut self, key: &str, value: Function) -> Result<(), ParseError> {
        if !self.functions.contains_key(key) {
            self.functions.insert(key.to_string(), value);
            Ok(())
        } else if let Some(var) = self.functions.get_key_value(key) {
            if *var.1 == value {
                return Err(ParseError::DefinedVariable(format!("1.name")));
            }
            self.functions.insert(key.to_string(), value);
            Ok(())
        } else {
            self.functions.insert(key.to_string(), value);
            Ok(())
        }
    }

    pub fn update(&mut self, key: &str, value: &mut Function) -> Result<Function, ParseError> {
        if let Some(mut var) = self.functions.get_mut(key) {
            let aux = var.clone();
            var = value;
            self.functions.insert(key.to_string(), var.clone());
            Ok(aux)
        } else {
            Err(ParseError::UndefinedVariable(format!(
                "The function {key} doesn't exists."
            )))
        }
    }

    fn delete_one(&mut self, key: &str) -> Result<Function, ParseError> {
        if let Some(var) = self.functions.remove(key) {
            Ok(var)
        } else {
            Err(ParseError::UndefinedVariable(format!(
                "The function {key} doesn't exists."
            )))
        }
    }

    pub fn delete(&mut self, keys: Vec<&str>) -> (Vec<Function>, Vec<String>) {
        let mut result: Vec<Function> = vec![];
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
