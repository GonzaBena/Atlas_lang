use std::{cell::RefCell, rc::Rc};

use atlas_lang::compiler::{elements::token::Token, lexer::Lexer, parser::Parser};

#[test]
fn create_function_test() {
    let code = "
    func hello(hola: Int32 = 10) {
        hola + 10
    }
    "
    .trim();

    let mut lex = Lexer::new(code);
    let tokens = lex.lex();
    println!("{tokens:?}");
    let mut parser = Parser::new(tokens, None, None);
    let parse = parser.parse();

    println!("\nparse: {parse:?}");

    for (name, func) in parser.get_functions() {
        if name == "hello" {
            let result = func.call(
                vec![],
                Rc::new(RefCell::new(parser.get_variable_table())),
                Rc::new(RefCell::new(parser.get_function_table())),
            );
            println!("result: {result:?}");
            assert_eq!(result.unwrap(), Token::Void);
        }
    }
}

#[test]
fn std_function_test() {
    let code = "
    func hello() {
        print(\"hola mundo\")
    }
    "
    .trim();

    let mut lex = Lexer::new(code);
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens, None, None);
    let _ = parser.parse();

    for (name, func) in parser.get_functions() {
        if name == "hello" {
            let result = func.call(
                vec![],
                Rc::new(RefCell::new(parser.get_variable_table())),
                Rc::new(RefCell::new(parser.get_function_table())),
            );
            println!("result: {result:?}");
            assert_eq!(result.unwrap(), Token::Void);
        }
    }
}
