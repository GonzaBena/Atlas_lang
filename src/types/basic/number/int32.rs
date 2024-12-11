use std::fmt;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy)]
pub struct Int32 {
    data: i32,
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
