use std::{
    fmt::Display,
    num::ParseIntError,
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use super::super::interfaces::types::string::ToString;
use crate::std::collections::{array::Array, iterator::Iter, vector::Vec};

use super::char::Char;

#[derive(Debug, Clone)]
pub struct String {
    pub data: Vec<Char>,
}

impl Default for String {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_string: String = self.data.iter().map(|c| c.to_string()).collect();
        write!(f, "\"{}\"", formatted_string)
    }
}

impl ToString for String {
    fn to_string(&self) -> Self {
        self.clone()
    }
}

impl ToString for &str {
    fn to_string(&self) -> String {
        String::from(*self)
    }
}

impl ToString for &String {
    fn to_string(&self) -> String {
        (*self).clone()
    }
}

impl<T: ToString> FromIterator<T> for String {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut data = String::new();
        for item in iter {
            let str = item.to_string();
            for c in str.chars() {
                data.push(c.clone());
            }
        }
        Self {
            data: data.chars().collect(),
        }
    }
}

impl<T: ToString> From<(usize, &T)> for String {
    fn from(data: (usize, &T)) -> Self {
        let mut string = String::new();
        for c in data.1.to_string().chars() {
            string.push(c.clone());
        }
        string
    }
}

impl<T: ToString> From<&T> for String {
    fn from(data: &T) -> Self {
        let mut string = String::new();
        for c in data.to_string().chars() {
            string.push(c.clone());
        }
        string
    }
}

impl From<usize> for String {
    fn from(data: usize) -> Self {
        let mut string = String::new();
        for c in ToString::to_string(&data).chars() {
            string.push(c);
        }
        string
    }
}

impl From<&str> for String {
    fn from(data: &str) -> Self {
        Self::from(data.to_string())
    }
}

impl From<std::string::String> for String {
    fn from(s: std::string::String) -> Self {
        let mut string = String::new();
        for c in s.chars() {
            string.push(Char::new(c as u8));
        }
        string
    }
}

impl From<Vec<Char>> for String {
    fn from(v: Vec<Char>) -> Self {
        Self { data: v }
    }
}

// MARK: Math Operations
impl Add for String {
    type Output = Self;

    /// #### Concatenates `self` and `other`.
    /// ##### Transform the `other` into a `String` and append it to `self`.
    /// For Example:
    /// ```rust
    ///     use atlas_language::std::types::string::String;
    ///
    ///     let string = String::from("Hello, ");
    ///     let other = String::from("World!");
    ///     let new_string = string + other;
    ///
    ///     assert_eq!(new_string, String::from("Hello, World!"));
    /// ```
    fn add(self, other: Self) -> Self {
        Self {
            data: self.data + other.data,
        }
    }
}

impl<T: Into<std::string::String> + ToString> Add<T> for String {
    type Output = Self;

    fn add(self, other: T) -> Self {
        let data = self.data + other.to_string().data;
        Self { data }
    }
}

impl Sub for String {
    type Output = Self;

    /// #### Removes all occurrences of `other` from `self`.
    /// For Example:
    /// ```rust
    ///     use atlas_language::std::types::string::String;
    ///
    ///     let mut string = String::from("Hello, World!");
    ///     string = string - "l";
    ///     assert_eq!(string, String::from("Heo, Word!"));
    /// ```
    fn sub(self, other: Self) -> Self {
        let data = self.data - other.data;
        Self { data }
    }
}

impl<T: Into<std::string::String> + ToString> Sub<T> for String {
    type Output = Self;

    fn sub(self, other: T) -> Self {
        let data = self.data - other.to_string().data;
        Self { data }
    }
}

impl<T: ToString> Mul<T> for String {
    type Output = Self;

    fn mul(self, n: T) -> Self {
        let mut new_string = self.clone();
        // verify if n is a valid number
        if let Ok(n) = n.to_string().parse::<usize>() {
            for _ in 1..n {
                new_string.data += self.data.clone();
            }
        } else {
            panic!(
                "Invalid number: {}, You can't operate a String with this type",
                n.to_string()
            );
        }

        new_string
    }
}

impl<T: ToString + std::string::ToString> PartialEq<T> for String {
    fn eq(&self, other: &T) -> bool {
        self.data == String::from(other).data
    }
}

impl PartialEq<std::string::String> for String {
    fn eq(&self, other: &std::string::String) -> bool {
        self == other
    }
}

impl String {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, c: Char) {
        self.data.push(c);
    }

    pub fn push_str(&mut self, c: &String) {
        for i in 0..c.len() {
            self.data.push(c.data[i].clone());
        }
    }

    pub fn parse<T: FromStr>(&self) -> Result<T, ParseIntError>
    where
        String: From<T>,
        T: FromStr<Err = std::num::ParseIntError>,
    {
        let mut number = String::new();
        let mut all_numbers = true;
        for c in self.chars() {
            if !c.is_digit() {
                all_numbers = false;
            }
            number.push(c.clone());
        }

        let mut result = number.clone();
        if all_numbers {
            result = String::from(number.to_std_string().parse::<T>().unwrap());
        }
        result.to_std_string().parse::<T>()
    }

    pub fn pop(&mut self) -> Option<Char> {
        self.data.pop()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn to_std_string(&self) -> std::string::String {
        let mut string = std::string::String::new();
        for c in self.chars() {
            string.push(c.data as char);
        }
        string
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn chars<'a>(&self) -> Iter<'a, Char> {
        let mut chars: Array<'a, Char> = Array::new(self.data.len());
        for (i, c) in self.data.iter().enumerate() {
            chars[i] = Char::from(**c);
        }
        chars.iter().collect()
    }

    pub fn to_string(&self) -> String {
        self.clone()
    }
}

#[cfg(test)]
mod string_tests {
    use super::*;

    #[test]
    fn test_string_new() {
        let string = String::new();
        assert_eq!(string.len(), 0);
    }

    #[test]
    fn test_string_push() {
        let mut string = String::new();
        string.push(Char::from('a' as u8));
        assert_eq!(string.len(), 1);
    }

    #[test]
    fn test_string_push_str() {
        let mut string = String::new();
        let other = String::from("Hello, World!");
        string.push_str(&other);
        assert_eq!(string.len(), 13);
    }

    #[test]
    fn test_string_pop() {
        let mut string = String::from("Hello, World!");
        let c = string.pop();
        assert_eq!(c, Some(Char::from('!')));
    }

    #[test]
    fn test_string_len() {
        let string = String::from("Hello, World!");
        assert_eq!(string.len(), 13);
    }

    #[test]
    fn test_string_is_empty() {
        let string = String::new();
        assert_eq!(string.is_empty(), true);
    }

    #[test]
    fn test_string_to_std_string() {
        let string = String::from("Hello, World!");
        assert_eq!(string.to_std_string(), "Hello, World!".to_string());
    }

    #[test]
    fn test_string_clear() {
        let mut string = String::from("Hello, World!");
        string.clear();
        assert_eq!(string.len(), 0);
    }

    #[test]
    fn test_string_chars() {
        let string = String::from("Hello, World!");
        let chars = string.chars();
        assert_eq!(chars.len(), 13);
    }

    #[test]
    fn test_string_to_string() {
        let string = String::from("Hello, World!");
        assert_eq!(string.to_string(), string);
    }

    #[test]
    fn test_string_add() {
        let string = String::from("Hello, ");
        let other = String::from("World!");
        let new_string = string + other;
        assert_eq!(new_string, String::from("Hello, World!"));
    }

    #[test]
    fn test_string_sub() {
        let mut string = String::from("Hello, World!");
        string = string - "l";
        assert_eq!(string, String::from("Heo, Word!"));
    }

    #[test]
    fn test_string_mul() {
        let string = String::from("Hello, ");
        let new_string = string * 3;
        assert_eq!(new_string, String::from("Hello, Hello, Hello, "));
    }

    #[test]
    fn test_string_eq() {
        let string = String::from("Hello, World!");
        assert_eq!(string, "Hello, World!");
    }

    #[test]
    fn test_string_parse() {
        let string = String::from("123");
        let number: usize = string.parse().unwrap();
        assert_eq!(number, 123);
    }

    #[test]
    fn test_string_from_iter() {
        let string = String::from_iter(vec!["Hello", ", ", "World!"]);
        assert_eq!(string, "Hello, World!");
    }
}
