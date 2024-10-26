use crate::{
    compiler::identifier::IdentifierTable,
    compiler::tokens::{
        operand::Operand,
        operator::Operator,
        token::{Number, Token},
    },
    error::math_errors::MathError,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Operation<'a> {
    pub operator: Operator,
    pub left: Box<Operand<'a>>,
    pub right: Box<Operand<'a>>,
}

#[allow(dead_code)]
impl<'a> Operation<'a> {
    /// this function resolves the operation
    pub fn resolve(&self, table: &IdentifierTable<'a>) -> Result<Token<'a>, MathError> {
        let left = self.left.resolve(table)?;
        let right = self.right.resolve(table)?;

        match self.operator {
            Operator::Add => Ok(left + right),
            Operator::Sub => Ok(left - right),
            Operator::Mul => Ok(left * right),
            Operator::Div => {
                if right == Token::Number(Number::Int(0)) {
                    Err(MathError::ZeroDivision(
                        "Division by zero isn't mathematically possible".to_string(),
                    ))
                } else {
                    Ok(left / right)
                }
            }
            Operator::DivInt => {
                if let Token::Number(f) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Number(f / r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for DivInt in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for DivInt".to_string(),
                    ))
                }
            }
            Operator::Mod => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Number(l % r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for Mod in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for Mod in the left element ".to_string(),
                    ))
                }
            }
            Operator::Pow => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        return match r {
                            Number::Int(i) => Ok(Token::Number(l.pow(i as i32))),
                            Number::Float(f) => Ok(Token::Number(l.powf(f))),
                        };
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for Pow in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for Pow in the left element".to_string(),
                    ))
                }
            }
            Operator::Greater => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Bool(l > r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for Greater in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for Greater in the left element".to_string(),
                    ))
                }
            }
            Operator::Less => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Bool(l < r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for Less in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for Less in the left element".to_string(),
                    ))
                }
            }
            Operator::GreaterEqual => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Bool(l >= r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for GreaterEqual in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for GreaterEqual in the left element".to_string(),
                    ))
                }
            }
            Operator::LessEqual => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Bool(l <= r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for LessEqual in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for LessEqual in the left element".to_string(),
                    ))
                }
            }
            Operator::Equal => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Bool(l == r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for Equal in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for Equal in the left element".to_string(),
                    ))
                }
            }
            Operator::NotEqual => {
                if let Token::Number(l) = left {
                    if let Token::Number(r) = right {
                        Ok(Token::Bool(l != r))
                    } else {
                        Err(MathError::InvalidOperation(
                            "Invalid type for NotEqual in the right element".to_string(),
                        ))
                    }
                } else {
                    // Handle other number types appropriately
                    Err(MathError::InvalidOperation(
                        "Invalid type for NotEqual in the left element".to_string(),
                    ))
                }
            }
            Operator::Asign => {
                // Asignar el valor de la derecha a la variable de la izquierda
                // let mut variables = VARIABLES.lock().unwrap();
                // variables.insert(self.left.resolve().to_string(), self.right.resolve());
                println!("Asignación: {:?}", right);
                Ok(right)
            }
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
            _ => false,
        };

        let right_valid = match &*self.right {
            Operand::Number(_) => true,
            Operand::Operation(op) => op.is_valid(),
            _ => false,
        };

        if *self.left == Operand::End && *self.right == Operand::End {
            return true;
        }

        return left_valid && right_valid;
    }
}
