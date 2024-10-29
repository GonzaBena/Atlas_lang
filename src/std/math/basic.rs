use std::ops::{Add, Div, Mul, Sub};

pub fn add<S: Add>(a: S, b: S) -> <S as Add>::Output {
    a + b
}

pub fn sub<S: Sub>(a: S, b: S) -> <S as Sub>::Output {
    a - b
}

pub fn mul<S: Mul>(a: S, b: S) -> <S as Mul>::Output {
    a * b
}

pub fn div<S: Div>(a: S, b: S) -> <S as Div>::Output {
    a / b
}

pub fn pow<S: Mul<Output = S> + Copy>(a: S, b: u32) -> S {
    let mut result = a;
    for _ in 1..b {
        result = result * a;
    }
    result
}

#[cfg(test)]
mod basics_math_tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(2, 1), 1);
    }

    #[test]
    fn test_mul() {
        assert_eq!(mul(2, 3), 6);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(6, 3), 2);
    }

    #[test]
    fn test_pow() {
        assert_eq!(pow(2, 3), 8);
    }
}
