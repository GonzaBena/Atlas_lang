use std::{
    fmt,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, Sub},
};

use double::Double;
use int32::Int32;
use int64::Int64;

use crate::compiler::types::Types;
use serde::Serialize;

pub mod double;
pub mod int32;
pub mod int64;
pub mod num;

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd, Copy)]
#[allow(dead_code)]
pub enum Number {
    // Int8(i8),
    // Int16(i16),
    Int32(Int32),
    Int64(Int64),

    //decimal numbers
    Double(Double),
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::Int32(value.into())
    }
}

impl From<Int32> for Number {
    fn from(value: Int32) -> Self {
        Self::Int32(value)
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::Int64(value.into())
    }
}

impl From<Int64> for Number {
    fn from(value: Int64) -> Self {
        Self::Int64(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::Double(value.into())
    }
}

impl From<Double> for Number {
    fn from(value: Double) -> Self {
        Self::Double(value)
    }
}

#[allow(dead_code)]
impl Number {
    pub fn new<T: Into<f64>>(value: T) -> Self {
        type Output = f64;
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
            Self::Double(_) => Types::Double,
        }
    }

    pub fn as_int64(&self) -> Int64 {
        type Output = Int64;
        match self {
            // Number::Int8(v) => *v as i64,
            // Number::Int16(v) => *v as i64,
            Number::Int32(v) => Output::from(*v),
            Number::Int64(v) => *v as Output,
            Number::Double(double) => (**double as i64).into(),
        }
    }

    pub fn as_double(&self) -> Double {
        type Output = Double;
        match self {
            // Number::Int8(v) => *v as i64,
            // Number::Int16(v) => *v as i64,
            Number::Int32(v) => Output::from(**v as f64),
            Number::Int64(v) => Output::from(**v as f64),
            Number::Double(double) => *double,
        }
    }

    pub fn as_int32(&self) -> Int32 {
        type Output = Int32;
        match self {
            // Number::Int8(v) => *v as i64,
            // Number::Int16(v) => *v as i64,
            Number::Int32(v) => Output::from(*v),
            Number::Int64(v) => Output::from(*v),
            Number::Double(double) => Output::from(*double),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int32(num) => write!(f, "{}", num),
            Number::Int64(num) => write!(f, "{}", num),
            Number::Double(num) => write!(f, "{}", num),
        }
    }
}

impl PartialOrd<i32> for Number {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        match self {
            Self::Int32(data) => (*data).partial_cmp(other),
            Self::Int64(data) => (*data).partial_cmp(&(*other as i64)),
            Number::Double(data) => (*data).partial_cmp(&(*other as f64)),
        }
    }
}

impl PartialEq<i32> for Number {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Self::Int32(data) => *data == *other,
            Self::Int64(data) => *data == *other as i64,
            Self::Double(data) => *data == *other as f64,
        }
    }
}

impl PartialOrd<i64> for Number {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        match self {
            Self::Int32(data) => (*data).partial_cmp(&(*other as i32)),
            Self::Int64(data) => (*data).partial_cmp(other),
            Self::Double(data) => (*data).partial_cmp(&(*other as f64)),
        }
    }
}

impl PartialEq<i64> for Number {
    fn eq(&self, other: &i64) -> bool {
        match self {
            Self::Int32(data) => *data == *other as i32,
            Self::Int64(data) => *data == *other,
            Self::Double(data) => *data == *other as f64,
        }
    }
}

impl PartialOrd<f64> for Number {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        match self {
            Self::Int32(data) => (*data).partial_cmp(&(*other as i32)),
            Self::Int64(data) => (*data).partial_cmp(&(*other as i64)),
            Self::Double(data) => (*data).partial_cmp(other),
        }
    }
}

impl PartialEq<f64> for Number {
    fn eq(&self, other: &f64) -> bool {
        match self {
            Self::Int32(data) => *data == *other as i32,
            Self::Int64(data) => *data == *other as i64,
            Self::Double(data) => data == other,
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Int32(num1), Number::Int32(num2)) => Self::Int32(num1 + num2),
            (Number::Int32(num1), Number::Int64(num2)) => Self::Int64(num1 + num2),
            (Number::Int64(num1), Number::Int32(num2)) => Self::Int64(num1 + num2),
            (Number::Int64(num1), Number::Int64(num2)) => Self::Int64(num1 + num2),
            (Number::Int32(num1), Number::Double(num2)) => Self::Double(num1 + num2),
            (Number::Int64(num1), Number::Double(num2)) => Self::Double(num1 + num2),
            (Number::Double(num1), Number::Int32(num2)) => Self::Double((num1 + num2).into()),
            (Number::Double(num1), Number::Int64(num2)) => Self::Double((num1 + num2).into()),
            (Number::Double(num1), Number::Double(num2)) => Self::Double(num1 + num2),
        }
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = match (&self, rhs) {
            (Number::Int32(num1), Number::Int32(num2)) => Self::Int32(*num1 + num2),
            (Number::Int32(num1), Number::Int64(num2)) => Self::Int64(*num1 + num2),
            (Number::Int64(num1), Number::Int32(num2)) => Self::Int64(*num1 + num2),
            (Number::Int64(num1), Number::Int64(num2)) => Self::Int64(*num1 + num2),
            (Number::Int32(num1), Number::Double(num2)) => Self::Double(*num1 + num2),
            (Number::Int64(num1), Number::Double(num2)) => Self::Double(*num1 + num2),
            (Number::Double(num1), Number::Int32(num2)) => Self::Double((*num1 + num2).into()),
            (Number::Double(num1), Number::Int64(num2)) => Self::Double(*num1 + num2),
            (Number::Double(num1), Number::Double(num2)) => Self::Double(*num1 + num2),
        };
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Int32(num1), Number::Int32(num2)) => Self::Output::from(num1 - num2),
            (Number::Int32(num1), Number::Int64(num2)) => Self::Output::from(num1 - num2),
            (Number::Int32(num1), Number::Double(num2)) => Self::Output::from(num1 - num2),
            (Number::Int64(num1), Number::Int32(num2)) => Self::Output::from(num1 - num2),
            (Number::Int64(num1), Number::Int64(num2)) => Self::Output::from(num1 - num2),
            (Number::Int64(num1), Number::Double(num2)) => Self::Output::from(num1 - num2),
            (Number::Double(num1), Number::Int32(num2)) => Self::Output::from(num1 - num2),
            (Number::Double(num1), Number::Int64(num2)) => Self::Output::from(num1 - num2),
            (Number::Double(num1), Number::Double(num2)) => Self::Output::from(num1 - num2),
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Int32(num1), Number::Int32(num2)) => Self::Output::from(num1 * num2),
            (Number::Int32(num1), Number::Int64(num2)) => Self::Output::from(num1 * num2),
            (Number::Int32(num1), Number::Double(num2)) => Self::Output::from(num1 * num2),
            (Number::Int64(num1), Number::Int32(num2)) => Self::Output::from(num1 * num2),
            (Number::Int64(num1), Number::Int64(num2)) => Self::Output::from(num1 * num2),
            (Number::Int64(num1), Number::Double(num2)) => Self::Output::from(num1 * num2),
            (Number::Double(num1), Number::Int32(num2)) => Self::Output::from(num1 * num2),
            (Number::Double(num1), Number::Int64(num2)) => Self::Output::from(num1 * num2),
            (Number::Double(num1), Number::Double(num2)) => Self::Output::from(num1 * num2),
        }
    }
}

impl MulAssign for Number {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Int32(num1), Number::Int32(num2)) => Self::Output::from(num1 / num2),
            (Number::Int32(num1), Number::Int64(num2)) => Self::Output::from(num1 / num2),
            (Number::Int32(num1), Number::Double(num2)) => Self::Output::from(num1 / num2),
            (Number::Int64(num1), Number::Int32(num2)) => Self::Output::from(num1 / num2),
            (Number::Int64(num1), Number::Int64(num2)) => Self::Output::from(num1 / num2),
            (Number::Int64(num1), Number::Double(num2)) => Self::Output::from(num1 / num2),
            (Number::Double(num1), Number::Int32(num2)) => Self::Output::from(num1 / num2),
            (Number::Double(num1), Number::Int64(num2)) => Self::Output::from(num1 / num2),
            (Number::Double(num1), Number::Double(num2)) => Self::Output::from(num1 / num2),
        }
    }
}

impl Rem for Number {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Int32(num1), Number::Int32(num2)) => Number::Int32(num1 % num2),
            (Number::Int32(num1), Number::Int64(num2)) => Number::Int32(num1 % num2),
            (Number::Int32(num1), Number::Double(num2)) => Number::Int32(num1 % num2),
            (Number::Int64(num1), Number::Int32(num2)) => Number::Int32((num1 % num2).into()),
            (Number::Int64(num1), Number::Int64(num2)) => Number::Int32((num1 % num2).into()),
            (Number::Int64(num1), Number::Double(num2)) => Number::Int32((num1 % num2).into()),
            (Number::Double(num1), Number::Int32(num2)) => Number::Int32(num1 % num2),
            (Number::Double(num1), Number::Int64(num2)) => Number::Int32(num1 % num2),
            (Number::Double(num1), Number::Double(num2)) => Number::Int32(num1 % num2),
        }
    }
}
