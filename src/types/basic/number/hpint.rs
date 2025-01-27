use std::fmt;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, MulAssign, Rem, Sub, SubAssign};

use super::double::Double;
use super::int32::Int32;
use super::Number;
use num::ToPrimitive;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HPInt {
    data: i128,
}

impl HPInt {
    pub fn new(num: i128) -> Self {
        Self { data: num }
    }
}

impl Deref for HPInt {
    type Target = i128;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for HPInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl fmt::Display for HPInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl ToPrimitive for HPInt {
    fn to_i64(&self) -> Option<i64> {
        self.data.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.data.to_u64()
    }

    fn to_f64(&self) -> Option<f64> {
        self.data.to_f64()
    }

    fn to_i128(&self) -> Option<i128> {
        self.data.to_i128()
    }
}

impl Number for HPInt {
    type Pow = Self;
    type Output = Self;

    fn add<T: Number>(&self, other: T) -> Self::Output {
        Self::Output::new(self.data + other.to_i128().unwrap_or_default())
    }

    fn sub<T: Number>(&self, other: T) -> Self::Output {
        Self::Output::new(self.data - other.to_i128().unwrap_or_default())
    }

    fn mul<T: Number>(&self, other: T) -> Self::Output {
        todo!()
    }

    fn div<T: Number>(&self, other: T) -> Self::Output {
        todo!()
    }

    fn module<T: Number>(&self, other: T) -> Self::Output {
        todo!()
    }

    fn abs<T: Number>(&self) -> Self::Output {
        todo!()
    }

    fn power<T: Into<i32> + PartialOrd<i32> + Clone>(&self, other: T) -> Self::Pow {
        todo!()
    }
}

impl<T> From<T> for HPInt
where
    T: ToPrimitive + Into<i128>,
{
    fn from(value: T) -> Self {
        let val = T::to_i128(&value).unwrap();
        Self { data: val }
    }
}

// impl<T: Number> Add<T> for HPInt {
//     type Output = Self;

//     fn add(self, rhs: T) -> Self::Output {
//         let result = Number::add(&self, rhs);
//         result
//     }
// }

impl<T> Add<T> for HPInt
where
    T: Number + Into<f64>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let result = Number::add(&self, rhs);
        result
    }
}

impl<T: Number> AddAssign<T> for HPInt {
    fn add_assign(&mut self, rhs: T) {
        *self = Number::add(&*self, rhs);
    }
}

impl<T: Number> Sub<T> for HPInt {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let result = Number::sub(&self, rhs);
        result
    }
}

impl<T: Number> SubAssign<T> for HPInt {
    fn sub_assign(&mut self, rhs: T) {
        *self = Number::sub(&*self, rhs);
    }
}

impl<T: Number> Mul<T> for HPInt {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let result = Number::mul(&self, rhs);
        result
    }
}

impl<T: Number> MulAssign<T> for HPInt {
    fn mul_assign(&mut self, rhs: T) {
        *self = Number::mul(&*self, rhs);
    }
}

impl<T: Number> Div<T> for HPInt {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let result = Number::div(&self, rhs);
        result
    }
}

impl Rem for HPInt {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output::new(*self % *rhs)
    }
}

impl Rem<Double> for HPInt {
    type Output = Self;
    fn rem(self, rhs: Double) -> Self::Output {
        Self::Output::new(*self % *rhs as i128)
    }
}

impl Rem<Int32> for HPInt {
    type Output = Self;
    fn rem(self, rhs: Int32) -> Self::Output {
        Self::Output::new(*self % *rhs as i128)
    }
}

impl PartialOrd<i128> for HPInt {
    fn partial_cmp(&self, other: &i128) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(other)
    }
}

impl PartialEq<i128> for HPInt {
    fn eq(&self, other: &i128) -> bool {
        self.data == *other
    }
}
