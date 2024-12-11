use atlas_lang::cli::Args;
use clap::Parser;

#[test]
fn creation_test() {
    let args = Args::parse();
    assert!(args.into_iter().len() > 0);
}
