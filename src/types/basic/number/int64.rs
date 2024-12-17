use std::fmt;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, MulAssign, Rem, Sub};

use super::double::Double;
use super::int32::Int32;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Int64 {
    data: i64,
}

impl Int64 {
    pub fn new(num: i64) -> Self {
        Self { data: num }
    }
}

impl Deref for Int64 {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Int64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl fmt::Display for Int64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl From<i64> for Int64 {
    fn from(value: i64) -> Self {
        Self { data: value }
    }
}

impl From<i32> for Int64 {
    fn from(value: i32) -> Self {
        Self { data: value as i64 }
    }
}

impl From<Int32> for Int64 {
    fn from(value: Int32) -> Self {
        Self {
            data: *value as i64,
        }
    }
}

impl From<f64> for Int64 {
    fn from(value: f64) -> Self {
        Self { data: value as i64 }
    }
}

impl From<Double> for Int64 {
    fn from(value: Double) -> Self {
        Self {
            data: *value as i64,
        }
    }
}

impl From<u32> for Int64 {
    fn from(value: u32) -> Self {
        Self {
            data: value.to_string().parse().expect("Espected a u32"),
        }
    }
}

impl Add for Int64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let result = *self + *rhs;
        Self { data: result }
    }
}

impl Add<Double> for Int64 {
    type Output = Double;

    fn add(self, rhs: Double) -> Self::Output {
        let result = *self as f64 + *rhs;
        Self::Output::new(result)
    }
}

impl Add<Int32> for Int64 {
    type Output = Self;

    fn add(self, rhs: Int32) -> Self::Output {
        let result = *self + *rhs as i64;
        Self::Output::new(result)
    }
}

impl AddAssign for Int64 {
    fn add_assign(&mut self, rhs: Self) {
        self.data += *rhs;
    }
}

impl Sub for Int64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let result = *self - *rhs;
        Self { data: result }
    }
}

impl Sub<Double> for Int64 {
    type Output = Double;

    fn sub(self, rhs: Double) -> Self::Output {
        let result = *self as f64 - *rhs;
        Self::Output::new(result)
    }
}

impl Sub<Int32> for Int64 {
    type Output = Self;

    fn sub(self, rhs: Int32) -> Self::Output {
        let result = *self - *rhs as i64;
        Self::Output::new(result)
    }
}

impl Mul for Int64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = *self * *rhs;
        Self { data: result }
    }
}

impl Mul<Double> for Int64 {
    type Output = Double;

    fn mul(self, rhs: Double) -> Self::Output {
        let result = *self as f64 * *rhs;
        Self::Output::new(result)
    }
}

impl Mul<Int32> for Int64 {
    type Output = Self;

    fn mul(self, rhs: Int32) -> Self::Output {
        let result = *self * *rhs as i64;
        Self::Output::new(result)
    }
}

impl MulAssign for Int64 {
    fn mul_assign(&mut self, rhs: Self) {
        self.data *= rhs.data
    }
}

impl Div for Int64 {
    type Output = Double;

    fn div(self, rhs: Self) -> Self::Output {
        let result = *self as f64 / *rhs as f64;
        Self::Output::new(result)
    }
}

impl Div<Double> for Int64 {
    type Output = Double;

    fn div(self, rhs: Double) -> Self::Output {
        let result = *self as f64 / *rhs;
        Self::Output::new(result)
    }
}

impl Div<Int32> for Int64 {
    type Output = Self;

    fn div(self, rhs: Int32) -> Self::Output {
        let result = *self / *rhs as i64;
        Self::Output::new(result)
    }
}

impl Rem for Int64 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output::new(*self % *rhs)
    }
}

impl Rem<Double> for Int64 {
    type Output = Self;
    fn rem(self, rhs: Double) -> Self::Output {
        Self::Output::new(*self % *rhs as i64)
    }
}

impl Rem<Int32> for Int64 {
    type Output = Self;
    fn rem(self, rhs: Int32) -> Self::Output {
        Self::Output::new(*self % *rhs as i64)
    }
}
