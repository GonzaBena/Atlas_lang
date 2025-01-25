use std::fmt;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Rem, Sub};

use super::int32::Int32;
use super::int64::Int64;
use num::ToPrimitive;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, PartialOrd)]
pub struct Float {
    data: f32,
}

#[allow(dead_code)]
impl Float {
    pub fn new(num: f32) -> Self {
        Self { data: num }
    }

    pub fn trunc(&mut self) -> Int32 {
        Int32::new(self.data as i32)
    }
}

impl Deref for Float {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl Into<f32> for Float {
    fn into(self) -> f32 {
        *self
    }
}

// impl From<f32> for Float {
//     fn from(value: f32) -> Self {
//         Self { data: value }
//     }
// }

impl<T> From<T> for Float
where
    T: ToPrimitive,
{
    fn from(value: T) -> Self {
        let val: f32 = T::to_f32(&value).unwrap();
        Self { data: val }
    }
}

impl Add for Float {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let result = *self + *rhs;
        Self { data: result }
    }
}

impl Add<Int32> for Float {
    type Output = Int32;

    fn add(self, rhs: Int32) -> Self::Output {
        let result = *self as i32 - *rhs;
        Self::Output::new(result)
    }
}

impl Add<Int64> for Float {
    type Output = Self;

    fn add(self, rhs: Int64) -> Self::Output {
        let result = *self + *rhs as f32;
        Self::Output::new(result)
    }
}

impl AddAssign for Float {
    fn add_assign(&mut self, rhs: Self) {
        self.data += *rhs;
    }
}

impl Sub for Float {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let result = *self - *rhs;
        Self { data: result }
    }
}

impl Sub<Int32> for Float {
    type Output = Int32;

    fn sub(self, rhs: Int32) -> Self::Output {
        let result = *self as i32 - *rhs;
        Self::Output::new(result)
    }
}

impl Sub<Int64> for Float {
    type Output = Int64;

    fn sub(self, rhs: Int64) -> Self::Output {
        let result = *self as i64 - *rhs;
        Self::Output::new(result)
    }
}

impl Mul for Float {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = *self * *rhs;
        Self { data: result }
    }
}

impl Mul<Int32> for Float {
    type Output = Int32;

    fn mul(self, rhs: Int32) -> Self::Output {
        let result = *self as i32 * *rhs;
        Self::Output::new(result)
    }
}

impl Mul<Int64> for Float {
    type Output = Float;

    fn mul(self, rhs: Int64) -> Self::Output {
        let result = *self as f32 * *rhs as f32;
        Self::Output::new(result)
    }
}

impl Div for Float {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let result = *self as f32 / *rhs as f32;
        Self { data: result }
    }
}

impl Div<Int32> for Float {
    type Output = Self;

    fn div(self, rhs: Int32) -> Self::Output {
        let result = *self / *rhs as f32;
        Self::Output::new(result)
    }
}

impl Div<Int64> for Float {
    type Output = Self;

    fn div(self, rhs: Int64) -> Self::Output {
        let result = *self / *rhs as f32;
        Self::Output::new(result)
    }
}

impl Rem for Float {
    type Output = Int32;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output::new(*self as i32 % *rhs as i32)
    }
}

impl Rem<Int32> for Float {
    type Output = Int32;
    fn rem(self, rhs: Int32) -> Self::Output {
        Self::Output::new(*self as i32 % *rhs)
    }
}

impl Rem<Int64> for Float {
    type Output = Int32;
    fn rem(self, rhs: Int64) -> Self::Output {
        Self::Output::new(*self as i32 % *rhs as i32)
    }
}

impl PartialOrd<f32> for Float {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(other)
    }
}

impl PartialEq<f32> for Float {
    fn eq(&self, other: &f32) -> bool {
        self.data == *other
    }
}
