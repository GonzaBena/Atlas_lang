use atlas_language::compiler::ast::AST;
use atlas_language::compiler::parser::Parser;
use atlas_language::compiler::token::Token;

#[test]
fn test_div() {
    let ast = AST::from_expression("5/2").unwrap();
    let binding = ast.expresion().clone();
    let mut parser = Parser::new(&binding);
    let eq_result = parser.parse().resolve();
    match eq_result {
        Token::Number(n) => assert_eq!(n.value_float(), 2.5),
        _ => panic!("Error en el resultado"),
    }
}

#[test]
fn test_multi() {
    let ast = AST::from_expression("5 * 2").unwrap();
    let binding = ast.expresion().clone();
    let mut parser = Parser::new(&binding);
    let eq_result = parser.parse().resolve();
    match eq_result {
        Token::Number(n) => assert_eq!(n.value_int(), 10),
        _ => panic!("Error en el resultado"),
    }
}

#[test]
fn test_add() {
    let ast = AST::from_expression("5 + 2").unwrap();
    let binding = ast.expresion().clone();
    let mut parser = Parser::new(&binding);
    let eq_result = parser.parse().resolve();
    match eq_result {
        Token::Number(n) => assert_eq!(n.value_int(), 7),
        _ => panic!("Error en el resultado"),
    }
}

#[test]
fn test_sub() {
    let ast = AST::from_expression("5 - 2").unwrap();
    let binding = ast.expresion().clone();
    let mut parser = Parser::new(&binding);
    let eq_result = parser.parse().resolve();
    match eq_result {
        Token::Number(n) => assert_eq!(n.value_int(), 3),
        _ => panic!("Error en el resultado"),
    }
}

#[test]
fn test_pow() {
    let ast = AST::from_expression("5 ** 2").unwrap();
    let binding = ast.expresion().clone();
    let mut parser = Parser::new(&binding);
    let eq_result = parser.parse().resolve();
    match eq_result {
        Token::Number(n) => assert_eq!(n.value_int(), 25),
        _ => panic!("Error en el resultado"),
    }
}

#[test]
fn test_sqrt() {
    let ast = AST::from_expression("4 ** 0.5").unwrap();
    let binding = ast.expresion().clone();
    let mut parser = Parser::new(&binding);
    let eq_result = parser.parse().resolve();
    println!("Resultado: {} {:?}", eq_result, binding);
    match eq_result {
        Token::Number(n) => assert_eq!(n.value_int(), 2),
        _ => panic!("Error en el resultado"),
    }
}
