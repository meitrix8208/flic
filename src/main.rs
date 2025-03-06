use clap::Parser;
use console::style;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,

    #[clap(short, long)]
    insensitive: bool,
}

fn main() {
    let args = Cli::parse();

    let file = File::open(&args.path);
    let mut count = 1;

    if file.is_err() {
        println!("Error reading file, {}", args.path.display());
        return;
    }

    let content = BufReader::new(file.unwrap());
    let regex_pattern = if args.insensitive {
        format!("(?i){}", regex::escape(&args.pattern))
    } else {
        regex::escape(&args.pattern)
    };
    let regex = Regex::new(&regex_pattern).unwrap();

    for line in content.lines() {
        let line = line.unwrap();
        let str_count = style(count).cyan().bold().to_string();
        let mut colored_line = line.to_string();

        for captures in regex.captures_iter(&line) {
            let pattern_match = captures.get(0).unwrap().as_str();
            let colored_pattern = if line == pattern_match {
                style(pattern_match).blue().bold()
            } else if args.insensitive && line.to_lowercase() == pattern_match.to_lowercase() {
                style(pattern_match).green().bold()
            } else if args.insensitive {
                style(pattern_match).yellow().bold()
            } else {
                style(pattern_match).red().bold()
            };
            colored_line = colored_line.replace(pattern_match, &colored_pattern.to_string());
        }

        if colored_line != line {
            println!("{str_count}: {colored_line}", str_count = str_count, colored_line = colored_line.trim());
        }
        count += 1;
    }
}
