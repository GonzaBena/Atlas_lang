pub mod format;

use std::process;

use colored::Colorize;

pub fn panic(msg: &str) {
    eprintln!("{}", msg.red());
    process::exit(1);
}
