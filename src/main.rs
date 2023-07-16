#![allow(unused)]
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut content = BufReader::new(File::open(&args.path).unwrap());
    // let mut buffer = String::new();
    // content.read_to_string(&mut buffer).unwrap();
    // let mut lines = buffer.lines();
    // let mut line = lines.next();
    // while line != None {
    //     if line.as_ref().unwrap().contains(&args.pattern) {
    //         println!("{:?}", line.unwrap().trim());
    //     }
    //     line = lines.next();
    // }
    for line in content.lines() {
        if line.as_ref().unwrap().contains(&args.pattern) {
            println!("{:?}", line.unwrap().trim());
        }
    }
}
