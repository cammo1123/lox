use std::{env, process::exit};
use crafting_interpreters::run_file;
use crafting_interpreters::run_prompt;

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
