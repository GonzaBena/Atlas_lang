use std::collections::HashMap;

use super::{error::parse_error::ParseError, variable::Variable};

#[derive(Debug, Clone)]
pub struct VariableTable {
    pub(crate) variables: HashMap<String, Variable>,
    pub(crate) length: usize,
}

#[allow(dead_code)]
impl VariableTable {
    pub fn new() -> Self {
        VariableTable {
            variables: HashMap::new(),
            length: 0,
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
            let mut var = value.clone();
            var.set_id(self.length + 1);
            self.variables.insert(key.to_string(), var);
            self.length += 1;

            Ok(())
        } else if let Some((name, var)) = self.variables.get_key_value(key) {
            if *var == value {
                return Err(ParseError::DefinedVariable(format!("{name}")));
            }
            if var.id == 0 {
                let mut var = value.clone();
                var.set_id(self.length + 1);
                self.variables.insert(key.to_string(), var);
                self.length += 1;
                return Ok(());
            } else {
                self.variables.insert(key.to_string(), value);
                self.length += 1;
                Ok(())
            }
        } else {
            if value.id == 0 {
                let mut var = value.clone();
                var.set_id(self.length + 1);
                self.variables.insert(key.to_string(), value);
                self.length += 1;
                return Ok(());
            } else {
                self.variables.insert(key.to_string(), value);
                self.length += 1;
                Ok(())
            }
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
            self.length -= 1;
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

    pub fn pop_scope(&mut self, scope: usize) -> Vec<Variable> {
        let mut result: Vec<Variable> = vec![];
        let mut keys: Vec<String> = vec![];
        for (key, var) in self.variables.iter() {
            if var.scope >= scope {
                result.push(var.clone());
                keys.push(key.to_string());
            }
        }
        for key in keys {
            self.variables.remove(&key);
        }
        result
    }

    pub fn clear(&mut self) {
        self.variables.clear();
        self.length = 0;
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    pub fn show_vars(&self) -> String {
        let mut largest_id = "".to_string();
        let mut largest_name = "".to_string();
        let mut largest_type = "".to_string();
        let mut largest_value = "".to_string();
        let mut largest_scope = "".to_string();

        for key in self.variables.values() {
            if key.id.to_string().len() > largest_id.len() {
                largest_id = key.id.to_string();
            }
            if key.name.len() > largest_name.len() {
                largest_name = key.name.clone();
            }
            if key.var_type.to_string().len() > largest_type.len() {
                largest_type = key.var_type.to_string();
            }
            if key.value.to_string().len() > largest_value.len() {
                largest_value = key.value.to_string();
            }
            if key.scope.to_string().len() > largest_scope.len() {
                largest_scope = key.scope.to_string();
            }
        }
        let mut txt = String::new();
        for _ in 0..self.variables.iter().len() {
            let msg = format!(
                "{} - {}: {:?} = {} | scope: {}\n",
                largest_id, largest_name, largest_type, largest_value, largest_scope
            );
            txt.push_str(&msg);
        }
        txt
    }
}
