#![allow(unused)]
use clap::Parser;
#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("args: {:?}", args);
}
