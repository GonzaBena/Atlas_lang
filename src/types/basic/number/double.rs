use std::fmt;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Rem, Sub};

use super::int32::Int32;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Double {
    data: f64,
}

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
        write!(f, "{}", self.data)
    }
}

impl From<f64> for Double {
    fn from(value: f64) -> Self {
        Self { data: value }
    }
}

impl From<Int32> for Double {
    fn from(value: Int32) -> Self {
        Self {
            data: *value as f64,
        }
    }
}

impl Add for Double {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let result = *self + *rhs;
        Self { data: result }
    }
}

impl Add<Int32> for Double {
    type Output = Int32;

    fn add(self, rhs: Int32) -> Self::Output {
        let result = *self as i32 - *rhs;
        Self::Output::new(result)
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

impl Div for Double {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let result = *self as f64 / *rhs as f64;
        Self { data: result }
    }
}

impl Div<Int32> for Double {
    type Output = Int32;

    fn div(self, rhs: Int32) -> Self::Output {
        let result = *self as i32 / *rhs;
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
