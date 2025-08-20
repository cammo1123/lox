use std::fmt::Error;
use std::io::{self, Write};
use std::sync::{LazyLock, Mutex};
use std::{env, fs::File, io::Read, process::exit};

use crafting_interpreters::error::{had_error, had_runtime_error};
use crafting_interpreters::interpreter::Interpreter;
use crafting_interpreters::parser::Parser;
use crafting_interpreters::scanner::Scanner;
use crafting_interpreters::token::Token;

static INTERPRETER: LazyLock<Mutex<Interpreter>> = LazyLock::new(|| Mutex::new(Interpreter::new()));

pub fn run_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    run(&contents)?;

    if had_error() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Compilation errors in file '{}'", path)
        )));
    }

    if had_runtime_error() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Runtime error while running '{}'", path)
        )));
    }

    Ok(())
}

pub fn run_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    loop {
        print!("> ");
        stdout.flush()?;

        let mut line = String::new();
        let bytes = stdin.read_line(&mut line)?;

        if bytes == 0 {
            break;
        }

        let line = line.trim_end();
        if line == "exit" || line == "quit" {
            break;
        }

        if let Err(e) = run(line) {
            eprintln!("Error {}", e);
        }

    }
    
    Ok(())
}

fn run(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);

    if had_error() {
        return Err(Box::new(Error));
    }

    let mut interpreter = INTERPRETER.lock()?;
    if let Some(expression) = parser.parse() {
        interpreter.interpret(&expression);
    }
    
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            if let Err(e) = run_prompt() {
                eprintln!("REPL Error: {}", e);
                exit(70);
            }
        }

        2 => {
            if let Err(e) = run_file(&args[1]) {
                eprintln!("Error running file: {}", e);
                exit(65);
            }
        }

        _ => {
            println!("Usage: jlox [script]");
            exit(64);
        }
    }
}
