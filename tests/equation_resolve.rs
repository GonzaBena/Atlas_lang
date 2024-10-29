use atlas_language::compiler::tokens::token::Number;

fn num1() -> Number {
    Number::new(5)
}

fn num2() -> Number {
    Number::new(2)
}

fn num3() -> Number {
    Number::new(2.5)
}

#[test]
fn test_div() {
    let n1 = num1();
    let n2 = num2();
    let n3 = num3();
    let result1 = n1.clone() / n2.clone();
    let result2 = n1.clone() / n3;
    let div_int = (n1.clone() / n2).floor().value_int();
    let div_zero = n1 / 0;
    assert_eq!(result1.value_float(), 2.5);
    assert_eq!(result2.value_float(), 2.0);
    assert_eq!(div_int, 2);
    assert_eq!(div_zero.value_float(), f64::INFINITY);
}

#[test]
fn test_multi() {
    let n1 = num1();
    let n2 = num2();
    let n3 = num3();
    let result1 = n1.clone() * n2;
    let result2 = n1 * n3;
    assert_eq!(result1.value_int(), 10);
    assert_eq!(result2.value_float(), 12.5);
}

#[test]
fn test_add() {
    let n1 = num1();
    let n2 = num2();
    let n3 = num3();
    let result1 = n1.clone() + n2;
    let result2 = n1 + n3;
    assert_eq!(result1.value_int(), 7);
    assert_eq!(result2.value_float(), 7.5);
}

#[test]
fn test_sub() {
    let n1 = num1();
    let n2 = num2();
    let result = n1 - n2;
    assert_eq!(result.value_int(), 3);
}

#[test]
fn test_pow() {
    let n1 = num1(); // 5
    let n2 = num2(); // 2
    let n3 = num3(); // 2.5
    let result1 = n1.pow(n2);
    let result2 = n1.pow(n3);
    assert_eq!(result1.value_int(), 25);
    assert!(result2.value_float().to_string().starts_with("55.9016"));
}

#[test]
fn test_sqrt() {
    let n2 = num2() * num2(); // 4
    let result1 = n2.pow(0.5);
    assert_eq!(result1.value_float(), num2().value_float());
}

#[test]
fn test_mod() {
    let n1 = num1();
    let n2 = num2();
    let result = n1 % n2;
    assert_eq!(result.value_int(), 1);
}
