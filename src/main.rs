use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout};
use std::process;

extern crate librlox;
use librlox::errors::ParseError;
use librlox::scanner;

type ParseResult<T> = Result<T, ParseError>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    match args_len {
        al if al > 2 => {
            println!("Usage: jlox [script]");
            process::exit(64);
        }
        2 => run_file(&args[1]).expect("Unable to parse file"),
        _ => run_prompt(),
    }
}

fn run_file(filename: &str) -> Result<(), String> {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_) => {
            run(contents).unwrap();
            Ok(())
        }
        Err(error) => Err(format!("error: {}", error)),
    }
}

fn run_prompt() {
    loop {
        let mut input = String::new();
        print!("> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut input).expect("execution error");
        run(input).unwrap();
    }
}

fn run(source: String) -> ParseResult<usize> {
    let s = scanner::Scanner::new(source);

    let tokens = s.scan_tokens().into_iter();
    let token_count = tokens.len();

    for token_result in tokens {
        match token_result {
            Ok(token) => println!("{}", token),
            Err(e) => println!("{}", e),
        }
    }

    Ok(token_count)
}
