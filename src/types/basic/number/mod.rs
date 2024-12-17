use std::fmt;

use int32::Int32;
use int64::Int64;

use crate::compiler::types::Types;
use serde::Serialize;

pub mod double;
pub mod int32;
pub mod int64;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Number {
    // Int8(i8),
    // Int16(i16),
    Int32(Int32),
    Int64(Int64),
}

#[allow(dead_code)]
impl Number {
    pub fn new<T: Into<i64>>(value: T) -> Self {
        type Output = i64;
        let value = value.into();
        match value {
            // v if v >= i8::MIN as i64 && v <= i8::MAX as i64 => Number::Int8(v as i8),
            // v if v >= i16::MIN as i64 && v <= i16::MAX as i64 => Number::Int16(v as i16),
            v if v >= i32::MIN as Output && v <= i32::MAX as Output => Number::Int32(v.into()),
            _ => Number::Int64(value.into()),
        }
    }

    pub fn as_type(&self) -> Types {
        match self {
            Self::Int32(_) => Types::Int32,
            Self::Int64(_) => Types::Int32,
        }
    }

    pub fn as_int64(&self) -> Int64 {
        type Output = Int64;
        match self {
            // Number::Int8(v) => *v as i64,
            // Number::Int16(v) => *v as i64,
            Number::Int32(v) => Output::from(*v),
            Number::Int64(v) => *v as Output,
        }
    }

    pub fn as_int32(&self) -> Int32 {
        type Output = Int32;
        match self {
            // Number::Int8(v) => *v as i64,
            // Number::Int16(v) => *v as i64,
            Number::Int32(v) => Output::from(*v),
            Number::Int64(v) => Output::from(*v),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
