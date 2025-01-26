use crate::{
    compiler::{error::parse_error::ParseError, types::Types},
    types::basic::number::{double::Double, float::Float, int32::Int32, int64::Int64},
};

use super::token::Token;

/// Represents all possible operators in the language, including:
/// - Assignment operators (e.g., `=`),
/// - Arithmetic operators (e.g., `+`, `-`, `*`, `/`),
/// - Logical operators (e.g., `&&`, `||`),
/// - Comparison operators (e.g., `==`, `!=`, `<`, `>`), and others.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    /// Assignation
    Assign,

    /// Addition
    Add,

    /// Addition and Assignation
    AddAssign,

    /// Subtraction
    Sub,

    /// Subtraction and Assignation
    SubAssign,

    /// Multiplication
    Mul,

    /// Multiplication and Assignation
    MulAssign,

    /// Power
    Pow,

    /// Power and Assignation
    PowAssign,

    /// Divition
    Div,

    /// Divition and Assignation
    DivAssign,

    /// Integer Divition
    DivInt,

    /// Integer Divition and Assignation
    DivIntAssign,

    /// Module
    Mod,

    /// Module and Assignation
    ModAssign,

    /// useless operator, It doesn't have use
    Null,
}

impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::Assign => String::from("="),
            Operator::Add => String::from("+"),
            Operator::AddAssign => String::from("+="),
            Operator::Sub => String::from("-"),
            Operator::SubAssign => String::from("-="),
            Operator::Mul => String::from("*"),
            Operator::MulAssign => String::from("*="),
            Operator::Pow => String::from("**"),
            Operator::PowAssign => String::from("**="),
            Operator::Div => String::from("/"),
            Operator::DivAssign => String::from("/="),
            Operator::DivInt => String::from("//"),
            Operator::DivIntAssign => String::from("//="),
            Operator::Mod => String::from("%"),
            Operator::ModAssign => String::from("%="),
            _ => String::from(""),
        }
    }
}

#[allow(dead_code)]
impl Operator {
    pub fn execute(&self, left: Token, right: Token) -> Result<Token, ParseError> {
        let mut left = left;
        let mut right = right;
        if let Token::Operation(mut op) = left {
            left = op.resolve().unwrap();
        }
        if let Token::Operation(mut op) = right {
            right = op.resolve().unwrap();
        }

        match self {
            Self::Add | Self::AddAssign => match (left, right) {
                (Token::Int32(val1), Token::Int32(val2)) => {
                    Self::add_integer_integer(Integer::Int32(val1), Integer::Int32(val2))
                }
                (Token::Int32(val1), Token::Int64(val2)) => {
                    Self::add_integer_integer(Integer::Int32(val1), Integer::Int64(val2))
                }
                (Token::Int32(val1), Token::Float(val2)) => {
                    Self::add_integer_decimal(Number::Int32(val1), Number::Float(val2))
                }
                (Token::Int32(val1), Token::Double(val2)) => {
                    Self::add_integer_decimal(Number::Int32(val1), Number::Double(val2))
                }
                (Token::Int32(val1), Token::String(val2)) => {
                    Self::add_number_string(val1.to_string(), val2)
                }
                (Token::Int32(val1), Token::Str(val2)) => {
                    Self::add_number_string(val1.to_string(), val2.to_string())
                }
                (Token::Int64(val1), Token::Int32(val2)) => {
                    Self::add_integer_integer(Integer::Int64(val1), Integer::Int32(val2))
                }
                (Token::Int64(val1), Token::Int64(val2)) => {
                    Self::add_integer_integer(Integer::Int64(val1), Integer::Int64(val2))
                }
                (Token::Int64(val1), Token::Float(val2)) => {
                    Self::add_integer_decimal(Number::Int64(val1), Number::Float(val2))
                }
                (Token::Int64(val1), Token::Double(val2)) => {
                    Self::add_integer_decimal(Number::Int64(val1), Number::Double(val2))
                }
                (Token::Int64(val1), Token::String(val2)) => {
                    Self::add_number_string(val1.to_string(), val2)
                }
                (Token::Int64(val1), Token::Str(val2)) => {
                    Self::add_number_string(val1.to_string(), val2.to_string())
                }
                (Token::Float(val1), Token::Int32(val2)) => {
                    Self::add_integer_decimal(Number::Float(val1), Number::Int32(val2))
                }
                (Token::Float(val1), Token::Int64(val2)) => {
                    Self::add_integer_decimal(Number::Float(val1), Number::Int64(val2))
                }
                (Token::Float(val1), Token::Float(val2)) => {
                    Self::add_decimal_decimal(Decimal::Float(val1), Decimal::Float(val2))
                }
                (Token::Float(val1), Token::Double(val2)) => {
                    Self::add_decimal_decimal(Decimal::Float(val1), Decimal::Double(val2))
                }
                (Token::Float(val1), Token::String(val2)) => {
                    Self::add_number_string(val1.to_string(), val2)
                }
                (Token::Float(val1), Token::Str(val2)) => {
                    Self::add_number_string(val1.to_string(), val2.to_string())
                }
                (Token::Double(val1), Token::Int32(val2)) => {
                    Self::add_integer_decimal(Number::Double(val1), Number::Int32(val2))
                }
                (Token::Double(val1), Token::Int64(val2)) => {
                    Self::add_integer_decimal(Number::Double(val1), Number::Int64(val2))
                }
                (Token::Double(val1), Token::Float(val2)) => {
                    Self::add_decimal_decimal(Decimal::Double(val1), Decimal::Float(val2))
                }
                (Token::Double(val1), Token::Double(val2)) => {
                    Self::add_decimal_decimal(Decimal::Double(val1), Decimal::Double(val2))
                }
                (Token::Double(val1), Token::String(val2)) => {
                    Self::add_number_string(val1.to_string(), val2)
                }
                (Token::Double(val1), Token::Str(val2)) => {
                    Self::add_number_string(val1.to_string(), val2.to_string())
                }
                (Token::String(val1), Token::Int32(val2)) => {
                    Self::add_number_string(val1, val2.to_string())
                }
                (Token::String(val1), Token::Int64(val2)) => {
                    Self::add_number_string(val1, val2.to_string())
                }
                (Token::String(val1), Token::Float(val2)) => {
                    Self::add_number_string(val1, val2.to_string())
                }
                (Token::String(val1), Token::Double(val2)) => {
                    Self::add_number_string(val1, val2.to_string())
                }
                (Token::String(val1), Token::String(val2)) => {
                    Self::add_number_string(val1, val2.to_string())
                }
                (Token::String(val1), Token::Str(val2)) => {
                    Self::add_number_string(val1.to_string(), val2.to_string())
                }
                (Token::Str(val1), Token::Int32(val2)) => {
                    Self::add_number_string(val1.to_string(), val2.to_string())
                }
                (Token::Str(val1), Token::Int64(val2)) => {
                    Self::add_number_string(val1.to_string(), val2.to_string())
                }
                (Token::Str(val1), Token::Float(val2)) => {
                    Self::add_number_string(val1.to_string(), val2.to_string())
                }
                (Token::Str(val1), Token::Double(val2)) => {
                    Self::add_number_string(val1.to_string(), val2.to_string())
                }
                (Token::Str(val1), Token::String(val2)) => {
                    Self::add_number_string(val1.to_string(), val2.to_string())
                }
                (Token::Str(val1), Token::Str(val2)) => {
                    Self::add_number_string(val1.to_string(), val2.to_string())
                }

                (left, right) => Err(ParseError::InvalidOperation {
                    operation: self.to_string(),
                    type1: Types::inferred(&left)?.to_string(),
                    type2: Types::inferred(&right)?.to_string(),
                }),
            },

            Self::Sub | Self::SubAssign => match (left, right) {
                (Token::Int32(val1), Token::Int32(val2)) => {
                    Self::sub_integer_integer(Integer::Int32(val1), Integer::Int32(val2))
                }
                (Token::Int32(val1), Token::Int64(val2)) => {
                    Self::sub_integer_integer(Integer::Int32(val1), Integer::Int64(val2))
                }
                (Token::Int32(val1), Token::Float(val2)) => {
                    Self::sub_integer_decimal(Number::Int32(val1), Number::Float(val2))
                }
                (Token::Int32(val1), Token::Double(val2)) => {
                    Self::sub_integer_decimal(Number::Int32(val1), Number::Double(val2))
                }
                (Token::Int64(val1), Token::Int32(val2)) => {
                    Self::sub_integer_integer(Integer::Int64(val1), Integer::Int32(val2))
                }
                (Token::Int64(val1), Token::Int64(val2)) => {
                    Self::sub_integer_integer(Integer::Int64(val1), Integer::Int64(val2))
                }
                (Token::Int64(val1), Token::Float(val2)) => {
                    Self::sub_integer_decimal(Number::Int64(val1), Number::Float(val2))
                }
                (Token::Int64(val1), Token::Double(val2)) => {
                    Self::sub_integer_decimal(Number::Int64(val1), Number::Double(val2))
                }
                (Token::Float(val1), Token::Int32(val2)) => {
                    Self::sub_integer_decimal(Number::Float(val1), Number::Int32(val2))
                }
                (Token::Float(val1), Token::Int64(val2)) => {
                    Self::sub_integer_decimal(Number::Float(val1), Number::Int64(val2))
                }
                (Token::Float(val1), Token::Float(val2)) => {
                    Self::sub_decimal_decimal(Decimal::Float(val1), Decimal::Float(val2))
                }
                (Token::Float(val1), Token::Double(val2)) => {
                    Self::sub_decimal_decimal(Decimal::Float(val1), Decimal::Double(val2))
                }
                (Token::Double(val1), Token::Int32(val2)) => {
                    Self::sub_integer_decimal(Number::Double(val1), Number::Int32(val2))
                }
                (Token::Double(val1), Token::Int64(val2)) => {
                    Self::sub_integer_decimal(Number::Double(val1), Number::Int64(val2))
                }
                (Token::Double(val1), Token::Float(val2)) => {
                    Self::sub_decimal_decimal(Decimal::Double(val1), Decimal::Float(val2))
                }
                (Token::Double(val1), Token::Double(val2)) => {
                    Self::sub_decimal_decimal(Decimal::Double(val1), Decimal::Double(val2))
                }
                (left, right) => Err(ParseError::InvalidOperation {
                    operation: self.to_string(),
                    type1: Types::inferred(&left)?.to_string(),
                    type2: Types::inferred(&right)?.to_string(),
                }),
            },

            Self::Mul | Self::MulAssign => match (left, right) {
                (Token::Int32(num1), Token::Int32(num2)) => {
                    Ok(Token::to_number(num1 * num2, Types::Double))
                }
                (Token::Double(num1), Token::Double(num2)) => {
                    Ok(Token::to_number(num1 * num2, Types::Double))
                }
                (Token::Double(num1), Token::Int32(num2)) => {
                    Ok(Token::to_number(num1 * num2, Types::Double))
                }
                (Token::Int32(num1), Token::Double(num2)) => {
                    Ok(Token::to_number(num1 * num2, Types::Double))
                }
                (left, right) => Err(ParseError::InvalidOperation {
                    operation: self.to_string(),
                    type1: Types::inferred(&left)?.to_string(),
                    type2: Types::inferred(&right)?.to_string(),
                }),
            },

            Self::Pow | Self::PowAssign => match (left, right) {
                (Token::Int32(num1), Token::Int32(num2)) => {
                    let mut result = *num1;

                    if *num2 <= 1 {
                        return Ok(Token::Int32(num1));
                    }

                    for _ in 2..=*num2 {
                        result = result.checked_mul(*num1).unwrap_or_else(|| i32::MAX);
                    }

                    return Ok(Token::Int32(result.into()));
                }
                (left, right) => Err(ParseError::InvalidOperation {
                    operation: self.to_string(),
                    type1: Types::inferred(&left)?.to_string(),
                    type2: Types::inferred(&right)?.to_string(),
                }),
            },

            Self::Div | Self::DivAssign => {
                return match (left, right) {
                    (Token::Int32(num1), Token::Int32(num2)) => {
                        Ok(Token::to_number(num1 / num2, Types::Double))
                    }
                    (Token::Double(num1), Token::Double(num2)) => {
                        Ok(Token::to_number(num1 / num2, Types::Double))
                    }
                    (Token::Double(num1), Token::Int32(num2)) => {
                        Ok(Token::to_number(num1 / num2, Types::Double))
                    }
                    (Token::Int32(num1), Token::Double(num2)) => {
                        Ok(Token::to_number(num1 / num2, Types::Double))
                    }
                    (left, right) => Err(ParseError::InvalidOperation {
                        operation: self.to_string(),
                        type1: Types::inferred(&left)?.to_string(),
                        type2: Types::inferred(&right)?.to_string(),
                    }),
                };
            }

            Self::DivInt | Self::DivIntAssign => {
                match (left, right) {
                    // MARK: ------ Int32 ------
                    (Token::Int32(num1), Token::Int32(num2)) => {
                        Ok(Token::Int32(Int32::from((num1 / num2).ceil() as i32)))
                    }
                    (Token::Int32(num1), Token::Int64(num2)) => {
                        Ok(Token::Int32(Int32::from((num1 / num2).ceil() as i32)))
                    }
                    (Token::Int32(num1), Token::Double(num2)) => {
                        Ok(Token::Int32(Int32::from((num1 / num2).ceil() as i32)))
                    }

                    // MARK: ------ Int64 ------
                    (Token::Int64(num1), Token::Int32(num2)) => {
                        Ok(Token::Int32(Int32::from((num1 / num2).ceil() as i32)))
                    }
                    (Token::Int64(num1), Token::Int64(num2)) => {
                        Ok(Token::Int32(Int32::from((num1 / num2).ceil() as i32)))
                    }
                    (Token::Int64(num1), Token::Double(num2)) => {
                        Ok(Token::Int32(Int32::from((num1 / num2).ceil() as i32)))
                    }

                    // MARK: ------ Double ------
                    (Token::Double(num1), Token::Int32(num2)) => {
                        Ok(Token::Int32(Int32::from((num1 / num2).ceil() as i32)))
                    }
                    (Token::Double(num1), Token::Int64(num2)) => {
                        Ok(Token::Int32(Int32::from((num1 / num2).ceil() as i32)))
                    }
                    (Token::Double(num1), Token::Double(num2)) => {
                        Ok(Token::Int32(Int32::from((num1 / num2).ceil() as i32)))
                    }

                    (left, right) => Err(ParseError::InvalidOperation {
                        operation: self.to_string(),
                        type1: Types::inferred(&left)?.to_string(),
                        type2: Types::inferred(&right)?.to_string(),
                    }),
                }
            }

            Self::Mod | Self::ModAssign => match (left, right) {
                (Token::Int32(num1), Token::Int32(num2)) => Ok(Token::Int32(num1 % num2)),
                (Token::Double(num1), Token::Double(num2)) => Ok(Token::Int32(num1 % num2)),
                (Token::Double(num1), Token::Int32(num2)) => Ok(Token::Int32(num1 % num2)),
                (Token::Int32(num1), Token::Double(num2)) => Ok(Token::Int32(num1 % num2)),
                (left, right) => Err(ParseError::InvalidOperation {
                    operation: self.to_string(),
                    type1: Types::inferred(&left)?.to_string(),
                    type2: Types::inferred(&right)?.to_string(),
                }),
            },

            _ => todo!("Hola mundo"),
        }
    }

    fn add_integer_integer(left: Integer, right: Integer) -> Result<Token, ParseError> {
        match (left, right) {
            (Integer::Int32(num_1), Integer::Int32(num_2)) => Ok(Token::Int32(num_1 + num_2)),
            (Integer::Int32(num_1), Integer::Int64(num_2)) => {
                Ok(Token::Int32(num_1 + (*num_2 as i32)))
            }
            (Integer::Int64(num_1), Integer::Int32(num_2)) => Ok(Token::Int64(num_1 + num_2)),
            (Integer::Int64(num_1), Integer::Int64(num_2)) => Ok(Token::Int64(num_1 + num_2)),
        }
    }

    fn add_integer_decimal(left: Number, right: Number) -> Result<Token, ParseError> {
        match (left, right) {
            (Number::Int32(num1), Number::Int32(num2)) => {
                Self::add_integer_integer(Integer::Int32(num1), Integer::Int32(num2))
            }
            (Number::Int32(num1), Number::Int64(num2)) => {
                Self::add_integer_integer(Integer::Int32(num1), Integer::Int64(num2))
            }
            (Number::Int32(num1), Number::Float(num2)) => {
                Ok(Token::Float(Float::new(*num1 as f32) + num2))
            }
            (Number::Int32(num1), Number::Double(num2)) => {
                Ok(Token::Double(Double::new((*num1).into()) + num2))
            }

            (Number::Int64(num1), Number::Int32(num2)) => {
                Self::add_integer_integer(Integer::Int64(num1), Integer::Int32(num2))
            }
            (Number::Int64(num1), Number::Int64(num2)) => {
                Self::add_integer_integer(Integer::Int64(num1), Integer::Int64(num2))
            }
            (Number::Int64(num1), Number::Float(num2)) => {
                Ok(Token::Float(Float::new(*num1 as f32) + num2))
            }
            (Number::Int64(num1), Number::Double(num2)) => {
                Ok(Token::Double(Double::new(num1.into()) + num2))
            }

            (Number::Float(num1), Number::Int32(num2)) => {
                Ok(Token::Float(num1 + Float::new(*num2 as f32)))
            }
            (Number::Float(num1), Number::Int64(num2)) => {
                Ok(Token::Float(num1 + Float::new(*num2 as f32)))
            }
            (Number::Float(num1), Number::Float(num2)) => {
                Self::add_decimal_decimal(Decimal::Float(num1), Decimal::Float(num2))
            }
            (Number::Float(num1), Number::Double(num2)) => {
                Self::add_decimal_decimal(Decimal::Float(num1), Decimal::Double(num2))
            }

            (Number::Double(num1), Number::Int32(num2)) => {
                Ok(Token::Double(num1 + Double::new(*num2 as f64)))
            }
            (Number::Double(num1), Number::Int64(num2)) => {
                Ok(Token::Double(num1 + Double::new(*num2 as f64)))
            }
            (Number::Double(num1), Number::Float(num2)) => {
                Self::add_decimal_decimal(Decimal::Double(num1), Decimal::Float(num2))
            }
            (Number::Double(num1), Number::Double(num2)) => {
                Self::add_decimal_decimal(Decimal::Double(num1), Decimal::Double(num2))
            }
        }
    }

    fn add_decimal_decimal(left: Decimal, right: Decimal) -> Result<Token, ParseError> {
        match (left, right) {
            (Decimal::Float(num1), Decimal::Float(num2)) => Ok(Token::Float(num1 + num2)),
            (Decimal::Float(num1), Decimal::Double(num2)) => {
                Ok(Token::Float(num1 + Float::from(*num2)))
            }
            (Decimal::Double(num1), Decimal::Float(num2)) => {
                Ok(Token::Double(num1 + Double::from(*num2)))
            }
            (Decimal::Double(num1), Decimal::Double(num2)) => Ok(Token::Double(num1 + num2)),
        }
    }

    fn add_number_string(left: String, right: String) -> Result<Token, ParseError> {
        Ok(Token::String(left + right.as_str()))
    }

    fn sub_integer_integer(left: Integer, right: Integer) -> Result<Token, ParseError> {
        match (left, right) {
            (Integer::Int32(num_1), Integer::Int32(num_2)) => Ok(Token::Int32(num_1 - num_2)),
            (Integer::Int32(num_1), Integer::Int64(num_2)) => {
                Ok(Token::Int32(num_1 - (*num_2 as i32)))
            }
            (Integer::Int64(num_1), Integer::Int32(num_2)) => Ok(Token::Int64(num_1 - num_2)),
            (Integer::Int64(num_1), Integer::Int64(num_2)) => Ok(Token::Int64(num_1 - num_2)),
        }
    }

    fn sub_integer_decimal(left: Number, right: Number) -> Result<Token, ParseError> {
        match (left, right) {
            (Number::Int32(num1), Number::Int32(num2)) => {
                Self::sub_integer_integer(Integer::Int32(num1), Integer::Int32(num2))
            }
            (Number::Int32(num1), Number::Int64(num2)) => {
                Self::sub_integer_integer(Integer::Int32(num1), Integer::Int64(num2))
            }
            (Number::Int32(num1), Number::Float(num2)) => {
                Ok(Token::Float(Float::new(*num1 as f32) - num2))
            }
            (Number::Int32(num1), Number::Double(num2)) => {
                Ok(Token::Double(Double::new((*num1).into()) - num2))
            }

            (Number::Int64(num1), Number::Int32(num2)) => {
                Self::sub_integer_integer(Integer::Int64(num1), Integer::Int32(num2))
            }
            (Number::Int64(num1), Number::Int64(num2)) => {
                Self::sub_integer_integer(Integer::Int64(num1), Integer::Int64(num2))
            }
            (Number::Int64(num1), Number::Float(num2)) => {
                Ok(Token::Float(Float::new(*num1 as f32) - num2))
            }
            (Number::Int64(num1), Number::Double(num2)) => {
                Ok(Token::Double(Double::new(num1.into()) - num2))
            }

            (Number::Float(num1), Number::Int32(num2)) => {
                Ok(Token::Float(num1 - Float::new(*num2 as f32)))
            }
            (Number::Float(num1), Number::Int64(num2)) => {
                Ok(Token::Float(num1 - Float::new(*num2 as f32)))
            }
            (Number::Float(num1), Number::Float(num2)) => {
                Self::sub_decimal_decimal(Decimal::Float(num1), Decimal::Float(num2))
            }
            (Number::Float(num1), Number::Double(num2)) => {
                Self::sub_decimal_decimal(Decimal::Float(num1), Decimal::Double(num2))
            }

            (Number::Double(num1), Number::Int32(num2)) => {
                Ok(Token::Double(num1 - Double::new(*num2 as f64)))
            }
            (Number::Double(num1), Number::Int64(num2)) => {
                Ok(Token::Double(num1 - Double::new(*num2 as f64)))
            }
            (Number::Double(num1), Number::Float(num2)) => {
                Self::sub_decimal_decimal(Decimal::Double(num1), Decimal::Float(num2))
            }
            (Number::Double(num1), Number::Double(num2)) => {
                Self::sub_decimal_decimal(Decimal::Double(num1), Decimal::Double(num2))
            }
        }
    }

    fn sub_decimal_decimal(left: Decimal, right: Decimal) -> Result<Token, ParseError> {
        match (left, right) {
            (Decimal::Float(num1), Decimal::Float(num2)) => Ok(Token::Float(num1 - num2)),
            (Decimal::Float(num1), Decimal::Double(num2)) => {
                Ok(Token::Float(num1 - Float::from(*num2)))
            }
            (Decimal::Double(num1), Decimal::Float(num2)) => {
                Ok(Token::Double(num1 - Double::from(*num2)))
            }
            (Decimal::Double(num1), Decimal::Double(num2)) => Ok(Token::Double(num1 - num2)),
        }
    }

    pub fn is_assignation(&self) -> bool {
        match self {
            Self::AddAssign
            | Self::SubAssign
            | Self::MulAssign
            | Self::DivAssign
            | Self::ModAssign
            | Self::PowAssign
            | Self::DivIntAssign => true,
            _ => false,
        }
    }
}

enum Integer {
    Int32(Int32),
    Int64(Int64),
}

enum Decimal {
    Float(Float),
    Double(Double),
}

enum Number {
    Int32(Int32),
    Int64(Int64),
    Float(Float),
    Double(Double),
}
