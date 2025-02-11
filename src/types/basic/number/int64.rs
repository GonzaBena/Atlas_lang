use std::fmt;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, MulAssign, Rem, Sub};

use super::double::Double;
use super::int32::Int32;
use num::ToPrimitive;
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

impl Into<f64> for Int64 {
    fn into(self) -> f64 {
        self.data as f64
    }
}

impl<T> From<T> for Int64
where
    T: ToPrimitive,
{
    fn from(value: T) -> Self {
        let val = T::to_i64(&value).unwrap();
        Self { data: val }
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
    type Output = Double;

    fn div(self, rhs: Int32) -> Self::Output {
        let result = *self as f64 / *rhs as f64;
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

impl PartialOrd<i64> for Int64 {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(other)
    }
}

impl PartialEq<i64> for Int64 {
    fn eq(&self, other: &i64) -> bool {
        self.data == *other
    }
}
