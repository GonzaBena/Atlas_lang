mod cli;
mod compiler;
mod utils;

use clap::Parser;
use cli::Args;
use compiler::ast::AST;
use compiler::{parser, parser::Operand as Op};
use utils::file_handling::File;
fn main() {
    let args = Args::parse();

    println!("Args: {:?}", args);

    // print my actual path
    let path = std::env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let file = File::open("./prueba/index.atl").unwrap();
    println!("Nombre: {}", file.get_name());
    println!("Tamaño: {}", file.get_size());
    println!("Extension: {}", file.get_extension());
    let content = file.get_content();
    // println!("Contenido: {}", content);

    for line in content.lines() {
        let ast = AST::from_expression(line).unwrap();

        let binding = ast.expresion().clone();
        let mut parser = parser::Parser::new(&binding);
        let result = parser.parse();
        let eq_result = result.resolve();

        println!("Resultado: {}\n\n", eq_result);
        match result {
            Op::Operation(o) => {
                println!("Operación válida: {}", o.is_valid());
                println!("Operación válida: {:#?}", o);
            }
            Op::End => {
                println!("Operación válida: true");
            }
            _ => {
                println!("Operación válida: false");
            }
        }
    }
}
