#![allow(unused)]
use clap::Parser;
use console::strip_ansi_codes;
use console::style;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}


fn main() {
    let args = Cli::parse();
    let file = File::open(&args.path);
    let mut count = 1;
    if file.is_err() {
        println!("Error reading file, {}", args.path.display());
        return;
    } else {
        // lee el archivo
        let mut content = BufReader::new(file.unwrap());
        // lee el contenido del archivo
        let mut buffer: String = String::new();

        content.read_to_string(&mut buffer).unwrap();
        let mut lines = buffer.lines();
        let mut line = lines.next();
        let to_lower_case = args.pattern.as_str().to_lowercase();
        let to_upper_case = some_kind_of_uppercase_first_letter(&args.pattern);
        while line != None {
            // incluir también palabras que contengan el patron incluso si están capitalizadas
            let str_count = style(count.to_string()).cyan().bold().to_string();
            if line.as_ref().unwrap().contains(&to_lower_case) {
                println!(
                    "{str_count}: {}",
                    line.unwrap().replace(
                        &to_lower_case,
                        &style(&to_lower_case).green().bold().to_string()
                    )
                );
            } else if line.as_ref().unwrap().contains(&to_upper_case) {
                println!(
                    "{str_count}: {}",
                    line.unwrap()
                        .replace(&to_upper_case, &style(&to_upper_case).red().bold().to_string())
                );
            }
            line = lines.next();
            count += 1;
        }
        buffer.clear();
    }
}
