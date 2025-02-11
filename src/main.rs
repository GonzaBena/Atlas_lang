mod cli;
mod compiler;
mod std;
mod types;
mod utils;

use ::std::process;
use atlas_lang::compiler::{lexer::Lexer, parser};
use clap::Parser;
use cli::Args;
use utils::panic;

fn main() {
    let args = Args::parse();

    // Validación personalizada: si --y está presente, verificar que --init también lo esté
    if args.y && !args.init {
        eprintln!("Error: '--y' can only be used if '--init' is present.");
        process::exit(1);
    }

    let verify = args.verify();

    if let Err(error) = &verify {
        panic(&format!("{:?}", error));
    }

    let project = verify.unwrap();

    for file in project.files {
        let mut lex = Lexer::new(&file.content);
        let tokens = lex.lex();
        println!("Tokens: {:?}", tokens);

        let mut parser = parser::Parser::new(tokens, None, None);
        // println!("\n\nParser: {:?}", parser);
        let parse = parser.parse();
        // println!("\n\nParser: {:?}", parse);
        if parse.is_err() {
            panic(&format!("{:?}", parse.unwrap_err()));
        }
    }

    // println!("{:#?}", args)
}
