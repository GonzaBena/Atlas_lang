use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::{Index, IndexMut};
use std::slice::IterMut;
use std::usize;

use crate::error::collection_errors::CollectionErrors;
use crate::std::collections::{iterator::Iter, vector::Vec};
use crate::std::interfaces::types::string::ToString;
use crate::std::types::string::String;

#[derive(Debug, PartialEq)]
pub struct Array<'a, T> {
    pub(in crate::std::collections) data: &'a mut [T],
    pub(in crate::std::collections) length: usize,
    pub(in crate::std::collections) capacity: usize,
}

impl<'a, T> Clone for Array<'a, T>
where
    T: Default + PartialEq + Clone + Copy,
{
    fn clone(&self) -> Self {
        let mut new_data = Vec::new();
        for i in 0..self.len() {
            new_data.push(self[i].clone());
        }

        Array::from(new_data)
    }
}

impl<'a, T: Default + PartialEq + Clone> From<&'a mut [T]> for Array<'a, T> {
    fn from(data: &'a mut [T]) -> Self {
        // capacity is the maximum number of elements that can be stored in the array
        let capacity = data.len();

        // length is the number of elements currently stored in the array
        let mut length = 0;
        for i in 0..data.len() {
            if data[i] != Default::default() {
                length += 1;
            }
        }

        Self {
            data,
            length,
            capacity,
        }
    }
}

impl<'a, T: Default + PartialEq + Clone, const N: usize> From<&'a mut [T; N]> for Array<'a, T> {
    fn from(data: &'a mut [T; N]) -> Self {
        // Convert the fixed-size array to a mutable slice
        let slice: &'a mut [T] = data.as_mut_slice();

        // Use the existing implementation for slices
        Array::from(slice)
    }
}

// from Vector to Array
impl<'a, T> From<Vec<T>> for Array<'a, T>
where
    T: Default + PartialEq + Clone + Copy,
{
    fn from(vector: Vec<T>) -> Self {
        let boxed_slice = vector.data.into_boxed_slice();
        let slice = Box::leak(boxed_slice);
        let length = slice.len();
        let capacity = slice.len();

        Self {
            data: slice,
            length,
            capacity,
        }
    }
}

impl<T: Default + PartialEq + Clone> From<std::vec::Vec<T>> for Array<'_, T> {
    fn from(vector: std::vec::Vec<T>) -> Self {
        let boxed_slice = vector.into_boxed_slice();
        let slice = Box::leak(boxed_slice);
        let length = slice.len();
        let capacity = slice.len();

        Self {
            data: slice,
            length,
            capacity,
        }
    }
}

impl<'a, T> Index<usize> for Array<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<'a, T> IndexMut<usize> for Array<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Default + Clone> Default for Array<'_, T> {
    fn default() -> Self {
        Self {
            data: vec![].leak(),
            length: 0,
            capacity: 0,
        }
    }
}

impl<T: Debug> Display for Array<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<'a, T: Copy + Default + PartialEq> Add for Array<'a, T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new_array = Array::<'a, T>::new(self.len() + other.len());
        for i in 0..self.len() {
            new_array.set(i, self[i]).unwrap();
        }
        for i in 0..other.len() {
            new_array.set(i + self.len(), other[i]).unwrap();
        }

        new_array
    }
}

impl ToString for usize {
    fn to_string(&self) -> String {
        String::from(format!("{}", self))
    }
}

impl<'a, T: Clone + PartialEq> Array<'a, T> {
    pub fn new(size: usize) -> Self
    where
        T: Default,
    {
        let mut vec = vec![];
        for _ in 0..size {
            vec.push(Default::default());
        }
        Self {
            data: vec.leak(),
            length: 0,
            capacity: size,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            return Some(&self[index]);
        }

        return None;
    }

    pub fn set(&mut self, index: usize, value: T) -> Result<(), CollectionErrors> {
        if index < self.len() {
            self.data[index] = value;
            self.length += 1;
            return Ok(());
        }

        return Err(CollectionErrors::IndexOutOfBounds(format!(
            "the len of the array is {} and you tried to access index {}",
            self.len(),
            index
        )));
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Default + Clone + PartialEq + Copy,
    {
        let mut vec = Vec::new();
        for i in 0..self.len() {
            vec.push(self[i].clone());
        }
        vec
    }

    pub fn iter(&'a self) -> Iter<'a, T>
    where
        T: Clone + PartialEq + Default + Copy,
    {
        Iter::from(self.clone())
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a, T> {
        self.data.iter_mut()
    }

    pub fn join(&self, separator: &String) -> String
    where
        T: ToString + Default + Copy,
    {
        let mut result = String::new();
        for (i, item) in self.iter().enumerate() {
            result.push_str(&item.to_string());
            if i < self.len() - 1 {
                result.push_str(separator);
            }
        }

        result
    }

    /// Returns a new array with the elements that satisfy the predicate
    /// The predicate is a function that takes an element and returns a boolean
    /// For example:
    /// ```rust
    /// let array = Array::from(&mut [1, 2, 3, 4, 5]);
    /// let new_array = array.filter(|&x| x % 2 == 0);
    ///
    /// assert_eq!(new_array.len(), 2);
    /// assert_eq!(new_array.get(0), Some(&2));
    ///
    /// ```
    pub fn filter(&self, predicate: fn(&T) -> bool) -> Self
    where
        T: Clone + Default,
    {
        let mut new_array = Array::new(self.len());
        for i in 0..self.len() {
            if predicate(&self[i]) {
                new_array.set(new_array.len(), self[i].clone()).unwrap();
            }
        }

        new_array
    }

    // pub fn map<U: Clone + Default + PartialEq + ToString + Into<String>, F>(
    //     &self,
    //     mapper: F,
    // ) -> Array<'a, U>
    // where
    //     F: FnMut(T) -> U,
    // {
    //     let new_data: Vec<U> = self.data.iter().cloned().map(mapper).collect();

    //     Array::from(new_data)
    // }
}

#[cfg(test)]
mod array_tests {
    use super::*;
    use crate::error::collection_errors::CollectionErrors::IndexOutOfBounds;

    #[test]
    fn test_array_new() {
        let array = Array::<i32>::new(10);
        let mut elems = 0;
        for i in array.iter() {
            if i != i32::default() {
                elems += 1;
            }
        }

        // capacity
        assert_eq!(array.len(), 10);
        // length
        assert_eq!(elems, 0);
    }

    #[test]
    fn test_array_from() {
        let mut data = [1, 2, 3, 4, 5];
        let array = Array::from(&mut data[..]);
        let mut elems = 0;
        for i in array.iter() {
            if i != i32::default() {
                elems += 1;
            }
        }

        // capacity
        assert_eq!(array.len(), 5);
        // length
        assert_eq!(elems, 5);
    }

    #[test]
    fn test_array_get() {
        let mut data = [1, 2, 3, 4, 5];
        let array = Array::from(&mut data);

        assert_eq!(array.get(0), Some(&1));
        assert_eq!(array.get(1), Some(&2));
        assert_eq!(array.get(2), Some(&3));
        assert_eq!(array.get(3), Some(&4));
        assert_eq!(array.get(4), Some(&5));
        assert_eq!(array.get(5), None);
    }

    #[test]
    fn test_array_set() {
        let mut data = [1, 2, 3, 4, 5];
        let mut array = Array::from(&mut data);

        assert_eq!(array.set(0, 10), Ok(()));
        assert_eq!(array.set(1, 20), Ok(()));
        assert_eq!(array.set(2, 30), Ok(()));
        assert_eq!(array.set(3, 40), Ok(()));
        assert_eq!(array.set(4, 50), Ok(()));
        assert_eq!(
            array.set(5, 60),
            Err(IndexOutOfBounds(
                "the len of the array is 5 and you tried to access index 5".to_string()
            ))
        );

        assert_eq!(array.get(0), Some(&10));
        assert_eq!(array.get(1), Some(&20));
        assert_eq!(array.get(2), Some(&30));
        assert_eq!(array.get(3), Some(&40));
        assert_eq!(array.get(4), Some(&50));
    }

    #[test]
    fn test_array_clone() {
        let mut data = [1, 2, 3, 4, 5];
        let array = Array::from(&mut data);
        let cloned_array = array.clone();

        assert_eq!(array, cloned_array);
    }

    #[test]
    fn test_array_to_vec() {
        let mut data = [1, 2, 3, 4, 5];
        let array = Array::from(&mut data);
        let vec = array.to_vec();

        assert_eq!(
            vec,
            Vec {
                data: vec![1, 2, 3, 4, 5]
            }
        );
    }

    #[test]
    fn test_array_join() {
        let mut data = [1, 2, 3, 4, 5];
        let array = Array::from(&mut data);
        let joined = array.join(&String::from(", "));

        assert_eq!(joined, "1, 2, 3, 4, 5");
    }
}
