use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the configuration file
    // #[arg(short, long, value_name = "FILE")]
    // pub config: Option<PathBuf>,

    /// Source files to compile
    #[arg(value_name = "FILE")]
    pub input_files: Vec<PathBuf>,

    /// Initialize a default configuration file in the current directory
    #[arg(short = 'i', long = "init")]
    pub init: bool,
}
