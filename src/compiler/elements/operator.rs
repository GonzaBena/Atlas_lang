use crate::{
    compiler::{error::parse_error::ParseError, types::Types},
    types::basic::number::{
        double::Double, float::Float, hpint::HPInt, int32::Int32, int64::Int64,
    },
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

    /// One is greater than other
    Greater,

    GreaterOrEqual,

    Lower,

    LowerOrEqual,

    Equal,

    StrictEqual,

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

            // MARK: Comparation
            Operator::Greater => String::from(">"),
            Operator::GreaterOrEqual => String::from(">="),
            Operator::Lower => String::from("<"),
            Operator::LowerOrEqual => String::from("<="),
            Operator::Equal => String::from("=="),
            Operator::StrictEqual => String::from("==="),

            Operator::Null => String::from("null"),
            // _ => String::from(""),
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

            Self::Mul | Self::MulAssign => match (left, right.clone()) {
                (Token::Int32(val1), Token::Int32(val2)) => {
                    Self::mul_integer_integer(Integer::Int32(val1), Integer::Int32(val2))
                }
                (Token::Int32(val1), Token::Int64(val2)) => {
                    Self::mul_integer_integer(Integer::Int32(val1), Integer::Int64(val2))
                }
                (Token::Int32(val1), Token::Float(val2)) => {
                    Self::mul_integer_decimal(Number::Int32(val1), Number::Float(val2))
                }
                (Token::Int32(val1), Token::Double(val2)) => {
                    Self::mul_integer_decimal(Number::Int32(val1), Number::Double(val2))
                }
                (Token::Int32(val1), _) => Self::mul_string_number(val1.to_string(), &right),
                (Token::Int64(val1), Token::Int32(val2)) => {
                    Self::mul_integer_integer(Integer::Int64(val1), Integer::Int32(val2))
                }
                (Token::Int64(val1), Token::Int64(val2)) => {
                    Self::mul_integer_integer(Integer::Int64(val1), Integer::Int64(val2))
                }
                (Token::Int64(val1), Token::Float(val2)) => {
                    Self::mul_integer_decimal(Number::Int64(val1), Number::Float(val2))
                }
                (Token::Int64(val1), Token::Double(val2)) => {
                    Self::mul_integer_decimal(Number::Int64(val1), Number::Double(val2))
                }
                (Token::Int64(val1), _) => Self::mul_string_number(val1.to_string(), &right),
                (Token::Float(val1), Token::Int32(val2)) => {
                    Self::mul_integer_decimal(Number::Float(val1), Number::Int32(val2))
                }
                (Token::Float(val1), Token::Int64(val2)) => {
                    Self::mul_integer_decimal(Number::Float(val1), Number::Int64(val2))
                }
                (Token::Float(val1), Token::Float(val2)) => {
                    Self::mul_decimal_decimal(Decimal::Float(val1), Decimal::Float(val2))
                }
                (Token::Float(val1), Token::Double(val2)) => {
                    Self::mul_decimal_decimal(Decimal::Float(val1), Decimal::Double(val2))
                }
                (Token::Float(val1), _) => Self::mul_string_number(val1.to_string(), &right),
                (Token::Double(val1), Token::Int32(val2)) => {
                    Self::mul_integer_decimal(Number::Double(val1), Number::Int32(val2))
                }
                (Token::Double(val1), Token::Int64(val2)) => {
                    Self::mul_integer_decimal(Number::Double(val1), Number::Int64(val2))
                }
                (Token::Double(val1), Token::Float(val2)) => {
                    Self::mul_decimal_decimal(Decimal::Double(val1), Decimal::Float(val2))
                }
                (Token::Double(val1), Token::Double(val2)) => {
                    Self::mul_decimal_decimal(Decimal::Double(val1), Decimal::Double(val2))
                }
                (Token::Double(val1), _) => Self::mul_string_number(val1.to_string(), &right),
                (Token::String(val1), _) => Self::mul_string_number(val1, &right),
                (Token::Str(val1), _) => Self::mul_string_number(val1.to_string(), &right),

                (left, right) => Err(ParseError::InvalidOperation {
                    operation: self.to_string(),
                    type1: Types::inferred(&left)?.to_string(),
                    type2: Types::inferred(&right)?.to_string(),
                }),
            },

            Self::Pow | Self::PowAssign => match right {
                Token::Int32(power) => match left {
                    Token::Int32(num1) => {
                        if *power <= 1 {
                            Ok(left)
                        } else {
                            let mut result: i128 = *num1 as i128;

                            for _ in 2..=*power {
                                result *= *num1 as i128;
                            }

                            if let Ok(value) = result.try_into() as Result<i32, _> {
                                Ok(Token::Int32(Int32::from(value)))
                            } else if let Ok(value) = result.try_into() as Result<i64, _> {
                                Ok(Token::Int64(Int64::from(value)))
                            } else {
                                Ok(Token::HPInt(HPInt::from(result)))
                            }
                        }
                    }

                    Token::Int64(num1) => {
                        if *power <= 1 {
                            Ok(left)
                        } else {
                            let mut result: i128 = *num1 as i128;

                            for _ in 2..=*power {
                                result *= *num1 as i128;
                            }

                            if let Ok(value) = result.try_into() as Result<i64, _> {
                                Ok(Token::Int64(Int64::from(value)))
                            } else {
                                Ok(Token::HPInt(HPInt::from(result)))
                            }
                        }
                    }

                    Token::Float(num1) => {
                        if *power <= 1 {
                            Ok(Token::Double(Double::from(*num1)))
                        } else {
                            Ok(Token::Double(Double::from(num1.powi(*power))))
                        }
                    }

                    Token::Double(num1) => {
                        if *power <= 1 {
                            return Ok(Token::Double(Double::from(*num1)));
                        }

                        return Ok(Token::Double(Double::from(num1.powi(*power))));
                    }

                    _ => Err(ParseError::InvalidOperation {
                        operation: self.to_string(),
                        type1: Types::inferred(&left)?.to_string(),
                        type2: Types::inferred(&right)?.to_string(),
                    }),
                },
                _ => Err(ParseError::InvalidOperation {
                    operation: self.to_string(),
                    type1: Types::inferred(&left)?.to_string(),
                    type2: Types::inferred(&right)?.to_string(),
                }),
            },

            Self::Div | Self::DivAssign => match (left, right.clone()) {
                (Token::Int32(val1), Token::Int32(val2)) => Self::div_decimal_decimal(
                    Decimal::Double(Double::from(*val1)),
                    Decimal::Double(Double::from(*val2)),
                ),
                (Token::Int32(val1), Token::Int64(val2)) => Self::div_decimal_decimal(
                    Decimal::Double(Double::from(*val1)),
                    Decimal::Double(Double::from(*val2 as f64)),
                ),
                (Token::Int32(val1), Token::Float(val2)) => Self::div_decimal_decimal(
                    Decimal::Double(Double::from(*val1)),
                    Decimal::Double(Double::from(*val2)),
                ),
                (Token::Int32(val1), Token::Double(val2)) => Self::div_decimal_decimal(
                    Decimal::Double(Double::from(*val1)),
                    Decimal::Double(Double::from(*val2)),
                ),
                (Token::Int32(val1), _) => Self::div_string_number(val1.to_string(), &right),
                (Token::Int64(val1), Token::Int32(val2)) => Self::div_decimal_decimal(
                    Decimal::Double(Double::from(*val1 as f64)),
                    Decimal::Double(Double::from(*val2)),
                ),
                (Token::Int64(val1), Token::Int64(val2)) => Self::div_decimal_decimal(
                    Decimal::Double(Double::from(*val1 as f64)),
                    Decimal::Double(Double::from(*val2 as f64)),
                ),
                (Token::Int64(val1), Token::Float(val2)) => Self::div_decimal_decimal(
                    Decimal::Double(Double::from(*val1 as f64)),
                    Decimal::Double(Double::from(*val2)),
                ),
                (Token::Int64(val1), Token::Double(val2)) => Self::div_decimal_decimal(
                    Decimal::Double(Double::from(*val1 as f64)),
                    Decimal::Double(Double::from(*val2)),
                ),
                (Token::Int64(val1), _) => Self::div_string_number(val1.to_string(), &right),
                (Token::Float(val1), Token::Int32(val2)) => Self::div_decimal_decimal(
                    Decimal::Double(Double::from(*val1)),
                    Decimal::Double(Double::from(*val2)),
                ),
                (Token::Float(val1), Token::Int64(val2)) => Self::div_decimal_decimal(
                    Decimal::Double(Double::from(*val1)),
                    Decimal::Double(Double::from(*val2 as f64)),
                ),
                (Token::Float(val1), Token::Float(val2)) => {
                    Self::div_decimal_decimal(Decimal::Float(val1), Decimal::Float(val2))
                }
                (Token::Float(val1), Token::Double(val2)) => {
                    Self::div_decimal_decimal(Decimal::Float(val1), Decimal::Double(val2))
                }
                (Token::Float(val1), _) => Self::div_string_number(val1.to_string(), &right),
                (Token::Double(val1), Token::Int32(val2)) => Self::div_decimal_decimal(
                    Decimal::Double(Double::from(*val1)),
                    Decimal::Double(Double::from(*val2)),
                ),
                (Token::Double(val1), Token::Int64(val2)) => Self::div_decimal_decimal(
                    Decimal::Double(Double::from(*val1)),
                    Decimal::Double(Double::from(*val2 as f64)),
                ),
                (Token::Double(val1), Token::Float(val2)) => {
                    Self::div_decimal_decimal(Decimal::Double(val1), Decimal::Float(val2))
                }
                (Token::Double(val1), Token::Double(val2)) => {
                    Self::div_decimal_decimal(Decimal::Double(val1), Decimal::Double(val2))
                }
                (Token::Double(val1), _) => Self::div_string_number(val1.to_string(), &right),

                (left, right) => Err(ParseError::InvalidOperation {
                    operation: self.to_string(),
                    type1: Types::inferred(&left)?.to_string(),
                    type2: Types::inferred(&right)?.to_string(),
                }),
            },

            Self::DivInt | Self::DivIntAssign => match (left, right.clone()) {
                (Token::Int32(val1), Token::Int32(val2)) => {
                    Self::div_int_integer_integer(Integer::Int32(val1), Integer::Int32(val2))
                }
                (Token::Int32(val1), Token::Int64(val2)) => {
                    Self::div_int_integer_integer(Integer::Int32(val1), Integer::Int64(val2))
                }
                (Token::Int32(val1), Token::Float(val2)) => {
                    Self::div_int_integer_decimal(Number::Int32(val1), Number::Float(val2))
                }
                (Token::Int32(val1), Token::Double(val2)) => {
                    Self::div_int_integer_decimal(Number::Int32(val1), Number::Double(val2))
                }
                (Token::Int32(val1), _) => Self::div_string_number(val1.to_string(), &right),
                (Token::Int64(val1), Token::Int32(val2)) => {
                    Self::div_int_integer_integer(Integer::Int64(val1), Integer::Int32(val2))
                }
                (Token::Int64(val1), Token::Int64(val2)) => {
                    Self::div_int_integer_integer(Integer::Int64(val1), Integer::Int64(val2))
                }
                (Token::Int64(val1), Token::Float(val2)) => {
                    Self::div_int_integer_decimal(Number::Int64(val1), Number::Float(val2))
                }
                (Token::Int64(val1), Token::Double(val2)) => {
                    Self::div_int_integer_decimal(Number::Int64(val1), Number::Double(val2))
                }
                (Token::Int64(val1), _) => Self::div_string_number(val1.to_string(), &right),
                (Token::Float(val1), Token::Int32(val2)) => {
                    Self::div_int_integer_decimal(Number::Float(val1), Number::Int32(val2))
                }
                (Token::Float(val1), Token::Int64(val2)) => {
                    Self::div_int_integer_decimal(Number::Float(val1), Number::Int64(val2))
                }
                (Token::Float(val1), Token::Float(val2)) => {
                    Self::div_int_decimal_decimal(Decimal::Float(val1), Decimal::Float(val2))
                }
                (Token::Float(val1), Token::Double(val2)) => {
                    Self::div_int_decimal_decimal(Decimal::Float(val1), Decimal::Double(val2))
                }
                (Token::Float(val1), _) => Self::div_string_number(val1.to_string(), &right),

                (Token::Double(val1), Token::Int32(val2)) => {
                    Self::div_int_integer_decimal(Number::Double(val1), Number::Int32(val2))
                }
                (Token::Double(val1), Token::Int64(val2)) => {
                    Self::div_int_integer_decimal(Number::Double(val1), Number::Int64(val2))
                }
                (Token::Double(val1), Token::Float(val2)) => {
                    Self::div_int_decimal_decimal(Decimal::Double(val1), Decimal::Float(val2))
                }
                (Token::Double(val1), Token::Double(val2)) => {
                    Self::div_int_decimal_decimal(Decimal::Double(val1), Decimal::Double(val2))
                }
                (Token::Double(val1), _) => Self::div_string_number(val1.to_string(), &right),

                (left, right) => Err(ParseError::InvalidOperation {
                    operation: self.to_string(),
                    type1: Types::inferred(&left)?.to_string(),
                    type2: Types::inferred(&right)?.to_string(),
                }),
            },

            Self::Mod | Self::ModAssign => match (left, right.clone()) {
                (Token::Int32(val1), Token::Int32(val2)) => {
                    Ok(Token::Int32(Int32::from(*val1 % *val2)))
                }
                (Token::Int32(val1), Token::Int64(val2)) => {
                    Ok(Token::Int32(Int32::from(*val1 % *val2 as i32)))
                }
                (Token::Int32(val1), Token::Float(val2)) => {
                    Ok(Token::Int32(Int32::from(*val1 % *val2 as i32)))
                }
                (Token::Int32(val1), Token::Double(val2)) => {
                    Ok(Token::Int32(Int32::from(*val1 % *val2 as i32)))
                }
                (Token::Int64(val1), Token::Int32(val2)) => {
                    Ok(Token::Int64(Int64::from(*val1 % *val2 as i64)))
                }
                (Token::Int64(val1), Token::Int64(val2)) => {
                    Ok(Token::Int64(Int64::from(*val1 % *val2)))
                }
                (Token::Int64(val1), Token::Float(val2)) => {
                    Ok(Token::Int64(Int64::from(*val1 % *val2 as i64)))
                }
                (Token::Int64(val1), Token::Double(val2)) => {
                    Ok(Token::Int64(Int64::from(*val1 % *val2 as i64)))
                }

                (Token::Float(val1), Token::Int32(val2)) => {
                    Ok(Token::Float(Float::from(*val1 % *val2 as f32)))
                }
                (Token::Float(val1), Token::Int64(val2)) => {
                    Ok(Token::Float(Float::from(*val1 % *val2 as f32)))
                }
                (Token::Float(val1), Token::Float(val2)) => {
                    Ok(Token::Float(Float::from(*val1 % *val2)))
                }
                (Token::Float(val1), Token::Double(val2)) => {
                    Ok(Token::Float(Float::from(*val1 % *val2 as f32)))
                }

                (Token::Double(val1), Token::Int32(val2)) => {
                    Ok(Token::Double(Double::from(*val1 % *val2 as f64)))
                }
                (Token::Double(val1), Token::Int64(val2)) => {
                    Ok(Token::Double(Double::from(*val1 % *val2 as f64)))
                }
                (Token::Double(val1), Token::Float(val2)) => {
                    Ok(Token::Double(Double::from(*val1 % *val2 as f64)))
                }
                (Token::Double(val1), Token::Double(val2)) => {
                    Ok(Token::Double(Double::from(*val1 % *val2)))
                }

                (left, right) => Err(ParseError::InvalidOperation {
                    operation: self.to_string(),
                    type1: Types::inferred(&left)?.to_string(),
                    type2: Types::inferred(&right)?.to_string(),
                }),
            },

            Self::Greater => match (left, right.clone()) {
                (Token::Int32(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num > *int32)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num > *int64 as i32)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num > *hpint as i32)),
                    Token::Float(float) => Ok(Token::Boolean(*num > *float as i32)),
                    Token::Double(double) => Ok(Token::Boolean(*num > *double as i32)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Int32"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::Int64(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num > *int32 as i64)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num > *int64)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num > *hpint as i64)),
                    Token::Float(float) => Ok(Token::Boolean(*num > *float as i64)),
                    Token::Double(double) => Ok(Token::Boolean(*num > *double as i64)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Int64"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::HPInt(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num > *int32 as i128)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num > *int64 as i128)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num > *hpint)),
                    Token::Float(float) => Ok(Token::Boolean(*num > *float as i128)),
                    Token::Double(double) => Ok(Token::Boolean(*num > *double as i128)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("HPInt"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::Float(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num > *int32 as f32)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num > *int64 as f32)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num > *hpint as f32)),
                    Token::Float(float) => Ok(Token::Boolean(*num > *float)),
                    Token::Double(double) => Ok(Token::Boolean(*num > *double as f32)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Float"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::Double(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num > *int32 as f64)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num > *int64 as f64)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num > *hpint as f64)),
                    Token::Float(float) => Ok(Token::Boolean(*num > *float as f64)),
                    Token::Double(double) => Ok(Token::Boolean(*num > *double)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Double"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::String(str1), Token::String(str2)) => Ok(Token::Boolean(str1 > str2)),
                (Token::Str(str1), Token::Str(str2)) => Ok(Token::Boolean(str1 > str2)),
                (Token::String(str1), Token::Str(str2)) => {
                    Ok(Token::Boolean(str1.to_string() > str2.to_string()))
                }
                (Token::Str(str1), Token::String(str2)) => {
                    Ok(Token::Boolean(str1.to_string() > str2.to_string()))
                }

                _ => todo!("Cmp"),
            },

            Self::GreaterOrEqual => match (left, right.clone()) {
                (Token::Int32(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num >= *int32)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num >= *int64 as i32)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num >= *hpint as i32)),
                    Token::Float(float) => Ok(Token::Boolean(*num >= *float as i32)),
                    Token::Double(double) => Ok(Token::Boolean(*num >= *double as i32)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Int32"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::Int64(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num >= *int32 as i64)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num >= *int64)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num >= *hpint as i64)),
                    Token::Float(float) => Ok(Token::Boolean(*num >= *float as i64)),
                    Token::Double(double) => Ok(Token::Boolean(*num >= *double as i64)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Int64"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::HPInt(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num >= *int32 as i128)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num >= *int64 as i128)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num >= *hpint)),
                    Token::Float(float) => Ok(Token::Boolean(*num >= *float as i128)),
                    Token::Double(double) => Ok(Token::Boolean(*num >= *double as i128)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("HPInt"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::Float(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num >= *int32 as f32)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num >= *int64 as f32)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num >= *hpint as f32)),
                    Token::Float(float) => Ok(Token::Boolean(*num >= *float)),
                    Token::Double(double) => Ok(Token::Boolean(*num >= *double as f32)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Float"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::Double(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num >= *int32 as f64)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num >= *int64 as f64)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num >= *hpint as f64)),
                    Token::Float(float) => Ok(Token::Boolean(*num >= *float as f64)),
                    Token::Double(double) => Ok(Token::Boolean(*num >= *double)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Double"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::String(str1), Token::String(str2)) => Ok(Token::Boolean(str1 >= str2)),
                (Token::Str(str1), Token::Str(str2)) => Ok(Token::Boolean(str1 >= str2)),
                (Token::String(str1), Token::Str(str2)) => {
                    Ok(Token::Boolean(str1.to_string() >= str2.to_string()))
                }
                (Token::Str(str1), Token::String(str2)) => {
                    Ok(Token::Boolean(str1.to_string() >= str2.to_string()))
                }

                _ => todo!("Cmp"),
            },

            Self::Lower => match (left, right.clone()) {
                (Token::Int32(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num < *int32)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num < *int64 as i32)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num < *hpint as i32)),
                    Token::Float(float) => Ok(Token::Boolean(*num < *float as i32)),
                    Token::Double(double) => Ok(Token::Boolean(*num < *double as i32)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Int32"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::Int64(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num < *int32 as i64)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num < *int64)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num < *hpint as i64)),
                    Token::Float(float) => Ok(Token::Boolean(*num < *float as i64)),
                    Token::Double(double) => Ok(Token::Boolean(*num < *double as i64)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Int64"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::HPInt(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num < *int32 as i128)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num < *int64 as i128)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num < *hpint)),
                    Token::Float(float) => Ok(Token::Boolean(*num < *float as i128)),
                    Token::Double(double) => Ok(Token::Boolean(*num < *double as i128)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("HPInt"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::Float(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num < *int32 as f32)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num < *int64 as f32)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num < *hpint as f32)),
                    Token::Float(float) => Ok(Token::Boolean(*num < *float)),
                    Token::Double(double) => Ok(Token::Boolean(*num < *double as f32)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Float"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::Double(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num < *int32 as f64)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num < *int64 as f64)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num < *hpint as f64)),
                    Token::Float(float) => Ok(Token::Boolean(*num < *float as f64)),
                    Token::Double(double) => Ok(Token::Boolean(*num < *double)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Double"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::String(str1), Token::String(str2)) => Ok(Token::Boolean(str1 < str2)),
                (Token::Str(str1), Token::Str(str2)) => Ok(Token::Boolean(str1 < str2)),
                (Token::String(str1), Token::Str(str2)) => {
                    Ok(Token::Boolean(str1.to_string() < str2.to_string()))
                }
                (Token::Str(str1), Token::String(str2)) => {
                    Ok(Token::Boolean(str1.to_string() < str2.to_string()))
                }

                _ => todo!("Cmp"),
            },

            Self::LowerOrEqual => match (left, right.clone()) {
                (Token::Int32(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num > *int32)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num > *int64 as i32)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num > *hpint as i32)),
                    Token::Float(float) => Ok(Token::Boolean(*num > *float as i32)),
                    Token::Double(double) => Ok(Token::Boolean(*num > *double as i32)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Int32"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::Int64(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num > *int32 as i64)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num > *int64)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num > *hpint as i64)),
                    Token::Float(float) => Ok(Token::Boolean(*num > *float as i64)),
                    Token::Double(double) => Ok(Token::Boolean(*num > *double as i64)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Int64"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::HPInt(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num > *int32 as i128)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num > *int64 as i128)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num > *hpint)),
                    Token::Float(float) => Ok(Token::Boolean(*num > *float as i128)),
                    Token::Double(double) => Ok(Token::Boolean(*num > *double as i128)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("HPInt"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::Float(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num > *int32 as f32)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num > *int64 as f32)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num > *hpint as f32)),
                    Token::Float(float) => Ok(Token::Boolean(*num > *float)),
                    Token::Double(double) => Ok(Token::Boolean(*num > *double as f32)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Float"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::Double(num), num2) => match num2 {
                    Token::Int32(int32) => Ok(Token::Boolean(*num > *int32 as f64)),
                    Token::Int64(int64) => Ok(Token::Boolean(*num > *int64 as f64)),
                    Token::HPInt(hpint) => Ok(Token::Boolean(*num > *hpint as f64)),
                    Token::Float(float) => Ok(Token::Boolean(*num > *float as f64)),
                    Token::Double(double) => Ok(Token::Boolean(*num > *double)),
                    _ => Err(ParseError::NotOrd {
                        type1: String::from("Double"),
                        type2: Types::from(num2).to_string(),
                    }),
                },

                (Token::String(str1), Token::String(str2)) => Ok(Token::Boolean(str1 <= str2)),
                (Token::Str(str1), Token::Str(str2)) => Ok(Token::Boolean(str1 <= str2)),
                (Token::String(str1), Token::Str(str2)) => {
                    Ok(Token::Boolean(str1.to_string() <= str2.to_string()))
                }
                (Token::Str(str1), Token::String(str2)) => {
                    Ok(Token::Boolean(str1.to_string() <= str2.to_string()))
                }

                _ => todo!("Cmp"),
            },

            Self::Equal => Ok(Token::Boolean(left == right)),
            Self::StrictEqual => Ok(Token::Boolean(
                left == right && Types::from(left) == Types::from(right),
            )),

            _ => todo!("Hola mundo"),
        }
    }

    // MARK: ADD
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
                Ok(Token::Int32(Int32::new(*num1 + (*num2 as i32))))
            }
            (Number::Int32(num1), Number::Double(num2)) => {
                Ok(Token::Int32(Int32::new(*num1 + (*num2 as i32))))
            }

            (Number::Int64(num1), Number::Int32(num2)) => {
                Self::add_integer_integer(Integer::Int64(num1), Integer::Int32(num2))
            }
            (Number::Int64(num1), Number::Int64(num2)) => {
                Self::add_integer_integer(Integer::Int64(num1), Integer::Int64(num2))
            }
            (Number::Int64(num1), Number::Float(num2)) => {
                Ok(Token::Int64(Int64::new(*num1 + (*num2 as i64))))
            }
            (Number::Int64(num1), Number::Double(num2)) => {
                Ok(Token::Int64(Int64::new(*num1 + (*num2 as i64))))
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

    // MARK: SUB
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
                Ok(Token::Int32(Int32::new(*num1 - (*num2 as i32))))
            }
            (Number::Int32(num1), Number::Double(num2)) => {
                Ok(Token::Int32(Int32::new(*num1 - (*num2 as i32))))
            }

            (Number::Int64(num1), Number::Int32(num2)) => {
                Self::sub_integer_integer(Integer::Int64(num1), Integer::Int32(num2))
            }
            (Number::Int64(num1), Number::Int64(num2)) => {
                Self::sub_integer_integer(Integer::Int64(num1), Integer::Int64(num2))
            }
            (Number::Int64(num1), Number::Float(num2)) => {
                Ok(Token::Int64(Int64::new(*num1 - (*num2 as i64))))
            }
            (Number::Int64(num1), Number::Double(num2)) => {
                Ok(Token::Int64(Int64::new(*num1 - (*num2 as i64))))
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

    // MARK: MUL
    fn mul_integer_integer(left: Integer, right: Integer) -> Result<Token, ParseError> {
        match (left, right) {
            (Integer::Int32(num_1), Integer::Int32(num_2)) => {
                Ok(Token::Int64(Int64::from((*num_1 * *num_2) as i64)))
            }
            (Integer::Int32(num_1), Integer::Int64(num_2)) => {
                Ok(Token::Int64(Int64::from(*num_1 as i64 * *num_2)))
            }
            (Integer::Int64(num_1), Integer::Int32(num_2)) => Ok(Token::Int64(num_1 * num_2)),
            (Integer::Int64(num_1), Integer::Int64(num_2)) => Ok(Token::Int64(num_1 * num_2)),
        }
    }

    fn mul_integer_decimal(left: Number, right: Number) -> Result<Token, ParseError> {
        match (left, right) {
            (Number::Int32(num1), Number::Int32(num2)) => {
                Self::mul_integer_integer(Integer::Int32(num1), Integer::Int32(num2))
            }
            (Number::Int32(num1), Number::Int64(num2)) => {
                Self::mul_integer_integer(Integer::Int32(num1), Integer::Int64(num2))
            }
            (Number::Int32(num1), Number::Float(num2)) => Ok(Token::Double(
                Double::new(*num1 as f64) * Double::new(*num2 as f64),
            )),
            (Number::Int32(num1), Number::Double(num2)) => Ok(Token::Double(
                Double::new(*num1 as f64) * Double::new(*num2 as f64),
            )),

            (Number::Int64(num1), Number::Int32(num2)) => {
                Self::mul_integer_integer(Integer::Int64(num1), Integer::Int32(num2))
            }
            (Number::Int64(num1), Number::Int64(num2)) => {
                Self::mul_integer_integer(Integer::Int64(num1), Integer::Int64(num2))
            }
            (Number::Int64(num1), Number::Float(num2)) => Ok(Token::Double(
                Double::new(*num1 as f64) * Double::new(*num2 as f64),
            )),
            (Number::Int64(num1), Number::Double(num2)) => Ok(Token::Double(
                Double::new(*num1 as f64) * Double::new(*num2 as f64),
            )),

            (Number::Float(num1), Number::Int32(num2)) => Ok(Token::Double(
                Double::new(*num1 as f64) * Double::new(*num2 as f64),
            )),
            (Number::Float(num1), Number::Int64(num2)) => Ok(Token::Double(
                Double::new(*num1 as f64) * Double::new(*num2 as f64),
            )),
            (Number::Float(num1), Number::Float(num2)) => {
                Self::mul_decimal_decimal(Decimal::Float(num1), Decimal::Float(num2))
            }
            (Number::Float(num1), Number::Double(num2)) => {
                Self::mul_decimal_decimal(Decimal::Float(num1), Decimal::Double(num2))
            }

            (Number::Double(num1), Number::Int32(num2)) => {
                Ok(Token::Double(num1 * Double::new(*num2 as f64)))
            }
            (Number::Double(num1), Number::Int64(num2)) => {
                Ok(Token::Double(num1 * Double::new(*num2 as f64)))
            }
            (Number::Double(num1), Number::Float(num2)) => {
                Self::mul_decimal_decimal(Decimal::Double(num1), Decimal::Float(num2))
            }
            (Number::Double(num1), Number::Double(num2)) => {
                Self::mul_decimal_decimal(Decimal::Double(num1), Decimal::Double(num2))
            }
        }
    }

    fn mul_decimal_decimal(left: Decimal, right: Decimal) -> Result<Token, ParseError> {
        match (left, right) {
            (Decimal::Float(num1), Decimal::Float(num2)) => Ok(Token::Double(
                Double::new(*num1 as f64) * Double::new(*num2 as f64),
            )),
            (Decimal::Float(num1), Decimal::Double(num2)) => {
                Ok(Token::Double(Double::new(*num1 as f64) * num2))
            }
            (Decimal::Double(num1), Decimal::Float(num2)) => {
                Ok(Token::Double(num1 + Double::new(*num2 as f64)))
            }
            (Decimal::Double(num1), Decimal::Double(num2)) => Ok(Token::Double(num1 * num2)),
        }
    }

    fn mul_string_number(left: String, right: &Token) -> Result<Token, ParseError> {
        match right {
            Token::Int32(num1) => Ok(Token::String(left.repeat((**num1) as usize))),
            Token::Int64(num1) => Ok(Token::String(left.repeat((**num1) as usize))),
            Token::Float(num1) => Ok(Token::String(left.repeat((**num1) as usize))),
            Token::Double(num1) => Ok(Token::String(left.repeat((**num1) as usize))),
            _ => Err(ParseError::InvalidOperation {
                operation: Self::Mul.to_string(),
                type1: Types::String.to_string(),
                type2: Types::inferred(&right)?.to_string(),
            }),
        }
    }

    // MARK: Div
    fn div_integer_integer(left: Integer, right: Integer) -> Result<Token, ParseError> {
        match (left, right) {
            (Integer::Int32(num_1), Integer::Int32(num_2)) => {
                Ok(Token::Double(Double::from((*num_1 / *num_2) as f64)))
            }
            (Integer::Int32(num_1), Integer::Int64(num_2)) => Ok(Token::Double(Double::from(
                (*num_1 as f64) / (*num_2 as f64),
            ))),
            (Integer::Int64(num_1), Integer::Int32(num_2)) => Ok(Token::Double(Double::from(
                (*num_1 as f64) / (*num_2 as f64),
            ))),
            (Integer::Int64(num_1), Integer::Int64(num_2)) => Ok(Token::Double(Double::from(
                (*num_1 as f64) / (*num_2 as f64),
            ))),
        }
    }

    fn div_integer_decimal(left: Number, right: Number) -> Result<Token, ParseError> {
        match (left, right) {
            (Number::Int32(num1), Number::Int32(num2)) => {
                Self::div_integer_integer(Integer::Int32(num1), Integer::Int32(num2))
            }
            (Number::Int32(num1), Number::Int64(num2)) => {
                Self::div_integer_integer(Integer::Int32(num1), Integer::Int64(num2))
            }
            (Number::Int32(num1), Number::Float(num2)) => {
                Ok(Token::Double(Double::from((*num1 as f64) / (*num2 as f64))))
            }
            (Number::Int32(num1), Number::Double(num2)) => {
                Ok(Token::Double(Double::from((*num1 as f64) / (*num2 as f64))))
            }

            (Number::Int64(num1), Number::Int32(num2)) => {
                Self::div_integer_integer(Integer::Int64(num1), Integer::Int32(num2))
            }
            (Number::Int64(num1), Number::Int64(num2)) => {
                Self::div_integer_integer(Integer::Int64(num1), Integer::Int64(num2))
            }
            (Number::Int64(num1), Number::Float(num2)) => {
                Ok(Token::Double(Double::from((*num1 as f64) / (*num2 as f64))))
            }
            (Number::Int64(num1), Number::Double(num2)) => {
                Ok(Token::Double(Double::from((*num1 as f64) / (*num2 as f64))))
            }

            (Number::Float(num1), Number::Int32(num2)) => {
                Ok(Token::Double(Double::from((*num1 as f64) / (*num2 as f64))))
            }
            (Number::Float(num1), Number::Int64(num2)) => {
                Ok(Token::Double(Double::from((*num1 as f64) / (*num2 as f64))))
            }
            (Number::Float(num1), Number::Float(num2)) => {
                Self::div_decimal_decimal(Decimal::Float(num1), Decimal::Float(num2))
            }
            (Number::Float(num1), Number::Double(num2)) => {
                Self::div_decimal_decimal(Decimal::Float(num1), Decimal::Double(num2))
            }

            (Number::Double(num1), Number::Int32(num2)) => {
                Ok(Token::Double(Double::from((*num1 as f64) / (*num2 as f64))))
            }
            (Number::Double(num1), Number::Int64(num2)) => {
                Ok(Token::Double(Double::from((*num1 as f64) / (*num2 as f64))))
            }
            (Number::Double(num1), Number::Float(num2)) => {
                Self::div_decimal_decimal(Decimal::Double(num1), Decimal::Float(num2))
            }
            (Number::Double(num1), Number::Double(num2)) => {
                Self::div_decimal_decimal(Decimal::Double(num1), Decimal::Double(num2))
            }
        }
    }

    fn div_decimal_decimal(left: Decimal, right: Decimal) -> Result<Token, ParseError> {
        match (left, right) {
            (Decimal::Float(num1), Decimal::Float(num2)) => {
                Ok(Token::Double(Double::from((*num1 as f64) / (*num2 as f64))))
            }
            (Decimal::Float(num1), Decimal::Double(num2)) => {
                Ok(Token::Double(Double::from((*num1 as f64) / (*num2 as f64))))
            }
            (Decimal::Double(num1), Decimal::Float(num2)) => {
                Ok(Token::Double(Double::from((*num1 as f64) / (*num2 as f64))))
            }
            (Decimal::Double(num1), Decimal::Double(num2)) => Ok(Token::Double(num1 * num2)),
        }
    }

    fn div_string_number(left: String, right: &Token) -> Result<Token, ParseError> {
        match right {
            Token::Int32(num1) => Ok(Token::List(Self::split_string(left, **num1 as usize))),
            Token::Int64(num1) => Ok(Token::List(Self::split_string(left, **num1 as usize))),
            Token::Float(num1) => Ok(Token::List(Self::split_string(left, **num1 as usize))),
            Token::Double(num1) => Ok(Token::List(Self::split_string(left, **num1 as usize))),
            _ => Err(ParseError::InvalidOperation {
                operation: Self::Div.to_string(),
                type1: Types::String.to_string(),
                type2: Types::inferred(&right)?.to_string(),
            }),
        }
    }

    fn split_string(string: String, num: usize) -> Vec<Token> {
        string
            .chars()
            .collect::<Vec<char>>() // Convertir a un vector de caracteres
            .chunks(num.div_ceil(2)) // Dividir en partes de tamaño `num`
            .map(|chunk| Token::String(chunk.iter().collect::<String>())) // Convertir cada parte a Token::String
            .collect() // Collect the iterator into a vector
    }

    // MARK: Div
    fn div_int_integer_integer(left: Integer, right: Integer) -> Result<Token, ParseError> {
        match (left, right) {
            (Integer::Int32(num_1), Integer::Int32(num_2)) => {
                if *num_2 == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num_1 as i128 / *num_2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
            (Integer::Int32(num_1), Integer::Int64(num_2)) => {
                if *num_2 == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num_1 as i128 / *num_2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
            (Integer::Int64(num_1), Integer::Int32(num_2)) => {
                if *num_2 == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num_1 as i128 / *num_2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
            (Integer::Int64(num_1), Integer::Int64(num_2)) => {
                if *num_2 == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num_1 as i128 / *num_2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
        }
    }

    fn div_int_integer_decimal(left: Number, right: Number) -> Result<Token, ParseError> {
        match (left, right) {
            (Number::Int32(num1), Number::Int32(num2)) => {
                Self::div_int_integer_integer(Integer::Int32(num1), Integer::Int32(num2))
            }
            (Number::Int32(num1), Number::Int64(num2)) => {
                Self::div_int_integer_integer(Integer::Int32(num1), Integer::Int64(num2))
            }
            (Number::Int32(num1), Number::Float(num2)) => {
                if (*num2 as i32) == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num1 as i128 / *num2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
            (Number::Int32(num1), Number::Double(num2)) => {
                if (*num2 as i64) == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num1 as i128 / *num2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }

            (Number::Int64(num1), Number::Int32(num2)) => {
                Self::div_int_integer_integer(Integer::Int64(num1), Integer::Int32(num2))
            }
            (Number::Int64(num1), Number::Int64(num2)) => {
                Self::div_int_integer_integer(Integer::Int64(num1), Integer::Int64(num2))
            }
            (Number::Int64(num1), Number::Float(num2)) => {
                if (*num2 as i32) == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num1 as i128 / *num2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
            (Number::Int64(num1), Number::Double(num2)) => {
                if (*num2 as i64) == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num1 as i128 / *num2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }

            (Number::Float(num1), Number::Int32(num2)) => {
                if *num2 == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num1 as i128 / *num2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
            (Number::Float(num1), Number::Int64(num2)) => {
                if *num2 == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num1 as i128 / *num2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
            (Number::Float(num1), Number::Float(num2)) => {
                Self::div_int_decimal_decimal(Decimal::Float(num1), Decimal::Float(num2))
            }
            (Number::Float(num1), Number::Double(num2)) => {
                Self::div_int_decimal_decimal(Decimal::Float(num1), Decimal::Double(num2))
            }

            (Number::Double(num1), Number::Int32(num2)) => {
                if *num2 == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num1 as i128 / *num2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
            (Number::Double(num1), Number::Int64(num2)) => {
                if *num2 == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num1 as i128 / *num2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
            (Number::Double(num1), Number::Float(num2)) => {
                Self::div_int_decimal_decimal(Decimal::Double(num1), Decimal::Float(num2))
            }
            (Number::Double(num1), Number::Double(num2)) => {
                Self::div_int_decimal_decimal(Decimal::Double(num1), Decimal::Double(num2))
            }
        }
    }

    fn div_int_decimal_decimal(left: Decimal, right: Decimal) -> Result<Token, ParseError> {
        match (left, right) {
            (Decimal::Float(num1), Decimal::Float(num2)) => {
                if (*num2 as i32) == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num1 as i128 / *num2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
            (Decimal::Float(num1), Decimal::Double(num2)) => {
                if (*num2 as i64) == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num1 as i128 / *num2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
            (Decimal::Double(num1), Decimal::Float(num2)) => {
                if (*num2 as i32) == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num1 as i128 / *num2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
            (Decimal::Double(num1), Decimal::Double(num2)) => {
                if (*num2 as i64) == 0 {
                    return Err(ParseError::DivisionByZero);
                }

                let result = *num1 as i128 / *num2 as i128;

                if result <= i32::MAX as i128 && result >= i32::MIN as i128 {
                    Ok(Token::Int32(Int32::from(result as i32)))
                } else if result <= i64::MAX as i128 && result >= i64::MIN as i128 {
                    Ok(Token::Int64(Int64::from(result)))
                } else {
                    Ok(Token::HPInt(HPInt::from(result)))
                }
            }
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
