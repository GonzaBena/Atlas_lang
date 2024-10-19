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
    let _args = Args::parse();

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
    for r in result.statements {
        println!("Resultado: {}", r.resolve());
    }
    println!("Tabla de identificadores: {:?}", result.identifier_table);
}
