use num::ToPrimitive;
use serde::Serialize;
use std::fmt;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, MulAssign, Rem, Sub};

use super::double::Double;
use super::int64::Int64;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Int32 {
    data: i32,
}

impl ToPrimitive for Int32 {
    fn to_i64(&self) -> Option<i64> {
        Some(self.data as i64)
    }

    fn to_f64(&self) -> Option<f64> {
        Some(self.data as f64)
    }

    fn to_u64(&self) -> Option<u64> {
        Some(self.data as u64)
    }
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

impl<T: Into<i32>> From<T> for Int32 {
    fn from(value: T) -> Self {
        Self { data: value.into() }
    }
}

impl Add for Int32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let result = *self + *rhs;
        Self { data: result }
    }
}

impl<T> Add<T> for Int32
where
    T: Into<i32>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let result = *self + rhs.into();
        Self { data: result }
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

impl<T> Sub<T> for Int32
where
    T: Into<i32>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let result = *self - rhs.into();
        Self { data: result }
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
