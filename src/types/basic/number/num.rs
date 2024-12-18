use std::ops::{Add, Div, Mul, Sub};

use super::{double::Double, int32::Int32, int64::Int64, Number};

pub trait Num<Rhs = Self>:
    Add<Rhs, Output = Self::AddOutput>
    + Sub<Rhs, Output = Self::SubOutput>
    + Mul<Rhs, Output = Self::MulOutput>
    + Div<Rhs, Output = Self::DivOutput>
    + Sized
    + Copy
    + PartialOrd
    + PartialEq
    + Default
{
    type AddOutput;
    type SubOutput;
    type MulOutput;
    type DivOutput: Into<f64>;

    fn zero() -> Self;
    fn one() -> Self;
}

impl From<Number> for f64 {
    fn from(value: Number) -> Self {
        *value.as_double()
    }
}

impl Default for Number {
    fn default() -> Self {
        Number::zero()
    }
}

impl Default for Int32 {
    fn default() -> Self {
        Int32::zero()
    }
}

impl Default for Int64 {
    fn default() -> Self {
        Int64::zero()
    }
}

impl Default for Double {
    fn default() -> Self {
        Double::zero()
    }
}

impl Num for Number {
    type AddOutput = Self;
    type SubOutput = Self;
    type MulOutput = Self;
    type DivOutput = Self;

    fn zero() -> Self {
        Number::Int32(0.into())
    }

    fn one() -> Self {
        Number::Int32(1.into())
    }
}

impl Num for Int32 {
    type AddOutput = Self;
    type SubOutput = Self;
    type MulOutput = Self;
    type DivOutput = Double;

    fn zero() -> Self {
        Int32::new(0.into())
    }

    fn one() -> Self {
        Int32::new(1.into())
    }
}

impl Num for Int64 {
    type AddOutput = Self;
    type SubOutput = Self;
    type MulOutput = Self;
    type DivOutput = Double;

    fn zero() -> Self {
        Int64::new(0.into())
    }

    fn one() -> Self {
        Int64::new(1.into())
    }
}

impl Num for Double {
    type AddOutput = Self;
    type SubOutput = Self;
    type MulOutput = Self;
    type DivOutput = Self;

    fn zero() -> Self {
        Double::new(0.into())
    }

    fn one() -> Self {
        Double::new(1.into())
    }
}
