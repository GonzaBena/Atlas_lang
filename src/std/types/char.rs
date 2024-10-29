use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use crate::std::interfaces::types::string::ToString;
use crate::std::types::string::String;

#[derive(Debug, Clone, Copy)]
pub struct Char {
    pub data: u8,
}

impl Default for Char {
    fn default() -> Self {
        Self::new(b'\0')
    }
}

impl PartialEq for Char {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl From<u8> for Char {
    fn from(data: u8) -> Self {
        Self { data }
    }
}

impl From<char> for Char {
    fn from(data: char) -> Self {
        Self { data: data as u8 }
    }
}

impl From<&str> for Char {
    fn from(data: &str) -> Self {
        Self {
            data: data.as_bytes()[0],
        }
    }
}

impl From<Char> for u8 {
    fn from(data: Char) -> Self {
        data.data
    }
}

impl Deref for Char {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Char {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl ToString for Char {
    fn to_string(&self) -> String {
        self.to_string()
    }
}

impl Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Char {
    pub fn new(data: u8) -> Self {
        Self { data }
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        string.push(self.clone());
        string
    }

    pub fn is_digit(&self) -> bool {
        self.data.is_ascii_digit()
    }

    pub fn is_alphabetic(&self) -> bool {
        self.data.is_ascii_alphabetic()
    }

    pub fn is_alphanumeric(&self) -> bool {
        self.data.is_ascii_alphanumeric()
    }

    pub fn is_puntuation(&self) -> bool {
        self.data.is_ascii_punctuation()
    }

    pub fn is_whitespace(&self) -> bool {
        self.data.is_ascii_whitespace()
    }

    pub fn is_uppercase(&self) -> bool {
        self.data.is_ascii_uppercase()
    }

    pub fn is_lowercase(&self) -> bool {
        self.data.is_ascii_lowercase()
    }

    pub fn to_uppercase(&self) -> Self {
        Self::from(self.data.to_ascii_uppercase())
    }

    pub fn to_lowercase(&self) -> Self {
        Self::from(self.data.to_ascii_lowercase())
    }

    pub fn to_ascii_uppercase(&self) -> Self {
        Self::from(self.data.to_ascii_uppercase())
    }

    pub fn to_ascii_lowercase(&self) -> Self {
        Self::from(self.data.to_ascii_lowercase())
    }

    pub fn to_ascii_digit(&self) -> Option<Self> {
        if self.is_digit() {
            Some(self.clone())
        } else {
            None
        }
    }

    pub fn to_ascii_alphabetic(&self) -> Option<Self> {
        if self.is_alphabetic() {
            Some(self.clone())
        } else {
            None
        }
    }

    pub fn to_ascii_alphanumeric(&self) -> Option<Self> {
        if self.is_alphanumeric() {
            Some(self.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod char_tests {
    use super::*;

    #[test]
    fn test_char_default() {
        let char = Char::default();
        assert_eq!(char, Char::new(b'\0'));
    }

    #[test]
    fn test_char_from() {
        let char = Char::from(b'a');
        assert_eq!(char, Char { data: b'a' });
    }

    #[test]
    fn test_char_deref() {
        let char = Char::from(b'a');
        assert_eq!(*char, b'a');
    }

    #[test]
    fn test_char_deref_mut() {
        let mut char = Char::from(b'a');
        *char = b'b';
        assert_eq!(*char, b'b');
    }

    #[test]
    fn test_char_to_string() {
        let char = Char::from(b'a');
        assert_eq!(char.to_string(), String::from("a"));
    }

    #[test]
    fn test_char_display() {
        let char = Char::from(b'a');
        assert_eq!(format!("{}", char), "97");
    }
}
