use serde::Serialize;
use std::fmt;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, MulAssign, Rem, Sub};

use super::double::Double;
use super::int64::Int64;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Int32 {
    data: i32,
}

#[allow(dead_code)]
impl Int32 {
    pub const MAX: Int32 = Int32 { data: i32::MAX };
    pub const MIN: Int32 = Int32 { data: i32::MIN };

    pub fn new(num: i32) -> Self {
        Self { data: num }
    }
}

impl Deref for Int32 {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Int32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl fmt::Display for Int32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Into<f64> for Int32 {
    fn into(self) -> f64 {
        self.data as f64
    }
}

impl From<i32> for Int32 {
    fn from(value: i32) -> Self {
        Self { data: value }
    }
}

impl From<i64> for Int32 {
    fn from(value: i64) -> Self {
        Self { data: value as i32 }
    }
}

impl From<Int64> for Int32 {
    fn from(value: Int64) -> Self {
        Self {
            data: *value as i32,
        }
    }
}

impl From<f64> for Int32 {
    fn from(value: f64) -> Self {
        Self { data: value as i32 }
    }
}

impl From<u32> for Int32 {
    fn from(value: u32) -> Self {
        Self {
            data: value.to_string().parse().expect("Espected a u32"),
        }
    }
}

impl From<Double> for Int32 {
    fn from(value: Double) -> Int32 {
        Int32::new(*value as i32)
    }
}

impl Add for Int32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let result = *self + *rhs;
        Self { data: result }
    }
}

impl Add<Double> for Int32 {
    type Output = Double;

    fn add(self, rhs: Double) -> Self::Output {
        let result = *self as f64 + *rhs;
        Self::Output::new(result)
    }
}

impl Add<Int64> for Int32 {
    type Output = Int64;

    fn add(self, rhs: Self::Output) -> Self::Output {
        let result = *self as i64 + *rhs;
        Self::Output::new(result)
    }
}

impl AddAssign for Int32 {
    fn add_assign(&mut self, rhs: Self) {
        self.data += *rhs;
    }
}

impl Sub for Int32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let result = *self - *rhs;
        Self { data: result }
    }
}

impl Sub<Double> for Int32 {
    type Output = Double;

    fn sub(self, rhs: Double) -> Self::Output {
        let result = *self as f64 - *rhs;
        Self::Output::new(result)
    }
}

impl Sub<Int64> for Int32 {
    type Output = Int64;

    fn sub(self, rhs: Int64) -> Self::Output {
        let result = *self as i64 - *rhs;
        Self::Output::new(result)
    }
}

impl Mul for Int32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = *self * *rhs;
        Self { data: result }
    }
}

impl Mul<Double> for Int32 {
    type Output = Double;

    fn mul(self, rhs: Double) -> Self::Output {
        let result = *self as f64 * *rhs;
        Self::Output::new(result)
    }
}

impl Mul<Int64> for Int32 {
    type Output = Int64;

    fn mul(self, rhs: Int64) -> Self::Output {
        let result = *self as i64 * *rhs;
        Self::Output::new(result)
    }
}

impl MulAssign for Int32 {
    fn mul_assign(&mut self, rhs: Self) {
        self.data *= rhs.data
    }
}

impl Div for Int32 {
    type Output = Double;

    fn div(self, rhs: Self) -> Self::Output {
        let result = *self as f64 / *rhs as f64;
        Self::Output::new(result)
    }
}

impl Div<Double> for Int32 {
    type Output = Double;

    fn div(self, rhs: Double) -> Self::Output {
        let result = *self as f64 / *rhs;
        Self::Output::new(result)
    }
}

impl Div<Int64> for Int32 {
    type Output = Double;

    fn div(self, rhs: Int64) -> Self::Output {
        let result: f64 = *self as f64 / *rhs as f64;
        Self::Output::new(result)
    }
}

impl Rem for Int32 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output::new(*self % *rhs)
    }
}

impl Rem<Double> for Int32 {
    type Output = Self;
    fn rem(self, rhs: Double) -> Self::Output {
        Self::Output::new(*self % *rhs as i32)
    }
}

impl Rem<Int64> for Int32 {
    type Output = Self;
    fn rem(self, rhs: Int64) -> Self::Output {
        Self::Output::new(*self % *rhs as i32)
    }
}

impl PartialOrd<i32> for Int32 {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(other)
    }
}

impl PartialEq<i32> for Int32 {
    fn eq(&self, other: &i32) -> bool {
        self.data == *other
    }
}
