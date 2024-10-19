mod cli;
mod compiler;
mod error;
mod utils;

use clap::Parser;
use cli::Args;
use compiler::lexer::Lexer;
use compiler::parser::Parser as Pars;
use utils::file_handling::File;
fn main() {
    let args = Args::parse();

    println!("Args: {:?}", args);

    // print my actual path
    // let path = std::env::current_dir().unwrap();
    // println!("The current directory is {}", path.display());

    let file = File::open("./prueba/index.atlas").unwrap();
    let content = file.get_content();
    // println!("Contenido: {}", content);
    let mut lexer = Lexer::new(&content);
    let tokens = lexer.tokenizer();
    println!("Tokens: {:#?}", tokens);

    let mut parser = Pars::new(&tokens);
    let result = parser.parse();
    // let eq_result = result.resolve();
    println!("Resultado: {:#?}\n", result);

    // for line in content.lines() {
    //     if line.trim().is_empty() {
    //         continue;
    //     }
    //     let ast = AST::from_expression(line).unwrap();

    //     let binding = ast.expresion().clone();
    //     println!("Expresión: {:?}", binding);
    //     let mut parser = parser::Parser::new(&binding);
    //     let result = parser.parse();
    //     let eq_result = result.resolve();

    //     println!("Resultado: {}\n", eq_result);
    //     // match result {
    //     //     Op::Operation(o) => {
    //     //         println!("Operación válida: {}", o.is_valid());
    //     //     }
    //     //     Op::End => {
    //     //         println!("Operación válida: true");
    //     //     }
    //     //     _ => {
    //     //         println!("Operación válida: false");
    //     //     }
    //     // }
    // }
}
