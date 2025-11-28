use std::{
    env,
    fs::{self},
    io,
    process::exit,
};
pub mod error;
pub mod token;
mod token_scanner;
use token_scanner::TokenScanner;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() > 1 {
        eprintln!("Usage: rlox [script]");
        exit(64);
    } else if args.len() == 1 {
        run_file(args.get(0).unwrap()); // no or because already checked if arg exists
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) {
    let file_contents: String =
        fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read file: {}", path));
    run(file_contents);
}

fn run_prompt() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

fn run(contents: String) {
    let mut scanner = TokenScanner::new(contents);
    while let Some(token) = scanner.next_token() {
        println!("{:?}", token);
    }
}
