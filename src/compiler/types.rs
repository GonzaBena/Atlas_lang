use std::str::FromStr;

use super::error::parse_error::ParseError;

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Types {
    Int32,
    Double,
    String,
    Str,
    Boolean,
    Void,
    Function,
}

#[allow(dead_code)]
impl Types {
    pub fn is_integer(&self) -> bool {
        match self {
            Self::Int32 => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Self::Double => true,
            _ => false,
        }
    }

    pub fn is_numeric(&self) -> bool {
        self.is_float() && self.is_integer()
    }
}

impl FromStr for Types {
    type Err = ParseError<'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Boolean" => Ok(Self::Boolean),
            "Int32" => Ok(Self::Int32),
            "Double" => Ok(Self::Double),
            "String" => Ok(Self::String),
            "Str" => Ok(Self::Str),
            "Void" => Ok(Self::Void),
            "Function" => Ok(Self::Function),
            _ => Err(ParseError::InvalidType("invalid type")),
        }
    }
}
