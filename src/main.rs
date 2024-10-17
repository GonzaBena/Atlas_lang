mod cli;
mod compiler;
mod utils;

use clap::Parser;
use cli::Args;
use compiler::ast::AST;
use compiler::parser;
use utils::file_handling::File;
fn main() {
    let args = Args::parse();

    println!("Args: {:?}", args);

    let file = File::open("/home/atlas/workspace/atlas_test/index.atl").unwrap();
    println!("Nombre: {}", file.get_name());
    println!("Tamaño: {}", file.get_size());
    println!("Extension: {}", file.get_extension());
    let content = file.get_content();
    // println!("Contenido: {}", content);

    let ast = AST::from_expression(&content).unwrap();

    let binding = ast.expresion().clone();
    let mut parser = parser::Parser::new(&binding);
    let result = parser.parse();
    let eq_result = result.resolve();
    println!("Resultado: {}", eq_result);
}
