use std::collections::HashMap;

use super::{error::parse_error::ParseError, variable::Variable};

#[derive(Debug, Clone)]
pub struct VariableTable {
    pub(crate) variables: HashMap<String, Variable>,
}

#[allow(dead_code)]
impl VariableTable {
    pub fn new() -> Self {
        VariableTable {
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Result<&Variable, ParseError> {
        if let Some(var) = self.variables.get(key) {
            Ok(var)
        } else {
            Err(ParseError::UndefinedVariable(format!(
                "The variable {key} doesn't exists."
            )))
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Result<&mut Variable, ParseError> {
        if let Some(var) = self.variables.get_mut(key) {
            Ok(var)
        } else {
            Err(ParseError::UndefinedVariable(format!(
                "The variable {key} doesn't exists."
            )))
        }
    }

    pub fn insert(&mut self, key: &str, value: Variable) -> Result<(), ParseError> {
        if !self.variables.contains_key(key) {
            self.variables.insert(key.to_string(), value);
            Ok(())
        } else if let Some(var) = self.variables.get_key_value(key) {
            if *var.1 == value {
                return Err(ParseError::DefinedVariable("{var.1.name}".into()));
            }
            self.variables.insert(key.to_string(), value);
            Ok(())
        } else {
            self.variables.insert(key.to_string(), value);
            Ok(())
        }
    }

    pub fn update(&mut self, key: &str, value: &mut Variable) -> Result<Variable, ParseError> {
        if let Some(mut var) = self.variables.get_mut(key) {
            let aux = var.clone();
            var = value;
            self.variables.insert(key.to_string(), var.clone());
            Ok(aux)
        } else {
            Err(ParseError::UndefinedVariable(format!(
                "The variable {key} doesn't exists."
            )))
        }
    }

    fn delete_one(&mut self, key: &str) -> Result<Variable, ParseError> {
        if let Some(var) = self.variables.remove(key) {
            Ok(var)
        } else {
            Err(ParseError::UndefinedVariable(format!(
                "The variable {key} doesn't exists."
            )))
        }
    }

    pub fn delete(&mut self, keys: Vec<&str>) -> (Vec<Variable>, Vec<String>) {
        let mut result: Vec<Variable> = vec![];
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
