use std::fmt;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, MulAssign, Rem, Sub};

use super::double::Double;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Int32 {
    data: i32,
}

impl Int32 {
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

impl From<i32> for Int32 {
    fn from(value: i32) -> Self {
        Self { data: value }
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

impl MulAssign for Int32 {
    fn mul_assign(&mut self, rhs: Self) {
        self.data *= *rhs
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
