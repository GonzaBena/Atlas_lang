use std::{
    fmt::{Debug, Display},
    iter::FromIterator,
    ops::{Add, AddAssign, Index, IndexMut, Mul, Sub},
    result::Result,
    slice::{Iter, IterMut},
    vec::Vec as StdVec,
};

use super::super::{
    collections::array::Array, interfaces::collections::array::Join, types::string::String,
};
use crate::{error::collection_errors::CollectionErrors, std::interfaces::types::string::ToString};

#[derive(Debug, Clone)]
pub struct Vec<T>
where
    T: PartialEq,
{
    pub data: StdVec<T>,
}

impl<T: PartialEq> PartialEq for Vec<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T> Index<usize> for Vec<T>
where
    T: PartialEq,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Vec<T>
where
    T: PartialEq,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Default for Vec<i32> {
    fn default() -> Self {
        Vec::new()
    }
}

impl<T> Add for Vec<T>
where
    T: PartialEq,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec {
            data: self
                .data
                .into_iter()
                .chain(other.data.into_iter())
                .collect(),
        }
    }
}

impl<T> AddAssign for Vec<T>
where
    T: PartialEq,
{
    fn add_assign(&mut self, other: Self) {
        self.data.extend(other.data);
    }
}

impl<T> Sub for Vec<T>
where
    T: PartialEq,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec {
            data: self
                .data
                .into_iter()
                .filter(|item| !other.data.contains(item))
                .collect(),
        }
    }
}

impl<T> Mul for Vec<T>
where
    T: PartialEq + Clone,
{
    type Output = Self;

    /// #### Multiplies `self` and `v`.
    /// Create a merge of the two vectors.
    fn mul(self, v: Self) -> Self {
        let mut new_vec = Vec::new();
        let length1 = self.len();
        let length2 = v.len();
        for i in 0..length1 {
            for j in 0..length2 {
                new_vec.push(self.data[i].clone());
                new_vec.push(v.data[j].clone());
            }
        }
        new_vec
    }
}

impl<T: Display + ToString> Display for Vec<T>
where
    T: PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joined = self
            .data
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
            .join(String::from(", "));
        write!(f, "[{}]", joined)
    }
}

impl<T: Clone> FromIterator<T> for Vec<T>
where
    T: PartialEq,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Vec::new();
        for c in iter {
            vec.push(c);
        }

        vec
    }
}

impl<'a, T: Clone> From<&'a Array<'a, T>> for Vec<T>
where
    T: PartialEq,
{
    /// Convierte una referencia a Array en una instancia de Vec
    fn from(array: &'a Array<'a, T>) -> Self {
        Vec {
            data: array.data[..array.length].to_vec(),
        }
    }
}

impl<T: Display + Clone> Join<String> for Vec<T>
where
    T: PartialEq,
    String: From<T> + From<usize>,
{
    fn join(&self, separator: String) -> String {
        let mut result = String::new();
        for (index, value) in self.data.iter().enumerate() {
            result = result + String::from(value.clone());
            if index < (self.len() - 1) as usize {
                result = result + separator.clone();
            }
        }
        result
    }
}

impl<T: Clone> Vec<T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        Self {
            data: StdVec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn set(&mut self, index: usize, value: T) -> std::result::Result<(), CollectionErrors> {
        if index >= self.len() {
            return Err(CollectionErrors::IndexOutOfBounds(format!(
                "the len of the array is {} and you tried to access index {}",
                self.len(),
                index
            )));
        }

        self.data[index] = value;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Result<&T, CollectionErrors> {
        if index >= self.len() {
            return Err(CollectionErrors::IndexOutOfBounds(format!(
                "the len of the array is {} and you tried to access index {}",
                self.len(),
                index
            )));
        }

        Ok(&self.data[index])
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    pub fn to_array(&self) -> Array<'_, T>
    where
        T: PartialEq + Default,
    {
        Array::from(self.data.clone())
    }
}

#[cfg(test)]
mod vector_tests {
    use super::*;

    #[test]
    fn test_new() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_push() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_pop() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_set() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.set(1, 4).unwrap();
        assert_eq!(vec.get(1).unwrap(), &4);
    }

    #[test]
    fn test_get() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.get(1).unwrap(), &2);
    }

    #[test]
    fn test_display() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.join(String::from(", ")), "[1, 2, 3]");
    }
}
