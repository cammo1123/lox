use std::{ fmt::Error, fs::File, io::{self, Read, Write} };

use crate::{ast_printer::AstPrinter, error::had_error, parser::Parser, token::Token};
use crate::scanner::Scanner;


pub fn run_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    run(&contents)?;
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

    if let Some(expression) = parser.parse() {
        println!("{}", AstPrinter::new().print(&expression));
    }
    
    Ok(())
}

