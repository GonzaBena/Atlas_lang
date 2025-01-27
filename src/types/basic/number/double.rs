use std::fmt;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Rem, Sub};

use super::int32::Int32;
use super::int64::Int64;
use super::Number;
use num::ToPrimitive;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, PartialOrd)]
pub struct Double {
    data: f64,
}

#[allow(dead_code)]
impl Double {
    pub fn new(num: f64) -> Self {
        Self { data: num }
    }

    pub fn trunc(&mut self) -> Int32 {
        Int32::new(self.data as i32)
    }
}

impl Deref for Double {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Double {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl fmt::Display for Double {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl Into<f64> for Double {
    fn into(self) -> f64 {
        *self
    }
}

impl ToPrimitive for Double {
    fn to_i64(&self) -> Option<i64> {
        self.data.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.data.to_u64()
    }

    fn to_f64(&self) -> Option<f64> {
        Some(self.data)
    }
}

impl Double {
    pub fn from<T: Into<f64>>(value: T) -> Self {
        Self { data: value.into() }
    }
}

impl Number for Double {
    type Pow = Self;
    type Output = Self;

    fn add<T: Number>(&self, other: T) -> Self::Output {
        Self::new(**self + other.to_f64().unwrap())
    }

    fn sub<T: Number>(&self, other: T) -> Self::Output {
        Self::new(**self - other.to_f64().unwrap())
    }

    fn mul<T: Number>(&self, other: T) -> Self::Output {
        Self::new(**self * other.to_f64().unwrap())
    }

    fn div<T: Number>(&self, other: T) -> Self::Output {
        Self::new(**self / other.to_f64().unwrap())
    }

    fn module<T: Number>(&self, other: T) -> Self::Output {
        let result = **self % other.to_f64().unwrap();
        Self::new(result)
    }

    fn abs<T: Number>(&self) -> Self::Output {
        Self::new((**self).abs())
    }

    fn power<T: Into<i32> + PartialOrd<i32> + Clone>(&self, other: T) -> Self::Pow {
        let mut result = **self;

        if other <= 1 {
            return Self::Output::new(result);
        }

        for _ in 2..=(other.clone().into()) {
            result = result.powi(other.clone().into());
        }

        Self::Output::new(result)
    }
}

impl<T> Add<T> for Double
where
    T: Into<f64> + TryInto<f64>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let result = *self + rhs.into();
        Self { data: result }
    }
}

impl AddAssign for Double {
    fn add_assign(&mut self, rhs: Self) {
        self.data += *rhs;
    }
}

impl Sub for Double {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let result = *self - *rhs;
        Self { data: result }
    }
}

impl Sub<Int32> for Double {
    type Output = Int32;

    fn sub(self, rhs: Int32) -> Self::Output {
        let result = *self as i32 - *rhs;
        Self::Output::new(result)
    }
}

impl Sub<Int64> for Double {
    type Output = Int64;

    fn sub(self, rhs: Int64) -> Self::Output {
        let result = *self as i64 - *rhs;
        Self::Output::new(result)
    }
}

impl Mul for Double {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = *self * *rhs;
        Self { data: result }
    }
}

impl Mul<Int32> for Double {
    type Output = Int32;

    fn mul(self, rhs: Int32) -> Self::Output {
        let result = *self as i32 * *rhs;
        Self::Output::new(result)
    }
}

impl Mul<Int64> for Double {
    type Output = Double;

    fn mul(self, rhs: Int64) -> Self::Output {
        let result = *self as f64 * *rhs as f64;
        Self::Output::new(result)
    }
}

impl Div for Double {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let result = *self as f64 / *rhs as f64;
        Self { data: result }
    }
}

impl Div<Int32> for Double {
    type Output = Self;

    fn div(self, rhs: Int32) -> Self::Output {
        let result = *self / *rhs as f64;
        Self::Output::new(result)
    }
}

impl Div<Int64> for Double {
    type Output = Self;

    fn div(self, rhs: Int64) -> Self::Output {
        let result = *self / *rhs as f64;
        Self::Output::new(result)
    }
}

impl Rem for Double {
    type Output = Int32;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output::new(*self as i32 % *rhs as i32)
    }
}

impl Rem<Int32> for Double {
    type Output = Int32;
    fn rem(self, rhs: Int32) -> Self::Output {
        Self::Output::new(*self as i32 % *rhs)
    }
}

impl Rem<Int64> for Double {
    type Output = Int32;
    fn rem(self, rhs: Int64) -> Self::Output {
        Self::Output::new(*self as i32 % *rhs as i32)
    }
}

impl PartialOrd<f64> for Double {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(other)
    }
}

impl PartialEq<f64> for Double {
    fn eq(&self, other: &f64) -> bool {
        self.data == *other
    }
}
