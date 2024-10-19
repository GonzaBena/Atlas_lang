use std::{
    fmt::{Display, Error, Formatter},
    ops::{Add, Div, Mul, Rem, Sub},
    str::FromStr,
};

use crate::error::lexic_errors::LexicError;

// MARK: Number
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Number {
    Int(i64),
    Float(f64),
}

// MARK: Token
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum Token {
    //Basics
    Identifier(String),
    Operand(String),
    Comment(String),
    EOF,
    NewLine,

    // Basic Data Types
    Number(Number),
    Char(char),
    String(String),
    Bool(bool),

    // Puntuation
    StartParenthesis,
    EndParenthesis,
}

// MARK: Number impl
impl Number {
    // General Methods for Token
    pub fn to_string(&self) -> String {
        match self {
            Number::Int(n) => n.to_string(),
            Number::Float(n) => n.to_string(),
        }
    }

    pub fn pow(&self, power: i32) -> Number {
        return match self {
            Number::Int(n) => Number::Int(n.pow(power as u32)),
            Number::Float(n) => Number::Float(n.powi(power)),
        };
    }

    pub fn powf(&self, power: f64) -> Number {
        println!("power: {:?} {:?}", power, self);
        return match self {
            Number::Int(n) => Number::Float((*n as f64).powf(power)),
            Number::Float(n) => Number::Float(n.powf(power)),
        };
    }

    pub fn value_int(&self) -> i64 {
        match self {
            Number::Int(i) => *i,
            Number::Float(f) => *f as i64,
        }
    }

    pub fn value_float(&self) -> f64 {
        match self {
            Number::Int(i) => *i as f64,
            Number::Float(f) => *f,
        }
    }
}

impl FromStr for Number {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.to_string();
        if s.contains(',') {
            s = s.replace(",", ".");
        }
        if s.contains(".") {
            if let Ok(num) = s.parse::<f64>() {
                Ok(Number::Float(num))
            } else {
                Err(format!("Número inválido: {}", s))
            }
        } else if let Ok(num) = s.parse::<i64>() {
            Ok(Number::Int(num))
        } else {
            Err(format!("Invalid Number: {}", s))
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.to_string())
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Number::Int(a + b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a + b),
            (Number::Float(a), Number::Int(b)) => Number::Float(a + b as f64),
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f64 + b),
        }
    }
}

impl Add for Token {
    type Output = Token;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Token::Number(a), Token::Number(b)) => Token::Number(a + b),
            (Token::String(a), Token::String(b)) => Token::String(a + &b),
            (Token::Char(a), Token::Char(b)) => Token::String(a.to_string() + &b.to_string()),
            (Token::Char(a), Token::String(b)) => Token::String(a.to_string() + &b),
            (Token::String(a), Token::Char(b)) => Token::String(a + &b.to_string()),
            (Token::String(a), Token::Number(b)) => Token::String(a + &b.to_string()),
            (Token::Number(a), Token::String(b)) => Token::String(a.to_string() + &b),
            _ => panic!(
                "{}",
                LexicError::OperatorError("Invalid Operation".to_string())
            ),
        }
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Number::Int(a - b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a - b),
            (Number::Float(a), Number::Int(b)) => Number::Float(a - b as f64),
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f64 - b),
        }
    }
}

impl Sub for Token {
    type Output = Token;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Token::Number(a), Token::Number(b)) => Token::Number(a - b),
            (Token::String(a), Token::String(b)) => Token::String(a.replace(&b, "")),
            (Token::String(a), Token::Char(b)) => Token::String(a.replace(b, "")),
            (Token::String(a), Token::Number(b)) => Token::String(a.replace(&b.to_string(), "")),
            (Token::Number(a), Token::String(b)) => {
                if let Ok(num) = Number::from_str(&b) {
                    return Token::Number(a - num);
                } else {
                    panic!(
                        "{}",
                        LexicError::OperatorError("Invalid Operation".to_string())
                    );
                }
            }
            _ => panic!(
                "{}",
                LexicError::OperatorError("Invalid Operation".to_string())
            ),
        }
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Number::Int(a * b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a * b),
            (Number::Float(a), Number::Int(b)) => Number::Float(a * b as f64),
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f64 * b),
        }
    }
}

impl Mul for Token {
    type Output = Token;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Token::Number(a), Token::Number(b)) => Token::Number(a * b),
            (Token::String(a), Token::Number(b)) => {
                return Token::String(a.repeat(b.value_int() as usize));
            }
            (Token::Number(a), Token::String(b)) => {
                if let Ok(num) = Number::from_str(&b) {
                    return Token::Number(a * num);
                } else {
                    return Token::String(b.repeat(a.value_int() as usize));
                }
            }
            _ => panic!(
                "{}",
                LexicError::OperatorError("Invalid Operation".to_string())
            ),
        }
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Number::Float(a as f64 / b as f64),
            (Number::Float(a), Number::Float(b)) => Number::Float(a as f64 / b as f64),
            (Number::Float(a), Number::Int(b)) => Number::Float(a as f64 / b as f64),
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f64 / b as f64),
        }
    }
}

impl Div for Token {
    type Output = Token;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Token::Number(a), Token::Number(b)) => Token::Number(a / b),
            (Token::String(a), Token::String(b)) => {
                return Token::String(a.split(b.as_str()).collect::<String>());
            }
            (Token::String(a), Token::Char(b)) => {
                return Token::String(a.split(b).collect::<String>());
            }
            (Token::String(a), Token::Number(b)) => {
                let mut words: Vec<String> = Vec::new();
                let mut index = 0;
                while index < a.len() {
                    let word = a
                        .chars()
                        .skip(index)
                        .take(b.value_int() as usize)
                        .collect::<String>();
                    words.push(word);
                    index += b.value_int() as usize;
                }
                return Token::String(words.join(" "));
            }
            (Token::Number(a), Token::String(b)) => {
                if let Ok(num) = Number::from_str(&b) {
                    return Token::Number(a / num);
                } else {
                    let mut words: Vec<String> = Vec::new();
                    let mut index = 0;
                    while index < b.len() {
                        let word = b
                            .chars()
                            .skip(index)
                            .take(a.value_int() as usize)
                            .collect::<String>();
                        words.push(word);
                        index += a.value_int() as usize;
                    }
                    return Token::String(words.join(" "));
                }
            }
            _ => panic!(
                "{}",
                LexicError::OperatorError("Invalid Operation".to_string())
            ),
        }
    }
}

impl Rem for Number {
    type Output = Number;

    fn rem(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Number::Int(a % b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a % b),
            (Number::Float(a), Number::Int(b)) => Number::Float(a % b as f64),
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f64 % b),
        }
    }
}

impl Rem for Token {
    type Output = Token;

    fn rem(self, other: Self) -> Self::Output {
        match (self, other) {
            (Token::Number(a), Token::Number(b)) => Token::Number(a % b),
            (Token::String(a), Token::String(b)) => {
                return Token::String(a.split(b.as_str()).last().unwrap().to_string());
            }
            (Token::String(a), Token::Char(b)) => {
                return Token::String(a.split(b).last().unwrap().to_string());
            }
            (Token::String(a), Token::Number(b)) => {
                let mut index = 0;
                let value = b.value_int() as usize;
                while index + value < a.len() {
                    index += value;
                }
                println!("Index: {}", index);
                return Token::String(a.chars().skip(index).collect::<String>());
            }
            (Token::Number(a), Token::String(b)) => {
                if let Ok(num) = Number::from_str(&b) {
                    return Token::Number(a / num);
                } else {
                    let mut index = 0;
                    let value = a.value_int() as usize;
                    println!("Value: {}", b.len());
                    while index + value < b.len() {
                        index += value;
                    }
                    println!("Index: {}", index);

                    return Token::String(b.chars().skip(index).collect::<String>());
                }
            }
            _ => panic!(
                "{}",
                LexicError::OperatorError("Invalid Operation".to_string())
            ),
        }
    }
}

// MARK: Token impl
impl Token {
    // General Methods for Token
    pub fn to_string(&self) -> String {
        match self {
            Token::Identifier(name) => name.clone(),
            Token::Number(value) => value.to_string(),
            Token::Char(c) => format!("'{}'", c.clone()),
            Token::String(s) => format!("\"{}\"", s.clone()),
            Token::Operand(op) => op.clone(),
            Token::StartParenthesis => "(".to_string(),
            Token::EndParenthesis => ")".to_string(),
            Token::Comment(c) => c.clone(),
            Token::EOF => "EOF".to_string(),
            Token::Bool(b) => b.to_string(),
            Token::NewLine => "\n".to_string(),
        }
    }

    pub fn pow(&self, power: Token) -> Token {
        match (self, power) {
            (Token::Number(n), Token::Number(p)) => match p {
                Number::Int(p) => Token::Number(n.pow(p as i32)),
                Number::Float(p) => Token::Number(n.powf(p)),
            },
            _ => panic!(
                "{}",
                LexicError::OperatorError("Invalid Operation".to_string())
            ),
        }
    }

    pub fn floor(&self) -> Token {
        match self {
            Token::Number(Number::Int(n)) => Token::Number(Number::Int(*n)),
            Token::Number(Number::Float(n)) => Token::Number(Number::Int(n.floor() as i64)),
            _ => panic!(
                "{}",
                LexicError::OperatorError("Invalid Operation".to_string())
            ),
        }
    }
}
