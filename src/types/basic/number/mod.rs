use double::Double;
use num::ToPrimitive;

pub mod double;
pub mod float;
pub mod hpint;
pub mod int32;
pub mod int64;

#[allow(dead_code)]
pub(crate) trait Number: Sized + ToPrimitive {
    type Pow;
    type Output;

    fn add<T: Number>(&self, other: T) -> Self::Output;
    fn sub<T: Number>(&self, other: T) -> Self::Output;
    fn mul<T: Number>(&self, other: T) -> Self::Output;
    fn div<T: Number>(&self, other: T) -> Double;
    fn module<T: Number>(&self, other: T) -> Self::Output;
    fn abs<T: Number>(&self) -> Self::Output;
    fn power<T: Into<i32> + PartialOrd<i32> + Clone>(&self, other: T) -> Self::Pow;
}
