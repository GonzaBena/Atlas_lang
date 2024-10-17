mod compiler;
mod utils;

use compiler::ast::{Token, AST};
use compiler::parser::Parser;
fn main() {
    let ast = AST::from_expression("1-5/2").unwrap();
    println!("Resultado: {:?}", ast.expresion());

    let binding = ast.expresion().clone();
    let mut parser = Parser::new(&binding);
    let result = parser.parse();
    let eq_result = result.resolve();
    println!("Resultado: {}", eq_result);

    let string = Token::Identifier("hola".to_string());
    println!("String: {}", string.to_string());
}
