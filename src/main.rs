mod cli;
mod compiler;
mod error;
mod std;
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
    let tokens = lexer.tokenizer().unwrap();
    // println!("Tokens: {:#?}", tokens);

    let mut parser = Pars::new(&tokens);
    // println!("Parser: {:#?}", parser);
    let result = parser.parse();
    match result {
        Ok(r) => {
            for re in r.statements {
                println!("Resultado: {}", re.resolve(&r.identifier_table).unwrap());
            }
            println!("Tabla de identificadores: \n{}", r.identifier_table);
        }
        Err(e) => println!("Error: {:#?}", e),
    }
}
