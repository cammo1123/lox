use std::io::{self, Write};
use std::{env, fs::File, io::Read, process::exit};

use rlox::error::RLoxError;
use rlox::vm::VM;

pub fn repl() -> Result<(), Box<dyn std::error::Error>> {
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

        if let Err(e) = interpret(line) {
            eprintln!("Error {}", e);
        }

    }
    
    Ok(())
}

pub fn run_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
	match interpret(&contents) {
		Err(err) => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", err)))),
		_ => Ok(())
	}
}

fn interpret(source: &str) -> Result<(), RLoxError> {
    VM::interpret(source)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            if let Err(e) = repl() {
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