use std::{
    fmt::{Display, Error, Formatter},
    ops::{Add, Div, Mul, Rem, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

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

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Token {
    Identifier(String),
    Number(Number),
    Char(char),
    Operand(String),
    StartParenthesis,
    EndParenthesis,
    String(String),
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
            _ => panic!("Invalid Operation"),
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
            _ => panic!("Invalid Operation"),
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
            _ => panic!("Invalid Operation"),
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
            _ => panic!("Invalid Operation"),
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
            _ => panic!("Invalid Operation"),
        }
    }
}

impl Token {
    // General Methods for Token
    pub fn to_string(&self) -> String {
        match self {
            Token::Identifier(name) => name.clone(),
            Token::Number(value) => value.to_string(),
            Token::Char(c) => c.to_string(),
            Token::String(s) => s.clone(),
            Token::Operand(op) => op.clone(),
            Token::StartParenthesis => "(".to_string(),
            Token::EndParenthesis => ")".to_string(),
        }
    }

    pub fn pow(&self, power: Token) -> Token {
        match (self, power) {
            (Token::Number(n), Token::Number(p)) => match p {
                Number::Int(p) => Token::Number(n.pow(p as i32)),
                Number::Float(p) => Token::Number(n.powf(p)),
            },
            _ => panic!("Invalid Operation"),
        }
    }

    pub fn floor(&self) -> Token {
        match self {
            Token::Number(Number::Int(n)) => Token::Number(Number::Int(*n)),
            Token::Number(Number::Float(n)) => Token::Number(Number::Int(n.floor() as i64)),
            _ => panic!("Invalid Operation"),
        }
    }
}
